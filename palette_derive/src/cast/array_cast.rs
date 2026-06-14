use palette_codegen::util::IdentOrIndex;
use proc_macro::TokenStream;
use proc_macro2::Span;

use quote::{quote, ToTokens};
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Data, DeriveInput, Fields, Meta, Path, Type,
};

use crate::meta::{self, FieldAttributes};
use crate::util;

pub fn derive(tokens: TokenStream) -> std::result::Result<TokenStream, Vec<syn::Error>> {
    let DeriveInput {
        ident,
        attrs,
        generics,
        data,
        ..
    } = syn::parse(tokens).map_err(|error| vec![error])?;

    let allowed_repr = is_allowed_repr(&attrs)?;

    let mut number_of_channels = 0usize;
    let mut field_type: Option<Type> = None;

    let (all_fields, fields_meta, field_errors) = match data {
        Data::Struct(struct_item) => {
            let (fields_meta, field_errors) =
                meta::parse_field_attributes::<FieldAttributes>(struct_item.fields.clone());
            let all_fields = match struct_item.fields {
                Fields::Named(fields) => fields.named,
                Fields::Unnamed(fields) => fields.unnamed,
                Fields::Unit => Default::default(),
            };

            (all_fields, fields_meta, field_errors)
        }
        Data::Enum(_) => {
            return Err(vec![syn::Error::new(
                Span::call_site(),
                "`ArrayCast` cannot be derived for enums",
            )]);
        }
        Data::Union(_) => {
            return Err(vec![syn::Error::new(
                Span::call_site(),
                "`ArrayCast` cannot be derived for unions",
            )]);
        }
    };

    let fields = all_fields
        .into_iter()
        .enumerate()
        .map(|(index, field)| {
            (
                field
                    .ident
                    .map(IdentOrIndex::Ident)
                    .unwrap_or_else(|| IdentOrIndex::Index(index.into())),
                field.ty,
            )
        })
        .filter(|(field, _)| !fields_meta.zero_size_fields.contains(field));

    let mut errors = Vec::new();

    for (field, ty) in fields {
        let ty = fields_meta
            .type_substitutes
            .get(&field)
            .cloned()
            .unwrap_or(ty);
        number_of_channels += 1;

        if let Some(field_type) = field_type.clone() {
            if field_type != ty {
                errors.push(syn::Error::new_spanned(
                    &field,
                    format!(
                        "expected fields to have type `{}`",
                        field_type.into_token_stream()
                    ),
                ));
            }
        } else {
            field_type = Some(ty);
        }
    }

    if !allowed_repr {
        errors.push(syn::Error::new(
            Span::call_site(),
            format!("a `#[repr(C)]` or `#[repr(transparent)]` attribute is required to give `{ident}` a fixed memory layout"),
        ));
    }

    let mut implementation = if let Some(field_type) = field_type {
        let palette_name = util::find_crate_name();

        let mut struct_info =
            palette_codegen::array_cast::StructInfo::new(ident, field_type, number_of_channels);
        struct_info.generics = generics;

        // SAFETY: The invariants of `ArrayCast` are checked by this derive macro:
        // * Only structs with at least one field are allowed
        // * Must be `repr(C)` or `repr(transparent)`
        // * All fields must have the same type, which becomes the array item type
        unsafe { palette_codegen::array_cast::derive(&struct_info, &palette_name) }
    } else {
        errors.push(syn::Error::new(
            Span::call_site(),
            "`ArrayCast` can only be derived for structs with one or more fields".to_string(),
        ));

        return Err(errors);
    };

    implementation.extend(errors.iter().map(syn::Error::to_compile_error));

    let field_errors = field_errors
        .into_iter()
        .map(|error| error.into_compile_error());

    Ok(quote! {
        #(#field_errors)*

        #implementation
    }
    .into())
}

fn is_allowed_repr(attributes: &[Attribute]) -> std::result::Result<bool, Vec<syn::Error>> {
    let mut errors = Vec::new();

    for attribute in attributes {
        let attribute_name = attribute.path().get_ident().map(ToString::to_string);

        if let Some("repr") = attribute_name.as_deref() {
            let meta_list = match attribute.meta.require_list() {
                Ok(list) => list,
                Err(error) => {
                    errors.push(error);
                    continue;
                }
            };

            let items = match meta_list.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated)
            {
                Ok(items) => items,
                Err(error) => {
                    errors.push(error);
                    continue;
                }
            };

            let contains_allowed_repr = items.iter().any(|item| {
                item.require_path_only()
                    .ok()
                    .and_then(Path::get_ident)
                    .is_some_and(|ident| ident == "C" || ident == "transparent")
            });

            if contains_allowed_repr {
                return Ok(true);
            }
        }
    }

    if errors.is_empty() {
        Ok(false)
    } else {
        Err(errors)
    }
}
