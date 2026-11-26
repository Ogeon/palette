use palette_codegen::util::IdentOrIndex;

use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parse::{Parser, Result},
    token::Comma,
};
use syn::{Attribute, Fields, Meta, Type};

pub use self::field_attributes::*;
pub use self::type_item_attributes::*;

mod field_attributes;
mod type_item_attributes;

pub fn parse_namespaced_attributes<T: AttributeArgumentParser>(
    attributes: Vec<Attribute>,
) -> (T, Vec<::syn::parse::Error>) {
    let mut result = T::default();
    let mut errors = Vec::new();

    for attribute in attributes {
        let is_palette_attribute = attribute
            .meta
            .path()
            .get_ident()
            .map(|name| name == "palette")
            .unwrap_or(false);

        if !is_palette_attribute {
            continue;
        }

        let meta_list = match attribute.meta.require_list() {
            Ok(list) => list,
            Err(error) => {
                errors.push(error);
                continue;
            }
        };

        if meta_list.tokens.is_empty() {
            errors.push(::syn::parse::Error::new(
                attribute.path().span(),
                "expected `palette(...)`",
            ));

            continue;
        }

        let parse_result =
            Punctuated::<_, Comma>::parse_terminated.parse2(meta_list.tokens.clone());
        match parse_result {
            Ok(meta) => {
                for argument in meta {
                    if let Err(new_error) = result.argument(argument) {
                        errors.extend(new_error);
                    }
                }
            }
            Err(error) => errors.push(error),
        }
    }

    (result, errors)
}

pub fn parse_field_attributes<T: FieldAttributeArgumentParser>(
    fields: Fields,
) -> (T, Vec<::syn::parse::Error>) {
    let mut result = T::default();
    let mut errors = Vec::new();

    let attributes = fields.into_iter().enumerate().flat_map(|(index, field)| {
        let field_name = field
            .ident
            .map(IdentOrIndex::Ident)
            .unwrap_or_else(|| IdentOrIndex::Index(index.into()));
        let ty = field.ty;

        field
            .attrs
            .into_iter()
            .map(move |attribute| (field_name.clone(), ty.clone(), attribute))
    });

    for (field_name, ty, attribute) in attributes {
        let is_palette_attribute = attribute
            .path()
            .get_ident()
            .map(|name| name == "palette")
            .unwrap_or(false);

        if !is_palette_attribute {
            continue;
        }

        let meta_list = match attribute.meta.require_list() {
            Ok(list) => list,
            Err(error) => {
                errors.push(error);
                continue;
            }
        };

        if meta_list.tokens.is_empty() {
            errors.push(::syn::parse::Error::new(
                attribute.path().span(),
                "expected `palette(...)`",
            ));

            continue;
        }

        let parse_result =
            Punctuated::<_, Comma>::parse_terminated.parse2(meta_list.tokens.clone());
        match parse_result {
            Ok(meta) => {
                for argument in meta {
                    if let Err(new_errors) = result.argument(&field_name, &ty, argument) {
                        errors.extend(new_errors);
                    }
                }
            }
            Err(error) => errors.push(error),
        }
    }

    (result, errors)
}

pub fn assert_path_meta(meta: &Meta) -> Result<()> {
    if !matches!(meta, Meta::Path(_)) {
        return Err(::syn::parse::Error::new(
            meta.span(),
            "expected the attribute to be just an identifier or a path",
        ));
    }

    Ok(())
}

pub trait AttributeArgumentParser: Default {
    fn argument(&mut self, argument: Meta) -> std::result::Result<(), Vec<syn::Error>>;
}

pub trait FieldAttributeArgumentParser: Default {
    fn argument(
        &mut self,
        field_name: &IdentOrIndex,
        ty: &Type,
        argument: Meta,
    ) -> std::result::Result<(), Vec<syn::Error>>;
}
