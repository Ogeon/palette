use std::collections::BTreeMap;

use anyhow::Result;
use palette_codegen::with_alpha::{self, StructInfo};
use proc_macro2::{Ident, Span};
use quote::quote;

use crate::codegen_file::CodegenFile;

pub fn generate() -> Result<()> {
    let palette_name = Ident::new("crate", Span::call_site());
    let mut color_per_module: BTreeMap<&'static str, Vec<StructInfo>> = BTreeMap::new();

    for &color in palette_codegen::color_types::COLORS {
        color_per_module
            .entry(color.module)
            .or_default()
            .push(StructInfo::from(color));
    }

    for (module, colors) in color_per_module {
        let mut file = CodegenFile::create(format!("palette/src/{module}/codegen_with_alpha.rs"))?;

        let color_names = colors.iter().map(|color| &color.name);
        file.append(quote! {use super::{#(#color_names),*};})?;

        for struct_info in colors {
            file.append(with_alpha::derive(&struct_info, &palette_name))?;
        }
    }

    Ok(())
}
