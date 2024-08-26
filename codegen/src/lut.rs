use anyhow::Result;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::codegen_file::CodegenFile;

pub fn generate() -> Result<()> {
    let mut file = CodegenFile::create("palette/src/encoding/lut/codegen.rs")?;

    let transfer_fn_u8 = vec![
        LutEntryU8::new("Srgb", "SRGB", Some((12.92, 0.0031308)), 2.4),
        LutEntryU8::new(
            "RecOetf",
            "REC_OETF",
            Some((4.5, 0.018053968510807)),
            1.0 / 0.45,
        ),
        LutEntryU8::new("AdobeRgb", "ADOBE_RGB", None, 563.0 / 256.0),
        LutEntryU8::new("P3Gamma", "P3_GAMMA", None, 2.6),
    ];

    let transfer_fn_u16 = vec![LutEntryU16::new(
        "ProPhotoRgb",
        "PROPHOTO_RGB",
        Some((16.0, 0.001953125)),
        1.8,
    )];

    let ident_u8 = transfer_fn_u8
        .iter()
        .map(|LutEntryU8 { fn_type, .. }| fn_type);
    file.append(quote! {
        use crate::encoding::{lut, FromLinear, IntoLinear, #(#ident_u8),*};
    })?;

    let ident_u16 = transfer_fn_u16
        .iter()
        .map(|LutEntryU16 { fn_type, .. }| fn_type);
    file.append(quote! {
        #[cfg(feature = "gamma_lut_u16")]
        use crate::encoding::{#(#ident_u16),*};
    })?;

    file.append(build_u8_to_f64_lut(&transfer_fn_u8))?;
    file.append(build_u16_to_f64_lut(&transfer_fn_u16))?;
    file.append(build_f32_to_u8_lut(&transfer_fn_u8))?;
    file.append(build_f32_to_u16_lut(&transfer_fn_u16))?;

    Ok(())
}

type TransferFn = Box<dyn Fn(f64) -> f64>;

struct LutEntryU8 {
    fn_type: Ident,
    fn_type_uppercase: String,
    into_linear: TransferFn,
    linear_scale: Option<f64>,
    alpha: f64,
    beta: f64,
    gamma: f64,
}

struct LutEntryU16 {
    fn_type: Ident,
    fn_type_uppercase: String,
    into_linear: TransferFn,
    linear_scale: Option<f64>,
    alpha: f64,
    beta: f64,
    gamma: f64,
}

impl LutEntryU8 {
    fn new(
        fn_type: &str,
        fn_type_uppercase: &str,
        is_linear_as_until: Option<(f64, f64)>,
        gamma: f64,
    ) -> Self {
        let (linear_scale, alpha, beta) =
            if let Some((linear_scale, linear_end)) = is_linear_as_until {
                (
                    Some(linear_scale),
                    (linear_scale * linear_end - 1.0) / (linear_end.powf(gamma.recip()) - 1.0),
                    linear_end,
                )
            } else {
                (None, 1.0, 0.0)
            };
        Self {
            fn_type: format_ident!("{fn_type}"),
            fn_type_uppercase: fn_type_uppercase.to_owned(),
            into_linear: Box::new(move |encoded| match linear_scale {
                Some(scale) if encoded <= scale * beta => encoded / scale,
                _ => ((encoded + alpha - 1.0) / alpha).powf(gamma),
            }),
            linear_scale,
            alpha,
            beta,
            gamma,
        }
    }
}

impl LutEntryU16 {
    fn new(
        fn_type: &str,
        fn_type_uppercase: &str,
        is_linear_as_until: Option<(f64, f64)>,
        gamma: f64,
    ) -> Self {
        let (linear_scale, alpha, beta) =
            if let Some((linear_scale, linear_end)) = is_linear_as_until {
                (
                    Some(linear_scale),
                    (linear_scale * linear_end - 1.0) / (linear_end.powf(gamma.recip()) - 1.0),
                    linear_end,
                )
            } else {
                (None, 1.0, 0.0)
            };
        Self {
            fn_type: format_ident!("{fn_type}"),
            fn_type_uppercase: fn_type_uppercase.to_owned(),
            into_linear: Box::new(move |encoded| match linear_scale {
                Some(scale) if encoded <= scale * beta => encoded / scale,
                _ => ((encoded + alpha - 1.0) / alpha).powf(gamma),
            }),
            linear_scale,
            alpha,
            beta,
            gamma,
        }
    }
}

fn build_u8_to_f64_lut(entries: &[LutEntryU8]) -> TokenStream {
    let tables = entries.iter().map(
        |LutEntryU8 {
             fn_type,
             fn_type_uppercase,
             into_linear,
             ..
         }| {
            let table = (0..=u8::MAX).map(|i| into_linear((i as f64) / 255.0));
            let table_ident = format_ident!("{fn_type_uppercase}_U8_TO_F64");
            quote! {
                const #table_ident: [f64; 256] = [
                    #(#table),*
                ];

                impl IntoLinear<f64, u8> for #fn_type {
                    #[inline]
                    fn into_linear(encoded: u8) -> f64 {
                        #table_ident[encoded as usize]
                    }
                }

                impl IntoLinear<f32, u8> for #fn_type {
                    #[inline]
                    fn into_linear(encoded: u8) -> f32 {
                        #table_ident[encoded as usize] as f32
                    }
                }
            }
        },
    );

    quote! {
        #(#tables)*
    }
}

fn build_u16_to_f64_lut(entries: &[LutEntryU16]) -> TokenStream {
    let tables = entries.iter().map(
        |LutEntryU16 {
             fn_type,
             fn_type_uppercase,
             into_linear,
             ..
         }| {
            let table = (0..=u16::MAX).map(|i| into_linear((i as f64) / 65535.0));
            let table_ident = format_ident!("{fn_type_uppercase}_U16_TO_F64");
            quote! {
                #[cfg(feature = "gamma_lut_u16")]
                static #table_ident: [f64; 65536] = [
                    #(#table),*
                ];

                #[cfg(feature = "gamma_lut_u16")]
                impl IntoLinear<f64, u16> for #fn_type {
                    #[inline]
                    fn into_linear(encoded: u16) -> f64 {
                        #table_ident[encoded as usize]
                    }
                }

                #[cfg(feature = "gamma_lut_u16")]
                impl IntoLinear<f32, u16> for #fn_type {
                    #[inline]
                    fn into_linear(encoded: u16) -> f32 {
                        #table_ident[encoded as usize] as f32
                    }
                }
            }
        },
    );

    quote! {
        #(#tables)*
    }
}

fn integrate_linear(
    (start_x, start_t): (f64, f64),
    (end_x, end_t): (f64, f64),
    linear_scale: f64,
    exp_scale: f64,
) -> (f64, f64) {
    let antiderive_y = |x: f64| 0.5 * linear_scale * exp_scale * x * x;
    let antiderive_ty =
        |x: f64, t: f64| 0.5 * linear_scale * exp_scale * x * x * (t - exp_scale * x / 3.0);

    (
        antiderive_y(end_x) - antiderive_y(start_x),
        antiderive_ty(end_x, end_t) - antiderive_ty(start_x, start_t),
    )
}

fn integrate_exponential(
    (start_x, start_t): (f64, f64),
    (end_x, end_t): (f64, f64),
    alpha: f64,
    gamma: f64,
    exp_scale: f64,
) -> (f64, f64) {
    let antiderive_y = |x: f64, t: f64| {
        alpha * gamma * exp_scale * x * x.powf(gamma.recip()) / (1.0 + gamma) + (1.0 - alpha) * t
    };
    let antiderive_ty = |x: f64, t: f64| {
        alpha
            * gamma
            * exp_scale
            * x
            * x.powf(gamma.recip())
            * (t - gamma * exp_scale * x / (1.0 + 2.0 * gamma))
            / (1.0 + gamma)
            + 0.5 * (1.0 - alpha) * t * t
    };

    (
        antiderive_y(end_x, end_t) - antiderive_y(start_x, start_t),
        antiderive_ty(end_x, end_t) - antiderive_ty(start_x, start_t),
    )
}

fn build_f32_to_u8_lut(entries: &[LutEntryU8]) -> TokenStream {
    // 1.0 - f32::EPSILON
    const MAX_FLOAT_BITS: u32 = 0x3f7fffff;
    // The number of mantissa bits used to index into the lookup table
    const MAN_INDEX_WIDTH: u32 = 3;
    // The number of bits in the remainder of the mantissa
    const BUCKET_INDEX_WIDTH: i32 = 20;
    const BUCKET_SIZE: u32 = 1 << BUCKET_INDEX_WIDTH;
    let tables = entries.iter().map(
        |LutEntryU8 {
             fn_type,
             fn_type_uppercase,
             into_linear,
             linear_scale,
             alpha,
             beta,
             gamma,
         }| {
            // Any input less than or equal to this maps to 0
            let min_float_bits = ((into_linear(0.5 / 255.0) as f32).to_bits() - 1) & 0xff800000;

            let exp_table_size = ((MAX_FLOAT_BITS - min_float_bits) >> 23) + 1;
            let table_size = exp_table_size << MAN_INDEX_WIDTH;

            let table = (0..table_size).map(|i| {
                let start = min_float_bits + (i << BUCKET_INDEX_WIDTH);
                let end = start + BUCKET_SIZE;
                let start_x = f32::from_bits(start) as f64;
                let end_x = f32::from_bits(end) as f64;

                let beta_bits = (*beta as f32).to_bits();
                let exp_scale = 2.0f64.powi(158 - ((start >> 23) as i32) - BUCKET_INDEX_WIDTH);

                let (integral_y, integral_ty) = match linear_scale {
                    Some(linear_scale) if end <= beta_bits => {
                        integrate_linear((start_x, 0.0), (end_x, 256.0), *linear_scale, exp_scale)
                    }
                    Some(linear_scale) if start < beta_bits => {
                        let beta_t = (beta_bits & (BUCKET_SIZE - 1)) as f64
                            * 2.0f64.powi(8 - BUCKET_INDEX_WIDTH);
                        let integral_linear = integrate_linear(
                            (start_x, 0.0),
                            (*beta, beta_t),
                            *linear_scale,
                            exp_scale,
                        );
                        let integral_exponential = integrate_exponential(
                            (*beta, beta_t),
                            (end_x, 256.0),
                            *alpha,
                            *gamma,
                            exp_scale,
                        );
                        (
                            integral_linear.0 + integral_exponential.0,
                            integral_linear.1 + integral_exponential.1,
                        )
                    }
                    _ => integrate_exponential(
                        (start_x, 0.0),
                        (end_x, 256.0),
                        *alpha,
                        *gamma,
                        exp_scale,
                    ),
                };

                const INTEGRAL_T: f64 = 32768.0;
                const INTEGRAL_T2: f64 = 16777216.0 / 3.0;

                let scale = (256.0 * integral_ty - INTEGRAL_T * integral_y)
                    / (256.0 * INTEGRAL_T2 - INTEGRAL_T * INTEGRAL_T);
                let bias = (integral_y - scale * INTEGRAL_T) / 256.0;
                let scale_uint = (255.0 * scale * 65536.0 + 0.5) as u32;
                let bias_uint = (((255.0 * bias + 0.5) * 128.0 + 0.5) as u32) << 9;
                (bias_uint << 7) | scale_uint
            });

            let table_ident = format_ident!("TO_{fn_type_uppercase}_U8");
            let table_size_usize = table_size as usize;
            quote! {
                const #table_ident: [u32; #table_size_usize] = [
                    #(#table),*
                ];

                impl FromLinear<f64, u8> for #fn_type {
                    #[inline]
                    fn from_linear(linear: f64) -> u8 {
                        <#fn_type>::from_linear(linear as f32)
                    }
                }

                impl FromLinear<f32, u8> for #fn_type {
                    #[inline]
                    fn from_linear(linear: f32) -> u8 {
                        lut::linear_f32_to_encoded_u8(linear, #min_float_bits, &#table_ident)
                    }
                }
            }
        },
    );

    quote! {
        #(#tables)*
    }
}

fn build_f32_to_u16_lut(entries: &[LutEntryU16]) -> TokenStream {
    // 1.0 - f32::EPSILON
    const MAX_FLOAT_BITS: u32 = 0x3f7fffff;
    // The number of mantissa bits used to index into the lookup table
    const MAN_INDEX_WIDTH: u32 = 7;
    // The number of bits in the remainder of the mantissa
    const BUCKET_INDEX_WIDTH: i32 = 16;
    const BUCKET_SIZE: u32 = 1 << BUCKET_INDEX_WIDTH;
    let tables = entries.iter().map(
        |LutEntryU16 {
             fn_type,
             fn_type_uppercase,
             into_linear,
             linear_scale,
             alpha,
             beta,
             gamma,
         }| {
            let min_float_bits = (*beta as f32)
                .to_bits()
                .max((into_linear(0.5 / 65535.0) as f32).to_bits() - 1)
                & 0xff800000;
            let exp_table_size = ((MAX_FLOAT_BITS - min_float_bits) >> 23) + 1;
            let table_size = exp_table_size << MAN_INDEX_WIDTH;
            let table = (0..table_size).map(|i| {
                let start = min_float_bits + (i << BUCKET_INDEX_WIDTH);
                let end = start + BUCKET_SIZE;
                let start_x = f32::from_bits(start) as f64;
                let end_x = f32::from_bits(end) as f64;

                let beta_bits = (*beta as f32).to_bits();
                let exp_scale = 2.0f64.powi(166 - ((start >> 23) as i32) - BUCKET_INDEX_WIDTH);

                let (integral_y, integral_ty) = match linear_scale {
                    Some(linear_scale) if end <= beta_bits => {
                        integrate_linear((start_x, 0.0), (end_x, 65536.0), *linear_scale, exp_scale)
                    }
                    Some(linear_scale) if start < beta_bits => {
                        let beta_t = (beta_bits & (BUCKET_SIZE - 1)) as f64
                            * 2.0f64.powi(16 - BUCKET_INDEX_WIDTH);
                        let integral_linear =
                            integrate_linear((start_x, 0.0), (*beta, beta_t), *linear_scale, exp_scale);
                        let integral_exponential = integrate_exponential(
                            (*beta, beta_t),
                            (end_x, 65536.0),
                            *alpha,
                            *gamma,
                            exp_scale,
                        );
                        (
                            integral_linear.0 + integral_exponential.0,
                            integral_linear.1 + integral_exponential.1,
                        )
                    }
                    _ => integrate_exponential(
                        (start_x, 0.0),
                        (end_x, 65536.0),
                        *alpha,
                        *gamma,
                        exp_scale,
                    ),
                };

                const INTEGRAL_T: f64 = 2147483648.0;
                const INTEGRAL_T2: f64 = 281474976710656.0 / 3.0;

                let scale = (65536.0 * integral_ty - INTEGRAL_T * integral_y)
                    / (65536.0 * INTEGRAL_T2 - INTEGRAL_T * INTEGRAL_T);
                let bias = (integral_y - scale * INTEGRAL_T) / 65536.0;
                let scale_uint = (65535.0 * scale * 4294967296.0 + 0.5) as u64;
                let bias_uint = (((65535.0 * bias + 0.5) * 32768.0 + 0.5) as u64) << 17;
                (bias_uint << 15) | scale_uint
            });

            let table_ident = format_ident!("TO_{fn_type_uppercase}_U16");
            let table_size_usize = table_size as usize;
            let linear_scale = 65535.0 * (linear_scale.unwrap_or_default() as f32);
            quote! {
                #[cfg(feature = "gamma_lut_u16")]
                const #table_ident: [u64; #table_size_usize] = [
                    #(#table),*
                ];

                #[cfg(feature = "gamma_lut_u16")]
                impl FromLinear<f64, u16> for #fn_type {
                    #[inline]
                    fn from_linear(linear: f64) -> u16 {
                        <#fn_type>::from_linear(linear as f32)
                    }
                }

                #[cfg(feature = "gamma_lut_u16")]
                impl FromLinear<f32, u16> for #fn_type {
                    #[inline]
                    fn from_linear(linear: f32) -> u16 {
                        lut::linear_f32_to_encoded_u16_with_linear_scale(linear, #linear_scale, #min_float_bits, &#table_ident)
                    }
                }
            }
        },
    );

    quote! {
        #(#tables)*
    }
}
