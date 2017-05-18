use num::Float;

use std::ops::{Add, Sub};
use std::marker::PhantomData;

use {Alpha, RgbLinear, Xyz, Hsv, Limited, Mix, Shade, GetHue, Hue, Saturate, RgbHue, FromColor, IntoColor, clamp, flt};
use white_point::{WhitePoint, D65};
use profile::{Primaries, SrgbProfile};

///Linear HSL with an alpha component. See the [`Hsla` implementation in `Alpha`](struct.Alpha.html#Hsla).
pub type Hsla<Wp = D65, T = f32> = Alpha<Hsl<Wp, T>, T>;

///Linear HSL color space.
///
///The HSL color space can be seen as a cylindrical version of
///[RGB](struct.Rgb.html), where the `hue` is the angle around the color
///cylinder, the `saturation` is the distance from the center, and the
///`lightness` is the height from the bottom. Its composition makes it
///especially good for operations like changing green to red, making a color
///more gray, or making it darker.
///
///See [HSV](struct.Hsv.html) for a very similar color space, with brightness instead of lightness.
#[derive(Debug, PartialEq)]
pub struct Hsl<Wp = D65, T = f32>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///The hue of the color, in degrees. Decides if it's red, blue, purple,
    ///etc.
    pub hue: RgbHue<T>,

    ///The colorfulness of the color. 0.0 gives gray scale colors and 1.0 will
    ///give absolutely clear colors.
    pub saturation: T,

    ///Decides how light the color will look. 0.0 will be black, 0.5 will give
    ///a clear color, and 1.0 will give white.
    pub lightness: T,

    ///The white point associated with the color's illuminant and observer.
    ///D65 for 2 degree observer is used by default.
    pub white_point: PhantomData<Wp>,

}

impl<Wp, T> Copy for Hsl<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{}

impl<Wp, T> Clone for Hsl<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn clone(&self) -> Hsl<Wp, T> { *self }
}


impl<T> Hsl<D65, T>
    where T: Float,
{
    ///Linear HSL with white point D65.
    pub fn new(hue: RgbHue<T>, saturation: T, lightness: T) -> Hsl<D65,T> {
        Hsl {
            hue: hue,
            saturation: saturation,
            lightness: lightness,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Hsl<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///Linear HSL.
    pub fn with_wp(hue: RgbHue<T>, saturation: T, lightness: T) -> Hsl<Wp, T> {
        Hsl {
            hue: hue,
            saturation: saturation,
            lightness: lightness,
            white_point: PhantomData,
        }
    }
}

///<span id="Hsla"></span>[`Hsla`](type.Hsla.html) implementations.
impl<T> Alpha<Hsl<D65, T>, T>
    where T: Float,
{
    ///Linear HSL and transparency and white point D65.
    pub fn new(hue: RgbHue<T>, saturation: T, lightness: T, alpha: T) -> Hsla<D65, T> {
        Alpha {
            color: Hsl::new(hue, saturation, lightness),
            alpha: alpha,
        }
    }
}

///<span id="Hsla"></span>[`Hsla`](type.Hsla.html) implementations.
impl<Wp, T> Alpha<Hsl<Wp, T>, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    ///Linear HSL and transparency.
    pub fn with_wp(hue: RgbHue<T>, saturation: T, lightness: T, alpha: T) -> Hsla<Wp, T> {
        Alpha {
            color: Hsl::with_wp(hue, saturation, lightness),
            alpha: alpha,
        }
    }
}

impl<Wp, T> FromColor<Wp, T> for Hsl<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn from_xyz(xyz: Xyz<Wp, T>) -> Self {
        let rgb: RgbLinear<SrgbProfile, Wp, T> = xyz.into_rgb();
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

        let sum = max + min;
        let l = sum / flt(2.0);
        if max != min {
            let d = max - min;
            s = if sum > T::one() { d /( flt::<T,_>(2.0) - sum) } else { d / sum };
            h = (( sep / d ) + coeff) *  flt(60.0);
        };

        Hsl {
            hue: h.into(),
            saturation: s,
            lightness: l,
            white_point: PhantomData,
        }
    }

    fn from_hsl(hsl: Hsl<Wp, T>) -> Self {
        hsl
    }

    fn from_hsv(hsv: Hsv<Wp, T>) -> Self {
        let x = (flt::<T,_>(2.0) - hsv.saturation) * hsv.value;
        let saturation = if !hsv.value.is_normal() {
            T::zero()
        } else if x < T::one() {
            if x.is_normal() { hsv.saturation * hsv.value / x } else { T::zero() }
        } else {
            let denom = flt::<T,_>(2.0) - x;
            if denom.is_normal() { hsv.saturation * hsv.value / denom } else { T::zero() }
        };

        Hsl {
            hue: hsv.hue,
            saturation: saturation,
            lightness: x / flt(2.0),
            white_point: PhantomData,
        }
    }

}

impl<Wp, T> Limited for Hsl<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn is_valid(&self) -> bool {
        self.saturation >= T::zero() && self.saturation <= T::one() &&
        self.lightness >= T::zero() && self.lightness <= T::one()
    }

    fn clamp(&self) -> Hsl<Wp, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.saturation = clamp(self.saturation, T::zero(), T::one());
        self.lightness = clamp(self.lightness, T::zero(), T::one());
    }
}

impl<Wp, T> Mix for Hsl<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn mix(&self, other: &Hsl<Wp, T>, factor: T) -> Hsl<Wp, T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();

        Hsl {
            hue: self.hue + factor * hue_diff,
            saturation: self.saturation + factor * (other.saturation - self.saturation),
            lightness: self.lightness + factor * (other.lightness - self.lightness),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Shade for Hsl<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Hsl<Wp, T> {
        Hsl {
            hue: self.hue,
            saturation: self.saturation,
            lightness: self.lightness + amount,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> GetHue for Hsl<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> Option<RgbHue<T>> {
        if self.saturation <= T::zero() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<Wp, T> Hue for Hsl<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn with_hue(&self, hue: RgbHue<T>) -> Hsl<Wp, T> {
        Hsl {
            hue: hue,
            saturation: self.saturation,
            lightness: self.lightness,
            white_point: PhantomData,
        }
    }

    fn shift_hue(&self, amount: RgbHue<T>) -> Hsl<Wp, T> {
        Hsl {
            hue: self.hue + amount,
            saturation: self.saturation,
            lightness: self.lightness,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Saturate for Hsl<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Scalar = T;

    fn saturate(&self, factor: T) -> Hsl<Wp, T> {
        Hsl {
            hue: self.hue,
            saturation: self.saturation * (T::one() + factor),
            lightness: self.lightness,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Default for Hsl<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    fn default() -> Hsl<Wp, T> {
        Hsl::with_wp(RgbHue::from(T::zero()), T::zero(), T::zero())
    }
}

impl<Wp, T> Add<Hsl<Wp, T>> for Hsl<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Hsl<Wp, T>;

    fn add(self, other: Hsl<Wp, T>) -> Hsl<Wp, T> {
        Hsl {
            hue: self.hue + other.hue,
            saturation: self.saturation + other.saturation,
            lightness: self.lightness + other.lightness,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Add<T> for Hsl<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Hsl<Wp, T>;

    fn add(self, c: T) -> Hsl<Wp, T> {
        Hsl {
            hue: self.hue + c,
            saturation: self.saturation + c,
            lightness: self.lightness + c,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<Hsl<Wp, T>> for Hsl<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Hsl<Wp, T>;

    fn sub(self, other: Hsl<Wp, T>) -> Hsl<Wp, T> {
        Hsl {
            hue: self.hue - other.hue,
            saturation: self.saturation - other.saturation,
            lightness: self.lightness - other.lightness,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Sub<T> for Hsl<Wp, T>
    where T: Float,
        Wp: WhitePoint<T>
{
    type Output = Hsl<Wp, T>;

    fn sub(self, c: T) -> Hsl<Wp, T> {
        Hsl {
            hue: self.hue - c,
            saturation: self.saturation - c,
            lightness: self.lightness - c,
            white_point: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Hsl;
    use {Rgb, Hsv};

    #[test]
    fn red() {
        let a = Hsl::from(Rgb::new(1.0, 0.0, 0.0));
        let b = Hsl::new(0.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::new(0.0.into(), 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);

    }

    #[test]
    fn orange() {
        let a = Hsl::from(Rgb::new(1.0, 0.5, 0.0));
        let b = Hsl::new(30.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::new(30.0.into(), 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn green() {
        let a = Hsl::from(Rgb::new(0.0, 1.0, 0.0));
        let b = Hsl::new(120.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::new(120.0.into(), 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn blue() {
        let a = Hsl::from(Rgb::new(0.0, 0.0, 1.0));
        let b = Hsl::new(240.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::new(240.0.into(), 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn purple() {
        let a = Hsl::from(Rgb::new(0.5, 0.0, 1.0));
        let b = Hsl::new(270.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::new(270.0.into(), 1.0, 1.0));

        assert_relative_eq!(a, b);
        assert_relative_eq!(a, c);
    }

    #[test]
    fn ranges() {
        assert_ranges!{
            Hsl;
            limited {
                saturation: 0.0 => 1.0,
                lightness: 0.0 => 1.0
            }
            limited_min {}
            unlimited {
                hue: -360.0 => 360.0
            }
        }
    }
}
