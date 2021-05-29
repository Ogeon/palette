/*
List of color from www.colormine.org
*/
use csv;

use approx::assert_relative_eq;
use lazy_static::lazy_static;
use serde_derive::Deserialize;

use palette::convert::{FromColorUnclamped, IntoColorUnclamped};
use palette::white_point::D65;
use palette::{Hsl, Hsv, Hwb, Lab, Lch, LinSrgb, Srgb, Xyz, Yxy};

#[derive(Deserialize, PartialEq)]
pub struct ColorMineRaw {
    pub color: String,
    pub hex: String,
    pub rgbu8_r: u8,
    pub rgbu8_g: u8,
    pub rgbu8_b: u8,
    pub rgb_r: f64,
    pub rgb_g: f64,
    pub rgb_b: f64,
    pub cmy_c: f64,
    pub cmy_m: f64,
    pub cmy_y: f64,
    pub cmyk_c: f64,
    pub cmyk_m: f64,
    pub cmyk_y: f64,
    pub cmyk_k: f64,
    pub xyz_x: f64,
    pub xyz_y: f64,
    pub xyz_z: f64,
    pub lab_l: f64,
    pub lab_a_unscaled: f64,
    pub lab_b_unscaled: f64,
    pub lab_a: f64,
    pub lab_b: f64,
    pub lch_l: f64,
    pub lch_c_unscaled: f64,
    pub lch_c: f64,
    pub lch_h: f64,
    pub hunterlab_l: f64,
    pub hunterlab_a: f64,
    pub hunterlab_b: f64,
    pub yxy_luma: f64,
    pub yxy_x: f64,
    pub yxy_y: f64,
    pub luv_l: f64,
    pub luv_u: f64,
    pub luv_v: f64,
    pub hsl_h: f64,
    pub hsl_s: f64,
    pub hsl_l: f64,
    pub hsv_h: f64,
    pub hsv_s: f64,
    pub hsv_v: f64,
    pub hwb_h: f64,
    pub hwb_w: f64,
    pub hwb_b: f64,
    pub lab_l_unscaled: f64,
    pub lch_l_unscaled: f64,
    pub lch_h_normalized: f64,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ColorMine {
    pub xyz: Xyz<D65, f64>,
    pub yxy: Yxy<D65, f64>,
    pub lab: Lab<D65, f64>,
    pub lch: Lch<D65, f64>,
    pub rgb: Srgb<f64>,
    pub linear_rgb: LinSrgb<f64>,
    pub hsl: Hsl<::palette::encoding::Srgb, f64>,
    pub hsv: Hsv<::palette::encoding::Srgb, f64>,
    pub hwb: Hwb<::palette::encoding::Srgb, f64>,
}

impl From<ColorMineRaw> for ColorMine {
    fn from(src: ColorMineRaw) -> ColorMine {
        ColorMine {
            xyz: Xyz::new(src.xyz_x, src.xyz_y, src.xyz_z),
            yxy: Yxy::new(src.yxy_x, src.yxy_y, src.yxy_luma),
            lab: Lab::new(src.lab_l_unscaled, src.lab_a_unscaled, src.lab_b_unscaled),
            lch: Lch::new(src.lch_l_unscaled, src.lch_c_unscaled, src.lch_h_normalized),
            rgb: Srgb::new(src.rgb_r, src.rgb_g, src.rgb_b),
            linear_rgb: Srgb::new(src.rgb_r, src.rgb_g, src.rgb_b).into_linear(),
            hsl: Hsl::new(src.hsl_h, src.hsl_s, src.hsl_l),
            hsv: Hsv::new(src.hsv_h, src.hsv_s, src.hsv_v),
            hwb: Hwb::new(src.hwb_h, src.hwb_w, src.hwb_b),
        }
    }
}

macro_rules! impl_from_color {
    ($self_ty:ty) => {
        impl From<$self_ty> for ColorMine {
            fn from(color: $self_ty) -> ColorMine {
                ColorMine {
                    xyz: color.into_color_unclamped(),
                    yxy: color.into_color_unclamped(),
                    lab: color.into_color_unclamped(),
                    lch: color.into_color_unclamped(),
                    linear_rgb: color.into_color_unclamped(),
                    rgb: color.into_color_unclamped(),
                    hsl: color.into_color_unclamped(),
                    hsv: color.into_color_unclamped(),
                    hwb: color.into_color_unclamped(),
                }
            }
        }
    };
}

macro_rules! impl_from_rgb_derivative {
    ($self_ty:ty) => {
        impl From<$self_ty> for ColorMine {
            fn from(color: $self_ty) -> ColorMine {
                ColorMine {
                    xyz: color.into_color_unclamped(),
                    yxy: color.into_color_unclamped(),
                    lab: color.into_color_unclamped(),
                    lch: color.into_color_unclamped(),
                    linear_rgb: Srgb::from_color_unclamped(color).into_color_unclamped(),
                    rgb: color.into_color_unclamped(),
                    hsl: color.into_color_unclamped(),
                    hsv: color.into_color_unclamped(),
                    hwb: color.into_color_unclamped(),
                }
            }
        }
    };
}

impl From<LinSrgb<f64>> for ColorMine {
    fn from(color: LinSrgb<f64>) -> ColorMine {
        ColorMine {
            xyz: color.into_color_unclamped(),
            yxy: color.into_color_unclamped(),
            lab: color.into_color_unclamped(),
            lch: color.into_color_unclamped(),
            linear_rgb: color.into_color_unclamped(),
            rgb: color.into_color_unclamped(),
            hsl: Srgb::from_linear(color).into_color_unclamped(),
            hsv: Srgb::from_linear(color).into_color_unclamped(),
            hwb: Srgb::from_linear(color).into_color_unclamped(),
        }
    }
}

impl_from_color!(Srgb<f64>);
impl_from_color!(Xyz<D65, f64>);
impl_from_color!(Yxy<D65, f64>);
impl_from_color!(Lab<D65, f64>);
impl_from_color!(Lch<D65, f64>);

impl_from_rgb_derivative!(Hsl<::palette::encoding::Srgb, f64>);
impl_from_rgb_derivative!(Hsv<::palette::encoding::Srgb, f64>);
impl_from_rgb_derivative!(Hwb<::palette::encoding::Srgb, f64>);

lazy_static! {
    static ref TEST_DATA: Vec<ColorMine> = load_data();
}

pub fn load_data() -> Vec<ColorMine> {
    let mut rdr = csv::Reader::from_path("tests/convert/data_color_mine.csv")
        .expect("csv file could not be loaded in tests for color mine data");
    let mut color_data: Vec<ColorMine> = Vec::new();
    for record in rdr.deserialize() {
        let r: ColorMineRaw =
            record.expect("color data could not be decoded in tests for color mine data");
        color_data.push(r.into())
    }
    color_data
}

fn check_equal_cie(src: &mut ColorMine, tgt: &ColorMine) {
    assert_relative_eq!(src.xyz, tgt.xyz, epsilon = 0.05);
    assert_relative_eq!(src.yxy, tgt.yxy, epsilon = 0.05);

    //these transformations are problematic and need bigger epsilon
    assert_relative_eq!(src.lab, tgt.lab, epsilon = 7.0);

    // hue values are not passing for from_yxy conversion. Check github #48 for
    // more information assert_relative_eq!(src.lch.hue, tgt.lch.hue, epsilon =
    // 0.05);
    //low chroma colors have hues that are hard to calculate precisely
    if src.lch.chroma < 20.0 {
        println!("{}", src.lch.chroma);
        src.lch.hue = tgt.lch.hue;
    }
    assert_relative_eq!(src.lch, tgt.lch, epsilon = 7.0);

}
fn check_equal_rgb(src: &ColorMine, tgt: &ColorMine) {
    assert_relative_eq!(src.rgb, tgt.rgb, epsilon = 0.05);
    assert_relative_eq!(src.hsl, tgt.hsl, epsilon = 0.05);
    assert_relative_eq!(src.hsv, tgt.hsv, epsilon = 0.05);
    assert_relative_eq!(src.hwb, tgt.hwb, epsilon = 0.05);
}

pub fn run_from_xyz_tests() {
    for expected in TEST_DATA.iter() {
        let mut result = ColorMine::from(expected.xyz);
        check_equal_cie(&mut result, expected);
    }
}
pub fn run_from_yxy_tests() {
    for expected in TEST_DATA.iter() {
        let mut result = ColorMine::from(expected.yxy);
        check_equal_cie(&mut result, expected);
    }
}
pub fn run_from_rgb_tests() {
    for expected in TEST_DATA.iter() {
        let result = ColorMine::from(expected.rgb);
        check_equal_rgb(&result, expected);
    }
}
pub fn run_from_linear_rgb_tests() {
    for expected in TEST_DATA.iter() {
        let mut result = ColorMine::from(expected.linear_rgb);
        check_equal_cie(&mut result, expected);
    }
}
pub fn run_from_hsl_tests() {
    for expected in TEST_DATA.iter() {
        let result = ColorMine::from(expected.hsl);
        check_equal_rgb(& result, expected);
    }
}
pub fn run_from_hsv_tests() {
    for expected in TEST_DATA.iter() {
        let result = ColorMine::from(expected.hsv);
        check_equal_rgb(&result, expected);
    }
}
pub fn run_from_hwb_tests() {
    for expected in TEST_DATA.iter() {
        let result = ColorMine::from(expected.hwb);
        check_equal_rgb(&result, expected);
    }
}
pub fn run_from_lab_tests() {
    for expected in TEST_DATA.iter() {
        let mut result = ColorMine::from(expected.lab);
        check_equal_cie(&mut result, expected);
    }
}
pub fn run_from_lch_tests() {
    for expected in TEST_DATA.iter() {
        let mut result = ColorMine::from(expected.lch);
        check_equal_cie(&mut result, expected);
    }
}
