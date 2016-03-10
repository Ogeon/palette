mod pointer_data;

#[test]
pub fn from_lab() {
    pointer_data::run_from_lab_tests();
}
#[test]
pub fn from_lch() {
    pointer_data::run_from_lch_tests();
}
