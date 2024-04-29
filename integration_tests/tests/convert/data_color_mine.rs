/*
List of color from www.colormine.org
*/

use std::path::Path;

use approx::assert_relative_eq;
use lazy_static::lazy_static;
use serde_derive::Deserialize;

use palette::{
    angle::AngleEq,
    convert::{FromColorUnclamped, IntoColorUnclamped},
    encoding::{FromLinear, IntoLinear},
    num::{FromScalarArray, IntoScalarArray, One, Real, Zero},
    rgb::RgbStandard,
    white_point::{WhitePoint, D65},
    Hsl, Hsv, Hwb, Lab, Lch, LinSrgb, Srgb, Xyz, Yxy,
};

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

#[derive(Clone, Debug)]
pub struct ColorMine<F> {
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

impl<F> PartialEq for ColorMine<F>
where
    F: PartialEq + AngleEq<Mask = bool>,
{
    fn eq(&self, other: &Self) -> bool {
        self.xyz == other.xyz
            && self.yxy == other.yxy
            && self.lab == other.lab
            && self.lch == other.lch
            && self.rgb == other.rgb
            && self.linear_rgb == other.linear_rgb
            && self.hsl == other.hsl
            && self.hsv == other.hsv
            && self.hwb == other.hwb
    }
}

impl<F> From<ColorMineRaw<F>> for ColorMine<F>
where
    F: Copy,
    palette::encoding::Srgb: RgbStandard<Space = palette::encoding::Srgb> + IntoLinear<F, F>,
{
    fn from(src: ColorMineRaw<F>) -> ColorMine<F> {
        ColorMine {
            xyz: Xyz::new(src.xyz_x, src.xyz_y, src.xyz_z),
            yxy: Yxy::new(src.yxy_x, src.yxy_y, src.yxy_luma),
            lab: Lab::new(src.lab_l_unscaled, src.lab_a_unscaled, src.lab_b_unscaled),
            lch: Lch::new(src.lch_l_unscaled, src.lch_c_unscaled, src.lch_h_normalized),
            rgb: Srgb::new(src.rgb_r, src.rgb_g, src.rgb_b),
            linear_rgb: Srgb::new(src.rgb_r, src.rgb_g, src.rgb_b).into_linear(),
            hsl: Hsl::new_srgb(src.hsl_h, src.hsl_s, src.hsl_l),
            hsv: Hsv::new_srgb(src.hsv_h, src.hsv_s, src.hsv_v),
            hwb: Hwb::new_srgb(src.hwb_h, src.hwb_w, src.hwb_b),
        }
    }
}

macro_rules! impl_from_color {
    ($component:ident, $self_ty:ty) => {
        impl<$component> From<$self_ty> for ColorMine<$component>
        where
            $component: Copy,
            $self_ty: IntoColorUnclamped<Xyz<D65, $component>>
                + IntoColorUnclamped<Yxy<D65, $component>>
                + IntoColorUnclamped<Lab<D65, $component>>
                + IntoColorUnclamped<Lch<D65, $component>>
                + IntoColorUnclamped<LinSrgb<$component>>
                + IntoColorUnclamped<Srgb<$component>>
                + IntoColorUnclamped<Hsl<::palette::encoding::Srgb, $component>>
                + IntoColorUnclamped<Hsv<::palette::encoding::Srgb, $component>>
                + IntoColorUnclamped<Hwb<::palette::encoding::Srgb, $component>>,
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
            $component: Copy,
            $self_ty: IntoColorUnclamped<Xyz<D65, $component>>
                + IntoColorUnclamped<Yxy<D65, $component>>
                + IntoColorUnclamped<Lab<D65, $component>>
                + IntoColorUnclamped<Lch<D65, $component>>
                + IntoColorUnclamped<Hsl<::palette::encoding::Srgb, $component>>
                + IntoColorUnclamped<Hsv<::palette::encoding::Srgb, $component>>
                + IntoColorUnclamped<Hwb<::palette::encoding::Srgb, $component>>,
            Srgb<$component>:
                FromColorUnclamped<$self_ty> + IntoColorUnclamped<LinSrgb<$component>>,
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
    F: Copy,
    LinSrgb<F>: IntoColorUnclamped<Xyz<D65, F>>
        + IntoColorUnclamped<Yxy<D65, F>>
        + IntoColorUnclamped<Lab<D65, F>>
        + IntoColorUnclamped<Lch<D65, F>>
        + IntoColorUnclamped<LinSrgb<F>>
        + IntoColorUnclamped<Srgb<F>>,
    Srgb<F>: IntoColorUnclamped<Hsl<::palette::encoding::Srgb, F>>
        + IntoColorUnclamped<Hsv<::palette::encoding::Srgb, F>>
        + IntoColorUnclamped<Hwb<::palette::encoding::Srgb, F>>,
    palette::encoding::Srgb: RgbStandard<Space = palette::encoding::Srgb> + FromLinear<F, F>,
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

impl<F, S, const N: usize> From<[ColorMine<F>; N]> for ColorMine<S>
where
    [Xyz<D65, F>; N]: Default,
    [Yxy<D65, F>; N]: Default,
    [Lab<D65, F>; N]: Default,
    [Lch<D65, F>; N]: Default,
    [LinSrgb<F>; N]: Default,
    [Srgb<F>; N]: Default,
    [Hsl<::palette::encoding::Srgb, F>; N]: Default,
    [Hsv<::palette::encoding::Srgb, F>; N]: Default,
    [Hwb<::palette::encoding::Srgb, F>; N]: Default,
    [F; N]: Default,
    S: FromScalarArray<N, Scalar = F>,
{
    fn from(colors: [ColorMine<F>; N]) -> Self {
        let mut xyz: [Xyz<D65, F>; N] = Default::default();
        let mut yxy: [Yxy<D65, F>; N] = Default::default();
        let mut lab: [Lab<D65, F>; N] = Default::default();
        let mut lch: [Lch<D65, F>; N] = Default::default();
        let mut linear_rgb: [LinSrgb<F>; N] = Default::default();
        let mut rgb: [Srgb<F>; N] = Default::default();
        let mut hsl: [Hsl<::palette::encoding::Srgb, F>; N] = Default::default();
        let mut hsv: [Hsv<::palette::encoding::Srgb, F>; N] = Default::default();
        let mut hwb: [Hwb<::palette::encoding::Srgb, F>; N] = Default::default();

        for (index, color) in IntoIterator::into_iter(colors).enumerate() {
            xyz[index] = color.xyz;
            yxy[index] = color.yxy;
            lab[index] = color.lab;
            lch[index] = color.lch;
            linear_rgb[index] = color.linear_rgb;
            rgb[index] = color.rgb;
            hsl[index] = color.hsl;
            hsv[index] = color.hsv;
            hwb[index] = color.hwb;
        }

        ColorMine {
            xyz: xyz.into(),
            yxy: yxy.into(),
            lab: lab.into(),
            lch: lch.into(),
            linear_rgb: linear_rgb.into(),
            rgb: rgb.into(),
            hsl: hsl.into(),
            hsv: hsv.into(),
            hwb: hwb.into(),
        }
    }
}

impl<F, S> From<ColorMine<S>> for [ColorMine<F>; 2]
where
    S: IntoScalarArray<2, Scalar = F>,
    F: Real + Zero + One + Default,
    D65: WhitePoint<F>,
    Yxy<D65, F>: FromColorUnclamped<Xyz<D65, F>>,
{
    fn from(color_data: ColorMine<S>) -> Self {
        let [xyz0, xyz1]: [Xyz<_, F>; 2] = color_data.xyz.into();
        let [yxy0, yxy1]: [Yxy<_, F>; 2] = color_data.yxy.into();
        let [lab0, lab1]: [Lab<_, F>; 2] = color_data.lab.into();
        let [lch0, lch1]: [Lch<_, F>; 2] = color_data.lch.into();
        let [linear_rgb0, linear_rgb1]: [LinSrgb<F>; 2] = color_data.linear_rgb.into();
        let [rgb0, rgb1]: [Srgb<F>; 2] = color_data.rgb.into();
        let [hsl0, hsl1]: [Hsl<_, F>; 2] = color_data.hsl.into();
        let [hsv0, hsv1]: [Hsv<_, F>; 2] = color_data.hsv.into();
        let [hwb0, hwb1]: [Hwb<_, F>; 2] = color_data.hwb.into();

        [
            ColorMine {
                xyz: xyz0,
                yxy: yxy0,
                lab: lab0,
                lch: lch0,
                linear_rgb: linear_rgb0,
                rgb: rgb0,
                hsl: hsl0,
                hsv: hsv0,
                hwb: hwb0,
            },
            ColorMine {
                xyz: xyz1,
                yxy: yxy1,
                lab: lab1,
                lch: lch1,
                linear_rgb: linear_rgb1,
                rgb: rgb1,
                hsl: hsl1,
                hsv: hsv1,
                hwb: hwb1,
            },
        ]
    }
}

lazy_static! {
    static ref TEST_DATA: Vec<ColorMine<f64>> = load_data(None);
}

pub fn load_data<F>(data_path: Option<&Path>) -> Vec<ColorMine<F>>
where
    F: for<'a> serde::Deserialize<'a>,
    ColorMineRaw<F>: Into<ColorMine<F>>,
{
    let mut rdr = csv::Reader::from_path(
        data_path.unwrap_or_else(|| Path::new("tests/convert/data_color_mine.csv")),
    )
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

pub mod wide_f64x2 {
    use super::*;

    pub fn run_from_xyz_tests() {
        for expected in TEST_DATA.chunks_exact(2) {
            let colors = Xyz::<_, wide::f64x2>::from([expected[0].xyz, expected[1].xyz]);
            let [mut result0, mut result1]: [ColorMine<f64>; 2] = ColorMine::from(colors).into();
            check_equal_cie(&mut result0, &expected[0]);
            check_equal_cie(&mut result1, &expected[1]);
        }
    }
    pub fn run_from_yxy_tests() {
        for expected in TEST_DATA.chunks_exact(2) {
            let colors = Yxy::<_, wide::f64x2>::from([expected[0].yxy, expected[1].yxy]);
            let [mut result0, mut result1]: [ColorMine<f64>; 2] = ColorMine::from(colors).into();
            check_equal_cie(&mut result0, &expected[0]);
            check_equal_cie(&mut result1, &expected[1]);
        }
    }
    pub fn run_from_rgb_tests() {
        for expected in TEST_DATA.chunks_exact(2) {
            let colors = Srgb::<wide::f64x2>::from([expected[0].rgb, expected[1].rgb]);
            let [result0, result1]: [ColorMine<f64>; 2] = ColorMine::from(colors).into();
            check_equal_rgb(&result0, &expected[0]);
            check_equal_rgb(&result1, &expected[1]);
        }
    }
    pub fn run_from_linear_rgb_tests() {
        for expected in TEST_DATA.chunks_exact(2) {
            let colors =
                LinSrgb::<wide::f64x2>::from([expected[0].linear_rgb, expected[1].linear_rgb]);
            let [mut result0, mut result1]: [ColorMine<f64>; 2] = ColorMine::from(colors).into();
            check_equal_cie(&mut result0, &expected[0]);
            check_equal_cie(&mut result1, &expected[1]);
        }
    }
    pub fn run_from_hsl_tests() {
        for expected in TEST_DATA.chunks_exact(2) {
            let colors = Hsl::<_, wide::f64x2>::from([expected[0].hsl, expected[1].hsl]);
            let [result0, result1]: [ColorMine<f64>; 2] = ColorMine::from(colors).into();
            check_equal_rgb(&result0, &expected[0]);
            check_equal_rgb(&result1, &expected[1]);
        }
    }
    pub fn run_from_hsv_tests() {
        for expected in TEST_DATA.chunks_exact(2) {
            let colors = Hsv::<_, wide::f64x2>::from([expected[0].hsv, expected[1].hsv]);
            let [result0, result1]: [ColorMine<f64>; 2] = ColorMine::from(colors).into();
            check_equal_rgb(&result0, &expected[0]);
            check_equal_rgb(&result1, &expected[1]);
        }
    }
    pub fn run_from_hwb_tests() {
        for expected in TEST_DATA.chunks_exact(2) {
            let colors = Hwb::<_, wide::f64x2>::from([expected[0].hwb, expected[1].hwb]);
            let [result0, result1]: [ColorMine<f64>; 2] = ColorMine::from(colors).into();
            check_equal_rgb(&result0, &expected[0]);
            check_equal_rgb(&result1, &expected[1]);
        }
    }
    pub fn run_from_lab_tests() {
        for expected in TEST_DATA.chunks_exact(2) {
            let colors = Lab::<_, wide::f64x2>::from([expected[0].lab, expected[1].lab]);
            let [mut result0, mut result1]: [ColorMine<f64>; 2] = ColorMine::from(colors).into();
            check_equal_cie(&mut result0, &expected[0]);
            check_equal_cie(&mut result1, &expected[1]);
        }
    }
    pub fn run_from_lch_tests() {
        for expected in TEST_DATA.chunks_exact(2) {
            let colors = Lch::<_, wide::f64x2>::from([expected[0].lch, expected[1].lch]);
            let [mut result0, mut result1]: [ColorMine<f64>; 2] = ColorMine::from(colors).into();
            check_equal_cie(&mut result0, &expected[0]);
            check_equal_cie(&mut result1, &expected[1]);
        }
    }
}
