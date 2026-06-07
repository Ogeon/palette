use core::marker::PhantomData;

use palette::{
    bool_mask::{BoolMask, HasBoolMask},
    convert::{FromColor, FromColorUnclamped, IntoColor},
    encoding::linear::Linear,
    luma::{Luma, LumaStandard},
    num::{One, Zero},
    rgb::{Rgb, RgbSpace},
    Alpha, Clamp, Hsl, Hsluv, Hsv, Hwb, IsWithinBounds, Lab, Lch, Luv, WithAlpha, Xyz, Yxy,
};

#[derive(FromColorUnclamped, WithAlpha)]
#[palette(skip_derives(Xyz, Luma), component = "f64", rgb_standard = "Linear<S>")]
struct WithXyz<S>(PhantomData<S>);

impl<S> Clone for WithXyz<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S> Copy for WithXyz<S> {}

impl<S> HasBoolMask for WithXyz<S> {
    type Mask = bool;
}

impl<S> IsWithinBounds for WithXyz<S> {
    fn is_within_bounds(&self) -> bool {
        true
    }
}

impl<S> Clamp for WithXyz<S> {
    fn clamp(self) -> Self {
        self
    }
}

impl<S1, S2> FromColorUnclamped<WithXyz<S2>> for WithXyz<S1>
where
    S1: RgbSpace,
    S2: RgbSpace<WhitePoint = S1::WhitePoint>,
{
    fn from_color_unclamped(_color: WithXyz<S2>) -> Self {
        WithXyz(PhantomData)
    }
}

impl<S: RgbSpace> FromColorUnclamped<Xyz<S::WhitePoint, f64>> for WithXyz<S> {
    fn from_color_unclamped(_color: Xyz<S::WhitePoint, f64>) -> Self {
        WithXyz(PhantomData)
    }
}

impl<S: RgbSpace> FromColorUnclamped<WithXyz<S>> for Xyz<S::WhitePoint, f64> {
    fn from_color_unclamped(_color: WithXyz<S>) -> Xyz<S::WhitePoint, f64> {
        Xyz::new(0.0, 1.0, 0.0)
    }
}

impl<Rs: RgbSpace, Ls: LumaStandard<WhitePoint = Rs::WhitePoint>> FromColorUnclamped<Luma<Ls, f64>>
    for WithXyz<Rs>
{
    fn from_color_unclamped(_color: Luma<Ls, f64>) -> Self {
        WithXyz(PhantomData)
    }
}

impl<Rs: RgbSpace, Ls: LumaStandard<WhitePoint = Rs::WhitePoint>> FromColorUnclamped<WithXyz<Rs>>
    for Luma<Ls, f64>
{
    fn from_color_unclamped(_color: WithXyz<Rs>) -> Self {
        Luma::new(1.0)
    }
}

#[derive(Copy, Clone, FromColorUnclamped, WithAlpha)]
#[palette(
    skip_derives(Lch, Luma),
    white_point = "palette::white_point::E",
    component = "T",
    rgb_standard = "Linear<(palette::encoding::Srgb, palette::white_point::E)>"
)]
struct WithoutXyz<T>(PhantomData<T>);

impl<T> HasBoolMask for WithoutXyz<T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<T> IsWithinBounds for WithoutXyz<T>
where
    T: HasBoolMask,
{
    fn is_within_bounds(&self) -> T::Mask {
        T::Mask::from_bool(true)
    }
}

impl<T> Clamp for WithoutXyz<T> {
    fn clamp(self) -> Self {
        self
    }
}

impl<T> FromColorUnclamped<WithoutXyz<T>> for WithoutXyz<T> {
    fn from_color_unclamped(color: WithoutXyz<T>) -> Self {
        color
    }
}

impl<T> FromColorUnclamped<Lch<palette::white_point::E, T>> for WithoutXyz<T> {
    fn from_color_unclamped(_color: Lch<palette::white_point::E, T>) -> Self {
        WithoutXyz(PhantomData)
    }
}

impl<T: One + Zero> FromColorUnclamped<WithoutXyz<T>> for Lch<palette::white_point::E, T> {
    fn from_color_unclamped(_color: WithoutXyz<T>) -> Lch<palette::white_point::E, T> {
        Lch::new(T::one(), T::zero(), T::zero())
    }
}

impl<T> FromColorUnclamped<Luma<Linear<palette::white_point::E>, T>> for WithoutXyz<T> {
    fn from_color_unclamped(_color: Luma<Linear<palette::white_point::E>, T>) -> Self {
        WithoutXyz(PhantomData)
    }
}

impl<T: One> FromColorUnclamped<WithoutXyz<T>> for Luma<Linear<palette::white_point::E>, T> {
    fn from_color_unclamped(_color: WithoutXyz<T>) -> Luma<Linear<palette::white_point::E>, T> {
        Luma::new(T::one())
    }
}

#[test]
fn from_with_xyz() {
    let color: WithXyz<palette::encoding::Srgb> = WithXyz(Default::default());
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(color);

    let xyz: Xyz<_, f64> = Default::default();
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(xyz);

    let yxy: Yxy<_, f64> = Default::default();
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(yxy);

    let lab: Lab<_, f64> = Default::default();
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(lab);

    let lch: Lch<_, f64> = Default::default();
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(lch);

    let luv: Hsl<_, f64> = Default::default();
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(luv);

    let rgb: Rgb<_, f64> = Default::default();
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(rgb);

    let hsl: Hsl<_, f64> = Default::default();
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(hsl);

    let hsluv: Hsluv<_, f64> = Default::default();
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(hsluv);

    let hsv: Hsv<_, f64> = Default::default();
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(hsv);

    let hwb: Hwb<_, f64> = Default::default();
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(hwb);

    let luma: Luma<palette::encoding::Srgb, f64> = Default::default();
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(luma);
}

#[test]
fn from_with_xyz_alpha() {
    let color: Alpha<WithXyz<palette::encoding::Srgb>, u8> =
        Alpha::from(WithXyz(Default::default()));
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(color);

    let xyz: Alpha<Xyz<_, f64>, u8> = Alpha::from(Xyz::default());
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(xyz);

    let yxy: Alpha<Yxy<_, f64>, u8> = Alpha::from(Yxy::default());
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(yxy);

    let lab: Alpha<Lab<_, f64>, u8> = Alpha::from(Lab::default());
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(lab);

    let lch: Alpha<Lch<_, f64>, u8> = Alpha::from(Lch::default());
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(lch);

    let luv: Alpha<Luv<_, f64>, u8> = Alpha::from(Luv::default());
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(luv);

    let rgb: Alpha<Rgb<_, f64>, u8> = Alpha::from(Rgb::default());
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(rgb);

    let hsl: Alpha<Hsl<_, f64>, u8> = Alpha::from(Hsl::default());
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(hsl);

    let hsluv: Alpha<Hsluv<_, f64>, u8> = Alpha::from(Hsluv::default());
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(hsluv);

    let hsv: Alpha<Hsv<_, f64>, u8> = Alpha::from(Hsv::default());
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(hsv);

    let hwb: Alpha<Hwb<_, f64>, u8> = Alpha::from(Hwb::default());
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(hwb);

    let luma: Alpha<Luma<palette::encoding::Srgb, f64>, u8> =
        Alpha::from(Luma::<palette::encoding::Srgb, f64>::default());
    let _ = WithXyz::<palette::encoding::Srgb>::from_color(luma);
}

#[test]
fn from_with_xyz_into_alpha() {
    let color: WithXyz<palette::encoding::Srgb> = WithXyz(Default::default());
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(color);

    let xyz: Xyz<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(xyz);

    let yxy: Yxy<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(yxy);

    let lab: Lab<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(lab);

    let lch: Lch<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(lch);

    let luv: Hsl<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(luv);

    let rgb: Rgb<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(rgb);

    let hsl: Hsl<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(hsl);

    let hsluv: Hsluv<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(hsluv);

    let hsv: Hsv<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(hsv);

    let hwb: Hwb<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(hwb);

    let luma: Luma<palette::encoding::Srgb, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(luma);
}

#[test]
fn from_with_xyz_alpha_into_alpha() {
    let color: Alpha<WithXyz<palette::encoding::Srgb>, u8> =
        Alpha::from(WithXyz(Default::default()));
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(color);

    let xyz: Xyz<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(xyz);

    let yxy: Yxy<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(yxy);

    let lab: Lab<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(lab);

    let lch: Lch<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(lch);

    let luv: Luv<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(luv);

    let rgb: Rgb<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(rgb);

    let hsl: Hsl<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(hsl);

    let hsluv: Hsluv<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(hsluv);

    let hsv: Hsv<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(hsv);

    let hwb: Hwb<_, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(hwb);

    let luma: Luma<palette::encoding::Srgb, f64> = Default::default();
    let _ = Alpha::<WithXyz<palette::encoding::Srgb>, u8>::from_color(luma);
}

#[test]
fn into_from_with_xyz() {
    let color = WithXyz::<palette::encoding::Srgb>(PhantomData);

    let _self: WithXyz<palette::encoding::Srgb> = color.into_color();
    let _xyz: Xyz<_, f64> = color.into_color();
    let _yxy: Yxy<_, f64> = color.into_color();
    let _lab: Lab<_, f64> = color.into_color();
    let _lch: Lch<_, f64> = color.into_color();
    let _luv: Luv<_, f64> = color.into_color();
    let _rgb: Rgb<_, f64> = color.into_color();
    let _hsl: Hsl<_, f64> = color.into_color();
    let _hsluv: Hsluv<_, f64> = color.into_color();
    let _hsv: Hsv<_, f64> = color.into_color();
    let _hwb: Hwb<_, f64> = color.into_color();
    let _luma: Luma<palette::encoding::Srgb, f64> = color.into_color();
}

#[test]
fn into_from_with_xyz_alpha() {
    let color: Alpha<WithXyz<palette::encoding::Srgb>, u8> =
        Alpha::from(WithXyz::<palette::encoding::Srgb>(PhantomData));

    let _self: WithXyz<palette::encoding::Srgb> = color.into_color();
    let _xyz: Xyz<_, f64> = color.into_color();
    let _yxy: Yxy<_, f64> = color.into_color();
    let _lab: Lab<_, f64> = color.into_color();
    let _lch: Lch<_, f64> = color.into_color();
    let _luv: Luv<_, f64> = color.into_color();
    let _rgb: Rgb<_, f64> = color.into_color();
    let _hsl: Hsl<_, f64> = color.into_color();
    let _hsluv: Hsluv<_, f64> = color.into_color();
    let _hsv: Hsv<_, f64> = color.into_color();
    let _hwb: Hwb<_, f64> = color.into_color();
    let _luma: Luma<palette::encoding::Srgb, f64> = color.into_color();
}

#[test]
fn into_alpha_from_with_xyz() {
    let color = WithXyz::<palette::encoding::Srgb>(PhantomData);

    let _self: Alpha<WithXyz<palette::encoding::Srgb>, u8> = color.into_color();
    let _xyz: Alpha<Xyz<_, f64>, u8> = color.into_color();
    let _yxy: Alpha<Yxy<_, f64>, u8> = color.into_color();
    let _lab: Alpha<Lab<_, f64>, u8> = color.into_color();
    let _lch: Alpha<Lch<_, f64>, u8> = color.into_color();
    let _luv: Alpha<Luv<_, f64>, u8> = color.into_color();
    let _rgb: Alpha<Rgb<_, f64>, u8> = color.into_color();
    let _hsl: Alpha<Hsl<_, f64>, u8> = color.into_color();
    let _hsluv: Alpha<Hsluv<_, f64>, u8> = color.into_color();
    let _hsv: Alpha<Hsv<_, f64>, u8> = color.into_color();
    let _hwb: Alpha<Hwb<_, f64>, u8> = color.into_color();
    let _luma: Alpha<Luma<palette::encoding::Srgb, f64>, u8> = color.into_color();
}

#[test]
fn into_alpha_from_with_xyz_alpha() {
    let color: Alpha<WithXyz<palette::encoding::Srgb>, u8> =
        Alpha::from(WithXyz::<palette::encoding::Srgb>(PhantomData));

    let _self: Alpha<WithXyz<palette::encoding::Srgb>, u8> = color.into_color();
    let _xyz: Alpha<Xyz<_, f64>, u8> = color.into_color();
    let _yxy: Alpha<Yxy<_, f64>, u8> = color.into_color();
    let _lab: Alpha<Lab<_, f64>, u8> = color.into_color();
    let _lch: Alpha<Lch<_, f64>, u8> = color.into_color();
    let _luv: Alpha<Luv<_, f64>, u8> = color.into_color();
    let _rgb: Alpha<Rgb<_, f64>, u8> = color.into_color();
    let _hsl: Alpha<Hsl<_, f64>, u8> = color.into_color();
    let _hsluv: Alpha<Hsluv<_, f64>, u8> = color.into_color();
    let _hsv: Alpha<Hsv<_, f64>, u8> = color.into_color();
    let _hwb: Alpha<Hwb<_, f64>, u8> = color.into_color();
    let _luma: Alpha<Luma<palette::encoding::Srgb, f64>, u8> = color.into_color();
}

#[test]
fn from_without_xyz() {
    let color: WithoutXyz<f64> = WithoutXyz(Default::default());
    let _ = WithoutXyz::<f64>::from_color(color);

    let xyz: Xyz<palette::white_point::E, f64> = Default::default();
    let _ = WithoutXyz::<f64>::from_color(xyz);

    let yxy: Yxy<palette::white_point::E, f64> = Default::default();
    let _ = WithoutXyz::<f64>::from_color(yxy);

    let lab: Lab<palette::white_point::E, f64> = Default::default();
    let _ = WithoutXyz::<f64>::from_color(lab);

    let lch: Lch<palette::white_point::E, f64> = Default::default();
    let _ = WithoutXyz::<f64>::from_color(lch);

    let luv: Luv<palette::white_point::E, f64> = Default::default();
    let _ = WithoutXyz::<f64>::from_color(luv);

    let rgb: Rgb<_, f64> = Default::default();
    let _ = WithoutXyz::<f64>::from_color(rgb);

    let hsl: Hsl<_, f64> = Default::default();
    let _ = WithoutXyz::<f64>::from_color(hsl);

    let hsluv: Hsluv<_, f64> = Default::default();
    let _ = WithoutXyz::<f64>::from_color(hsluv);

    let hsv: Hsv<_, f64> = Default::default();
    let _ = WithoutXyz::<f64>::from_color(hsv);

    let hwb: Hwb<_, f64> = Default::default();
    let _ = WithoutXyz::<f64>::from_color(hwb);

    let luma: Luma<Linear<palette::white_point::E>, f64> = Default::default();
    let _ = WithoutXyz::<f64>::from_color(luma);
}

#[test]
fn into_without_xyz() {
    let color = WithoutXyz::<f64>(PhantomData);

    let _self: WithoutXyz<f64> = color.into_color();
    let _xyz: Xyz<palette::white_point::E, f64> = color.into_color();
    let _yxy: Yxy<palette::white_point::E, f64> = color.into_color();
    let _lab: Lab<palette::white_point::E, f64> = color.into_color();
    let _lch: Lch<palette::white_point::E, f64> = color.into_color();
    let _luv: Luv<palette::white_point::E, f64> = color.into_color();
    let _rgb: Rgb<_, f64> = color.into_color();
    let _hsl: Hsl<_, f64> = color.into_color();
    let _hsluv: Hsluv<_, f64> = color.into_color();
    let _hsv: Hsv<_, f64> = color.into_color();
    let _hwb: Hwb<_, f64> = color.into_color();
    let _luma: Luma<Linear<palette::white_point::E>, f64> = color.into_color();
}
