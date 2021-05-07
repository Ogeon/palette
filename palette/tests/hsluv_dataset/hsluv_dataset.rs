//! rev4 of the verification dataset from HSLuv
//! https://github.com/hsluv/hsluv/blob/master/snapshots/snapshot-rev4.json

use lazy_static::lazy_static;
use approx::assert_relative_eq;
use serde_json;

use palette::convert::IntoColorUnclamped;
use palette::white_point::D65;
use palette::{Lch, Hsluv};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct HsluvExample {
    name: String,
    lch: Lch<D65, f64>,
    hsluv: Hsluv<D65, f64>,
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
	let hsluv_data: Vec<f64> = colors["hsluv"].as_array().unwrap().iter().flat_map(|x| x.as_f64()).collect();

	(k.clone(), HsluvExample {
	    name: k.clone(),
	    lch: Lch::new(lch_data[0], lch_data[1], lch_data[2]),
	    hsluv: Hsluv::new(hsluv_data[0], hsluv_data[1], hsluv_data[2]),

	})
    }).collect()

}

lazy_static! {
    static ref TEST_DATA: Examples = load_data();
}

#[test]
pub fn run_lch_to_hsluv_tests() {
    for (_, v) in TEST_DATA.iter() {
	let to_hsluv: Hsluv<D65, f64> = v.lch.into_color_unclamped();
	assert_relative_eq!(to_hsluv, v.hsluv, epsilon = 1.0e-8);
    }
}

#[test]
pub fn run_hsluv_to_lch_tests() {
    for (_, v) in TEST_DATA.iter() {
	let to_lch: Lch<D65, f64> = v.hsluv.into_color_unclamped();
	assert_relative_eq!(to_lch, v.lch, epsilon = 1e-8);
    }
}
