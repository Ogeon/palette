use num_traits::Float;
use approx::ApproxEq;

use std::ops::{Add, Sub};
use std::marker::PhantomData;
use std::any::TypeId;

use {Alpha, Xyz, Hsv, Limited, Mix, Shade, GetHue, Hue, RgbHue, FromColor, IntoColor, clamp};
use white_point::WhitePoint;
use rgb::RgbSpace;
use rgb::standards::Srgb;

///Linear HWB with an alpha component. See the [`Hwba` implementation in `Alpha`](struct.Alpha.html#Hwba).
pub type Hwba<S = Srgb, T = f32> = Alpha<Hwb<S, T>, T>;

///Linear HWB color space.
///
///HWB is a cylindrical version of [RGB](rgb/struct.LinRgb.html) and it's very
///closely related to [HSV](struct.Hsv.html).  It describes colors with a starting hue,
///then a degree of whiteness and blackness to mix into that base hue.
///
///It is very intuitive for humans to use and many color-pickers are based on the HWB color system
#[derive(Debug, PartialEq)]
pub struct Hwb<S = Srgb, T = f32>
    where T: Float,
        S: RgbSpace
{
    ///The hue of the color, in degrees. Decides if it's red, blue, purple,
    ///etc. Same as the hue for HSL and HSV.
    pub hue: RgbHue<T>,

    ///The whiteness of the color. It specifies the amount white to mix into the hue.
    ///It varies from 0 to 1, with 1 being always full white and 0
    ///always being the color shade (a mixture of a pure hue with black) chosen with the other two
    ///controls.
    pub whiteness: T,

    ///The blackness of the color. It specifies the amount black to mix into the hue.
    ///It varies from 0 to 1, with 1 being always full black and 0 always
    ///being the color tint (a mixture of a pure hue with white) chosen with the other two
    //controls.
    pub blackness: T,

    ///The white point and RGB primaries this color is adapted to. The default
    ///is the sRGB standard.
    pub space: PhantomData<S>,
}

impl<S, T> Copy for Hwb<S, T>
    where T: Float,
        S: RgbSpace
{}

impl<S, T> Clone for Hwb<S, T>
    where T: Float,
        S: RgbSpace
{
    fn clone(&self) -> Hwb<S, T> { *self }
}


impl<T> Hwb<Srgb, T>
    where T: Float,
{
    ///HWB for linear sRGB.
    pub fn new(hue: RgbHue<T>, whiteness: T, blackness: T) -> Hwb<Srgb, T> {
        Hwb {
            hue: hue,
            whiteness: whiteness,
            blackness: blackness,
            space: PhantomData,
        }
    }
}

impl<S, T> Hwb<S, T>
    where T: Float,
        S: RgbSpace
{
    ///Linear HWB.
    pub fn with_wp(hue: RgbHue<T>, whiteness: T, blackness: T) -> Hwb<S, T> {
        Hwb {
            hue: hue,
            whiteness: whiteness,
            blackness: blackness,
            space: PhantomData,
        }
    }

    #[inline]
    fn reinterpret_as<Sp: RgbSpace>(self) -> Hwb<Sp, T> {
        Hwb {
            hue: self.hue,
            whiteness: self.whiteness,
            blackness: self.blackness,
            space: PhantomData,
        }
    }
}

///<span id="Hwba"></span>[`Hwba`](type.Hwba.html) implementations.
impl<T> Alpha<Hwb<Srgb, T>, T>
    where T: Float,
{
    ///HWB and transparency for linear sRGB.
    pub fn new(hue: RgbHue<T>, whiteness: T, blackness: T, alpha: T) -> Hwba<Srgb, T> {
        Alpha {
            color: Hwb::new(hue, whiteness, blackness),
            alpha: alpha,
        }
    }
}

///<span id="Hwba"></span>[`Hwba`](type.Hwba.html) implementations.
impl<S, T> Alpha<Hwb<S, T>, T>
    where T: Float,
        S: RgbSpace
{
    ///Linear HWB and transparency.
    pub fn with_wp(hue: RgbHue<T>, whiteness: T, blackness: T, alpha: T) -> Hwba<S, T> {
        Alpha {
            color: Hwb::with_wp(hue, whiteness, blackness),
            alpha: alpha,
        }
    }
}

impl<S, Wp, T> FromColor<Wp, T> for Hwb<S, T>
    where T: Float,
        S: RgbSpace<WhitePoint=Wp>,
        Wp: WhitePoint,
{
    fn from_xyz(xyz: Xyz<Wp, T>) -> Self {
        let hsv: Hsv<S, T> = xyz.into_hsv();
        Self::from_hsv(hsv)
    }

    fn from_hsv<Sp: RgbSpace<WhitePoint=Wp>>(hsv: Hsv<Sp, T>) -> Self {
        let hsv = Hsv::<S, T>::from_hsv(hsv);

        Hwb {
            hue: hsv.hue,
            whiteness: (T::one() - hsv.saturation) * hsv.value,
            blackness: (T::one() - hsv.value),
            space: PhantomData,
        }
    }

    fn from_hwb<Sp: RgbSpace<WhitePoint=Wp>>(hwb: Hwb<Sp, T>) -> Self {
        if TypeId::of::<Sp::Primaries>() == TypeId::of::<S::Primaries>() {
            hwb.reinterpret_as()
        } else {
            Self::from_hsv(Hsv::<Sp, T>::from_hwb(hwb))
        }
    }

}

impl<S, T> Limited for Hwb<S, T>
    where T: Float,
        S: RgbSpace
{
    fn is_valid(&self) -> bool {
        self.blackness >= T::zero() && self.blackness <= T::one() &&
        self.whiteness >= T::zero() && self.whiteness <= T::one() &&
        (self.whiteness + self.blackness) <= T::one()
    }

    fn clamp(&self) -> Hwb<S, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.whiteness = self.whiteness.max(T::zero());
        self.blackness = self.blackness.max(T::zero());
        let sum = self.blackness + self.whiteness;
        if sum > T::one() {
            self.whiteness = self.whiteness / sum;
            self.blackness = self.blackness / sum;
        }
    }
}

impl<S, T> Mix for Hwb<S, T>
    where T: Float,
        S: RgbSpace
{
    type Scalar = T;

    fn mix(&self, other: &Hwb<S, T>, factor: T) -> Hwb<S, T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();

        Hwb {
            hue: self.hue + factor * hue_diff,
            whiteness: self.whiteness + factor * (other.whiteness - self.whiteness),
            blackness: self.blackness + factor * (other.blackness - self.blackness),
            space: PhantomData,
        }
    }
}

impl<S, T> Shade for Hwb<S, T>
    where T: Float,
        S: RgbSpace
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Hwb<S, T> {
        Hwb {
            hue: self.hue,
            whiteness: self.whiteness + amount,
            blackness: self.blackness - amount,
            space: PhantomData,
        }
    }
}

impl<S, T> GetHue for Hwb<S, T>
    where T: Float,
        S: RgbSpace
{
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> Option<RgbHue<T>> {
        if self.whiteness + self.blackness >= T::one() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<S, T> Hue for Hwb<S, T>
    where T: Float,
        S: RgbSpace
{
    fn with_hue(&self, hue: RgbHue<T>) -> Hwb<S, T> {
        Hwb {
            hue: hue,
            whiteness: self.whiteness,
            blackness: self.blackness,
            space: PhantomData,
        }
    }

    fn shift_hue(&self, amount: RgbHue<T>) -> Hwb<S, T> {
        Hwb {
            hue: self.hue + amount,
            whiteness: self.whiteness,
            blackness: self.blackness,
            space: PhantomData,
        }
    }
}

impl<S, T> Default for Hwb<S, T>
    where T: Float,
        S: RgbSpace
{
    fn default() -> Hwb<S, T> {
        Hwb::with_wp(RgbHue::from(T::zero()), T::zero(), T::one())
    }
}

impl<S, T> Add<Hwb<S, T>> for Hwb<S, T>
    where T: Float,
        S: RgbSpace
{
    type Output = Hwb<S, T>;

    fn add(self, other: Hwb<S, T>) -> Hwb<S, T> {
        Hwb {
            hue: self.hue + other.hue,
            whiteness: self.whiteness + other.whiteness,
            blackness: self.blackness + other.blackness,
            space: PhantomData,
        }
    }
}

impl<S, T> Add<T> for Hwb<S, T>
    where T: Float,
        S: RgbSpace
{
    type Output = Hwb<S, T>;

    fn add(self, c: T) -> Hwb<S, T> {
        Hwb {
            hue: self.hue + c,
            whiteness: self.whiteness + c,
            blackness: self.blackness + c,
            space: PhantomData,
        }
    }
}

impl<S, T> Sub<Hwb<S, T>> for Hwb<S, T>
    where T: Float,
        S: RgbSpace
{
    type Output = Hwb<S, T>;

    fn sub(self, other: Hwb<S, T>) -> Hwb<S, T> {
        Hwb {
            hue: self.hue - other.hue,
            whiteness: self.whiteness - other.whiteness,
            blackness: self.blackness - other.blackness,
            space: PhantomData,
        }
    }
}

impl<S, T> Sub<T> for Hwb<S, T>
    where T: Float,
        S: RgbSpace
{
    type Output = Hwb<S, T>;

    fn sub(self, c: T) -> Hwb<S, T> {
        Hwb {
            hue: self.hue - c,
            whiteness: self.whiteness - c,
            blackness: self.blackness - c,
            space: PhantomData,
        }
    }
}

impl<S, T> From<Alpha<Hwb<S, T>, T>> for Hwb<S, T>
    where T: Float,
        S: RgbSpace
{
    fn from(color: Alpha<Hwb<S, T>, T>) -> Hwb<S, T> {
        color.color
    }
}

impl<S, T> ApproxEq for Hwb<S, T>
    where T: Float + ApproxEq,
        T::Epsilon: Copy + Float,
        S: RgbSpace,
{
    type Epsilon = <T as ApproxEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }
    fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
        self.hue.relative_eq(&other.hue, epsilon, max_relative) &&
        self.whiteness.relative_eq(&other.whiteness, epsilon, max_relative) &&
        self.blackness.relative_eq(&other.blackness, epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool{
        self.hue.ulps_eq(&other.hue, epsilon, max_ulps) &&
        self.whiteness.ulps_eq(&other.whiteness, epsilon, max_ulps) &&
        self.blackness.ulps_eq(&other.blackness, epsilon, max_ulps)
    }
}

#[cfg(test)]
mod test {
    use super::Hwb;
    use ::{LinSrgb, Limited};

    #[test]
    fn red() {
        let a = Hwb::from(LinSrgb::new(1.0, 0.0, 0.0));
        let b = Hwb::new(0.0.into(), 0.0, 0.0);
        assert_relative_eq!(a, b);
    }

    #[test]
    fn orange() {
        let a = Hwb::from(LinSrgb::new(1.0, 0.5, 0.0));
        let b = Hwb::new(30.0.into(), 0.0, 0.0);
        assert_relative_eq!(a, b);
    }

    #[test]
    fn green() {
        let a = Hwb::from(LinSrgb::new(0.0, 1.0, 0.0));
        let b = Hwb::new(120.0.into(), 0.0, 0.0);
        assert_relative_eq!(a, b);
    }

    #[test]
    fn blue() {
        let a = Hwb::from(LinSrgb::new(0.0, 0.0, 1.0));
        let b = Hwb::new(240.0.into(), 0.0, 0.0);
        assert_relative_eq!(a, b);
    }

    #[test]
    fn purple() {
        let a = Hwb::from(LinSrgb::new(0.5, 0.0, 1.0));
        let b = Hwb::new(270.0.into(), 0.0, 0.0);
        assert_relative_eq!(a, b);
    }

    #[test]
    fn clamp_invalid() {
        let expected = Hwb::new((240.0).into(), 0.0, 0.0);

        let a = Hwb::new((240.0).into(), -3.0, -4.0);
        let calc_a = a.clamp();
        assert_relative_eq!(expected, calc_a);

    }

    #[test]
    fn clamp_none() {
        let expected = Hwb::new((240.0).into(), 0.3, 0.7);

        let a = Hwb::new((240.0).into(), 0.3, 0.7);
        let calc_a = a.clamp();
        assert_relative_eq!(expected, calc_a);
    }
    #[test]
    fn clamp_over_one() {
        let expected = Hwb::new((240.0).into(), 0.2, 0.8);

        let a = Hwb::new((240.0).into(), 5.0, 20.0);
        let calc_a = a.clamp();
        assert_relative_eq!(expected, calc_a);

    }
    #[test]
    fn clamp_under_one() {
        let expected = Hwb::new((240.0).into(), 0.3, 0.1);

        let a = Hwb::new((240.0).into(), 0.3, 0.1);
        let calc_a = a.clamp();
        assert_relative_eq!(expected, calc_a);
    }
}
