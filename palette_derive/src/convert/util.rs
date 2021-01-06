use std::collections::{HashMap, HashSet};

use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{parse_quote, GenericParam, Generics, Ident, Path, Result, Type, TypePath};

use crate::util;
use crate::{COLOR_TYPES, PREFERRED_CONVERSION_SOURCE};

pub fn find_in_generics(
    component: Option<&Type>,
    white_point: Option<&Type>,
    generics: &Generics,
) -> (bool, bool) {
    let mut generic_component = false;
    let mut generic_white_point = false;

    for param in &generics.params {
        if let GenericParam::Type(ref param) = *param {
            if let Some(&Type::Path(TypePath {
                qself: None,
                path:
                    Path {
                        segments: ref component,
                        leading_colon,
                    },
            })) = component
            {
                let first = component.first();
                let is_ident_path = leading_colon.is_none()
                    && component.len() == 1
                    && first.unwrap().arguments.is_empty()
                    && first.unwrap().ident == param.ident;

                if is_ident_path {
                    generic_component = true;
                }
            }

            if let Some(&Type::Path(TypePath {
                qself: None,
                path:
                    Path {
                        segments: ref white_point,
                        leading_colon,
                    },
            })) = white_point
            {
                let first = white_point.first();
                let is_ident_path = leading_colon.is_none()
                    && white_point.len() == 1
                    && first.unwrap().arguments.is_empty()
                    && first.unwrap().ident == param.ident;

                if is_ident_path {
                    generic_white_point = true;
                }
            }
        }
    }

    (generic_component, generic_white_point)
}

pub fn white_point_type(white_point: Option<Type>, internal: bool) -> Type {
    white_point.unwrap_or_else(|| util::path_type(&["white_point", "D65"], internal))
}

pub fn component_type(component: Option<Type>) -> Type {
    component.unwrap_or_else(|| parse_quote!(f32))
}

pub fn add_float_component_where_clause(component: &Type, generics: &mut Generics, internal: bool) {
    let component_trait_path = util::path(&["FloatComponent"], internal);

    generics
        .make_where_clause()
        .predicates
        .push(parse_quote!(#component: #component_trait_path));
}

pub fn add_white_point_where_clause(white_point: &Type, generics: &mut Generics, internal: bool) {
    let white_point_trait_path = util::path(&["white_point", "WhitePoint"], internal);

    generics
        .make_where_clause()
        .predicates
        .push(parse_quote!(#white_point: #white_point_trait_path));
}

pub fn get_convert_color_type(
    color: &str,
    white_point: &Type,
    component: &Type,
    rgb_standard: Option<&Type>,
    generics: &mut Generics,
    internal: bool,
) -> Type {
    let color_path = util::color_path(color, internal);

    match color {
        "Luma" => {
            let luma_standard_path = util::path(&["luma", "LumaStandard"], internal);
            generics.params.push(GenericParam::Type(
                Ident::new("_S", Span::call_site()).into(),
            ));

            generics
                .make_where_clause()
                .predicates
                .push(parse_quote!(_S: #luma_standard_path<WhitePoint = #white_point>));
            parse_quote!(#color_path<_S, #component>)
        }
        "Rgb" | "Hsl" | "Hsv" | "Hwb" => {
            let rgb_standard_path = util::path(&["rgb", "RgbStandard"], internal);
            let rgb_space_path = util::path(&["rgb", "RgbSpace"], internal);

            if let Some(rgb_standard) = rgb_standard {
                parse_quote!(#color_path<#rgb_standard, #component>)
            } else {
                generics.params.push(GenericParam::Type(
                    Ident::new("_S", Span::call_site()).into(),
                ));
                let where_clause = generics.make_where_clause();

                where_clause
                    .predicates
                    .push(parse_quote!(_S: #rgb_standard_path));
                where_clause
                    .predicates
                    .push(parse_quote!(_S::Space: #rgb_space_path<WhitePoint = #white_point>));

                parse_quote!(#color_path<_S, #component>)
            }
        }
        "Oklab" => parse_quote!(#color_path<#component>),
        _ => parse_quote!(#color_path<#white_point, #component>),
    }
}

pub fn find_nearest_color<'a>(color: &'a str, skip: &HashSet<String>) -> Result<&'a str> {
    let mut stack = vec![(color, 0)];
    let mut found = None;
    let mut visited = HashMap::new();

    // Make sure there is at least one valid color in the skip list
    assert!(!skip.is_empty());
    for skipped_color in skip {
        if !COLOR_TYPES
            .iter()
            .any(|valid_color| skipped_color == valid_color)
        {
            return Err(::syn::parse::Error::new(
                color.span(),
                format!("`{}` is not a valid color type", skipped_color),
            ));
        }
    }

    while let Some((color, distance)) = stack.pop() {
        if skip.contains(color) {
            if let Some((_, found_distance)) = found {
                if distance < found_distance {
                    found = Some((color, distance));
                    continue;
                }
            } else {
                found = Some((color, distance));
                continue;
            }
        }

        if let Some(&previous_distance) = visited.get(color) {
            if previous_distance <= distance {
                continue;
            }
        }

        visited.insert(color, distance);

        // Start by pushing the plan B routes...
        for &(destination, source) in PREFERRED_CONVERSION_SOURCE {
            if color == source {
                stack.push((destination, distance + 1));
            }
        }

        // ...then push the preferred routes. They will be popped first.
        for &(destination, source) in PREFERRED_CONVERSION_SOURCE {
            if color == destination {
                stack.push((source, distance + 1));
            }
        }
    }

    if let Some((color, _)) = found {
        Ok(color)
    } else {
        Err(::syn::parse::Error::new(
            color.span(),
            format!(
                "none of the skipped colors can be used for converting from {}",
                color
            ),
        ))
    }
}
