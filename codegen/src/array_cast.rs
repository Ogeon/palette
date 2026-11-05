use std::collections::BTreeMap;

use anyhow::Result;
use palette_codegen::array_cast::{self, StructInfo, StructInfoError};
use proc_macro2::{Ident, Span};
use quote::quote;

use crate::codegen_file::CodegenFile;

pub fn generate() -> Result<()> {
    let palette_name = Ident::new("crate", Span::call_site());
    let mut color_per_module: BTreeMap<&'static str, Vec<StructInfo>> = BTreeMap::new();

    for &color in palette_codegen::color_types::COLORS {
        let struct_info = match StructInfo::try_from(color) {
            Ok(struct_info) => struct_info,
            Err(StructInfoError::CannotImplement) => {
                continue;
            }
            Err(_) => panic!("unhandled error"),
        };

        color_per_module
            .entry(color.module)
            .or_default()
            .push(struct_info);
    }

    for (module, colors) in color_per_module {
        let mut file = CodegenFile::create(format!("palette/src/{module}/codegen_array_cast.rs"))?;

        let color_names = colors.iter().map(|color| &color.name);
        file.append(quote! {use super::{#(#color_names),*};})?;

        for struct_info in colors {
            // SAFETY: The struct info is generated from type info, which
            // should be kept up to date with the actual struct.
            let tokens = unsafe { array_cast::derive(&struct_info, &palette_name) };
            file.append(tokens)?;
        }
    }

    Ok(())
}
