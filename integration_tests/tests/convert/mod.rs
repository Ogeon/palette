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
pub fn color_mine_from_lab() {
    data_color_mine::run_from_lab_tests();
}
#[test]
pub fn color_mine_from_lch() {
    data_color_mine::run_from_lch_tests();
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

pub mod wide {
    #[test]
    pub fn xyz_yxy_conversion() {
        super::data_cie_15_2004::wide_f32x4::run_tests();
    }

    #[test]
    pub fn color_mine_from_xyz() {
        super::data_color_mine::wide_f64x2::run_from_xyz_tests();
    }
    #[test]
    pub fn color_mine_from_yxy() {
        super::data_color_mine::wide_f64x2::run_from_yxy_tests();
    }
    #[test]
    pub fn color_mine_from_linear_rgb() {
        super::data_color_mine::wide_f64x2::run_from_linear_rgb_tests();
    }
    #[test]
    pub fn color_mine_from_rgb() {
        super::data_color_mine::wide_f64x2::run_from_rgb_tests();
    }
    #[test]
    pub fn color_mine_from_hsl() {
        super::data_color_mine::wide_f64x2::run_from_hsl_tests();
    }
    #[test]
    pub fn color_mine_from_hsv() {
        super::data_color_mine::wide_f64x2::run_from_hsv_tests();
    }
    #[test]
    pub fn color_mine_from_hwb() {
        super::data_color_mine::wide_f64x2::run_from_hwb_tests();
    }
    #[test]
    pub fn color_mine_from_lab() {
        super::data_color_mine::wide_f64x2::run_from_lab_tests();
    }
    #[test]
    pub fn color_mine_from_lch() {
        super::data_color_mine::wide_f64x2::run_from_lch_tests();
    }
}
