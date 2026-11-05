//! Meta information for `palette`'s base color types.

use proc_macro2::{Span, TokenStream};
use syn::{parse_quote, parse_str, GenericParam, Generics, Ident, Type};

use crate::{
    from_color_unclamped::util::{InputUser, UsedInput, WhitePointSource},
    util,
};

/// Represents a group of colors with direct conversion.
///
/// These colors can be converted between each other without any additional
/// input.
pub struct ColorGroup {
    /// The root type is a color type that all other colors in the group can
    /// be converted through. It acts as a hub when finding a path from one
    /// color type to another.
    pub root_type: &'static ColorInfo,
    pub(crate) colors: &'static [ColorType],
}

impl ColorGroup {
    /// Check if a color type is part of this group.
    pub fn has_type(&self, name: &str) -> bool {
        if name == self.root_type.name {
            return true;
        }

        for color in self.colors {
            if name == color.info.name {
                return true;
            }
        }

        false
    }

    pub(crate) fn color_names(&'static self) -> ColorNames {
        ColorNames {
            root_type: Some(self.root_type),
            colors: self.colors.iter(),
        }
    }

    /// Find a color type in this group by its name.
    pub fn find_type_by_name(&self, name: &str) -> Option<&ColorType> {
        self.colors.iter().find(|color| color.info.name == name)
    }

    pub(crate) fn find_by_name(&self, name: &str) -> Option<&ColorInfo> {
        if self.root_type.name == name {
            Some(self.root_type)
        } else {
            self.find_type_by_name(name).map(|ty| ty.info)
        }
    }
}

/// Represents a color type in a color group.
pub struct ColorType {
    pub(crate) info: &'static ColorInfo,

    /// Tells whether this color type entry can be used when inferring a color
    /// group for a color type. Types with `infer_group: false` should be
    /// ignored for that group.
    pub infer_group: bool,

    pub(crate) preferred_source: &'static str,
}

/// Explicit meta types for when they can't be inferred.
#[derive(Clone, Copy)]
#[non_exhaustive]
pub struct ColorMeta<T = Type> {
    /// Which white point to use when interfacing with the color type.
    pub white_point: Option<T>,

    /// Which RGB standard to use when interfacing with the color type.
    pub rgb_standard: Option<T>,

    /// Which Luma standard to use when interfacing with the color type.
    pub luma_standard: Option<T>,
}

impl<T> ColorMeta<T> {
    /// An empty `ColorMeta` set, to be used in `static` and `const`.
    pub const DEFAULT: Self = Self {
        white_point: None,
        rgb_standard: None,
        luma_standard: None,
    };
}

impl<T> Default for ColorMeta<T> {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl<'a> TryFrom<ColorMeta<&'a str>> for ColorMeta {
    type Error = syn::Error;

    fn try_from(value: ColorMeta<&'a str>) -> Result<Self, Self::Error> {
        Ok(Self {
            white_point: value.white_point.map(parse_str).transpose()?,
            rgb_standard: value.rgb_standard.map(parse_str).transpose()?,
            luma_standard: value.luma_standard.map(parse_str).transpose()?,
        })
    }
}

type MetaTypeGeneratorFn = fn(
    self_color: &ColorInfo,
    meta_type_source: MetaTypeSource,
    white_point: &Type,
    used_input: &mut UsedInput,
    user: InputUser,
    meta: &ColorMeta,
    palette_name: &Ident,
) -> syn::Result<Type>;

/// Meta information for a `palette` base color.
pub struct ColorInfo {
    /// The name of the color type.
    pub name: &'static str,

    /// The module where the type can be found.
    pub module: &'static str,

    pub(crate) generics: ColorGenerics,
    pub(crate) array_cast: Option<ColorArrayCast>,
    pub(crate) from_color_unclamped: Option<ColorFromColorUnclamped>,
    pub(crate) default_white_point: InternalExternal<Option<&'static [&'static str]>>,
    pub(crate) get_meta_type: Option<MetaTypeGeneratorFn>,
}

impl ColorInfo {
    pub(crate) fn get_path(&self, palette_name: &Ident) -> TokenStream {
        util::path([self.module, self.name], palette_name)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_type(
        &self,
        meta_type_source: MetaTypeSource,
        component: &Type,
        white_point: &Type,
        used_input: &mut UsedInput,
        user: InputUser,
        meta: &ColorMeta,
        palette_name: &Ident,
    ) -> syn::Result<Type> {
        let meta_type: Option<Type> = self
            .get_meta_type
            .map(|get| {
                get(
                    self,
                    meta_type_source,
                    white_point,
                    used_input,
                    user,
                    meta,
                    palette_name,
                )
            })
            .transpose()?;

        let color_path = self.get_path(palette_name);

        if let Some(meta_type) = meta_type {
            Ok(parse_quote!(#color_path::<#meta_type, #component>))
        } else {
            Ok(parse_quote!(#color_path::<#component>))
        }
    }

    pub(crate) fn get_default_white_point(&self, palette_name: &Ident) -> (Type, WhitePointSource) {
        let path = if palette_name == "crate" {
            self.default_white_point.internal
        } else {
            self.default_white_point.external
        };

        path.map(|path| {
            (
                util::path_type(path, palette_name),
                WhitePointSource::ConcreteType,
            )
        })
        .unwrap_or_else(|| (parse_quote!(_Wp), WhitePointSource::GeneratedGeneric))
    }
}

pub(crate) struct ColorGenerics {
    pub(crate) component: &'static str,
    pub(crate) meta: Option<&'static str>,
}

pub(crate) struct ColorArrayCast {
    pub(crate) component_count: usize,
}

pub(crate) struct ColorFromColorUnclamped {
    pub(crate) skip_derives: &'static [&'static str],
    pub(crate) color_groups: &'static [&'static ColorGroup],
    pub(crate) color_meta: ColorMeta<&'static str>,
}

pub(crate) struct InternalExternal<T> {
    pub(crate) internal: T,
    pub(crate) external: T,
}

pub(crate) struct ColorNames {
    root_type: Option<&'static ColorInfo>,
    colors: std::slice::Iter<'static, ColorType>,
}

impl Iterator for ColorNames {
    type Item = &'static ColorInfo;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(root_type) = self.root_type.take() {
            return Some(root_type);
        }

        self.colors.next().map(|color| color.info)
    }
}

mod colors {
    use crate::color_types::{
        get_lms_meta, get_luma_standard, get_rgb_standard, get_white_point, ColorArrayCast,
        ColorGenerics, ColorInfo, ColorMeta, InternalExternal, CAM16_JCH_COLORS, CAM16_JMH_COLORS,
        CAM16_JSH_COLORS, CAM16_QCH_COLORS, CAM16_QMH_COLORS, CAM16_QSH_COLORS, XYZ_COLORS,
    };

    pub static XYZ: ColorInfo = ColorInfo {
        name: "Xyz",
        module: "xyz",
        generics: ColorGenerics {
            component: "T",
            meta: Some("Wp"),
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Xyz", "Yxy", "Luv", "Rgb", "Lab", "Oklab", "Luma", "Lms"],
            color_groups: &[&XYZ_COLORS],
            color_meta: ColorMeta {
                white_point: Some("Wp"),
                ..ColorMeta::DEFAULT
            },
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: Some(&["white_point", "D65"]),
        },
        get_meta_type: Some(get_white_point),
    };

    pub static RGB: ColorInfo = ColorInfo {
        name: "Rgb",
        module: "rgb",
        generics: ColorGenerics {
            component: "T",
            meta: Some("S"),
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Xyz", "Hsv", "Hsl", "Luma", "Rgb", "Oklab"],
            color_groups: &[&XYZ_COLORS],
            color_meta: ColorMeta {
                rgb_standard: Some("S"),
                ..ColorMeta::DEFAULT
            },
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: Some(&["white_point", "D65"]),
        },
        get_meta_type: Some(get_rgb_standard),
    };

    pub static LUMA: ColorInfo = ColorInfo {
        name: "Luma",
        module: "luma",
        generics: ColorGenerics {
            component: "T",
            meta: Some("S"),
        },
        array_cast: Some(ColorArrayCast { component_count: 1 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Xyz", "Yxy", "Luma"],
            color_groups: &[&XYZ_COLORS],
            color_meta: ColorMeta {
                luma_standard: Some("S"),
                ..ColorMeta::DEFAULT
            },
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: Some(&["white_point", "D65"]),
        },
        get_meta_type: Some(get_luma_standard),
    };

    pub static HSL: ColorInfo = ColorInfo {
        name: "Hsl",
        module: "hsl",
        generics: ColorGenerics {
            component: "T",
            meta: Some("S"),
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Rgb", "Hsv", "Hsl"],
            color_groups: &[&XYZ_COLORS],
            color_meta: ColorMeta {
                rgb_standard: Some("S"),
                ..ColorMeta::DEFAULT
            },
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: Some(&["white_point", "D65"]),
        },
        get_meta_type: Some(get_rgb_standard),
    };

    pub static HSLUV: ColorInfo = ColorInfo {
        name: "Hsluv",
        module: "hsluv",
        generics: ColorGenerics {
            component: "T",
            meta: Some("Wp"),
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Lchuv", "Hsluv"],
            color_groups: &[&XYZ_COLORS],
            color_meta: ColorMeta {
                white_point: Some("Wp"),
                ..ColorMeta::DEFAULT
            },
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: Some(&["white_point", "D65"]),
        },
        get_meta_type: Some(get_white_point),
    };

    pub static HSV: ColorInfo = ColorInfo {
        name: "Hsv",
        module: "hsv",
        generics: ColorGenerics {
            component: "T",
            meta: Some("S"),
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Rgb", "Hsl", "Hwb", "Hsv"],
            color_groups: &[&XYZ_COLORS],
            color_meta: ColorMeta {
                rgb_standard: Some("S"),
                ..ColorMeta::DEFAULT
            },
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: Some(&["white_point", "D65"]),
        },
        get_meta_type: Some(get_rgb_standard),
    };

    pub static HWB: ColorInfo = ColorInfo {
        name: "Hwb",
        module: "hwb",
        generics: ColorGenerics {
            component: "T",
            meta: Some("S"),
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Hsv", "Hwb"],
            color_groups: &[&XYZ_COLORS],
            color_meta: ColorMeta {
                rgb_standard: Some("S"),
                ..ColorMeta::DEFAULT
            },
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: Some(&["white_point", "D65"]),
        },
        get_meta_type: Some(get_rgb_standard),
    };

    pub static LAB: ColorInfo = ColorInfo {
        name: "Lab",
        module: "lab",
        generics: ColorGenerics {
            component: "T",
            meta: Some("Wp"),
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Xyz", "Lab", "Lch"],
            color_groups: &[&XYZ_COLORS],
            color_meta: ColorMeta {
                white_point: Some("Wp"),
                ..ColorMeta::DEFAULT
            },
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: Some(&["white_point", "D65"]),
        },
        get_meta_type: Some(get_white_point),
    };

    pub static LCH: ColorInfo = ColorInfo {
        name: "Lch",
        module: "lch",
        generics: ColorGenerics {
            component: "T",
            meta: Some("Wp"),
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Lab", "Lch"],
            color_groups: &[&XYZ_COLORS],
            color_meta: ColorMeta {
                white_point: Some("Wp"),
                ..ColorMeta::DEFAULT
            },
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: Some(&["white_point", "D65"]),
        },
        get_meta_type: Some(get_white_point),
    };

    pub static LCHUV: ColorInfo = ColorInfo {
        name: "Lchuv",
        module: "lchuv",
        generics: ColorGenerics {
            component: "T",
            meta: Some("Wp"),
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Luv", "Lchuv", "Hsluv"],
            color_groups: &[&XYZ_COLORS],
            color_meta: ColorMeta {
                white_point: Some("Wp"),
                ..ColorMeta::DEFAULT
            },
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: Some(&["white_point", "D65"]),
        },
        get_meta_type: Some(get_white_point),
    };

    pub static LMS: ColorInfo = ColorInfo {
        name: "Lms",
        module: "lms",
        generics: ColorGenerics {
            component: "T",
            meta: Some("M"),
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Lms", "Xyz"],
            color_groups: &[&XYZ_COLORS],
            color_meta: ColorMeta::DEFAULT,
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: Some(&["white_point", "D65"]),
        },
        get_meta_type: Some(get_lms_meta),
    };

    pub static LUV: ColorInfo = ColorInfo {
        name: "Luv",
        module: "luv",
        generics: ColorGenerics {
            component: "T",
            meta: Some("Wp"),
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Xyz", "Luv", "Lchuv"],
            color_groups: &[&XYZ_COLORS],
            color_meta: ColorMeta {
                white_point: Some("Wp"),
                ..ColorMeta::DEFAULT
            },
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: Some(&["white_point", "D65"]),
        },
        get_meta_type: Some(get_white_point),
    };

    pub static OKLAB: ColorInfo = ColorInfo {
        name: "Oklab",
        module: "oklab",
        generics: ColorGenerics {
            component: "T",
            meta: None,
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Oklab", "Oklch", "Okhsv", "Okhsl", "Xyz", "Rgb"],
            color_groups: &[&XYZ_COLORS],
            color_meta: ColorMeta {
                white_point: Some("crate::white_point::D65"),
                ..ColorMeta::DEFAULT
            },
        }),
        default_white_point: InternalExternal {
            internal: Some(&["white_point", "D65"]),
            external: Some(&["white_point", "D65"]),
        },
        get_meta_type: None,
    };

    pub static OKLCH: ColorInfo = ColorInfo {
        name: "Oklch",
        module: "oklch",
        generics: ColorGenerics {
            component: "T",
            meta: None,
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Oklab", "Oklch"],
            color_groups: &[&XYZ_COLORS],
            color_meta: ColorMeta {
                white_point: Some("crate::white_point::D65"),
                ..ColorMeta::DEFAULT
            },
        }),
        default_white_point: InternalExternal {
            internal: Some(&["white_point", "D65"]),
            external: Some(&["white_point", "D65"]),
        },
        get_meta_type: None,
    };

    pub static OKHSL: ColorInfo = ColorInfo {
        name: "Okhsl",
        module: "okhsl",
        generics: ColorGenerics {
            component: "T",
            meta: None,
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Okhsl", "Oklab"],
            color_groups: &[&XYZ_COLORS],
            color_meta: ColorMeta {
                white_point: Some("crate::white_point::D65"),
                ..ColorMeta::DEFAULT
            },
        }),
        default_white_point: InternalExternal {
            internal: Some(&["white_point", "D65"]),
            external: Some(&["white_point", "D65"]),
        },
        get_meta_type: None,
    };

    pub static OKHSV: ColorInfo = ColorInfo {
        name: "Okhsv",
        module: "okhsv",
        generics: ColorGenerics {
            component: "T",
            meta: None,
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Okhsv", "Oklab", "Okhwb"],
            color_groups: &[&XYZ_COLORS],
            color_meta: ColorMeta {
                white_point: Some("crate::white_point::D65"),
                ..ColorMeta::DEFAULT
            },
        }),
        default_white_point: InternalExternal {
            internal: Some(&["white_point", "D65"]),
            external: Some(&["white_point", "D65"]),
        },
        get_meta_type: None,
    };

    pub static OKHWB: ColorInfo = ColorInfo {
        name: "Okhwb",
        module: "okhwb",
        generics: ColorGenerics {
            component: "T",
            meta: None,
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Okhwb", "Okhsv"],
            color_groups: &[&XYZ_COLORS],
            color_meta: ColorMeta {
                white_point: Some("crate::white_point::D65"),
                ..ColorMeta::DEFAULT
            },
        }),
        default_white_point: InternalExternal {
            internal: Some(&["white_point", "D65"]),
            external: Some(&["white_point", "D65"]),
        },
        get_meta_type: None,
    };

    pub static YXY: ColorInfo = ColorInfo {
        name: "Yxy",
        module: "yxy",
        generics: ColorGenerics {
            component: "T",
            meta: Some("Wp"),
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Xyz", "Yxy", "Luma"],
            color_groups: &[&XYZ_COLORS],
            color_meta: ColorMeta {
                white_point: Some("Wp"),
                ..ColorMeta::DEFAULT
            },
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: Some(&["white_point", "D65"]),
        },
        get_meta_type: Some(get_white_point),
    };

    pub static CAM16: ColorInfo = ColorInfo {
        name: "Cam16",
        module: "cam16",
        generics: ColorGenerics {
            component: "T",
            meta: None,
        },
        array_cast: None,
        from_color_unclamped: None,
        default_white_point: InternalExternal {
            internal: None,
            external: None,
        },
        get_meta_type: None,
    };

    pub static CAM16_JCH: ColorInfo = ColorInfo {
        name: "Cam16Jch",
        module: "cam16",
        generics: ColorGenerics {
            component: "T",
            meta: None,
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Cam16", "Cam16Jch"],
            color_groups: &[&CAM16_JCH_COLORS],
            color_meta: ColorMeta::DEFAULT,
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: None,
        },
        get_meta_type: None,
    };

    pub static CAM16_JMH: ColorInfo = ColorInfo {
        name: "Cam16Jmh",
        module: "cam16",
        generics: ColorGenerics {
            component: "T",
            meta: None,
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Cam16", "Cam16Jmh", "Cam16UcsJmh"],
            color_groups: &[&CAM16_JMH_COLORS],
            color_meta: ColorMeta::DEFAULT,
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: None,
        },
        get_meta_type: None,
    };

    pub static CAM16_UCS_JMH: ColorInfo = ColorInfo {
        name: "Cam16UcsJmh",
        module: "cam16",
        generics: ColorGenerics {
            component: "T",
            meta: None,
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Cam16Jmh", "Cam16UcsJmh", "Cam16UcsJab"],
            color_groups: &[&CAM16_JMH_COLORS],
            color_meta: ColorMeta::DEFAULT,
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: None,
        },
        get_meta_type: None,
    };

    pub static CAM16_UCS_JAB: ColorInfo = ColorInfo {
        name: "Cam16UcsJab",
        module: "cam16",
        generics: ColorGenerics {
            component: "T",
            meta: None,
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Cam16UcsJmh", "Cam16UcsJab"],
            color_groups: &[&CAM16_JMH_COLORS],
            color_meta: ColorMeta::DEFAULT,
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: None,
        },
        get_meta_type: None,
    };

    pub static CAM16_JSH: ColorInfo = ColorInfo {
        name: "Cam16Jsh",
        module: "cam16",
        generics: ColorGenerics {
            component: "T",
            meta: None,
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Cam16", "Cam16Jsh"],
            color_groups: &[&CAM16_JSH_COLORS],
            color_meta: ColorMeta::DEFAULT,
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: None,
        },
        get_meta_type: None,
    };

    pub static CAM16_QCH: ColorInfo = ColorInfo {
        name: "Cam16Qch",
        module: "cam16",
        generics: ColorGenerics {
            component: "T",
            meta: None,
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Cam16", "Cam16Qch"],
            color_groups: &[&CAM16_QCH_COLORS],
            color_meta: ColorMeta::DEFAULT,
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: None,
        },
        get_meta_type: None,
    };

    pub static CAM16_QMH: ColorInfo = ColorInfo {
        name: "Cam16Qmh",
        module: "cam16",
        generics: ColorGenerics {
            component: "T",
            meta: None,
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Cam16", "Cam16Qmh"],
            color_groups: &[&CAM16_QMH_COLORS],
            color_meta: ColorMeta::DEFAULT,
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: None,
        },
        get_meta_type: None,
    };

    pub static CAM16_QSH: ColorInfo = ColorInfo {
        name: "Cam16Qsh",
        module: "cam16",
        generics: ColorGenerics {
            component: "T",
            meta: None,
        },
        array_cast: Some(ColorArrayCast { component_count: 3 }),
        from_color_unclamped: Some(super::ColorFromColorUnclamped {
            skip_derives: &["Cam16", "Cam16Qsh"],
            color_groups: &[&CAM16_QSH_COLORS],
            color_meta: ColorMeta::DEFAULT,
        }),
        default_white_point: InternalExternal {
            internal: None,
            external: None,
        },
        get_meta_type: None,
    };
}

/// All of the base colors in `pallete`.
pub static COLORS: &[&ColorInfo] = &[
    &colors::XYZ,
    &colors::RGB,
    &colors::LUMA,
    &colors::HSL,
    &colors::HSLUV,
    &colors::HSV,
    &colors::HWB,
    &colors::LAB,
    &colors::LCH,
    &colors::LCHUV,
    &colors::LMS,
    &colors::LUV,
    &colors::OKLAB,
    &colors::OKLCH,
    &colors::OKHSL,
    &colors::OKHSV,
    &colors::OKHWB,
    &colors::YXY,
    &colors::CAM16,
    &colors::CAM16_JCH,
    &colors::CAM16_JMH,
    &colors::CAM16_UCS_JMH,
    &colors::CAM16_UCS_JAB,
    &colors::CAM16_JSH,
    &colors::CAM16_QCH,
    &colors::CAM16_QMH,
    &colors::CAM16_QSH,
];

/// These are the disjoint networks of possible conversions. It's possible to
/// convert directly to and from each color within each group, while converting
/// between the groups requires additional runtime data.
pub static COLOR_GROUPS: &[&ColorGroup] = &[
    &XYZ_COLORS,
    &CAM16_JCH_COLORS,
    &CAM16_JMH_COLORS,
    &CAM16_JSH_COLORS,
    &CAM16_QCH_COLORS,
    &CAM16_QMH_COLORS,
    &CAM16_QSH_COLORS,
];

/// The XYZ color group is where most colors belong. All of these can be
/// converted to `Xyz` without additional input.
pub static XYZ_COLORS: ColorGroup = ColorGroup {
    root_type: &colors::XYZ,
    colors: &[
        ColorType {
            info: &colors::RGB,
            infer_group: true,
            preferred_source: "Xyz",
        },
        ColorType {
            info: &colors::LUMA,
            infer_group: true,
            preferred_source: "Xyz",
        },
        ColorType {
            info: &colors::HSL,
            infer_group: true,
            preferred_source: "Rgb",
        },
        ColorType {
            info: &colors::HSLUV,
            infer_group: true,
            preferred_source: "Lchuv",
        },
        ColorType {
            info: &colors::HSV,
            infer_group: true,
            preferred_source: "Rgb",
        },
        ColorType {
            info: &colors::HWB,
            infer_group: true,
            preferred_source: "Hsv",
        },
        ColorType {
            info: &colors::LAB,
            infer_group: true,
            preferred_source: "Xyz",
        },
        ColorType {
            info: &colors::LCH,
            infer_group: true,
            preferred_source: "Lab",
        },
        ColorType {
            info: &colors::LCHUV,
            infer_group: true,
            preferred_source: "Luv",
        },
        ColorType {
            info: &colors::LMS,
            infer_group: true,
            preferred_source: "Xyz",
        },
        ColorType {
            info: &colors::LUV,
            infer_group: true,
            preferred_source: "Xyz",
        },
        ColorType {
            info: &colors::OKLAB,
            infer_group: true,
            preferred_source: "Xyz",
        },
        ColorType {
            info: &colors::OKLCH,
            infer_group: true,
            preferred_source: "Oklab",
        },
        ColorType {
            info: &colors::OKHSL,
            infer_group: true,
            preferred_source: "Oklab",
        },
        ColorType {
            info: &colors::OKHSV,
            infer_group: true,
            preferred_source: "Oklab",
        },
        ColorType {
            info: &colors::OKHWB,
            infer_group: true,
            preferred_source: "Okhsv",
        },
        ColorType {
            info: &colors::YXY,
            infer_group: true,
            preferred_source: "Xyz",
        },
    ],
};

// The CAM16 groups are a bit special, since they require information about the
// viewing conditions to convert between each other.

static CAM16_JCH_COLORS: ColorGroup = ColorGroup {
    root_type: &colors::CAM16_JCH,
    colors: &[ColorType {
        info: &colors::CAM16,
        infer_group: false, // For generating connections only from `Cam16`, but not to it
        preferred_source: "Cam16Jch",
    }],
};

static CAM16_JMH_COLORS: ColorGroup = ColorGroup {
    root_type: &colors::CAM16_JMH,
    colors: &[
        ColorType {
            info: &colors::CAM16,
            infer_group: false, // For generating connections only from `Cam16`, but not to it
            preferred_source: "Cam16Jmh",
        },
        // CAM16 UCS
        ColorType {
            info: &colors::CAM16_UCS_JMH,
            infer_group: true,
            preferred_source: "Cam16Jmh",
        },
        ColorType {
            info: &colors::CAM16_UCS_JAB,
            infer_group: true,
            preferred_source: "Cam16UcsJmh",
        },
    ],
};

static CAM16_JSH_COLORS: ColorGroup = ColorGroup {
    root_type: &colors::CAM16_JSH,
    colors: &[ColorType {
        info: &colors::CAM16,
        infer_group: false, // For generating connections only from `Cam16`, but not to it
        preferred_source: "Cam16Jsh",
    }],
};

static CAM16_QCH_COLORS: ColorGroup = ColorGroup {
    root_type: &colors::CAM16_QCH,
    colors: &[ColorType {
        info: &colors::CAM16,
        infer_group: false, // For generating connections only from `Cam16`, but not to it
        preferred_source: "Cam16Qch",
    }],
};

static CAM16_QMH_COLORS: ColorGroup = ColorGroup {
    root_type: &colors::CAM16_QMH,
    colors: &[ColorType {
        info: &colors::CAM16,
        infer_group: false, // For generating connections only from `Cam16`, but not to it
        preferred_source: "Cam16Qmh",
    }],
};

static CAM16_QSH_COLORS: ColorGroup = ColorGroup {
    root_type: &colors::CAM16_QSH,
    colors: &[ColorType {
        info: &colors::CAM16,
        infer_group: false, // For generating connections only from `Cam16`, but not to it
        preferred_source: "Cam16Qsh",
    }],
};

fn get_rgb_standard(
    self_color: &ColorInfo,
    meta_type_source: MetaTypeSource,
    white_point: &Type,
    used_input: &mut UsedInput,
    user: InputUser,
    meta: &ColorMeta,
    palette_name: &Ident,
) -> syn::Result<Type> {
    if let Some(rgb_standard) = &meta.rgb_standard {
        Ok(rgb_standard.clone())
    } else {
        match meta_type_source {
            MetaTypeSource::Generics(generics) => {
                used_input.white_point.set_used(user);

                let rgb_standard_path = util::path(["rgb", "RgbStandard"], palette_name);
                let rgb_space_path = util::path(["rgb", "RgbSpace"], palette_name);

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

                Ok(parse_quote!(_S))
            }
            MetaTypeSource::OtherColor(other_color) => {
                match other_color.name {
                    "Rgb" | "Hsl" | "Hsv" | "Hwb" => Ok(parse_quote!(_S)),
                    _ => Err(syn::parse::Error::new(
                        Span::call_site(),
                        format!(
                            "could not determine which RGB standard to use when converting to and from `{}` via `{}`",
                            other_color.name,
                            self_color.name
                        ),
                    )),
                }
            }
        }
    }
}

fn get_luma_standard(
    _self_color: &ColorInfo,
    meta_type_source: MetaTypeSource,
    white_point: &Type,
    used_input: &mut UsedInput,
    user: InputUser,
    meta: &ColorMeta,
    palette_name: &Ident,
) -> syn::Result<Type> {
    if let Some(luma_standard) = meta.luma_standard.as_ref() {
        return Ok(luma_standard.clone());
    }

    used_input.white_point.set_used(user);

    match meta_type_source {
        MetaTypeSource::Generics(generics) => {
            let luma_standard_path = util::path(["luma", "LumaStandard"], palette_name);

            generics.params.push(GenericParam::Type(
                Ident::new("_S", Span::call_site()).into(),
            ));

            generics
                .make_where_clause()
                .predicates
                .push(parse_quote!(_S: #luma_standard_path<WhitePoint = #white_point>));

            Ok(parse_quote!(_S))
        }
        MetaTypeSource::OtherColor(_) => {
            let linear_path = util::path(["encoding", "Linear"], palette_name);

            Ok(parse_quote!(#linear_path<#white_point>))
        }
    }
}

fn get_white_point(
    _self_color: &ColorInfo,
    _meta_type_source: MetaTypeSource,
    white_point: &Type,
    used_input: &mut UsedInput,
    user: InputUser,
    _meta: &ColorMeta,
    _palette_name: &Ident,
) -> syn::Result<Type> {
    used_input.white_point.set_used(user);
    Ok(white_point.clone())
}

fn get_lms_meta(
    self_color: &ColorInfo,
    meta_type_source: MetaTypeSource,
    white_point: &Type,
    used_input: &mut UsedInput,
    user: InputUser,
    _meta: &ColorMeta,
    palette_name: &Ident,
) -> syn::Result<Type> {
    match meta_type_source {
        MetaTypeSource::Generics(generics) => {
            used_input.white_point.set_used(user);

            let has_xyz_meta_path = util::path(["xyz", "meta", "HasXyzMeta"], palette_name);

            generics.params.push(GenericParam::Type(
                Ident::new("_LmsM", Span::call_site()).into(),
            ));

            generics
                .make_where_clause()
                .predicates
                .push(parse_quote!(_LmsM: #has_xyz_meta_path<XyzMeta = #white_point>));

            Ok(parse_quote!(_LmsM))
        }
        MetaTypeSource::OtherColor(other_color) => Err(syn::parse::Error::new(
            Span::call_site(),
            format!(
                "could not determine the LMS meta when converting to and from `{}` via `{}`",
                other_color.name, self_color.name
            ),
        )),
    }
}

pub(crate) enum MetaTypeSource<'a> {
    OtherColor(&'a ColorInfo),
    Generics(&'a mut Generics),
}
