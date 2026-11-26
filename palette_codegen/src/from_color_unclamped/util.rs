use std::collections::{HashMap, HashSet};

use proc_macro2::Span;
use syn::{parse_quote, Generics, Ident, Result, Type};

use crate::{
    color_types::{ColorGroup, ColorInfo, ColorMeta, MetaTypeSource},
    util::{self, Ref},
};

pub(crate) fn white_point_type(
    white_point: Option<&Type>,
    rgb_standard: Option<&Type>,
    luma_standard: Option<&Type>,
    palette_name: &Ident,
) -> Option<(Type, WhitePointSource)> {
    white_point
        .map(|white_point| (white_point.clone(), WhitePointSource::WhitePoint))
        .or_else(|| {
            rgb_standard.map(|rgb_standard| {
                let rgb_standard_path = util::path(["rgb", "RgbStandard"], palette_name);
                let rgb_space_path = util::path(["rgb", "RgbSpace"], palette_name);
                (
                    parse_quote!(<<#rgb_standard as #rgb_standard_path>::Space as #rgb_space_path>::WhitePoint),
                    WhitePointSource::RgbStandard,
                )
            })
        })
        .or_else(|| {
            luma_standard.map(|luma_standard| {
                let luma_standard_path = util::path(["luma", "LumaStandard"], palette_name);
                (
                    parse_quote!(<#luma_standard as #luma_standard_path>::WhitePoint),
                    WhitePointSource::LumaStandard,
                )
            })
        })
}

pub(crate) fn get_convert_color_type(
    color: &ColorInfo,
    white_point: &Type,
    component: &Type,
    color_meta: &ColorMeta,
    generics: &mut Generics,
    palette_name: &Ident,
) -> syn::Result<(Type, UsedInput)> {
    let mut used_input = UsedInput::default();
    let color_type = color.get_type(
        MetaTypeSource::Generics(generics),
        component,
        white_point,
        &mut used_input,
        InputUser::Target,
        color_meta,
        palette_name,
    )?;

    Ok((color_type, used_input))
}

pub(crate) fn find_nearest_color<'a>(
    color: &'a ColorInfo,
    skip_derives: &HashSet<String>,
    color_groups: &HashSet<Ref<'static, ColorGroup>>,
) -> Result<&'a ColorInfo> {
    let mut stack = vec![(color, 0)];
    let mut found = None;
    let mut visited = HashMap::new();

    // Make sure there is at least one valid color in the skip list
    assert!(!skip_derives.is_empty());

    while let Some((color, distance)) = stack.pop() {
        if skip_derives.contains(color.name) {
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

        if let Some(&previous_distance) = visited.get(color.name) {
            if previous_distance <= distance {
                continue;
            }
        }

        visited.insert(color.name, distance);

        // Start by pushing the plan B routes...
        for group in color_groups {
            for candidate in group.colors {
                if color.name == candidate.preferred_source {
                    stack.push((candidate.info, distance + 1));
                }
            }
        }

        // ...then push the preferred routes. They will be popped first.
        for group in color_groups {
            for candidate in group.colors {
                if color.name == candidate.info.name {
                    let preferred = group
                        .find_by_name(candidate.preferred_source)
                        .expect("preferred sources have to exist in the group");
                    stack.push((preferred, distance + 1));
                }
            }
        }
    }

    if let Some((color, _)) = found {
        Ok(color)
    } else {
        Err(::syn::parse::Error::new(
            Span::call_site(),
            format!(
                "none of the skipped colors can be used for converting from {}",
                color.name
            ),
        ))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum WhitePointSource {
    WhitePoint,
    RgbStandard,
    LumaStandard,
    ConcreteType,
    GeneratedGeneric,
}

#[derive(Debug, Default)]
pub(crate) struct UsedInput {
    pub white_point: InputUsage,
}

#[derive(Debug, Default)]
pub(crate) struct InputUsage {
    used_by_target: bool,
    used_by_nearest: bool,
}

impl InputUsage {
    pub(crate) fn set_used(&mut self, user: InputUser) {
        match user {
            InputUser::Target => self.used_by_target = true,
            InputUser::Nearest => self.used_by_nearest = true,
        }
    }

    pub(crate) fn is_used(&self) -> bool {
        self.used_by_target || self.used_by_nearest
    }

    pub(crate) fn is_unconstrained(&self) -> bool {
        !self.used_by_target && self.used_by_nearest
    }
}

#[derive(Clone, Copy)]
pub(crate) enum InputUser {
    Target,
    Nearest,
}
