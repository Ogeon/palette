/*
Data is the color checker data from
http://www.babelcolor.com/colorchecker-2.htm

The Rgb colors in this data appear to be adapted to the D50 white_point from the reference white point for the color space

*/

use palette::{Xyz, Yxy, Lab, IntoColor};
use palette::white_point::D50;

use super::load_data::{ColorCheckerRaw, load_babel};
use super::MAX_ERROR;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BabelData {
    yxy: Yxy<D50, f64>,
    xyz: Xyz<D50, f64>,
    lab: Lab<D50, f64>,
}


impl From<ColorCheckerRaw> for BabelData {
    fn from(src: ColorCheckerRaw) -> BabelData {
        BabelData {
            yxy: Yxy::with_wp(src.yxy_x, src.yxy_y, src.yxy_luma),
            xyz: Xyz::with_wp(src.xyz_x, src.xyz_y, src.xyz_z),
            lab: Lab::with_wp(src.lab_l, src.lab_a, src.lab_b),
        }
    }
}

macro_rules! impl_from_color {
    ($self_ty:ident) => {
        impl From<$self_ty<D50, f64>> for BabelData {
            fn from(color: $self_ty<D50, f64>) -> BabelData {
                BabelData {
                    yxy: color.into_yxy(),
                    xyz: color.into_xyz(),
                    lab: color.into_lab(),
                }
            }
        }

    }
}

impl_from_color!(Yxy);
impl_from_color!(Xyz);
impl_from_color!(Lab);

lazy_static! {
    static ref TEST_DATA: Vec<BabelData> = load_babel();
}

fn check_equal(src: &BabelData, tgt: &BabelData) {
    assert_relative_eq!(src.xyz, tgt.xyz, epsilon = MAX_ERROR);
    assert_relative_eq!(src.yxy, tgt.yxy, epsilon = MAX_ERROR);
    assert_relative_eq!(src.lab, tgt.lab, epsilon = MAX_ERROR);
}

pub fn run_from_yxy_tests() {
    for expected in TEST_DATA.iter() {
        let result = BabelData::from(expected.yxy);
        check_equal(&result, expected);
    }
}
pub fn run_from_xyz_tests() {
    for expected in TEST_DATA.iter() {
        let result = BabelData::from(expected.xyz);
        check_equal(&result, expected);
    }
}
pub fn run_from_lab_tests() {
    for expected in TEST_DATA.iter() {
        let result = BabelData::from(expected.lab);
        check_equal(&result, expected);
    }
}
