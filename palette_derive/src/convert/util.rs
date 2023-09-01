use std::collections::{HashMap, HashSet};

use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{parse_quote, GenericParam, Generics, Ident, Result, Type};

use crate::util;
use crate::{COLOR_TYPES, PREFERRED_CONVERSION_SOURCE};

pub fn white_point_type(
    white_point: Option<&Type>,
    rgb_standard: Option<&Type>,
    luma_standard: Option<&Type>,
    internal: bool,
) -> (Type, Option<WhitePointSource>) {
    white_point
        .map(|white_point| (white_point.clone(), Some(WhitePointSource::WhitePoint)))
        .or_else(|| {
            rgb_standard.map(|rgb_standard| {
                let rgb_standard_path = util::path(["rgb", "RgbStandard"], internal);
                let rgb_space_path = util::path(["rgb", "RgbSpace"], internal);
                (
                    parse_quote!(<<#rgb_standard as #rgb_standard_path>::Space as #rgb_space_path>::WhitePoint),
                    Some(WhitePointSource::RgbStandard),
                )
            })
        })
        .or_else(|| {
            luma_standard.map(|luma_standard| {
                let luma_standard_path = util::path(["luma", "LumaStandard"], internal);
                (
                    parse_quote!(<#luma_standard as #luma_standard_path>::WhitePoint),
                    Some(WhitePointSource::LumaStandard),
                )
            })
        })
        .unwrap_or_else(|| {
            (
                util::path_type(&["white_point", "D65"], internal),
                None,
            )
        })
}

pub fn component_type(component: Option<Type>) -> Type {
    component.unwrap_or_else(|| parse_quote!(f32))
}

pub fn get_convert_color_type(
    color: &str,
    white_point: &Type,
    component: &Type,
    rgb_standard: Option<&Type>,
    luma_standard: Option<&Type>,
    generics: &mut Generics,
    internal: bool,
) -> (Type, UsedInput) {
    let color_path = util::color_path(color, internal);

    match color {
        "Luma" => {
            let luma_standard_path = util::path(["luma", "LumaStandard"], internal);

            if let Some(luma_standard) = luma_standard {
                (
                    parse_quote!(#color_path<#luma_standard, #component>),
                    UsedInput::default(),
                )
            } else {
                generics.params.push(GenericParam::Type(
                    Ident::new("_S", Span::call_site()).into(),
                ));

                generics
                    .make_where_clause()
                    .predicates
                    .push(parse_quote!(_S: #luma_standard_path<WhitePoint = #white_point>));
                (
                    parse_quote!(#color_path<_S, #component>),
                    UsedInput { white_point: true },
                )
            }
        }
        "Rgb" | "Hsl" | "Hsv" | "Hwb" => {
            let rgb_standard_path = util::path(["rgb", "RgbStandard"], internal);
            let rgb_space_path = util::path(["rgb", "RgbSpace"], internal);

            if let Some(rgb_standard) = rgb_standard {
                (
                    parse_quote!(#color_path<#rgb_standard, #component>),
                    UsedInput::default(),
                )
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

                (
                    parse_quote!(#color_path<_S, #component>),
                    UsedInput { white_point: true },
                )
            }
        }
        "Oklab" | "Oklch" | "Okhsv" | "Okhsl" | "Okhwb" => {
            (parse_quote!(#color_path<#component>), UsedInput::default())
        }
        _ => (
            parse_quote!(#color_path<#white_point, #component>),
            UsedInput { white_point: true },
        ),
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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum WhitePointSource {
    WhitePoint,
    RgbStandard,
    LumaStandard,
}

#[derive(Debug, Default)]
pub struct UsedInput {
    pub white_point: bool,
}
