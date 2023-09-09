mod babel;
mod color_checker;
mod load_data;

const MAX_ERROR: f64 = 0.000000000001;

#[test]
pub fn babel_from_yxy() {
    babel::run_from_yxy_tests();
}
#[test]
pub fn babel_from_xyz() {
    babel::run_from_xyz_tests();
}
#[test]
pub fn babel_from_lab() {
    babel::run_from_lab_tests();
}

#[test]
pub fn color_checker_from_yxy() {
    color_checker::run_from_yxy_tests();
}
#[test]
pub fn color_checker_from_xyz() {
    color_checker::run_from_xyz_tests();
}
#[test]
pub fn color_checker_from_lab() {
    color_checker::run_from_lab_tests();
}

pub mod wide {
    #[test]
    pub fn babel_from_yxy() {
        super::babel::wide_f64x2::run_from_yxy_tests();
    }
    #[test]
    pub fn babel_from_xyz() {
        super::babel::wide_f64x2::run_from_xyz_tests();
    }
    #[test]
    pub fn babel_from_lab() {
        super::babel::wide_f64x2::run_from_lab_tests();
    }

    #[test]
    pub fn color_checker_from_yxy() {
        super::color_checker::wide_f64x2::run_from_yxy_tests();
    }
    #[test]
    pub fn color_checker_from_xyz() {
        super::color_checker::wide_f64x2::run_from_xyz_tests();
    }
    #[test]
    pub fn color_checker_from_lab() {
        super::color_checker::wide_f64x2::run_from_lab_tests();
    }
}
