/*
Data is the color checker data from
http://www.babelcolor.com/colorchecker-2.htm

The Rgb colors in this data appear to be adapted to the reference white point for the color space

*/

use approx::assert_relative_eq;
use lazy_static::lazy_static;

use palette::{convert::IntoColorUnclamped, num::IntoScalarArray, white_point::D50, Lab, Xyz, Yxy};

use super::load_data::{load_color_checker, ColorCheckerRaw};
use super::MAX_ERROR;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ColorCheckerData<T = f64> {
    yxy: Yxy<D50, T>,
    xyz: Xyz<D50, T>,
    lab: Lab<D50, T>,
}

impl From<ColorCheckerRaw> for ColorCheckerData {
    fn from(src: ColorCheckerRaw) -> ColorCheckerData {
        ColorCheckerData {
            yxy: Yxy::new(src.yxy_x, src.yxy_y, src.yxy_luma),
            xyz: Xyz::new(src.xyz_x, src.xyz_y, src.xyz_z),
            lab: Lab::new(src.lab_l, src.lab_a, src.lab_b),
        }
    }
}

macro_rules! impl_from_color {
    ($self_ty:ident) => {
        impl<T> From<$self_ty<D50, T>> for ColorCheckerData<T>
        where
            T: Copy,
            $self_ty<D50, T>: IntoColorUnclamped<Yxy<D50, T>>
                + IntoColorUnclamped<Xyz<D50, T>>
                + IntoColorUnclamped<Lab<D50, T>>,
        {
            fn from(color: $self_ty<D50, T>) -> ColorCheckerData<T> {
                ColorCheckerData {
                    yxy: color.into_color_unclamped(),
                    xyz: color.into_color_unclamped(),
                    lab: color.into_color_unclamped(),
                }
            }
        }
    };
}

impl_from_color!(Yxy);
impl_from_color!(Xyz);
impl_from_color!(Lab);

impl<V> From<ColorCheckerData<V>> for [ColorCheckerData<V::Scalar>; 2]
where
    V: IntoScalarArray<2>,
    Xyz<D50, V>: Into<[Xyz<D50, V::Scalar>; 2]>,
    Yxy<D50, V>: Into<[Yxy<D50, V::Scalar>; 2]>,
    Lab<D50, V>: Into<[Lab<D50, V::Scalar>; 2]>,
{
    fn from(color_data: ColorCheckerData<V>) -> Self {
        let [xyz0, xyz1]: [_; 2] = color_data.xyz.into();
        let [yxy0, yxy1]: [_; 2] = color_data.yxy.into();
        let [lab0, lab1]: [_; 2] = color_data.lab.into();

        [
            ColorCheckerData {
                xyz: xyz0,
                yxy: yxy0,
                lab: lab0,
            },
            ColorCheckerData {
                xyz: xyz1,
                yxy: yxy1,
                lab: lab1,
            },
        ]
    }
}

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

pub mod wide_f64x2 {
    use super::*;

    pub fn run_from_yxy_tests() {
        for expected in TEST_DATA.chunks_exact(2) {
            let [result0, result1]: [ColorCheckerData; 2] =
                ColorCheckerData::from(Yxy::<_, wide::f64x2>::from([
                    expected[0].yxy,
                    expected[1].yxy,
                ]))
                .into();
            check_equal(&result0, &expected[0]);
            check_equal(&result1, &expected[1]);
        }
    }
    pub fn run_from_xyz_tests() {
        for expected in TEST_DATA.chunks_exact(2) {
            let [result0, result1]: [ColorCheckerData; 2] =
                ColorCheckerData::from(Xyz::<_, wide::f64x2>::from([
                    expected[0].xyz,
                    expected[1].xyz,
                ]))
                .into();
            check_equal(&result0, &expected[0]);
            check_equal(&result1, &expected[1]);
        }
    }
    pub fn run_from_lab_tests() {
        for expected in TEST_DATA.chunks_exact(2) {
            let [result0, result1]: [ColorCheckerData; 2] =
                ColorCheckerData::from(Lab::<_, wide::f64x2>::from([
                    expected[0].lab,
                    expected[1].lab,
                ]))
                .into();
            check_equal(&result0, &expected[0]);
            check_equal(&result1, &expected[1]);
        }
    }
}
