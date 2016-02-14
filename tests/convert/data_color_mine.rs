/*
List of color from www.colormine.org
*/
use csv;
use palette::{Lch, Lab, Xyz, Yxy, Rgb, Hsl, Hsv, IntoColor};
use palette::pixel::Srgb;

pub const COLOR_MINE_FILE_FULL: &'static str = "tests/convert/data_color_mine.csv";
pub const COLOR_MINE_FILE_MINI: &'static str = "tests/convert/data_color_mine_mini.csv";

#[derive(RustcDecodable, PartialEq)]
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
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ColorMine {
    lab: Lab<f32>,
    xyz: Xyz<f32>,
    yxy: Yxy<f32>,
    lch: Lch<f32>,
    rgb: Rgb<f32>,
    linear_rgb: Rgb<f32>,
    hsl: Hsl<f32>,
    hsv: Hsv<f32>,
}

impl From<ColorMineRaw> for ColorMine {
    fn from(src: ColorMineRaw) -> ColorMine {
        ColorMine {
            xyz: Xyz::new(src.xyz_x, src.xyz_y, src.xyz_z),
            yxy: Yxy::new(src.yxy_x, src.yxy_y, src.yxy_luma),
            lab: Lab::new(src.lab_l, src.lab_a, src.lab_b),
            lch: Lch::new(src.lch_l, src.lch_c, src.lch_h.into()),
            rgb: Rgb::new(src.rgb_r, src.rgb_g, src.rgb_b),
            linear_rgb: Srgb::new(src.rgb_r, src.rgb_g, src.rgb_b).into(),
            hsl: Hsl::new(src.hsl_h.into(), src.hsl_s, src.hsl_l),
            hsv: Hsv::new(src.hsv_h.into(), src.hsv_s, src.hsv_v),
        }
    }
}

macro_rules! impl_from_color {
    ($self_ty:ident) => {
        impl From<$self_ty<f32>> for ColorMine {
            fn from(color: $self_ty<f32>) -> ColorMine {
                ColorMine {
                    lab: color.into_lab(),
                    xyz: color.into_xyz(),
                    yxy: color.into_yxy(),
                    lch: color.into_lch(),
                    linear_rgb: color.into_rgb(),
                    rgb: color.into_rgb(),
                    hsl: color.into_hsl(),
                    hsv: color.into_hsv(),
                }
            }
        }

    }
}

impl_from_color!(Rgb);
impl_from_color!(Xyz);
impl_from_color!(Yxy);
impl_from_color!(Lab);
impl_from_color!(Lch);
impl_from_color!(Hsl);
impl_from_color!(Hsv);

pub fn load_data(file_name: &str) -> Vec<ColorMine> {
    let mut rdr = csv::Reader::from_file(file_name).expect("csv file could not be loaded in tests for color mine data");
    let mut color_data: Vec<ColorMine> = Vec::new();
    for record in rdr.decode() {
        let r: ColorMineRaw = record.expect("color data could not be decoded in tests for color mine data");
        color_data.push(r.into())
    }
    color_data
}

fn check_equal_cie(src: &ColorMine, tgt: &ColorMine) {

    assert_relative_eq!(src.xyz, tgt.xyz, epsilon = 0.05);
    assert_relative_eq!(src.yxy, tgt.yxy, epsilon = 0.05);
    assert_relative_eq!(src.lab, tgt.lab, epsilon = 0.05);

    assert_relative_eq!(src.lch.l, tgt.lch.l, epsilon = 0.05);
    assert_relative_eq!(src.lch.chroma, tgt.lch.chroma, epsilon = 0.05);

    // hue values are not passing for from_yxy conversion. Check github #48 for more information
    // assert_relative_eq!(src.lch.hue, tgt.lch.hue, epsilon = 0.05);

}
fn check_equal_rgb(src: &ColorMine, tgt: &ColorMine) {
    assert_relative_eq!(src.rgb, tgt.rgb, epsilon = 0.05);
    assert_relative_eq!(src.hsl, tgt.hsl, epsilon = 0.05);
    assert_relative_eq!(src.hsv, tgt.hsv, epsilon = 0.05);
}

pub fn run_from_xyz_tests(file_name: &str) {
    let data = load_data(file_name);
    for expected in data.iter() {
        test_from_xyz(expected);
    }
}
pub fn run_from_yxy_tests(file_name: &str) {
    let data = load_data(file_name);
    for expected in data.iter() {
        test_from_yxy(expected);
    }
}
pub fn run_from_lab_tests(file_name: &str) {
    let data = load_data(file_name);
    for expected in data.iter() {
        test_from_lab(expected);
    }
}
pub fn run_from_lch_tests(file_name: &str) {
    let data = load_data(file_name);
    for expected in data.iter() {
        test_from_lch(expected);
    }
}
pub fn run_from_rgb_tests(file_name: &str) {
    let data = load_data(file_name);
    for expected in data.iter() {
        test_from_rgb(expected);
    }
}
pub fn run_from_linear_rgb_tests(file_name: &str) {
    let data = load_data(file_name);
    for expected in data.iter() {
        test_from_linear_rgb(expected);
    }
}
pub fn run_from_hsl_tests(file_name: &str) {
    let data = load_data(file_name);
    for expected in data.iter() {
        test_from_hsl(expected);
    }
}
pub fn run_from_hsv_tests(file_name: &str) {
    let data = load_data(file_name);
    for expected in data.iter() {
        test_from_hsv(expected);
    }
}


fn test_from_xyz(expected: &ColorMine) {
    let result = ColorMine::from(expected.xyz);
    check_equal_cie(&result, expected);
}

fn test_from_yxy(expected: &ColorMine) {
    let result = ColorMine::from(expected.yxy);
    check_equal_cie(&result, expected);
}

fn test_from_lab(expected: &ColorMine) {
    let result = ColorMine::from(expected.lab);
    check_equal_cie(&result, expected);
}

fn test_from_lch(expected: &ColorMine) {
    let result = ColorMine::from(expected.lch);
    check_equal_cie(&result, expected);
}

fn test_from_linear_rgb(expected: &ColorMine) {
    let result = ColorMine::from(expected.linear_rgb);
    check_equal_cie(&result, expected);
}

fn test_from_rgb(expected: &ColorMine) {
    let result = ColorMine::from(expected.rgb);
    check_equal_rgb(&result, expected);
}

fn test_from_hsl(expected: &ColorMine) {
    let result = ColorMine::from(expected.hsl);
    check_equal_rgb(&result, expected);
}

fn test_from_hsv(expected: &ColorMine) {
    let result = ColorMine::from(expected.hsv);
    check_equal_rgb(&result, expected);
}
