use anyhow::Result;
use palette_math::gamma::lut::GammaLutBuilder;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::codegen_file::CodegenFile;

pub fn generate() -> Result<()> {
    let mut file = CodegenFile::create("palette/src/encoding/lut/codegen.rs")?;

    let transfer_fn_u8 = vec![
        LutEntryU8 {
            module: format_ident!("srgb"),
            fn_type_uppercase: "SRGB",
            transfer_fn: palette_math::gamma::srgb_lut_builder(),
        },
        LutEntryU8 {
            module: format_ident!("rec_standards"),
            fn_type_uppercase: "REC_OETF",
            transfer_fn: palette_math::gamma::rec_oetf_builder(),
        },
        LutEntryU8 {
            module: format_ident!("adobe"),
            fn_type_uppercase: "ADOBE_RGB",
            transfer_fn: palette_math::gamma::adobe_rgb_builder(),
        },
        LutEntryU8 {
            module: format_ident!("p3"),
            fn_type_uppercase: "P3_GAMMA",
            transfer_fn: palette_math::gamma::p3_builder(),
        },
    ];

    for entry in transfer_fn_u8 {
        let u8_to_float = build_u8_to_float_lut(&entry);
        let float_to_u8 = build_float_to_u8_lut(&entry);

        let module = entry.module;
        file.append(quote! {
            pub mod #module {
                #u8_to_float

                #float_to_u8
            }
        })?;
    }

    Ok(())
}

struct LutEntryU8 {
    module: Ident,
    fn_type_uppercase: &'static str,
    transfer_fn: GammaLutBuilder,
}

fn build_u8_to_float_lut(entry: &LutEntryU8) -> TokenStream {
    let fn_type_uppercase = entry.fn_type_uppercase;

    let table = entry.transfer_fn.u8_to_linear_entries();
    let table_ident = format_ident!("{fn_type_uppercase}_U8_TO_F64");
    let table_ty = quote! {palette_math::lut::Lut<u8, f64, palette_math::lut::ArrayTable<256>>};

    let table_f32 = entry.transfer_fn.u8_to_linear_entries().map(|f| f as f32);
    let table_f32_ident = format_ident!("{fn_type_uppercase}_U8_TO_F32");
    let table_f32_ty = quote! {palette_math::lut::Lut<u8, f32, palette_math::lut::ArrayTable<256>>};

    quote! {
        pub(crate) const #table_ident: #table_ty = palette_math::lut::Lut::new([
            #(#table),*
        ]);

        pub(crate) const #table_f32_ident: #table_f32_ty = palette_math::lut::Lut::new([
            #(#table_f32),*
        ]);
    }
}

/// See `palette_math::lut::GammaLutBuilder::linear_to_u8_entries` for details.
fn build_float_to_u8_lut(entry: &LutEntryU8) -> TokenStream {
    let fn_type_uppercase = entry.fn_type_uppercase;

    // Any input less than or equal to this maps to 0
    let min_float_bits = entry.transfer_fn.linear_to_u8_min_float_bits();
    let linear_slope = entry.transfer_fn.linear_to_u8_linear_slope();

    let table = entry.transfer_fn.linear_to_u8_entries();

    let table_ident = format_ident!("{fn_type_uppercase}_F32_TO_U8");
    let table_size_usize = table.len();

    let table_ty = quote! {palette_math::gamma::lut::GammaLut<f32, u8, palette_math::lut::ArrayTable<#table_size_usize>>};
    quote! {
        // SAFETY: Generated from a builder for the transfer function's gamma curve.
        pub const #table_ident: #table_ty = unsafe { palette_math::gamma::lut::GammaLut::from_parts(
            #min_float_bits,
            #linear_slope,
            [
                #(#table),*
            ]
        ) };
    }
}
