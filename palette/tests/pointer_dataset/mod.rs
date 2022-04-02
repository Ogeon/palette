mod pointer_data;

#[test]
pub fn from_lab() {
    pointer_data::run_from_lab_tests();
}
#[test]
pub fn from_lch() {
    pointer_data::run_from_lch_tests();
}

#[cfg(feature = "wide")]
mod wide {
    #[test]
    pub fn from_lab() {
        super::pointer_data::wide_f64x2::run_from_lab_tests();
    }
    #[test]
    pub fn from_lch() {
        super::pointer_data::wide_f64x2::run_from_lch_tests();
    }
}
