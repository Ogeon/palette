use proc_macro::TokenStream;
use proc_macro2::Span;

use quote::quote;
use syn::DeriveInput;

use crate::{
    meta::{parse_field_attributes, FieldAttributes},
    util,
};

pub fn derive(item: TokenStream) -> ::std::result::Result<TokenStream, Vec<::syn::parse::Error>> {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = syn::parse(item).map_err(|error| vec![error])?;

    let palette_name = util::find_crate_name();

    let (fields_meta, field_errors) = if let syn::Data::Struct(struct_data) = data {
        parse_field_attributes::<FieldAttributes>(struct_data.fields)
    } else {
        return Err(vec![syn::Error::new(
            Span::call_site(),
            "only structs are supported",
        )]);
    };

    let mut struct_info = palette_codegen::with_alpha::StructInfo::new(ident);
    struct_info.generics = generics;
    struct_info.alpha_field = fields_meta.alpha_property;

    let implementation = palette_codegen::with_alpha::derive(&struct_info, &palette_name);

    let field_errors = field_errors
        .into_iter()
        .map(|error| error.into_compile_error());

    Ok(quote! {
        #(#field_errors)*

        #implementation
    }
    .into())
}
