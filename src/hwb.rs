use num::Float;

use std::ops::{Add, Sub};
use std::marker::PhantomData;

use {Alpha, Xyz, Hsv, Limited, Mix, Shade, GetHue, Hue, RgbHue, FromColor, IntoColor, clamp};
use white_point::{WhitePoint, D65};

///Linear HWB with an alpha component. See the [`Hwba` implementation in `Alpha`](struct.Alpha.html#Hwba).
pub type Hwba<Wp = D65, T = f32> = Alpha<Hwb<Wp, T>, T>;

///Linear HWB color space.
///
///HWB is a cylindrical version of [RGB](struct.Rgb.html) and it's very
///closely related to [HSV](struct.Hsv.html).  It describes colors with a starting hue,
///then a degree of whiteness and blackness to mix into that base hue.
///
///It is very intuitive for humans to use and many color-pickers are based on the HWB color system
#[derive(Debug, PartialEq)]
pub struct Hwb<Wp = D65, T = f32>
    where T: Float,
        Wp: WhitePoint<T>
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

    ///The white point associated with the color's illuminant and observer.
    ///D65 for 2 degree observer is used by default.
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Copy for Hwb<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{}

impl<Wp, T> Clone for Hwb<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn clone(&self) -> Hwb<Wp, T> { *self }
}


impl<T> Hwb<D65, T>
    where T: Float,
{
    ///Linear HWB with white point D65.
    pub fn new(hue: RgbHue<T>, whiteness: T, blackness: T) -> Hwb<D65, T> {
        Hwb {
            hue: hue,
            whiteness: whiteness,
            blackness: blackness,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Hwb<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///Linear HWB.
    pub fn with_wp(hue: RgbHue<T>, whiteness: T, blackness: T) -> Hwb<Wp, T> {
        Hwb {
            hue: hue,
            whiteness: whiteness,
            blackness: blackness,
            white_point: PhantomData,
        }
    }
}

///<span id="Hwba"></span>[`Hwba`](type.Hwba.html) implementations.
impl<T> Alpha<Hwb<D65, T>, T>
    where T: Float,
{
    ///Linear HSV and transparency with white point D65.
    pub fn new(hue: RgbHue<T>, whiteness: T, blackness: T, alpha: T) -> Hwba<D65, T> {
        Alpha {
            color: Hwb::new(hue, whiteness, blackness),
            alpha: alpha,
        }
    }
}

///<span id="Hwba"></span>[`Hwba`](type.Hwba.html) implementations.
impl<Wp, T> Alpha<Hwb<Wp, T>, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///Linear HSV and transparency.
    pub fn with_wp(hue: RgbHue<T>, whiteness: T, blackness: T, alpha: T) -> Hwba<Wp, T> {
        Alpha {
            color: Hwb::with_wp(hue, whiteness, blackness),
            alpha: alpha,
        }
    }
}

impl<Wp, T> FromColor<Wp, T> for Hwb<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn from_xyz(xyz: Xyz<Wp, T>) -> Self {
        let hsv = xyz.into_hsv();
        Self::from_hsv(hsv)
    }

    fn from_hsv(hsv: Hsv<Wp, T>) -> Self {
        Hwb {
            hue: hsv.hue,
            whiteness: (T::one() - hsv.saturation) * hsv.value,
            blackness: (T::one() - hsv.value),
            white_point: PhantomData,
        }
    }

    fn from_hwb(hwb: Hwb<Wp, T>) -> Self {
        hwb
    }

}

impl<Wp, T> Limited for Hwb<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn is_valid(&self) -> bool {
        self.blackness >= T::zero() && self.blackness <= T::one() &&
        self.whiteness >= T::zero() && self.whiteness <= T::one() &&
        (self.whiteness + self.blackness) <= T::one()
    }

    fn clamp(&self) -> Hwb<Wp, T> {
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

impl<Wp, T> Mix for Hwb<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn mix(&self, other: &Hwb<Wp, T>, factor: T) -> Hwb<Wp, T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();

        Hwb {
            hue: self.hue + factor * hue_diff,
            whiteness: self.whiteness + factor * (other.whiteness - self.whiteness),
            blackness: self.blackness + factor * (other.blackness - self.blackness),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Shade for Hwb<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Hwb<Wp, T> {
        Hwb {
            hue: self.hue,
            whiteness: self.whiteness + amount,
            blackness: self.blackness - amount,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> GetHue for Hwb<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
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

impl<Wp, T> Hue for Hwb<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn with_hue(&self, hue: RgbHue<T>) -> Hwb<Wp, T> {
        Hwb {
            hue: hue,
            whiteness: self.whiteness,
            blackness: self.blackness,
            white_point: PhantomData,
        }
    }

    fn shift_hue(&self, amount: RgbHue<T>) -> Hwb<Wp, T> {
        Hwb {
            hue: self.hue + amount,
            whiteness: self.whiteness,
            blackness: self.blackness,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Default for Hwb<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn default() -> Hwb<Wp, T> {
        Hwb::with_wp(RgbHue::from(T::zero()), T::zero(), T::one())
    }
}

impl<Wp, T> Add<Hwb<Wp, T>> for Hwb<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Hwb<Wp, T>;

    fn add(self, other: Hwb<Wp, T>) -> Hwb<Wp, T> {
        Hwb {
            hue: self.hue + other.hue,
            whiteness: self.whiteness + other.whiteness,
            blackness: self.blackness + other.blackness,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Add<T> for Hwb<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Hwb<Wp, T>;

    fn add(self, c: T) -> Hwb<Wp, T> {
        Hwb {
            hue: self.hue + c,
            whiteness: self.whiteness + c,
            blackness: self.blackness + c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<Hwb<Wp, T>> for Hwb<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Hwb<Wp, T>;

    fn sub(self, other: Hwb<Wp, T>) -> Hwb<Wp, T> {
        Hwb {
            hue: self.hue - other.hue,
            whiteness: self.whiteness - other.whiteness,
            blackness: self.blackness - other.blackness,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<T> for Hwb<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Hwb<Wp, T>;

    fn sub(self, c: T) -> Hwb<Wp, T> {
        Hwb {
            hue: self.hue - c,
            whiteness: self.whiteness - c,
            blackness: self.blackness - c,
            white_point: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Hwb;
    use ::{Rgb, Limited};

    #[test]
    fn red() {
        let a = Hwb::from(Rgb::new(1.0, 0.0, 0.0));
        let b = Hwb::new(0.0.into(), 0.0, 0.0);
        assert_relative_eq!(a, b);
    }

    #[test]
    fn orange() {
        let a = Hwb::from(Rgb::new(1.0, 0.5, 0.0));
        let b = Hwb::new(30.0.into(), 0.0, 0.0);
        assert_relative_eq!(a, b);
    }

    #[test]
    fn green() {
        let a = Hwb::from(Rgb::new(0.0, 1.0, 0.0));
        let b = Hwb::new(120.0.into(), 0.0, 0.0);
        assert_relative_eq!(a, b);
    }

    #[test]
    fn blue() {
        let a = Hwb::from(Rgb::new(0.0, 0.0, 1.0));
        let b = Hwb::new(240.0.into(), 0.0, 0.0);
        assert_relative_eq!(a, b);
    }

    #[test]
    fn purple() {
        let a = Hwb::from(Rgb::new(0.5, 0.0, 1.0));
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
