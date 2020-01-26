/*
Data is the color checker data from
http://www.babelcolor.com/colorchecker-2.htm

The Rgb colors in this data appear to be adapted to the reference white point for the color space

*/

use palette::white_point::D50;
use palette::{IntoColor, Lab, Xyz, Yxy};

use super::load_data::{load_color_checker, ColorCheckerRaw};
use super::MAX_ERROR;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ColorCheckerData {
    yxy: Yxy<D50, f64>,
    xyz: Xyz<D50, f64>,
    lab: Lab<D50, f64>,
}

impl From<ColorCheckerRaw> for ColorCheckerData {
    fn from(src: ColorCheckerRaw) -> ColorCheckerData {
        ColorCheckerData {
            yxy: Yxy::with_wp(src.yxy_x, src.yxy_y, src.yxy_luma),
            xyz: Xyz::with_wp(src.xyz_x, src.xyz_y, src.xyz_z),
            lab: Lab::with_wp(src.lab_l, src.lab_a, src.lab_b),
        }
    }
}

macro_rules! impl_from_color {
    ($self_ty:ident) => {
        impl From<$self_ty<D50, f64>> for ColorCheckerData {
            fn from(color: $self_ty<D50, f64>) -> ColorCheckerData {
                ColorCheckerData {
                    yxy: color.into_yxy(),
                    xyz: color.into_xyz(),
                    lab: color.into_lab(),
                }
            }
        }
    };
}

impl_from_color!(Yxy);
impl_from_color!(Xyz);
impl_from_color!(Lab);

lazy_static! {
    static ref TEST_DATA: Vec<ColorCheckerData> = load_color_checker();
}

fn check_equal(src: &ColorCheckerData, tgt: &ColorCheckerData) {
    assert_relative_eq!(src.xyz, tgt.xyz, epsilon = MAX_ERROR);
    assert_relative_eq!(src.yxy, tgt.yxy, epsilon = MAX_ERROR);
    assert_relative_eq!(src.lab, tgt.lab, epsilon = MAX_ERROR);
}

pub fn run_from_yxy_tests() {
    for expected in TEST_DATA.iter() {
        let result = ColorCheckerData::from(expected.yxy);
        check_equal(&result, expected);
    }
}
pub fn run_from_xyz_tests() {
    for expected in TEST_DATA.iter() {
        let result = ColorCheckerData::from(expected.xyz);
        check_equal(&result, expected);
    }
}
pub fn run_from_lab_tests() {
    for expected in TEST_DATA.iter() {
        let result = ColorCheckerData::from(expected.lab);
        check_equal(&result, expected);
    }
}
