use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_quote, DeriveInput, Type};

use crate::meta::{
    parse_field_attributes, parse_namespaced_attributes, FieldAttributes, TypeItemAttributes,
};

use palette_codegen::color_types::XYZ_COLORS;

pub fn derive(item: TokenStream) -> ::std::result::Result<TokenStream, Vec<::syn::parse::Error>> {
    let DeriveInput {
        ident,
        generics,
        data,
        attrs,
        ..
    } = syn::parse(item).map_err(|error| vec![error])?;

    let (mut item_meta, item_errors) = parse_namespaced_attributes::<TypeItemAttributes>(attrs);
    let palette_name = item_meta.get_palette_name();

    let (fields_meta, field_errors) = if let syn::Data::Struct(struct_data) = data {
        parse_field_attributes::<FieldAttributes>(struct_data.fields)
    } else {
        return Err(vec![syn::Error::new(
            Span::call_site(),
            "only structs are supported",
        )]);
    };

    // Assume conversion from the root type (Xyz for the base group) by default
    if item_meta.color_groups.is_empty() {
        item_meta.color_groups.insert((&XYZ_COLORS).into());
    }

    if item_meta.skip_derives.is_empty() {
        for group in &item_meta.color_groups {
            item_meta.skip_derives.insert(group.root_type.name.into());
        }
    }

    let mut struct_info = palette_codegen::from_color_unclamped::StructInfo::new(
        ident,
        component_type(item_meta.component.clone()),
        item_meta.color_meta,
    );
    struct_info.generics = generics;
    struct_info.alpha_field = fields_meta.alpha_property;
    struct_info.skip_derives = item_meta.skip_derives;
    struct_info.color_groups = item_meta.color_groups;

    let item_errors = item_errors
        .into_iter()
        .map(|error| error.into_compile_error());
    let field_errors = field_errors
        .into_iter()
        .map(|error| error.into_compile_error());

    let implementation = palette_codegen::from_color_unclamped::derive(&struct_info, &palette_name);

    Ok(quote! {
        #(#item_errors)*
        #(#field_errors)*

        #implementation
    }
    .into())
}

pub fn component_type(component: Option<Type>) -> Type {
    component.unwrap_or_else(|| parse_quote!(f32))
}
