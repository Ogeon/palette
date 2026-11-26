use std::collections::HashSet;

use palette_codegen::util::Ref;
use proc_macro2::Span;
use quote::quote;
use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma, Expr, ExprLit};
use syn::{Ident, Lit, Meta, MetaNameValue, Type};

use palette_codegen::color_types::{ColorGroup, ColorMeta, COLOR_GROUPS};

use super::AttributeArgumentParser;

#[derive(Default)]
pub struct TypeItemAttributes {
    pub palette_name: Option<Ident>,
    pub skip_derives: HashSet<String>,
    pub component: Option<Type>,
    pub color_meta: ColorMeta,
    pub(crate) color_groups: HashSet<Ref<'static, ColorGroup>>,
}

impl TypeItemAttributes {
    pub(crate) fn get_palette_name(&self) -> Ident {
        self.palette_name
            .clone()
            .unwrap_or_else(|| Ident::new("palette", Span::call_site()))
    }
}

impl AttributeArgumentParser for TypeItemAttributes {
    fn argument(&mut self, argument: Meta) -> Result<(), Vec<syn::Error>> {
        let argument_name = argument.path().get_ident().map(ToString::to_string);

        match argument_name.as_deref() {
            Some("crate") => {
                assert_not_already_set(&argument, &self.palette_name)?;
                self.palette_name = Some(get_meta_ident(argument)?);
            }
            Some("skip_derives") => {
                if let Meta::List(list) = argument {
                    let skipped = list
                        .parse_args_with(Punctuated::<Ident, Comma>::parse_terminated)
                        .map_err(|error| vec![error])?;

                    let mut errors = Vec::new();
                    for skipped_color in skipped {
                        let color_name = skipped_color.to_string();
                        self.skip_derives.insert(color_name.clone());

                        let color_group = COLOR_GROUPS
                            .iter()
                            .find(|group| group.has_type(&color_name));

                        let group = if let Some(&group) = color_group {
                            group
                        } else {
                            errors.push(syn::Error::new(
                                skipped_color.span(),
                                format!("`{skipped_color}` is not a valid color type"),
                            ));
                            continue;
                        };

                        let infer_group = group
                            .find_type_by_name(&color_name)
                            .map_or(true, |ty| ty.infer_group);

                        if infer_group {
                            self.color_groups.insert(group.into());
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
                assert_not_already_set(&argument, &self.component)?;
                self.component = Some(get_meta_type(argument)?);
            }
            Some("white_point") => {
                assert_not_already_set(&argument, &self.color_meta.white_point)?;
                self.color_meta.white_point = Some(get_meta_type(argument)?);
            }
            Some("rgb_standard") => {
                assert_not_already_set(&argument, &self.color_meta.rgb_standard)?;
                self.color_meta.rgb_standard = Some(get_meta_type(argument)?);
            }
            Some("luma_standard") => {
                assert_not_already_set(&argument, &self.color_meta.luma_standard)?;
                self.color_meta.luma_standard = Some(get_meta_type(argument)?);
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

fn assert_not_already_set<T>(
    argument: &Meta,
    attribute: &Option<T>,
) -> Result<(), Vec<syn::Error>> {
    if attribute.is_some() {
        let name = argument.path().get_ident().unwrap();
        Err(vec![syn::Error::new(
            argument.path().span(),
            format!("`{name}` appears more than once"),
        )])
    } else {
        Ok(())
    }
}

fn get_meta_ident(argument: Meta) -> Result<Ident, Vec<syn::Error>> {
    if let Meta::NameValue(MetaNameValue {
        value: Expr::Lit(ExprLit {
            lit: Lit::Str(name),
            ..
        }),
        ..
    }) = argument
    {
        name.parse().map_err(|error| vec![error])
    } else {
        let name = argument.path().get_ident().unwrap();
        let message = format!(
            r#""expected `{name}` to be an identifier in a string, like `{name} = "name_here"`"#
        );
        Err(vec![syn::Error::new(argument.span(), message)])
    }
}

fn get_meta_type(argument: Meta) -> Result<Type, Vec<syn::Error>> {
    if let Meta::NameValue(MetaNameValue {
        value: Expr::Lit(ExprLit {
            lit: Lit::Str(ty), ..
        }),
        ..
    }) = argument
    {
        ty.parse().map_err(|error| vec![error])
    } else {
        let name = argument.path().get_ident().unwrap();
        let message = format!(
            "expected `{name}` to be a type or type parameter in a string, like `{name} = \"T\"`"
        );
        Err(vec![syn::Error::new(argument.span(), message)])
    }
}
