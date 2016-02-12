mod data_cie_15_2004;
mod data_color_mine;
mod lab_lch;

#[test]
pub fn xyz_yxy_conversion() {
    data_cie_15_2004::run_tests();
}

#[test]
pub fn color_mine_from_xyz() {
    data_color_mine::run_from_xyz_tests(data_color_mine::COLOR_MINE_FILE_MINI);
}
#[test]
pub fn color_mine_from_yxy() {
    data_color_mine::run_from_yxy_tests(data_color_mine::COLOR_MINE_FILE_MINI);
}
#[test]
pub fn color_mine_from_lab() {
    data_color_mine::run_from_lab_tests(data_color_mine::COLOR_MINE_FILE_MINI);
}
#[test]
pub fn color_mine_from_lch() {
    data_color_mine::run_from_lch_tests(data_color_mine::COLOR_MINE_FILE_MINI);
}
#[test]
pub fn color_mine_from_linear_rgb() {
    data_color_mine::run_from_linear_rgb_tests(data_color_mine::COLOR_MINE_FILE_MINI);
}
#[test]
pub fn color_mine_from_rgb() {
    data_color_mine::run_from_rgb_tests(data_color_mine::COLOR_MINE_FILE_MINI);
}
#[test]
pub fn color_mine_from_hsl() {
    data_color_mine::run_from_hsl_tests(data_color_mine::COLOR_MINE_FILE_MINI);
}
#[test]
pub fn color_mine_from_hsv() {
    data_color_mine::run_from_hsv_tests(data_color_mine::COLOR_MINE_FILE_MINI);
}


#[test]
pub fn color_mine_from_xyz_full() {
    data_color_mine::run_from_xyz_tests(data_color_mine::COLOR_MINE_FILE_FULL);
}
#[test]
pub fn color_mine_from_yxy_full() {
    data_color_mine::run_from_yxy_tests(data_color_mine::COLOR_MINE_FILE_FULL);
}
#[test]
pub fn color_mine_from_lab_full() {
    data_color_mine::run_from_lab_tests(data_color_mine::COLOR_MINE_FILE_FULL);
}
#[test]
pub fn color_mine_from_lch_full() {
    data_color_mine::run_from_lch_tests(data_color_mine::COLOR_MINE_FILE_FULL);
}
#[test]
pub fn color_mine_from_linear_rgb_full() {
    data_color_mine::run_from_linear_rgb_tests(data_color_mine::COLOR_MINE_FILE_FULL);
}
#[test]
pub fn color_mine_from_rgb_full() {
    data_color_mine::run_from_rgb_tests(data_color_mine::COLOR_MINE_FILE_FULL);
}
#[test]
pub fn color_mine_from_hsl_full() {
    data_color_mine::run_from_hsl_tests(data_color_mine::COLOR_MINE_FILE_FULL);
}
#[test]
pub fn color_mine_from_hsv_full() {
    data_color_mine::run_from_hsv_tests(data_color_mine::COLOR_MINE_FILE_FULL);
}
