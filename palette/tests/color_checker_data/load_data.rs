use csv;
use serde_derive::Deserialize;

use super::babel::BabelData;
use super::color_checker::ColorCheckerData;

#[derive(Deserialize, PartialEq)]
pub struct ColorCheckerRaw {
    pub color_name: String,
    pub yxy_x: f64,
    pub yxy_y: f64,
    pub yxy_luma: f64,
    pub xyz_x: f64,
    pub xyz_y: f64,
    pub xyz_z: f64,
    pub lab_l: f64,
    pub lab_a: f64,
    pub lab_b: f64,
    pub adobe_r: f64,
    pub adobe_g: f64,
    pub adobe_b: f64,
    pub apple_rgb_r: f64,
    pub apple_rgb_g: f64,
    pub apple_rgb_b: f64,
    pub bestrgb_r: f64,
    pub bestrgb_g: f64,
    pub bestrgb_b: f64,
    pub beta_rgb_r: f64,
    pub beta_rgb_g: f64,
    pub beta_rgb_b: f64,
    pub bruce_rgb_r: f64,
    pub bruce_rgb_g: f64,
    pub bruce_rgb_b: f64,
    pub cie_rgb_r: f64,
    pub cie_rgb_g: f64,
    pub cie_rgb_b: f64,
    pub colormatch_r: f64,
    pub colormatch_g: f64,
    pub colormatch_b: f64,
    pub donrgb4_r: f64,
    pub donrgb4_g: f64,
    pub donrgb4_b: f64,
    pub ecirgb_v2_r: f64,
    pub ecirgb_v2_g: f64,
    pub ecirgb_v2_b: f64,
    pub ekta_space_ps5_r: f64,
    pub ekta_space_ps5_g: f64,
    pub ekta_space_ps5_b: f64,
    pub hdtv_hd_cif_r: f64,
    pub hdtv_hd_cif_g: f64,
    pub hdtv_hd_cif_b: f64,
    pub ntsc_r: f64,
    pub ntsc_g: f64,
    pub ntsc_b: f64,
    pub pal_secam_r: f64,
    pub pal_secam_g: f64,
    pub pal_secam_b: f64,
    pub prophoto_r: f64,
    pub prophoto_g: f64,
    pub prophoto_b: f64,
    pub sgi_r: f64,
    pub sgi_g: f64,
    pub sgi_b: f64,
    pub smpte_240m_r: f64,
    pub smpte_240m_g: f64,
    pub smpte_240m_b: f64,
    pub smpte_c_r: f64,
    pub smpte_c_g: f64,
    pub smpte_c_b: f64,
    pub srgb_r: f64,
    pub srgb_g: f64,
    pub srgb_b: f64,
    pub wide_gamut_r: f64,
    pub wide_gamut_g: f64,
    pub wide_gamut_b: f64,
}

pub fn load_babel() -> Vec<BabelData> {
    let file_name = "tests/color_checker_data/babel.csv";
    let mut rdr = csv::Reader::from_path(file_name)
        .expect("csv file could not be loaded in tests for pointer data");
    let mut color_data: Vec<BabelData> = Vec::new();
    for record in rdr.deserialize() {
        let r: ColorCheckerRaw =
            record.expect("color data could not be decoded in tests for cie 2004 data");
        color_data.push(r.into())
    }
    color_data
}

pub fn load_color_checker() -> Vec<ColorCheckerData> {
    let file_name = "tests/color_checker_data/color_checker.csv";
    let mut rdr = csv::Reader::from_path(file_name)
        .expect("csv file could not be loaded in tests for pointer data");
    let mut color_data: Vec<ColorCheckerData> = Vec::new();
    for record in rdr.deserialize() {
        let r: ColorCheckerRaw =
            record.expect("color data could not be decoded in tests for cie 2004 data");
        color_data.push(r.into())
    }
    color_data
}
