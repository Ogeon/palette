mod data_cie_15_2004;
mod data_ciede_2000;
mod data_color_mine;
mod lab_lch;

#[test]
pub fn xyz_yxy_conversion() {
    data_cie_15_2004::run_tests();
}

#[test]
pub fn color_difference_ciede() {
    data_ciede_2000::run_tests();
}

#[test]
pub fn color_mine_from_xyz() {
    data_color_mine::run_from_xyz_tests();
}
#[test]
pub fn color_mine_from_yxy() {
    data_color_mine::run_from_yxy_tests();
}
#[test]
pub fn color_mine_from_linear_rgb() {
    data_color_mine::run_from_linear_rgb_tests();
}
#[test]
pub fn color_mine_from_rgb() {
    data_color_mine::run_from_rgb_tests();
}
#[test]
pub fn color_mine_from_hsl() {
    data_color_mine::run_from_hsl_tests();
}
#[test]
pub fn color_mine_from_hsv() {
    data_color_mine::run_from_hsv_tests();
}
#[test]
pub fn color_mine_from_hwb() {
    data_color_mine::run_from_hwb_tests();
}
