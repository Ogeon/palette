//! rev4 of the verification dataset from HSLuv
//! https://github.com/hsluv/hsluv/blob/master/snapshots/snapshot-rev4.json

use lazy_static::lazy_static;
use approx::assert_relative_eq;
use serde_json;

use palette::convert::IntoColorUnclamped;
use palette::white_point::D65;
use palette::{Lch, Luv, Xyz};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct HsluvExample {
    name: String,
    lch: Lch<D65, f64>,
    luv: Luv<D65, f64>,
    xyz: Xyz<D65, f64>,
}

type Examples = HashMap<String, HsluvExample>;

fn load_data() -> Examples {
    let filename = "tests/hsluv_dataset/hsluv_dataset.json";
    let data_str = std::fs::read_to_string(filename).unwrap();
    let raw_data: serde_json::Value = serde_json::from_str(&data_str).unwrap();


    let m = raw_data.as_object().expect("failed to parse dataset");
    m.iter().map(|(k, v)| {
	let colors = v.as_object().unwrap();
	let lch_data: Vec<f64> = colors["lch"].as_array().unwrap().iter().flat_map(|x| x.as_f64()).collect();
	let luv_data: Vec<f64> = colors["luv"].as_array().unwrap().iter().flat_map(|x| x.as_f64()).collect();
	let xyz_data: Vec<f64> = colors["xyz"].as_array().unwrap().iter().flat_map(|x| x.as_f64()).collect();

	(k.clone(), HsluvExample {
	    name: k.clone(),
	    lch: Lch::new(lch_data[0], lch_data[1], lch_data[2]),
	    luv: Luv::new(luv_data[0], luv_data[1], luv_data[2]),
	    xyz: Xyz::new(xyz_data[0], xyz_data[1], xyz_data[2]),
	})
    }).collect()

}

lazy_static! {
    static ref TEST_DATA: Examples = load_data();
}

#[test]
pub fn run_xyz_to_luv_tests() {
    for (_, v) in TEST_DATA.iter() {
	println!("{:?}", v.xyz);
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
