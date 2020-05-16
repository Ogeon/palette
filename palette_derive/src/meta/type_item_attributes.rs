use std::collections::HashSet;

use quote::quote;
use syn::spanned::Spanned;
use syn::{Ident, Lit, Meta, MetaNameValue, NestedMeta, Result, Type};

use super::AttributeArgumentParser;

#[derive(Default)]
pub struct TypeItemAttributes {
    pub skip_derives: HashSet<String>,
    pub internal: bool,
    pub internal_not_base_type: bool,
    pub component: Option<Type>,
    pub white_point: Option<Type>,
    pub rgb_standard: Option<Type>,
}

impl AttributeArgumentParser for TypeItemAttributes {
    fn argument(&mut self, argument: Meta) -> Result<()> {
        let argument_name = argument.path().get_ident().map(ToString::to_string);

        match argument_name.as_deref() {
            Some("skip_derives") => {
                let result = if let Meta::List(list) = argument {
                    let skipped: ::std::result::Result<_, _> = list
                        .nested
                        .into_iter()
                        .map(|value| match value {
                            NestedMeta::Meta(Meta::Path(path)) => {
                                if let Some(name) = path.get_ident() {
                                    Ok(name.clone())
                                } else {
                                    Err(path.span())
                                }
                            }
                            value => Err(value.span()),
                        })
                        .collect();

                    skipped.map(|values: Vec<Ident>| {
                        self.skip_derives
                            .extend(values.into_iter().map(|ident| ident.to_string()));
                    })
                } else {
                    Err(argument.span())
                };

                if let Err(span) = result {
                    return Err(::syn::parse::Error::new(
                        span,
                        "expected `skip` to have a list of color type names, like `skip(Xyz, Luma, Rgb)`",
                    ));
                }
            }
            Some("component") => {
                if self.component.is_none() {
                    let result = if let Meta::NameValue(MetaNameValue {
                        lit: Lit::Str(ty), ..
                    }) = argument
                    {
                        self.component = Some(ty.parse()?);
                        Ok(())
                    } else {
                        Err(argument.span())
                    };

                    if let Err(span) = result {
                        let message = "expected `component` to be a type or type parameter in a string, like `component = \"T\"`";
                        return Err(::syn::parse::Error::new(span, message));
                    }
                } else {
                    return Err(::syn::parse::Error::new(
                        argument.span(),
                        "`component` appears more than once",
                    ));
                }
            }
            Some("white_point") => {
                if self.white_point.is_none() {
                    let result = if let Meta::NameValue(MetaNameValue {
                        lit: Lit::Str(ty), ..
                    }) = argument
                    {
                        self.white_point = Some(ty.parse()?);
                        Ok(())
                    } else {
                        Err(argument.span())
                    };

                    if let Err(span) = result {
                        let message = "expected `white_point` to be a type or type parameter in a string, like `white_point = \"T\"`";
                        return Err(::syn::parse::Error::new(span, message));
                    }
                } else {
                    return Err(::syn::parse::Error::new(
                        argument.span(),
                        "`white_point` appears more than once",
                    ));
                }
            }
            Some("rgb_standard") => {
                if self.rgb_standard.is_none() {
                    let result = if let Meta::NameValue(MetaNameValue {
                        lit: Lit::Str(ty), ..
                    }) = argument
                    {
                        self.rgb_standard = Some(ty.parse()?);
                        Ok(())
                    } else {
                        Err(argument.span())
                    };

                    if let Err(span) = result {
                        let message = "expected `rgb_standard` to be a type or type parameter in a string, like `rgb_standard = \"T\"`";
                        return Err(::syn::parse::Error::new(span, message));
                    }
                } else {
                    return Err(::syn::parse::Error::new(
                        argument.span(),
                        "`rgb_standard` appears more than once",
                    ));
                }
            }
            Some("palette_internal") => {
                if let Meta::Path(_) = argument {
                    self.internal = true;
                } else {
                    return Err(::syn::parse::Error::new(
                        argument.span(),
                        "expected `palette_internal` to a literal without value",
                    ));
                }
            }
            Some("palette_internal_not_base_type") => {
                if let Meta::Path(_) = argument {
                    self.internal_not_base_type = true;
                } else {
                    return Err(::syn::parse::Error::new(
                        argument.span(),
                        "expected `palette_internal` to a literal without value",
                    ));
                }
            }
            _ => {
                return Err(::syn::parse::Error::new(
                    argument.span(),
                    format!("`{}` is not a known type item attribute", quote!(#argument)),
                ));
            }
        }

        Ok(())
    }
}
