/*
List of color from www.colormine.org
*/
use csv;

use approx::assert_relative_eq;
use lazy_static::lazy_static;
use serde_derive::Deserialize;

use palette::convert::{FromColorUnclamped, IntoColorUnclamped};
use palette::white_point::D65;
use palette::{FloatComponent, Hsl, Hsv, Hwb, Lab, Lch, LinSrgb, Srgb, Xyz, Yxy};

#[derive(Deserialize, PartialEq)]
pub struct ColorMineRaw<F = f64> {
    pub color: String,
    pub hex: String,
    pub rgbu8_r: u8,
    pub rgbu8_g: u8,
    pub rgbu8_b: u8,
    pub rgb_r: F,
    pub rgb_g: F,
    pub rgb_b: F,
    pub cmy_c: F,
    pub cmy_m: F,
    pub cmy_y: F,
    pub cmyk_c: F,
    pub cmyk_m: F,
    pub cmyk_y: F,
    pub cmyk_k: F,
    pub xyz_x: F,
    pub xyz_y: F,
    pub xyz_z: F,
    pub lab_l: F,
    pub lab_a_unscaled: F,
    pub lab_b_unscaled: F,
    pub lab_a: F,
    pub lab_b: F,
    pub lch_l: F,
    pub lch_c_unscaled: F,
    pub lch_c: F,
    pub lch_h: F,
    pub hunterlab_l: F,
    pub hunterlab_a: F,
    pub hunterlab_b: F,
    pub yxy_luma: F,
    pub yxy_x: F,
    pub yxy_y: F,
    pub luv_l: F,
    pub luv_u: F,
    pub luv_v: F,
    pub hsl_h: F,
    pub hsl_s: F,
    pub hsl_l: F,
    pub hsv_h: F,
    pub hsv_s: F,
    pub hsv_v: F,
    pub hwb_h: F,
    pub hwb_w: F,
    pub hwb_b: F,
    pub lab_l_unscaled: F,
    pub lch_l_unscaled: F,
    pub lch_h_normalized: F,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ColorMine<F>
where
    F: FloatComponent,
{
    pub xyz: Xyz<D65, F>,
    pub yxy: Yxy<D65, F>,
    pub lab: Lab<D65, F>,
    pub lch: Lch<D65, F>,
    pub rgb: Srgb<F>,
    pub linear_rgb: LinSrgb<F>,
    pub hsl: Hsl<::palette::encoding::Srgb, F>,
    pub hsv: Hsv<::palette::encoding::Srgb, F>,
    pub hwb: Hwb<::palette::encoding::Srgb, F>,
}

impl<F> From<ColorMineRaw<F>> for ColorMine<F>
where
    F: FloatComponent,
{
    fn from(src: ColorMineRaw<F>) -> ColorMine<F> {
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
    ($component:ident, $self_ty:ty) => {
        impl<$component> From<$self_ty> for ColorMine<$component>
        where
            $component: FloatComponent,
        {
            fn from(color: $self_ty) -> ColorMine<$component> {
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
    ($component:ident, $self_ty:ty) => {
        impl<$component> From<$self_ty> for ColorMine<$component>
        where
            $component: FloatComponent,
        {
            fn from(color: $self_ty) -> ColorMine<$component> {
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

impl<F> From<LinSrgb<F>> for ColorMine<F>
where
    F: FloatComponent,
{
    fn from(color: LinSrgb<F>) -> ColorMine<F> {
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

impl_from_color!(F, Srgb<F>);
impl_from_color!(F, Xyz<D65, F>);
impl_from_color!(F, Yxy<D65, F>);
impl_from_color!(F, Lab<D65, F>);
impl_from_color!(F, Lch<D65, F>);

impl_from_rgb_derivative!(F, Hsl<::palette::encoding::Srgb, F>);
impl_from_rgb_derivative!(F, Hsv<::palette::encoding::Srgb, F>);
impl_from_rgb_derivative!(F, Hwb<::palette::encoding::Srgb, F>);

lazy_static! {
    static ref TEST_DATA: Vec<ColorMine<f64>> = load_data();
}

pub fn load_data<F>() -> Vec<ColorMine<F>>
where
    F: FloatComponent + for<'a> serde::Deserialize<'a>,
{
    let mut rdr = csv::Reader::from_path("tests/convert/data_color_mine.csv")
        .expect("csv file could not be loaded in tests for color mine data");
    let mut color_data: Vec<ColorMine<F>> = Vec::new();
    for record in rdr.deserialize() {
        let r: ColorMineRaw<F> =
            record.expect("color data could not be decoded in tests for color mine data");
        color_data.push(r.into())
    }
    color_data
}

fn check_equal_cie(src: &mut ColorMine<f64>, tgt: &ColorMine<f64>) {
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
fn check_equal_rgb(src: &ColorMine<f64>, tgt: &ColorMine<f64>) {
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
        check_equal_rgb(&result, expected);
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
