/*
Data is the illuminant data for XYZ and YXY spaces from
CIE Technical Report Colorimetry 3rd Edition (CIE 15 :2004)
https://law.resource.org/pub/us/cfr/ibr/003/cie.15.2004.pdf

Tests XYZ and YXY conversion
*/

use csv;
use palette::white_point::D65;
use palette::{IntoColor, Xyz, Yxy};

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
struct Cie2004 {
    xyz: Xyz<D65, f32>,
    yxy: Yxy<D65, f32>,
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
        impl From<$self_ty> for Cie2004 {
            fn from(color: $self_ty) -> Cie2004 {
                Cie2004 {
                    xyz: color.into_xyz(),
                    yxy: color.into_yxy(),
                }
            }
        }
    };
}

impl_from_color_pointer!(Xyz);
impl_from_color_pointer!(Yxy);

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
