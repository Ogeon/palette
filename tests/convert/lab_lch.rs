use palette::{Lab, Lch, IntoColor};

#[test]
fn lab_lch_green() {
    let lab = Lab::new(46.23, -66.176, 63.872);
    let lch = Lch::new(46.23, 91.972, 136.015.into());
    let expect_lab = lch.into_lab();
    let expect_lch = lab.into_lch();

    assert_relative_eq!(lab, expect_lab, epsilon = 0.001);
    assert_relative_eq!(lch, expect_lch, epsilon = 0.001);
}

#[test]
fn lab_lch_magenta() {
    let lab = Lab::new(60.320, 98.254, -60.843);
    let lch = Lch::new(60.320, 115.567, 328.233.into());

    let expect_lab = lch.into_lab();
    let expect_lch = lab.into_lch();

    assert_relative_eq!(lab, expect_lab, epsilon = 0.001);
    assert_relative_eq!(lch, expect_lch, epsilon = 0.001);
}

#[test]
fn lab_lch_blue() {

    let lab = Lab::new(32.303, 79.197, -107.864);
    let lch = Lch::new(32.303, 133.816, 306.287.into());

    let expect_lab = lch.into_lab();
    let expect_lch = lab.into_lch();

    assert_relative_eq!(lab, expect_lab, epsilon = 0.001);
    assert_relative_eq!(lch, expect_lch, epsilon = 0.001);
}
