use num::Float;

use std::ops::{Add, Sub};
use std::marker::PhantomData;

use {Alpha, RgbLinear, Xyz, Hsl, Hwb};
use {Limited, Mix, Shade, GetHue, Hue, Saturate, RgbHue, FromColor};
use {clamp, flt};
use white_point::{WhitePoint, D65};
use profile::{Primaries, SrgbProfile};

///Linear HSV with an alpha component. See the [`Hsva` implementation in `Alpha`](struct.Alpha.html#Hsva).
pub type Hsva<Wp = D65, T = f32> = Alpha<Hsv<Wp, T>, T>;

///Linear HSV color space.
///
///HSV is a cylindrical version of [RGB](struct.Rgb.html) and it's very
///similar to [HSL](struct.Hsl.html). The difference is that the `value`
///component in HSV determines the _brightness_ of the color, and not the
///_lightness_. The difference is that, for example, red (100% R, 0% G, 0% B)
///and white (100% R, 100% G, 100% B) has the same brightness (or value), but
///not the same lightness.
#[derive(Debug, PartialEq)]
pub struct Hsv<Wp = D65, T = f32>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///The hue of the color, in degrees. Decides if it's red, blue, purple,
    ///etc.
    pub hue: RgbHue<T>,

    ///The colorfulness of the color. 0.0 gives gray scale colors and 1.0 will
    ///give absolutely clear colors.
    pub saturation: T,

    ///Decides how bright the color will look. 0.0 will be black, and 1.0 will
    ///give a bright an clear color that goes towards white when `saturation`
    ///goes towards 0.0.
    pub value: T,

    ///The white point associated with the color's illuminant and observer.
    ///D65 for 2 degree observer is used by default.
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Copy for Hsv<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{}

impl<Wp, T> Clone for Hsv<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn clone(&self) -> Hsv<Wp, T> { *self }
}


impl<T> Hsv<D65, T>
    where T: Float,
{
    ///Linear HSV with white point D65.
    pub fn new(hue: RgbHue<T>, saturation: T, value: T) -> Hsv<D65, T> {
        Hsv {
            hue: hue,
            saturation: saturation,
            value: value,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Hsv<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///Linear HSV.
    pub fn with_wp(hue: RgbHue<T>, saturation: T, value: T) -> Hsv<Wp, T> {
        Hsv {
            hue: hue,
            saturation: saturation,
            value: value,
            white_point: PhantomData,
        }
    }
}

///<span id="Hsva"></span>[`Hsva`](type.Hsva.html) implementations.
impl<T> Alpha<Hsv<D65, T>, T>
    where T: Float,
{
    ///Linear HSV and transparency with white point D65.
    pub fn new(hue: RgbHue<T>, saturation: T, value: T, alpha: T) -> Hsva<D65, T> {
        Alpha {
            color: Hsv::new(hue, saturation, value),
            alpha: alpha,
        }
    }
}

///<span id="Hsva"></span>[`Hsva`](type.Hsva.html) implementations.
impl<Wp, T> Alpha<Hsv<Wp, T>, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///Linear HSV and transparency.
    pub fn with_wp(hue: RgbHue<T>, saturation: T, value: T, alpha: T) -> Hsva<Wp, T> {
        Alpha {
            color: Hsv::with_wp(hue, saturation, value),
            alpha: alpha,
        }
    }
}

impl<Wp, T> FromColor<Wp, T> for Hsv<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn from_xyz(xyz: Xyz<Wp, T>) -> Self {
        let rgb: RgbLinear<SrgbProfile, Wp, T> = RgbLinear::from_xyz(xyz);
        // let hsl = xyz.into_hsl();
        Self::from_rgb(rgb)
    }

    fn from_rgb<P: Primaries<Wp, T>>(rgb: RgbLinear<P, Wp, T>) -> Self {
        let ( max, min, sep , coeff) = {
            let (max, min , sep, coeff) = if rgb.red > rgb.green {
                (rgb.red, rgb.green, rgb.green - rgb.blue, T::zero() )
            } else {
                (rgb.green, rgb.red, rgb.blue - rgb.red , flt(2.0))
            };
            if rgb.blue > max {
                ( rgb.blue, min , rgb.red - rgb.green , flt(4.0))
            } else {
                let min_val = if rgb.blue < min { rgb.blue } else { min };
                (max , min_val , sep, coeff)
            }
        };

        let mut h = T::zero();
        let mut s = T::zero();
        let v = max;

        if max != min {
            let d = max - min;
            s = d / max;
            h = (( sep / d ) + coeff) *  flt(60.0);
        };

        Hsv {
            hue: h.into(),
            saturation: s,
            value: v,
            white_point: PhantomData,
        }
    }

    fn from_hsl(hsl: Hsl<Wp, T>) -> Self {
        let x = hsl.saturation * if hsl.lightness < flt(0.5) {
            hsl.lightness
        } else {
            T::one() - hsl.lightness
        };
        let mut s = T::zero();

        // avoid divide by zero
        let denom = hsl.lightness + x;
        if denom.is_normal() {
            s = x * flt(2.0) / denom;
        }
        Hsv {
            hue: hsl.hue,
            saturation: s,
            value: hsl.lightness + x,
            white_point: PhantomData,
        }
    }

    fn from_hsv(hsv: Hsv<Wp, T>) -> Self {
        hsv
    }

    fn from_hwb(hwb: Hwb<Wp, T>) -> Self {
        let inv = T::one() - hwb.blackness;
        // avoid divide by zero
        let s = if inv.is_normal() {
            T::one() - ( hwb.whiteness / inv )
        } else {
            T::zero()
        };
        Hsv {
            hue: hwb.hue,
            saturation: s,
            value: inv,
            white_point: PhantomData,
        }
    }

}

impl<Wp, T> Limited for Hsv<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn is_valid(&self) -> bool {
        self.saturation >= T::zero() && self.saturation <= T::one() &&
        self.value >= T::zero() && self.value <= T::one()
    }

    fn clamp(&self) -> Hsv<Wp, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.saturation = clamp(self.saturation, T::zero(), T::one());
        self.value = clamp(self.value, T::zero(), T::one());
    }
}

impl<Wp, T> Mix for Hsv<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn mix(&self, other: &Hsv<Wp, T>, factor: T) -> Hsv<Wp, T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();

        Hsv {
            hue: self.hue + factor * hue_diff,
            saturation: self.saturation + factor * (other.saturation - self.saturation),
            value: self.value + factor * (other.value - self.value),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Shade for Hsv<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Hsv<Wp, T> {
        Hsv {
            hue: self.hue,
            saturation: self.saturation,
            value: self.value + amount,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> GetHue for Hsv<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> Option<RgbHue<T>> {
        if self.saturation <= T::zero() || self.value <= T::zero() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<Wp, T> Hue for Hsv<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn with_hue(&self, hue: RgbHue<T>) -> Hsv<Wp, T> {
        Hsv {
            hue: hue,
            saturation: self.saturation,
            value: self.value,
            white_point: PhantomData,
        }
    }

    fn shift_hue(&self, amount: RgbHue<T>) -> Hsv<Wp, T> {
        Hsv {
            hue: self.hue + amount,
            saturation: self.saturation,
            value: self.value,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Saturate for Hsv<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn saturate(&self, factor: T) -> Hsv<Wp, T> {
        Hsv {
            hue: self.hue,
            saturation: self.saturation * (T::one() + factor),
            value: self.value,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Default for Hsv<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn default() -> Hsv<Wp, T> {
        Hsv::with_wp(RgbHue::from(T::zero()), T::zero(), T::zero())
    }
}

impl<Wp, T> Add<Hsv<Wp, T>> for Hsv<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Hsv<Wp, T>;

    fn add(self, other: Hsv<Wp, T>) -> Hsv<Wp, T> {
        Hsv {
            hue: self.hue + other.hue,
            saturation: self.saturation + other.saturation,
            value: self.value + other.value,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Add<T> for Hsv<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Hsv<Wp, T>;

    fn add(self, c: T) -> Hsv<Wp, T> {
        Hsv {
            hue: self.hue + c,
            saturation: self.saturation + c,
            value: self.value + c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<Hsv<Wp, T>> for Hsv<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Hsv<Wp, T>;

    fn sub(self, other: Hsv<Wp, T>) -> Hsv<Wp, T> {
        Hsv {
            hue: self.hue - other.hue,
            saturation: self.saturation - other.saturation,
            value: self.value - other.value,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<T> for Hsv<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Hsv<Wp, T>;

    fn sub(self, c: T) -> Hsv<Wp, T> {
        Hsv {
            hue: self.hue - c,
            saturation: self.saturation - c,
            value: self.value - c,
            white_point: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Hsv;
    use {Rgb, Hsl};

    #[test]
    fn red() {
        let a = Hsv::from(Rgb::new(1.0, 0.0, 0.0));
        let b = Hsv::new(0.0.into(), 1.0, 1.0);
        let c = Hsv::from(Hsl::new(0.0.into(), 1.0, 0.5));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn orange() {
        let a = Hsv::from(Rgb::new(1.0, 0.5, 0.0));
        let b = Hsv::new(30.0.into(), 1.0, 1.0);
        let c = Hsv::from(Hsl::new(30.0.into(), 1.0, 0.5));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn green() {
        let a = Hsv::from(Rgb::new(0.0, 1.0, 0.0));
        let b = Hsv::new(120.0.into(), 1.0, 1.0);
        let c = Hsv::from(Hsl::new(120.0.into(), 1.0, 0.5));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn blue() {
        let a = Hsv::from(Rgb::new(0.0, 0.0, 1.0));
        let b = Hsv::new(240.0.into(), 1.0, 1.0);
        let c = Hsv::from(Hsl::new(240.0.into(), 1.0, 0.5));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn purple() {
        let a = Hsv::from(Rgb::new(0.5, 0.0, 1.0));
        let b = Hsv::new(270.0.into(), 1.0, 1.0);
        let c = Hsv::from(Hsl::new(270.0.into(), 1.0, 0.5));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn ranges() {
        assert_ranges!{
            Hsv;
            limited {
                saturation: 0.0 => 1.0,
                value: 0.0 => 1.0
            }
            limited_min {}
            unlimited {
                hue: -360.0 => 360.0
            }
        }
    }
}
