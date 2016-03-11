use {Color, Colora, LinRgb, LinRgba, Blend, ComponentWise};
use blend::PreAlpha;

#[test]
fn blend_color() {
    let a = Color::linear_rgb(1.0, 0.0, 0.0);
    let b = Color::linear_rgb(0.0, 0.0, 1.0);

    let c: LinRgb = a.blend(b, |a: PreAlpha<LinRgb<_>, _>, b: PreAlpha<LinRgb<_>, _>| a.component_wise(&b, |a, b| a + b)).into();
    assert_relative_eq!(LinRgb::new(1.0, 0.0, 1.0), c);
}

#[test]
fn blend_alpha_color() {
    let a = Colora::linear_rgb(1.0, 0.0, 0.0, 0.2);
    let b = Colora::linear_rgb(0.0, 0.0, 1.0, 0.2);

    let c: LinRgba = a.blend(b, |a: PreAlpha<LinRgb<_>, _>, b: PreAlpha<LinRgb<_>, _>| a.component_wise(&b, |a, b| a + b)).into();
    assert_relative_eq!(LinRgba::new(0.2 / 0.4, 0.0, 0.2 / 0.4, 0.4), c);
}

#[test]
fn over() {
    let a = LinRgb::new(0.5, 0.0, 0.3);
    let b = LinRgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(LinRgb::new(0.5, 0.0, 0.3), a.over(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 1.0), a.over(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 0.5);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.5 / 0.75, 0.05 / 0.75, 0.15 / 0.75, 0.75), a.over(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 1.0), a.over(b));
}

#[test]
fn inside() {
    let a = LinRgb::new(0.5, 0.0, 0.3);
    let b = LinRgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(LinRgb::new(0.5, 0.0, 0.3), a.inside(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 0.5), a.inside(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(LinRgba::new(0.0, 0.0, 0.0, 0.0), a.inside(b));
}

#[test]
fn outside() {
    let a = LinRgb::new(0.5, 0.0, 0.3);
    let b = LinRgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(LinRgb::new(0.0, 0.0, 0.0), a.outside(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 0.5), a.outside(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 1.0), a.outside(b));
}

#[test]
fn atop() {
    let a = LinRgb::new(0.5, 0.0, 0.3);
    let b = LinRgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(LinRgb::new(0.5, 0.0, 0.3), a.atop(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 0.5), a.atop(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 0.5);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.75, 0.1, 0.15, 0.5), a.atop(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(LinRgba::new(0.0, 0.0, 0.0, 0.0), a.atop(b));
}

#[test]
fn xor() {
    let a = LinRgb::new(0.5, 0.0, 0.3);
    let b = LinRgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(LinRgb::new(0.0, 0.0, 0.0), a.xor(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 0.5), a.xor(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 0.5);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.75, 0.1, 0.15, 0.5), a.xor(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 1.0), a.xor(b));
}

#[test]
fn plus() {
    let a = LinRgb::new(0.5, 0.0, 0.3);
    let b = LinRgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(LinRgb::new(1.5, 0.2, 0.3), a.plus(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(1.0, 0.1, 0.3, 1.0), a.plus(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 0.5);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.75, 0.1, 0.15, 1.0), a.plus(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 1.0), a.plus(b));
}

#[test]
fn multiply() {
    let a = LinRgb::new(0.5, 0.0, 0.3);
    let b = LinRgb::new(0.5, 0.2, 0.0);

    assert_relative_eq!(LinRgb::new(0.25, 0.0, 0.0), a.multiply(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(0.5, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.375, 0.0, 0.15, 1.0), a.multiply(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(0.5, 0.2, 0.0, 0.0);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 1.0), a.multiply(b));
}

#[test]
fn screen() {
    let a = LinRgb::new(0.5, 0.0, 0.3);
    let b = LinRgb::new(0.5, 0.2, 0.0);

    assert_relative_eq!(LinRgb::new(0.75, 0.2, 0.3), a.screen(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(0.5, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.625, 0.1, 0.3, 1.0), a.screen(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(0.5, 0.2, 0.0, 0.0);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 1.0), a.screen(b));
}

#[test]
fn overlay() {
    let a = LinRgb::new(0.5, 0.0, 0.3);
    let b = LinRgb::new(0.5, 0.2, 0.0);

    assert_relative_eq!(LinRgb::new(0.5, 0.0, 0.0), a.overlay(b));

    let a = LinRgb::new(0.5, 0.0, 0.3);
    let b = LinRgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(LinRgb::new(1.0, 0.0, 0.0), a.overlay(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.75, 0.0, 0.15, 1.0), a.overlay(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 1.0), a.overlay(b));
}

#[test]
fn darken() {
    let a = LinRgb::new(0.5, 0.0, 0.3);
    let b = LinRgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(LinRgb::new(0.5, 0.0, 0.0), a.darken(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.15, 1.0), a.darken(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 0.5);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.5 / 0.75, 0.05 / 0.75, 0.075 / 0.75, 0.75), a.darken(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 1.0), a.darken(b));
}

#[test]
fn lighten() {
    let a = LinRgb::new(0.5, 0.0, 0.3);
    let b = LinRgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(LinRgb::new(1.0, 0.2, 0.3), a.lighten(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.75, 0.1, 0.3, 1.0), a.lighten(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 0.5);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.625 / 0.75, 0.1 / 0.75, 0.15 / 0.75, 0.75), a.lighten(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 1.0), a.lighten(b));
}

#[test]
fn dodge() {
    let a = LinRgb::new(0.5, 0.0, 0.3);
    let b = LinRgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(LinRgb::new(1.0, 0.2, 0.0), a.dodge(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.75, 0.1, 0.15, 1.0), a.dodge(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 1.0), a.dodge(b));
}

#[test]
fn burn() {
    let a = LinRgb::new(0.5, 0.0, 0.3);
    let b = LinRgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(LinRgb::new(1.0, 0.0, 0.0), a.burn(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.75, 0.0, 0.15, 1.0), a.burn(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 1.0), a.burn(b));
}

#[test]
fn hard_light() {
    let a = LinRgb::new(0.5, 0.0, 0.3);
    let b = LinRgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(LinRgb::new(1.0, 0.0, 0.0), a.hard_light(b));

    let a = LinRgb::new(1.0, 0.2, 0.0);
    let b = LinRgb::new(0.5, 0.0, 0.3);

    assert_relative_eq!(LinRgb::new(1.0, 0.0, 0.0), a.hard_light(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.75, 0.0, 0.15, 1.0), a.hard_light(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 1.0), a.hard_light(b));
}

#[test]
fn soft_light() {
    let a = LinRgb::new(0.5, 0.0, 0.3);
    let b = LinRgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(LinRgb::new(1.0, 0.04, 0.0), a.soft_light(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.75, 0.02, 0.15, 1.0), a.soft_light(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 1.0), a.soft_light(b));
}

#[test]
fn difference() {
    let a = LinRgb::new(0.5, 0.0, 0.3);
    let b = LinRgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(LinRgb::new(0.5, 0.2, 0.3), a.difference(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.5, 0.1, 0.3, 1.0), a.difference(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 1.0), a.difference(b));
}

#[test]
fn exclusion() {
    let a = LinRgb::new(1.0, 0.5, 0.0);
    let b = LinRgb::new(0.8, 0.4, 0.3);

    assert_relative_eq!(LinRgb::new(0.2, 0.5, 0.3), a.exclusion(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(LinRgba::new(0.5, 0.1, 0.3, 1.0), a.difference(b));

    let a = LinRgba::new(0.5, 0.0, 0.3, 1.0);
    let b = LinRgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(LinRgba::new(0.5, 0.0, 0.3, 1.0), a.difference(b));
}