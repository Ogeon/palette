use palette::{Lab, Lch, IntoColor};

#[test]
fn lab_lch_green() {
    let lab = Lab::new(0.4623,-0.517,0.499);
    let lch = Lch::new(0.4623,	0.7185,	136.02.into());

    let expect_lab = lch.into_lab();
    let expect_lch = lab.into_lch();

    assert_relative_eq!(lab, expect_lab, epsilon = 0.0001);
    assert_relative_eq!(lch, expect_lch, epsilon = 0.01);
}

#[test]
fn lab_lch_magenta() {
    let lab = Lab::new(0.6032, 0.9825, -0.6084);
    let lch = Lch::new(0.6032, 1.1557, 328.23.into());

    let expect_lab = lch.into_lab();
    let expect_lch = lab.into_lch();

    assert_relative_eq!(lab, expect_lab, epsilon = 0.0001);
    assert_relative_eq!(lch, expect_lch, epsilon = 0.01);
}

#[test]
fn lab_lch_blue() {

    let lab = Lab::new(0.323, 0.792, -1.0786);
    let lch = Lch::new(0.323, 1.3382, 306.29.into());

    let expect_lab = lch.into_lab();
    let expect_lch = lab.into_lch();

    assert_relative_eq!(lab, expect_lab, epsilon = 0.0001);
    assert_relative_eq!(lch, expect_lch, epsilon = 0.01);
}
