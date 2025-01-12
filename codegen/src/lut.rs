use anyhow::Result;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::{codegen_file::CodegenFile, lut::model::LinearModel};

mod model;

pub fn generate() -> Result<()> {
    let mut file = CodegenFile::create("palette/src/encoding/lut/codegen.rs")?;

    let transfer_fn_u8 = vec![
        LutEntryU8::new(
            "srgb",
            "SRGB",
            TransferFn::new_with_linear(12.92, 0.0031308, 2.4),
        ),
        LutEntryU8::new(
            "rec_standards",
            "REC_OETF",
            TransferFn::new_with_linear(4.5, 0.018053968510807, 1.0 / 0.45),
        ),
        LutEntryU8::new(
            "adobe",
            "ADOBE_RGB",
            TransferFn::new_pure_gamma(563.0 / 256.0),
        ),
        LutEntryU8::new("p3", "P3_GAMMA", TransferFn::new_pure_gamma(2.6)),
    ];

    let transfer_fn_u16 = vec![LutEntryU16::new(
        "prophoto",
        "PROPHOTO_RGB",
        TransferFn::new_with_linear(16.0, 0.001953125, 1.8),
    )];

    for LutEntryU8 {
        module,
        fn_type_uppercase,
        transfer_fn,
    } in transfer_fn_u8
    {
        let u8_to_float = build_u8_to_float_lut(&fn_type_uppercase, &transfer_fn);
        let float_to_u8 = build_float_to_u8_lut(&fn_type_uppercase, &transfer_fn);

        file.append(quote! {
            pub mod #module {
                #u8_to_float

                #float_to_u8
            }
        })?;
    }

    for LutEntryU16 {
        module,
        fn_type_uppercase,
        transfer_fn,
    } in transfer_fn_u16
    {
        let u16_to_float = build_u16_to_float_lut(&fn_type_uppercase, &transfer_fn);
        let float_to_u8 = build_float_to_u16_lut(&fn_type_uppercase, &transfer_fn);

        file.append(quote! {
            #[cfg(feature = "gamma_lut_u16")]
            pub mod #module {
                #u16_to_float

                #float_to_u8
            }
        })?;
    }

    Ok(())
}

/// This struct is able to model a given transfer function.
///
/// Any transfer function will have a linear part (optional) for input values
/// less than some value `beta` and an exponential part determined by the function's
/// `gamma` value. For transfer functions with a linear part, `alpha` is chosen to
/// preserve function continuity.
struct TransferFn {
    into_linear: Box<dyn Fn(f64) -> f64>,
    linear_scale: Option<f64>,
    alpha: f64,
    beta: f64,
    gamma: f64,
}

impl TransferFn {
    fn new_with_linear(linear_scale: f64, linear_end: f64, gamma: f64) -> Self {
        let alpha = (linear_scale * linear_end - 1.0) / (linear_end.powf(gamma.recip()) - 1.0);
        let beta = linear_end;
        Self {
            into_linear: Box::new(move |encoded| {
                if encoded <= linear_scale * beta {
                    encoded / linear_scale
                } else {
                    ((encoded + alpha - 1.0) / alpha).powf(gamma)
                }
            }),
            linear_scale: Some(linear_scale),
            alpha,
            beta,
            gamma,
        }
    }

    fn new_pure_gamma(gamma: f64) -> Self {
        Self {
            into_linear: Box::new(move |encoded| encoded.powf(gamma)),
            linear_scale: None,
            alpha: 1.0,
            beta: 0.0,
            gamma,
        }
    }
}

struct LutEntryU8 {
    module: Ident,
    fn_type_uppercase: String,
    transfer_fn: TransferFn,
}

struct LutEntryU16 {
    module: Ident,
    fn_type_uppercase: String,
    transfer_fn: TransferFn,
}

impl LutEntryU8 {
    fn new(module: &str, fn_type_uppercase: &str, transfer_fn: TransferFn) -> Self {
        Self {
            module: format_ident!("{module}"),
            fn_type_uppercase: fn_type_uppercase.to_owned(),
            transfer_fn,
        }
    }
}

impl LutEntryU16 {
    fn new(module: &str, fn_type_uppercase: &str, transfer_fn: TransferFn) -> Self {
        Self {
            module: format_ident!("{module}"),
            fn_type_uppercase: fn_type_uppercase.to_owned(),
            transfer_fn,
        }
    }
}

fn build_u8_to_float_lut(fn_type_uppercase: &str, transfer_fn: &TransferFn) -> TokenStream {
    let table = (0..=u8::MAX).map(|i| (transfer_fn.into_linear)((i as f64) / 255.0));
    let table_ident = format_ident!("{fn_type_uppercase}_U8_TO_F64");
    let table_f32 = table.clone().map(|f| f as f32);
    let table_f32_ident = format_ident!("{fn_type_uppercase}_U8_TO_F32");
    quote! {
        pub const #table_ident: [f64; 256] = [
            #(#table),*
        ];

        pub const #table_f32_ident: [f32; 256] = [
            #(#table_f32),*
        ];
    }
}

fn build_u16_to_float_lut(fn_type_uppercase: &str, transfer_fn: &TransferFn) -> TokenStream {
    let table = (0..=u16::MAX).map(|i| (transfer_fn.into_linear)((i as f64) / 65535.0));
    let table_ident = format_ident!("{fn_type_uppercase}_U16_TO_F64");
    quote! {
        pub static #table_ident: [f64; 65536] = [
            #(#table),*
        ];
    }
}

/// This algorithm is an adaptation of [this C++ code](<https://gist.github.com/rygorous/2203834>)
/// by Fabian "ryg" Giesen, which utilizes simple linear regression on
/// sub-intervals of the transfer function's domain and stores the resulting
/// models' scales and biases into a lookup table.
///
/// The algorithm linked above calculates the transfer function for every
/// potential `f32` input and feeds that into the regression model. In
/// contrast, this algorithm replaces the discrete sums in the model with
/// continuous integrals in order to reduce the time it takes to generate
/// the tables. We are able to do this since transfer functions follow a
/// predictable pattern for which the anti-derivative is known.
fn build_float_to_u8_lut(fn_type_uppercase: &str, transfer_fn: &TransferFn) -> TokenStream {
    // 1.0 - f32::EPSILON
    const MAX_FLOAT_BITS: u32 = 0x3f7fffff;
    // The number of mantissa bits used to index into the lookup table
    const MAN_INDEX_WIDTH: u32 = 3;
    // The number of bits in the remainder of the mantissa
    const BUCKET_INDEX_WIDTH: u32 = 20;
    const BUCKET_SIZE: u32 = 1 << BUCKET_INDEX_WIDTH;
    // Any input less than or equal to this maps to 0
    let min_float_bits =
        (((transfer_fn.into_linear)(0.5 / 255.0) as f32).to_bits() - 1) & 0xff800000;

    let exp_table_size = ((MAX_FLOAT_BITS - min_float_bits) >> 23) + 1;
    let table_size = exp_table_size << MAN_INDEX_WIDTH;

    let table = (0..table_size).map(|i| {
        let start = min_float_bits + (i << BUCKET_INDEX_WIDTH);
        let end = start + BUCKET_SIZE;

        LinearModel::new(transfer_fn, start, end, MAN_INDEX_WIDTH, 8).into_u8_lookup()
    });

    let table_ident = format_ident!("TO_{fn_type_uppercase}_U8");
    let table_size_usize = table_size as usize;

    let float_const_ident = format_ident!("{fn_type_uppercase}_MIN_FLOAT");
    quote! {
        pub const #float_const_ident: u32 = #min_float_bits;

        pub const #table_ident: [u32; #table_size_usize] = [
            #(#table),*
        ];
    }
}

fn build_float_to_u16_lut(fn_type_uppercase: &str, transfer_fn: &TransferFn) -> TokenStream {
    // 1.0 - f32::EPSILON
    const MAX_FLOAT_BITS: u32 = 0x3f7fffff;
    // The number of mantissa bits used to index into the lookup table
    const MAN_INDEX_WIDTH: u32 = 7;
    // The number of bits in the remainder of the mantissa
    const BUCKET_INDEX_WIDTH: i32 = 16;
    const BUCKET_SIZE: u32 = 1 << BUCKET_INDEX_WIDTH;
    let TransferFn {
        into_linear,
        linear_scale,
        beta,
        ..
    } = transfer_fn;
    let min_float_bits = (*beta as f32)
        .to_bits()
        .max((into_linear(0.5 / 65535.0) as f32).to_bits() - 1)
        & 0xff800000;
    let exp_table_size = ((MAX_FLOAT_BITS - min_float_bits) >> 23) + 1;
    let table_size = exp_table_size << MAN_INDEX_WIDTH;
    let table = (0..table_size).map(|i| {
        let start = min_float_bits + (i << BUCKET_INDEX_WIDTH);
        let end = start + BUCKET_SIZE;

        LinearModel::new(transfer_fn, start, end, MAN_INDEX_WIDTH, 16).into_u16_lookup()
    });

    let table_ident = format_ident!("TO_{fn_type_uppercase}_U16");
    let table_size_usize = table_size as usize;
    let linear_scale = 65535.0 * (linear_scale.unwrap_or_default() as f32);

    let float_const_ident = format_ident!("{fn_type_uppercase}_MIN_FLOAT");
    let linear_scale_ident = format_ident!("{fn_type_uppercase}_LINEAR_SCALE");
    quote! {
        pub const #float_const_ident: u32 = #min_float_bits;
        pub const #linear_scale_ident: f32 = #linear_scale;

        pub const #table_ident: [u64; #table_size_usize] = [
            #(#table),*
        ];
    }
}
