/*
Data is the illuminant data for XYZ and YXY spaces from
CIE Technical Report Colorimetry 3rd Edition (CIE 15 :2004)
https://law.resource.org/pub/us/cfr/ibr/003/cie.15.2004.pdf

Tests XYZ and YXY conversion
*/

use approx::assert_relative_eq;
use csv;
use serde_derive::Deserialize;

use palette::{convert::IntoColorUnclamped, num::IntoScalarArray, white_point::D65, Xyz, Yxy};

#[derive(Deserialize, PartialEq)]
struct Cie2004Raw {
    xyz_x: f32,
    xyz_y: f32,
    xyz_z: f32,
    yxy_x: f32,
    yxy_y: f32,
    yxy_luma: f32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Cie2004<T = f32> {
    xyz: Xyz<D65, T>,
    yxy: Yxy<D65, T>,
}

impl From<Cie2004Raw> for Cie2004 {
    fn from(src: Cie2004Raw) -> Cie2004 {
        Cie2004 {
            xyz: Xyz::new(src.xyz_x, src.xyz_y, src.xyz_z),
            yxy: Yxy::new(src.yxy_x, src.yxy_y, src.yxy_luma),
        }
    }
}

macro_rules! impl_from_color_pointer {
    ($self_ty:ident) => {
        impl<T> From<$self_ty<D65, T>> for Cie2004<T>
        where
            T: Copy,
            $self_ty<D65, T>: IntoColorUnclamped<Xyz<D65, T>> + IntoColorUnclamped<Yxy<D65, T>>,
        {
            fn from(color: $self_ty<D65, T>) -> Cie2004<T> {
                Cie2004 {
                    xyz: color.into_color_unclamped(),
                    yxy: color.into_color_unclamped(),
                }
            }
        }
    };
}

impl_from_color_pointer!(Xyz);
impl_from_color_pointer!(Yxy);

impl<V> Into<[Cie2004<V::Scalar>; 4]> for Cie2004<V>
where
    V: IntoScalarArray<4>,
    Xyz<D65, V>: Into<[Xyz<D65, V::Scalar>; 4]>,
    Yxy<D65, V>: Into<[Yxy<D65, V::Scalar>; 4]>,
{
    fn into(self) -> [Cie2004<V::Scalar>; 4] {
        let [xyz0, xyz1, xyz2, xyz3]: [_; 4] = self.xyz.into();
        let [yxy0, yxy1, yxy2, yxy3]: [_; 4] = self.yxy.into();

        [
            Cie2004 {
                xyz: xyz0,
                yxy: yxy0,
            },
            Cie2004 {
                xyz: xyz1,
                yxy: yxy1,
            },
            Cie2004 {
                xyz: xyz2,
                yxy: yxy2,
            },
            Cie2004 {
                xyz: xyz3,
                yxy: yxy3,
            },
        ]
    }
}

fn load_data() -> Vec<Cie2004> {
    let file_name = "tests/convert/data_cie_15_2004.csv";
    let mut rdr = csv::Reader::from_path(file_name)
        .expect("csv file could not be loaded in tests for cie 2004 data");
    let mut color_data: Vec<Cie2004> = Vec::new();
    for record in rdr.deserialize() {
        let r: Cie2004Raw =
            record.expect("color data could not be decoded in tests for cie 2004 data");
        color_data.push(r.into())
    }
    color_data
}

fn check_equal(src: &Cie2004, tgt: &Cie2004) {
    assert_relative_eq!(src.xyz, tgt.xyz, epsilon = 0.0001);
    assert_relative_eq!(src.yxy, tgt.yxy, epsilon = 0.0001);
}

pub fn run_tests() {
    let data = load_data();

    for expected in data.iter() {
        let result_xyz = Cie2004::from(expected.xyz);
        check_equal(&result_xyz, expected);

        let result_yxy = Cie2004::from(expected.yxy);
        check_equal(&result_yxy, expected);
    }
}

#[cfg(feature = "wide")]
pub mod wide_f32x4 {
    use super::*;

    pub fn run_tests() {
        let data = load_data();

        for expected in data.chunks_exact(4) {
            let result_xyz = Cie2004::<wide::f32x4>::from(Xyz::from([
                expected[0].xyz,
                expected[1].xyz,
                expected[2].xyz,
                expected[3].xyz,
            ]));
            let [result_xyz0, result_xyz1, result_xyz2, result_xyz3]: [Cie2004; 4] =
                result_xyz.into();
            check_equal(&result_xyz0, &expected[0]);
            check_equal(&result_xyz1, &expected[1]);
            check_equal(&result_xyz2, &expected[2]);
            check_equal(&result_xyz3, &expected[3]);

            let result_yxy = Cie2004::<wide::f32x4>::from(Yxy::from([
                expected[0].yxy,
                expected[1].yxy,
                expected[2].yxy,
                expected[3].yxy,
            ]));
            let [result_yxy0, result_yxy1, result_yxy2, result_yxy3]: [Cie2004; 4] =
                result_yxy.into();
            check_equal(&result_yxy0, &expected[0]);
            check_equal(&result_yxy1, &expected[1]);
            check_equal(&result_yxy2, &expected[2]);
            check_equal(&result_yxy3, &expected[3]);
        }
    }
}
