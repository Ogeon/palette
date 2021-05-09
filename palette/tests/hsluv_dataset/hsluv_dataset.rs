//! rev4 of the verification dataset from HSLuv
//! https://github.com/hsluv/hsluv/blob/master/snapshots/snapshot-rev4.json

use approx::assert_relative_eq;
use lazy_static::lazy_static;
use serde_json;

use palette::convert::IntoColorUnclamped;
use palette::white_point::D65;
use palette::{Lchuv, Luv, LuvHue, Xyz};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct HsluvExample {
    name: String,
    lchuv: Lchuv<D65, f64>,
    luv: Luv<D65, f64>,
    xyz: Xyz<D65, f64>,
}

type Examples = HashMap<String, HsluvExample>;

fn load_data() -> Examples {
    let filename = "tests/hsluv_dataset/hsluv_dataset.json";
    let data_str = std::fs::read_to_string(filename).unwrap();
    let raw_data: serde_json::Value = serde_json::from_str(&data_str).unwrap();

    let m = raw_data.as_object().expect("failed to parse dataset");
    m.iter()
        .map(|(k, v)| {
            let colors = v.as_object().unwrap();
            let luv_data: Vec<f64> = colors["luv"]
                .as_array()
                .unwrap()
                .iter()
                .flat_map(|x| x.as_f64())
                .collect();
            let lchuv_data: Vec<f64> = colors["lch"]
                .as_array()
                .unwrap()
                .iter()
                .flat_map(|x| x.as_f64())
                .collect();
            let xyz_data: Vec<f64> = colors["xyz"]
                .as_array()
                .unwrap()
                .iter()
                .flat_map(|x| x.as_f64())
                .collect();

            (
                k.clone(),
                HsluvExample {
                    name: k.clone(),
                    luv: Luv::new(luv_data[0], luv_data[1], luv_data[2]),
                    lchuv: Lchuv::new(lchuv_data[0], lchuv_data[1], lchuv_data[2]),
                    xyz: Xyz::new(xyz_data[0], xyz_data[1], xyz_data[2]),
                },
            )
        })
        .collect()
}

lazy_static! {
    static ref TEST_DATA: Examples = load_data();
}

#[test]
pub fn run_xyz_to_luv_tests() {
    for (_, v) in TEST_DATA.iter() {
        let to_luv: Luv<D65, f64> = v.xyz.into_color_unclamped();
        assert_relative_eq!(to_luv, v.luv, epsilon = 0.1);
    }
}

#[test]
pub fn run_luv_to_xyz_tests() {
    for (_, v) in TEST_DATA.iter() {
        let to_xyz: Xyz<D65, f64> = v.luv.into_color_unclamped();
        assert_relative_eq!(to_xyz, v.xyz, epsilon = 0.001);
    }
}

#[test]
pub fn run_lchuv_to_luv_tests() {
    for (_, v) in TEST_DATA.iter() {
        let to_luv: Luv<D65, f64> = v.lchuv.into_color_unclamped();
        assert_relative_eq!(to_luv, v.luv, epsilon = 0.1);
    }
}

#[test]
pub fn run_luv_to_lchuv_tests() {
    for (_, v) in TEST_DATA.iter() {
        let mut to_lchuv: Lchuv<D65, f64> = v.luv.into_color_unclamped();
        if to_lchuv.chroma < 1e-8 {
            to_lchuv.hue = LuvHue::from_degrees(0.0);
        }
        assert_relative_eq!(to_lchuv, v.lchuv, epsilon = 0.001);
    }
}
