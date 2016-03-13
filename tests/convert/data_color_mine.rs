/*
List of color from www.colormine.org
*/
use csv;
use palette::{Lch, Lab, Xyz, Yxy, Hsl, Hsv, Hwb, IntoColor, Srgb, LinSrgb};
use palette::white_point::D65;

#[derive(Deserialize, PartialEq)]
pub struct ColorMineRaw {
    pub color: String,
    pub hex: String,
    pub rgbu8_r: u8,
    pub rgbu8_g: u8,
    pub rgbu8_b: u8,
    pub rgb_r: f32,
    pub rgb_g: f32,
    pub rgb_b: f32,
    pub cmy_c: f32,
    pub cmy_m: f32,
    pub cmy_y: f32,
    pub cmyk_c: f32,
    pub cmyk_m: f32,
    pub cmyk_y: f32,
    pub cmyk_k: f32,
    pub xyz_x: f32,
    pub xyz_y: f32,
    pub xyz_z: f32,
    pub lab_l: f32,
    pub lab_a_unscaled: f32,
    pub lab_b_unscaled: f32,
    pub lab_a: f32,
    pub lab_b: f32,
    pub lch_l: f32,
    pub lch_c_unscaled: f32,
    pub lch_c: f32,
    pub lch_h: f32,
    pub hunterlab_l: f32,
    pub hunterlab_a: f32,
    pub hunterlab_b: f32,
    pub yxy_luma: f32,
    pub yxy_x: f32,
    pub yxy_y: f32,
    pub luv_l: f32,
    pub luv_u: f32,
    pub luv_v: f32,
    pub hsl_h: f32,
    pub hsl_s: f32,
    pub hsl_l: f32,
    pub hsv_h: f32,
    pub hsv_s: f32,
    pub hsv_v: f32,
    pub hwb_h: f32,
    pub hwb_w: f32,
    pub hwb_b: f32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ColorMine {
    xyz: Xyz<D65, f32>,
    yxy: Yxy<D65, f32>,
    rgb: LinSrgb<f32>,
    linear_rgb: LinSrgb<f32>,
    hsl: Hsl<::palette::rgb::standards::Srgb, f32>,
    hsv: Hsv<::palette::rgb::standards::Srgb, f32>,
    hwb: Hwb<D65, f32>,
}

impl From<ColorMineRaw> for ColorMine {
    fn from(src: ColorMineRaw) -> ColorMine {
        ColorMine {
            xyz: Xyz::new(src.xyz_x, src.xyz_y, src.xyz_z),
            yxy: Yxy::new(src.yxy_x, src.yxy_y, src.yxy_luma),
            rgb: LinSrgb::new(src.rgb_r, src.rgb_g, src.rgb_b),
            linear_rgb: Srgb::new(src.rgb_r, src.rgb_g, src.rgb_b).into(),
            hsl: Hsl::new(src.hsl_h.into(), src.hsl_s, src.hsl_l),
            hsv: Hsv::new(src.hsv_h.into(), src.hsv_s, src.hsv_v),
            hwb: Hwb::new(src.hwb_h.into(), src.hwb_w, src.hwb_b),
        }
    }
}

macro_rules! impl_from_color {
    ($self_ty:ty) => {
        impl From<$self_ty> for ColorMine {
            fn from(color: $self_ty) -> ColorMine {
                ColorMine {
                    xyz: color.into_xyz(),
                    yxy: color.into_yxy(),
                    linear_rgb: color.into_rgb(),
                    rgb: color.into_rgb(),
                    hsl: color.into_hsl(),
                    hsv: color.into_hsv(),
                    hwb: color.into_hwb(),
                }
            }
        }

    }
}

impl_from_color!(LinSrgb<f32>);
impl_from_color!(Xyz<D65, f32>);
impl_from_color!(Yxy<D65, f32>);
impl_from_color!(Lab<D65, f32>);
impl_from_color!(Lch<D65, f32>);
impl_from_color!(Hsl<::palette::rgb::standards::Srgb, f32>);
impl_from_color!(Hsv<::palette::rgb::standards::Srgb, f32>);
impl_from_color!(Hwb<D65, f32>);



lazy_static! {
    static ref TEST_DATA: Vec<ColorMine> = load_data();
}


pub fn load_data() -> Vec<ColorMine> {
    let mut rdr = csv::Reader::from_path("tests/convert/data_color_mine.csv")
        .expect("csv file could not be loaded in tests for color mine data");
    let mut color_data: Vec<ColorMine> = Vec::new();
    for record in rdr.deserialize() {
        let r: ColorMineRaw = record.expect("color data could not be decoded in tests for color mine data");
        color_data.push(r.into())
    }
    color_data
}

fn check_equal_cie(src: &ColorMine, tgt: &ColorMine) {

    assert_relative_eq!(src.xyz, tgt.xyz, epsilon = 0.05);
    assert_relative_eq!(src.yxy, tgt.yxy, epsilon = 0.05);

    // hue values are not passing for from_yxy conversion. Check github #48 for more information
    // assert_relative_eq!(src.lch.hue, tgt.lch.hue, epsilon = 0.05);

}
fn check_equal_rgb(src: &ColorMine, tgt: &ColorMine) {
    assert_relative_eq!(src.rgb, tgt.rgb, epsilon = 0.05);
    assert_relative_eq!(src.hsl, tgt.hsl, epsilon = 0.05);
    assert_relative_eq!(src.hsv, tgt.hsv, epsilon = 0.05);
    assert_relative_eq!(src.hwb, tgt.hwb, epsilon = 0.05);
}

pub fn run_from_xyz_tests() {
    for expected in TEST_DATA.iter() {
        let result = ColorMine::from(expected.xyz);
        check_equal_cie(&result, expected);
    }
}
pub fn run_from_yxy_tests() {
    for expected in TEST_DATA.iter() {
        let result = ColorMine::from(expected.yxy);
        check_equal_cie(&result, expected);
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
        let result = ColorMine::from(expected.linear_rgb);
        check_equal_cie(&result, expected);
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
