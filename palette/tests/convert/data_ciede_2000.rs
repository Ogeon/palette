/*
Data from http://www2.ece.rochester.edu/~gsharma/ciede2000/

Tests Lab color differences with expected delta E*

Note: Test uses `f64` because `f32` failed Travis CI builds on Linux for Lch on
        case 13 or 14 which is noted in the paper as testing accuracy of hue
        angle and atan calcuation (calculated: 4.7460666, expected: 4.8045).
        MacOS and Windows passed the tests so be wary when using f32 on Linux.
*/

extern crate approx;

use csv;
use palette::white_point::D65;
use palette::ColorDifference;
use palette::{Lab, Lch};

#[derive(Deserialize, PartialEq)]
struct Cie2000Raw {
    lab1_l: f64,
    lab1_a: f64,
    lab1_b: f64,
    lab2_l: f64,
    lab2_a: f64,
    lab2_b: f64,
    delta_e: f64,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Cie2000 {
    c1: Lab<D65, f64>,
    c2: Lab<D65, f64>,
    delta_e: f64,
}

impl From<Cie2000Raw> for Cie2000 {
    fn from(src: Cie2000Raw) -> Cie2000 {
        Cie2000 {
            c1: Lab::new(src.lab1_l, src.lab1_a, src.lab1_b),
            c2: Lab::new(src.lab2_l, src.lab2_a, src.lab2_b),
            delta_e: src.delta_e,
        }
    }
}

fn load_data() -> Vec<Cie2000> {
    let file_name = "tests/convert/data_ciede_2000.csv";
    let mut rdr = csv::Reader::from_path(file_name)
        .expect("csv file could not be loaded in tests for cie 2000 data");
    let mut color_data: Vec<Cie2000> = Vec::new();
    for record in rdr.deserialize() {
        let r: Cie2000Raw =
            record.expect("color data could not be decoded in tests for cie 2000 data");
        color_data.push(r.into())
    }
    color_data
}

fn check_equal_lab(result: f64, expected: f64) {
    assert_relative_eq!(result, expected, epsilon = 0.0001);
}

fn check_equal_lch(result: f64, expected: f64) {
    assert_relative_eq!(result, expected, epsilon = 0.0001);
}

pub fn run_tests() {
    let data = load_data();

    for expected in data.iter() {
        let result_lab = expected.c1.get_color_difference(&expected.c2);
        check_equal_lab(result_lab, expected.delta_e);

        let lch1: Lch<_, f64> = Lch::from(expected.c1);
        let lch2: Lch<_, f64> = Lch::from(expected.c2);
        let result_lch = lch1.get_color_difference(&lch2);
        check_equal_lch(result_lch, expected.delta_e);
    }
}
