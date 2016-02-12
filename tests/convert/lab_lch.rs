use palette::{Lab, Lch, IntoColor};

#[test]
fn lab_lch_green() {
    let lab = Lab::new(0.4623,-0.517,0.499);
    let lch = Lch::new(0.4623,	0.7185,	136.02.into());

    let expect_lab = lch.into_lab();
    let expect_lch = lab.into_lch();

    assert_color_eq!(lab, expect_lab, [l, a, b]);
    assert_color_eq!(lch, expect_lch, [l, chroma]);
    assert_color_hue_eq!(lch, expect_lch, [hue], 0.1);

}

#[test]
fn lab_lch_magenta() {
    let lab = Lab::new(0.6032, 0.9825, -0.6084);
    let lch = Lch::new(0.6032, 1.1557, 328.23.into());

    let expect_lab = lch.into_lab();
    let expect_lch = lab.into_lch();

    assert_color_eq!(lab, expect_lab, [l, a, b]);
    assert_color_eq!(lch, expect_lch, [l, chroma]);
    assert_color_hue_eq!(lch, expect_lch, [hue], 0.1);
}

#[test]
fn lab_lch_blue() {

    let lab = Lab::new(0.323, 0.792, -1.0786);
    let lch = Lch::new(0.323, 1.3382, 306.29.into());

    let expect_lab = lch.into_lab();
    let expect_lch = lab.into_lch();

    assert_color_eq!(lab, expect_lab, [l, a, b]);
    assert_color_eq!(lch, expect_lch, [l, chroma]);
    assert_color_hue_eq!(lch, expect_lch, [hue], 0.1);
}
