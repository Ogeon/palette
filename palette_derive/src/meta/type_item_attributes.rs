use std::collections::HashSet;

use quote::quote;
use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma, Expr, ExprLit};
use syn::{Ident, Lit, Meta, MetaNameValue, Type};

use crate::color_types::{AvailableColorGroup, ColorError};

use super::AttributeArgumentParser;

#[derive(Default)]
pub struct TypeItemAttributes {
    pub skip_derives: HashSet<String>,
    pub internal: bool,
    pub internal_not_base_type: bool,
    pub component: Option<Type>,
    pub white_point: Option<Type>,
    pub rgb_standard: Option<Type>,
    pub luma_standard: Option<Type>,
    pub cam16_chromaticity: Option<Type>,
    pub cam16_luminance: Option<Type>,
    pub(crate) color_group: AvailableColorGroup,
}

impl AttributeArgumentParser for TypeItemAttributes {
    fn argument(&mut self, argument: Meta) -> Result<(), Vec<syn::Error>> {
        let argument_name = argument.path().get_ident().map(ToString::to_string);

        match argument_name.as_deref() {
            Some("skip_derives") => {
                if let Meta::List(list) = argument {
                    let skipped = list
                        .parse_args_with(Punctuated::<Ident, Comma>::parse_terminated)
                        .map_err(|error| vec![error])?;

                    let mut errors = Vec::new();
                    for skipped_color in skipped {
                        self.skip_derives.insert(skipped_color.to_string());
                        // Assumes that `color_group` has already been set.
                        match self
                            .color_group
                            .get_group()
                            .check_availability(&skipped_color.to_string())
                        {
                            Ok(()) => {}
                            Err(ColorError::UnknownColor) => errors.push(syn::Error::new(
                                skipped_color.span(),
                                format!("`{}` is not a valid color type", skipped_color),
                            )),
                            Err(ColorError::RequiresFeature(feature)) => {
                                errors.push(syn::Error::new(
                                    skipped_color.span(),
                                    format!(
                                        "`{}` is only usable with the `{}` feature",
                                        skipped_color, feature
                                    ),
                                ))
                            }
                        }
                    }

                    if !errors.is_empty() {
                        return Err(errors);
                    }
                } else {
                    return Err(vec![syn::Error::new(
                        argument.span(),
                        "expected `skip_derives` to have a list of color type names, like `skip_derives(Xyz, Luma, Rgb)`",
                    )]);
                }
            }
            Some("component") => {
                get_meta_type_argument(argument, &mut self.component)?;
            }
            Some("white_point") => {
                get_meta_type_argument(argument, &mut self.white_point)?;
            }
            Some("rgb_standard") => {
                get_meta_type_argument(argument, &mut self.rgb_standard)?;
            }
            Some("luma_standard") => {
                get_meta_type_argument(argument, &mut self.luma_standard)?;
            }
            #[cfg(feature = "cam16")]
            Some("cam16_chromaticity") => {
                get_meta_type_argument(argument, &mut self.cam16_chromaticity)?;
            }
            #[cfg(not(feature = "cam16"))]
            Some("cam16_chromaticity") => {
                return Err(vec![syn::Error::new(
                    argument.span(),
                    "`cam16_chromaticity` is only usable with the `cam16` feature",
                )]);
            }
            #[cfg(feature = "cam16")]
            Some("cam16_luminance") => {
                get_meta_type_argument(argument, &mut self.cam16_luminance)?;
            }
            #[cfg(not(feature = "cam16"))]
            Some("cam16_luminance") => {
                return Err(vec![syn::Error::new(
                    argument.span(),
                    "`cam16_luminance` is only usable with the `cam16` feature",
                )]);
            }
            Some("color_group") => {
                let mut errors = Vec::new();

                // This makes validation easier.
                if !self.skip_derives.is_empty() {
                    errors.push(::syn::parse::Error::new(
                        argument.span(),
                        "expected `color_group` to be specified before `skip_derives`",
                    ));
                }

                if let Meta::NameValue(MetaNameValue {
                    value:
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(string),
                            ..
                        }),
                    ..
                }) = argument
                {
                    self.color_group = match &*string.value() {
                        "base" => AvailableColorGroup::Base,
                        "cam16" => AvailableColorGroup::Cam16,
                        _ => {
                            errors.push(::syn::parse::Error::new(
                                string.span(),
                                "expected `\"base\"` or `\"cam16\"`",
                            ));

                            self.color_group
                        }
                    };
                } else {
                    errors.push(::syn::parse::Error::new(
                        argument.span(),
                        "expected `color_group = \"group_name\"`",
                    ));
                };

                if !errors.is_empty() {
                    return Err(errors);
                }
            }
            Some("palette_internal") => {
                if let Meta::Path(_) = argument {
                    self.internal = true;
                } else {
                    return Err(vec![syn::Error::new(
                        argument.span(),
                        "expected `palette_internal` to a literal without value",
                    )]);
                }
            }
            Some("palette_internal_not_base_type") => {
                if let Meta::Path(_) = argument {
                    self.internal_not_base_type = true;
                } else {
                    return Err(vec![syn::Error::new(
                        argument.span(),
                        "expected `palette_internal` to a literal without value",
                    )]);
                }
            }
            _ => {
                return Err(vec![syn::Error::new(
                    argument.span(),
                    format!("`{}` is not a known type item attribute", quote!(#argument)),
                )]);
            }
        }

        Ok(())
    }
}

fn get_meta_type_argument(
    argument: Meta,
    attribute: &mut Option<Type>,
) -> Result<(), Vec<syn::Error>> {
    if attribute.is_none() {
        let result = if let Meta::NameValue(MetaNameValue {
            value: Expr::Lit(ExprLit {
                lit: Lit::Str(ty), ..
            }),
            ..
        }) = argument
        {
            *attribute = Some(ty.parse().map_err(|error| vec![error])?);
            Ok(())
        } else {
            Err((argument.span(), argument.path()))
        };

        if let Err((span, path)) = result {
            let name = path.get_ident().unwrap();
            let message = format!("expected `{name}` to be a type or type parameter in a string, like `{name} = \"T\"`");
            Err(vec![syn::Error::new(span, message)])
        } else {
            Ok(())
        }
    } else {
        let name = argument.path().get_ident().unwrap();
        Err(vec![syn::Error::new(
            argument.span(),
            format!("`{name}` appears more than once"),
        )])
    }
}
