mod babel;
mod color_checker;
mod load_data;

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
