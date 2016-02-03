use {Color, Colora, Rgb, Rgba, Blend, ComponentWise};
use blend::PreAlpha;

#[test]
fn blend_color() {
    let a = Color::rgb(1.0, 0.0, 0.0);
    let b = Color::rgb(0.0, 0.0, 1.0);

    let c: Rgb = a.blend(b, |a: PreAlpha<Rgb<_>, _>, b: PreAlpha<Rgb<_>, _>| a.component_wise(&b, |a, b| a + b)).into();
    assert_relative_eq!(Rgb::new(1.0, 0.0, 1.0), c);
}

#[test]
fn blend_alpha_color() {
    let a = Colora::rgb(1.0, 0.0, 0.0, 0.2);
    let b = Colora::rgb(0.0, 0.0, 1.0, 0.2);

    let c: Rgba = a.blend(b, |a: PreAlpha<Rgb<_>, _>, b: PreAlpha<Rgb<_>, _>| a.component_wise(&b, |a, b| a + b)).into();
    assert_relative_eq!(Rgba::new(0.2 / 0.4, 0.0, 0.2 / 0.4, 0.4), c);
}

#[test]
fn over() {
    let a = Rgb::new(0.5, 0.0, 0.3);
    let b = Rgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(Rgb::new(0.5, 0.0, 0.3), a.over(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 1.0), a.over(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 0.5);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.5 / 0.75, 0.05 / 0.75, 0.15 / 0.75, 0.75), a.over(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 1.0), a.over(b));
}

#[test]
fn inside() {
    let a = Rgb::new(0.5, 0.0, 0.3);
    let b = Rgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(Rgb::new(0.5, 0.0, 0.3), a.inside(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 0.5), a.inside(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(Rgba::new(0.0, 0.0, 0.0, 0.0), a.inside(b));
}

#[test]
fn outside() {
    let a = Rgb::new(0.5, 0.0, 0.3);
    let b = Rgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(Rgb::new(0.0, 0.0, 0.0), a.outside(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 0.5), a.outside(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 1.0), a.outside(b));
}

#[test]
fn atop() {
    let a = Rgb::new(0.5, 0.0, 0.3);
    let b = Rgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(Rgb::new(0.5, 0.0, 0.3), a.atop(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 0.5), a.atop(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 0.5);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.75, 0.1, 0.15, 0.5), a.atop(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(Rgba::new(0.0, 0.0, 0.0, 0.0), a.atop(b));
}

#[test]
fn xor() {
    let a = Rgb::new(0.5, 0.0, 0.3);
    let b = Rgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(Rgb::new(0.0, 0.0, 0.0), a.xor(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 0.5), a.xor(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 0.5);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.75, 0.1, 0.15, 0.5), a.xor(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 1.0), a.xor(b));
}

#[test]
fn plus() {
    let a = Rgb::new(0.5, 0.0, 0.3);
    let b = Rgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(Rgb::new(1.5, 0.2, 0.3), a.plus(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(1.0, 0.1, 0.3, 1.0), a.plus(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 0.5);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.75, 0.1, 0.15, 1.0), a.plus(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 1.0), a.plus(b));
}

#[test]
fn multiply() {
    let a = Rgb::new(0.5, 0.0, 0.3);
    let b = Rgb::new(0.5, 0.2, 0.0);

    assert_relative_eq!(Rgb::new(0.25, 0.0, 0.0), a.multiply(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(0.5, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.375, 0.0, 0.15, 1.0), a.multiply(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(0.5, 0.2, 0.0, 0.0);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 1.0), a.multiply(b));
}

#[test]
fn screen() {
    let a = Rgb::new(0.5, 0.0, 0.3);
    let b = Rgb::new(0.5, 0.2, 0.0);

    assert_relative_eq!(Rgb::new(0.75, 0.2, 0.3), a.screen(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(0.5, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.625, 0.1, 0.3, 1.0), a.screen(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(0.5, 0.2, 0.0, 0.0);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 1.0), a.screen(b));
}

#[test]
fn overlay() {
    let a = Rgb::new(0.5, 0.0, 0.3);
    let b = Rgb::new(0.5, 0.2, 0.0);

    assert_relative_eq!(Rgb::new(0.5, 0.0, 0.0), a.overlay(b));

    let a = Rgb::new(0.5, 0.0, 0.3);
    let b = Rgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(Rgb::new(1.0, 0.0, 0.0), a.overlay(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.75, 0.0, 0.15, 1.0), a.overlay(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 1.0), a.overlay(b));
}

#[test]
fn darken() {
    let a = Rgb::new(0.5, 0.0, 0.3);
    let b = Rgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(Rgb::new(0.5, 0.0, 0.0), a.darken(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.15, 1.0), a.darken(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 0.5);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.5 / 0.75, 0.05 / 0.75, 0.075 / 0.75, 0.75), a.darken(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 1.0), a.darken(b));
}

#[test]
fn lighten() {
    let a = Rgb::new(0.5, 0.0, 0.3);
    let b = Rgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(Rgb::new(1.0, 0.2, 0.3), a.lighten(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.75, 0.1, 0.3, 1.0), a.lighten(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 0.5);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.625 / 0.75, 0.1 / 0.75, 0.15 / 0.75, 0.75), a.lighten(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 1.0), a.lighten(b));
}

#[test]
fn dodge() {
    let a = Rgb::new(0.5, 0.0, 0.3);
    let b = Rgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(Rgb::new(1.0, 0.2, 0.0), a.dodge(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.75, 0.1, 0.15, 1.0), a.dodge(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 1.0), a.dodge(b));
}

#[test]
fn burn() {
    let a = Rgb::new(0.5, 0.0, 0.3);
    let b = Rgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(Rgb::new(1.0, 0.0, 0.0), a.burn(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.75, 0.0, 0.15, 1.0), a.burn(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 1.0), a.burn(b));
}

#[test]
fn hard_light() {
    let a = Rgb::new(0.5, 0.0, 0.3);
    let b = Rgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(Rgb::new(1.0, 0.0, 0.0), a.hard_light(b));

    let a = Rgb::new(1.0, 0.2, 0.0);
    let b = Rgb::new(0.5, 0.0, 0.3);

    assert_relative_eq!(Rgb::new(1.0, 0.0, 0.0), a.hard_light(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.75, 0.0, 0.15, 1.0), a.hard_light(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 1.0), a.hard_light(b));
}

#[test]
fn soft_light() {
    let a = Rgb::new(0.5, 0.0, 0.3);
    let b = Rgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(Rgb::new(1.0, 0.04, 0.0), a.soft_light(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.75, 0.02, 0.15, 1.0), a.soft_light(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 1.0), a.soft_light(b));
}

#[test]
fn difference() {
    let a = Rgb::new(0.5, 0.0, 0.3);
    let b = Rgb::new(1.0, 0.2, 0.0);

    assert_relative_eq!(Rgb::new(0.5, 0.2, 0.3), a.difference(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.5, 0.1, 0.3, 1.0), a.difference(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 1.0), a.difference(b));
}

#[test]
fn exclusion() {
    let a = Rgb::new(1.0, 0.5, 0.0);
    let b = Rgb::new(0.8, 0.4, 0.3);

    assert_relative_eq!(Rgb::new(0.2, 0.5, 0.3), a.exclusion(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.5);

    assert_relative_eq!(Rgba::new(0.5, 0.1, 0.3, 1.0), a.difference(b));

    let a = Rgba::new(0.5, 0.0, 0.3, 1.0);
    let b = Rgba::new(1.0, 0.2, 0.0, 0.0);

    assert_relative_eq!(Rgba::new(0.5, 0.0, 0.3, 1.0), a.difference(b));
}