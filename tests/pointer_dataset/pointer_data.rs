/*
Data is the Pointer data set
https://www.rit.edu/cos/colorscience/rc_useful_data.php

White Point for the data is (using C illuminant)
Xn	Yn	Zn
SC		100	118.2254189827
x, y		0.310	0.3161578637
u', v'		0.2008907213	0.4608888395

Note: The xyz and yxy conversions do not use the updated conversion formula. So they are not used.
*/

use num::{Float, ToPrimitive, NumCast};
use csv;
use palette::{Xyz, Lch, Lab, IntoColor};
use palette::white_point::WhitePoint;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PointerWP;
impl<T: Float> WhitePoint<T> for PointerWP {
    fn get_xyz() -> Xyz<Self, T> {
        Xyz::with_wp(flt(0.980722647624), T::one(), flt(1.182254189827))
    }
}

///A convenience function to convert a constant number to Float Type
fn flt<T: Float, P: ToPrimitive>(prim: P) -> T {
    NumCast::from(prim).unwrap()
}

#[derive(RustcDecodable, PartialEq)]
struct PointerDataRaw{
    lch_l: f64,
    lch_c: f64,
    lch_h: f64,
    lab_l: f64,
    lab_a: f64,
    lab_b: f64,
    luv_l: f64,
    luv_u: f64,
    luv_v: f64,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct PointerData {
    lch: Lch<PointerWP, f64>,
    lab: Lab<PointerWP, f64>,
}

impl From<PointerDataRaw> for PointerData {
    fn from(src: PointerDataRaw) -> PointerData {
        PointerData {
            lch: Lch::with_wp(src.lch_l, src.lch_c, src.lch_h.into()),
            lab: Lab::with_wp(src.lab_l, src.lab_a, src.lab_b),
        }
    }
}

macro_rules! impl_from_color_pointer {
    ($self_ty:ident) => {
        impl From<$self_ty<PointerWP, f64>> for PointerData {
            fn from(color: $self_ty<PointerWP, f64>) -> PointerData {
                PointerData {
                    lch: color.into_lch(),
                    lab: color.into_lab(),
                }
            }
        }

    }
}

impl_from_color_pointer!(Lch);
impl_from_color_pointer!(Lab);

lazy_static! {
    static ref TEST_DATA: Vec<PointerData> = load_data();
}


fn load_data() -> Vec<PointerData> {
    let file_name = "tests/pointer_dataset/pointer_data.csv";
    let mut rdr = csv::Reader::from_file(file_name).expect("csv file could not be loaded in tests for pointer data");
    let mut color_data: Vec<PointerData> = Vec::new();
    for record in rdr.decode() {
        let r: PointerDataRaw = record.expect("color data could not be decoded in tests for cie 2004 data");
        color_data.push(r.into())
    }
    color_data
}

fn check_equal(src: &PointerData, tgt: &PointerData) {
    assert_relative_eq!(src.lch, tgt.lch, epsilon = 0.000000000001);
    assert_relative_eq!(src.lab, tgt.lab, epsilon = 0.000000000001);
}

pub fn run_from_lch_tests() {
    for expected in TEST_DATA.iter() {
        let result = PointerData::from(expected.lch);
        check_equal(&result, expected);
    }
}
pub fn run_from_lab_tests() {
    for expected in TEST_DATA.iter() {
        let result = PointerData::from(expected.lab);
        check_equal(&result, expected);
    }
}
