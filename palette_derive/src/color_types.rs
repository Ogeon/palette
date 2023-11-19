pub(crate) struct ColorGroup {
    pub(crate) root_type: ColorInfo,
    pub(crate) cargo_feature: Option<CargoFeature>,
    pub(crate) colors: &'static [ColorType],
}

pub(crate) struct ColorType {
    pub(crate) info: ColorInfo,
    pub(crate) preferred_source: &'static str,
    pub(crate) cargo_feature: Option<CargoFeature>,
}

pub(crate) struct ColorInfo {
    pub(crate) name: &'static str,
}

pub(crate) struct CargoFeature {
    name: &'static str,
    enabled: bool,
}

macro_rules! cargo_feature {
    ($name: literal) => {
        CargoFeature {
            name: $name,
            enabled: cfg!(feature = $name),
        }
    };
}

impl ColorGroup {
    pub(crate) fn check_availability(&self, name: &str) -> Result<(), ColorError> {
        if name == self.root_type.name {
            if let Some(CargoFeature {
                name: feature,
                enabled: false,
            }) = self.cargo_feature
            {
                return Err(ColorError::RequiresFeature(feature));
            }

            return Ok(());
        }

        for color in self.colors {
            if name != color.info.name {
                continue;
            }

            if let Some(CargoFeature {
                name: feature,
                enabled: false,
            }) = self.cargo_feature
            {
                return Err(ColorError::RequiresFeature(feature));
            }

            if let Some(CargoFeature {
                name: feature,
                enabled: false,
            }) = color.cargo_feature
            {
                return Err(ColorError::RequiresFeature(feature));
            }

            return Ok(());
        }

        Err(ColorError::UnknownColor)
    }

    pub(crate) fn color_names(&'static self) -> ColorNames {
        ColorNames {
            root_type: Some(&self.root_type),
            colors: self.colors.iter(),
        }
    }
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

        self.colors.next().map(|color| &color.info)
    }
}

const BASE_COLORS: ColorGroup = ColorGroup {
    root_type: ColorInfo { name: "Xyz" },
    cargo_feature: None,
    colors: &[
        ColorType {
            info: ColorInfo { name: "Rgb" },
            preferred_source: "Xyz",
            cargo_feature: None,
        },
        ColorType {
            info: ColorInfo { name: "Luma" },
            preferred_source: "Xyz",
            cargo_feature: None,
        },
        ColorType {
            info: ColorInfo { name: "Hsl" },
            preferred_source: "Rgb",
            cargo_feature: None,
        },
        ColorType {
            info: ColorInfo { name: "Hsluv" },
            preferred_source: "Lchuv",
            cargo_feature: None,
        },
        ColorType {
            info: ColorInfo { name: "Hsv" },
            preferred_source: "Rgb",
            cargo_feature: None,
        },
        ColorType {
            info: ColorInfo { name: "Hwb" },
            preferred_source: "Hsv",
            cargo_feature: None,
        },
        ColorType {
            info: ColorInfo { name: "Lab" },
            preferred_source: "Xyz",
            cargo_feature: None,
        },
        ColorType {
            info: ColorInfo { name: "Lch" },
            preferred_source: "Lab",
            cargo_feature: None,
        },
        ColorType {
            info: ColorInfo { name: "Lchuv" },
            preferred_source: "Luv",
            cargo_feature: None,
        },
        ColorType {
            info: ColorInfo { name: "Luv" },
            preferred_source: "Xyz",
            cargo_feature: None,
        },
        ColorType {
            info: ColorInfo { name: "Oklab" },
            preferred_source: "Xyz",
            cargo_feature: None,
        },
        ColorType {
            info: ColorInfo { name: "Oklch" },
            preferred_source: "Oklab",
            cargo_feature: None,
        },
        ColorType {
            info: ColorInfo { name: "Okhsl" },
            preferred_source: "Oklab",
            cargo_feature: None,
        },
        ColorType {
            info: ColorInfo { name: "Okhsv" },
            preferred_source: "Oklab",
            cargo_feature: None,
        },
        ColorType {
            info: ColorInfo { name: "Okhwb" },
            preferred_source: "Okhsv",
            cargo_feature: None,
        },
        ColorType {
            info: ColorInfo { name: "Yxy" },
            preferred_source: "Xyz",
            cargo_feature: None,
        },
    ],
};

const CAM16_COLORS: ColorGroup = ColorGroup {
    root_type: ColorInfo { name: "Cam16" },
    cargo_feature: Some(cargo_feature!("cam16")),
    colors: &[
        ColorType {
            info: ColorInfo {
                name: "PartialCam16",
            },
            preferred_source: "Cam16",
            cargo_feature: None,
        },
        ColorType {
            info: ColorInfo {
                name: "Cam16UcsJmh",
            },
            preferred_source: "PartialCam16",
            cargo_feature: None,
        },
        ColorType {
            info: ColorInfo {
                name: "Cam16UcsJab",
            },
            preferred_source: "Cam16UcsJmh",
            cargo_feature: None,
        },
    ],
};

#[derive(Copy, Clone)]
pub(crate) enum AvailableColorGroup {
    Base,
    Cam16,
}

impl AvailableColorGroup {
    pub(crate) fn get_group(&self) -> &'static ColorGroup {
        match self {
            AvailableColorGroup::Base => &BASE_COLORS,
            AvailableColorGroup::Cam16 => &CAM16_COLORS,
        }
    }
}

impl Default for AvailableColorGroup {
    fn default() -> Self {
        AvailableColorGroup::Base
    }
}

pub(crate) enum ColorError {
    UnknownColor,
    RequiresFeature(&'static str),
}
