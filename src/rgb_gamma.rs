use num::Float;

use std::ops::{Add, Sub, Mul, Div};
use std::marker::PhantomData;

use {Alpha, Luma, Xyz, Hsv, Hsl, RgbHue};
use {Limited, Mix, Shade, GetHue, FromColor, Blend, ComponentWise};
use white_point::{WhitePoint, D65};
use pixel::RgbPixel;
use profile::{Encoding, SrgbProfile};
use matrix::{matrix_inverse, multiply_xyz_to_rgb};
use {clamp, flt};
// use pixel::{RgbPixel, Srgb, GammaRgb};
use blend::PreAlpha;

///Linear RGB with an alpha component. See the [`Rgba` implementation in `Alpha`](struct.Alpha.html#Rgba).
pub type Rgba<E = SrgbProfile, Wp = D65, T = f32> = Alpha<Rgb<E, Wp, T>, T>;

///Linear RGB.
///
///RGB is probably the most common color space, when it comes to computer
///graphics, and it's defined as an additive mixture of red, green and blue
///light, where gray scale colors are created when these three channels are
///equal in strength. This particular RGB type is based on the ITU-R BT.709
///primaries, which makes it a linear version of sRGB.
///
///Conversions and operations on this color space assumes that it's linear,
///meaning that gamma correction is required when converting to and from a
///displayable RGB, such as sRGB. See the [`pixel`](pixel/index.html) module
///for encoding types.
#[derive(Debug, PartialEq)]
pub struct Rgb<E = SrgbProfile, Wp = D65, T = f32>
    where T: Float,
        Wp: WhitePoint<T>,
        E: Encoding<T>,
{
    ///The amount of red light, where 0.0 is no red light and 1.0 is the
    ///highest displayable amount.
    pub red: T,

    ///The amount of green light, where 0.0 is no green light and 1.0 is the
    ///highest displayable amount.
    pub green: T,

    ///The amount of blue light, where 0.0 is no blue light and 1.0 is the
    ///highest displayable amount.
    pub blue: T,

    ///The white point associated with the color's illuminant and observer.
    ///D65 for 2 degree observer is used by default.
    pub white_point: PhantomData<Wp>,

    ///The encoding associated with the color profile
    pub encoding: PhantomData<E>,
}

impl<E, Wp, T> Copy for Rgb<E, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        E: Encoding<T>,
{}

impl<E, Wp, T> Clone for Rgb<E, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        E: Encoding<T>,
{
    fn clone(&self) -> Rgb<E, Wp, T> { *self }
}

impl<T> Rgb<SrgbProfile, D65, T>
    where T: Float,
{
    ///Linear RGB with white point D65.
    pub fn new(red: T, green: T, blue: T) -> Rgb<SrgbProfile, D65, T> {
        Rgb {
            red: red,
            green: green,
            blue: blue,
            white_point: PhantomData,
            encoding: PhantomData,
        }
    }

    ///Linear RGB from 8 bit values with whtie point D65.
    pub fn new_u8(red: u8, green: u8, blue: u8) -> Rgb<SrgbProfile, D65, T> {
        Rgb {
            red: flt::<T,_>(red) / flt(255.0),
            green: flt::<T,_>(green) / flt(255.0),
            blue: flt::<T,_>(blue) / flt(255.0),
            white_point: PhantomData,
            encoding: PhantomData,
        }
    }
}

impl<E, Wp, T> Rgb<E, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        E: Encoding<T>,
{
    ///Linear RGB.
    pub fn with_wp(red: T, green: T, blue: T) -> Rgb<E, Wp, T> {
        Rgb {
            red: red,
            green: green,
            blue: blue,
            white_point: PhantomData,
            encoding: PhantomData,
        }
    }

    ///Linear RGB from 8 bit values.
    pub fn with_wp_u8(red: u8, green: u8, blue: u8) -> Rgb<E, Wp, T> {
        Rgb {
            red: flt::<T,_>(red) / flt(255.0),
            green: flt::<T,_>(green) / flt(255.0),
            blue: flt::<T,_>(blue) / flt(255.0),
            white_point: PhantomData,
            encoding: PhantomData,
        }
    }

    ///Linear RGB from a linear pixel value.
    pub fn from_pixel<P: RgbPixel<T>>(pixel: &P) -> Rgb<E, Wp, T> {
        let (r, g, b, _) = pixel.to_rgba();
        Rgb::with_wp(r, g, b)
    }

    ///Convert to a linear RGB pixel. `Rgb` is already assumed to be linear,
    ///so the components will just be clamped to [0.0, 1.0] before conversion.
    ///
    ///```
    ///use palette::Rgb;
    ///
    ///let c = Rgb::new(0.5, 0.3, 0.1);
    ///assert_eq!((c.red, c.green, c.blue), c.to_pixel());
    ///assert_eq!((0.5, 0.3, 0.1), c.to_pixel());
    ///```
    pub fn to_pixel<P: RgbPixel<T>>(&self) -> P {
        P::from_rgba(
            clamp(self.red, T::zero(), T::one()),
            clamp(self.green, T::zero(), T::one()),
            clamp(self.blue, T::zero(), T::one()),
            T::one(),
        )
    }
}

///<span id="Rgba"></span>[`Rgba`](type.Rgba.html) implementations.
impl<T> Alpha<Rgb<SrgbProfile, D65, T>, T>
    where T: Float,
{
    ///Linear RGB with transparency and with white point D65.
    pub fn new(red: T, green: T, blue: T, alpha: T) -> Rgba<SrgbProfile, D65, T> {
        Alpha {
            color: Rgb::new(red, green, blue),
            alpha: alpha,
        }
    }

    ///Linear RGB with transparency from 8 bit values and with white point D65.
    pub fn new_u8(red: u8, green: u8, blue: u8, alpha: u8) -> Rgba<SrgbProfile, D65, T> {
        Alpha {
            color: Rgb::new_u8(red, green, blue),
            alpha: flt::<T,_>(alpha) / flt(255.0),
        }
    }

}

///<span id="Rgba"></span>[`Rgba`](type.Rgba.html) implementations.
impl<E, Wp, T> Alpha<Rgb<E, Wp, T>, T>
    where T: Float,
        Wp: WhitePoint<T>,
        E: Encoding<T>,
{
    ///Linear RGB with transparency.
    pub fn with_wp(red: T, green: T, blue: T, alpha: T) -> Rgba<E, Wp, T> {
        Alpha {
            color: Rgb::with_wp(red, green, blue),
            alpha: alpha,
        }
    }

    ///Linear RGB with transparency from 8 bit values.
    pub fn with_wp_u8(red: u8, green: u8, blue: u8, alpha: u8) -> Rgba<E, Wp, T> {
        Alpha {
            color: Rgb::with_wp_u8(red, green, blue),
            alpha: flt::<T,_>(alpha) / flt(255.0),
        }
    }

    ///Linear RGB from a linear pixel value.
    pub fn from_pixel<P: RgbPixel<T>>(pixel: &P) -> Rgba<E, Wp, T> {
        let (r, g, b, a) = pixel.to_rgba();
        Rgba::with_wp(r, g, b, a)
    }

    ///Convert to a linear RGB pixel. `Rgb` is already assumed to be linear,
    ///so the components will just be clamped to [0.0, 1.0] before conversion.
    ///
    ///```
    ///use palette::Rgba;
    ///
    ///let c = Rgba::new(0.5, 0.3, 0.1, 0.5);
    ///assert_eq!((c.red, c.green, c.blue, c.alpha), c.to_pixel());
    ///assert_eq!((0.5, 0.3, 0.1, 0.5), c.to_pixel());
    ///```
    pub fn to_pixel<P: RgbPixel<T>>(&self) -> P {
        P::from_rgba(
            clamp(self.red, T::zero(), T::one()),
            clamp(self.green, T::zero(), T::one()),
            clamp(self.blue, T::zero(), T::one()),
            clamp(self.alpha, T::zero(), T::one())
        )
    }
}


impl<E, Wp, T> Limited for Rgb<E, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        E: Encoding<T>,
{
    fn is_valid(&self) -> bool {
        self.red >= T::zero() && self.red <= T::one() &&
        self.green >= T::zero() && self.green <= T::one() &&
        self.blue >= T::zero() && self.blue <= T::one()
    }

    fn clamp(&self) -> Rgb<E, Wp, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.red = clamp(self.red, T::zero(), T::one());
        self.green = clamp(self.green, T::zero(), T::one());
        self.blue = clamp(self.blue, T::zero(), T::one());
    }
}

impl<E, Wp, T> Mix for Rgb<E, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        E: Encoding<T>,
{
    type Scalar = T;

    fn mix(&self, other: &Rgb<E, Wp, T>, factor: T) -> Rgb<E, Wp, T> {
        let factor = clamp(factor, T::zero(), T::one());

        Rgb {
            red: self.red + factor * (other.red - self.red),
            green: self.green + factor * (other.green - self.green),
            blue: self.blue + factor * (other.blue - self.blue),
            white_point: PhantomData,
            encoding: PhantomData,
        }
    }
}

impl<E, Wp, T> Shade for Rgb<E, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        E: Encoding<T>,
{
    type Scalar = T;

    fn lighten(&self, amount: T) -> Rgb<E, Wp, T> {
        Rgb {
            red: self.red + amount,
            green: self.green + amount,
            blue: self.blue + amount,
            white_point: PhantomData,
            encoding: PhantomData,
        }
    }
}

impl<E, Wp, T> GetHue for Rgb<E, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        E: Encoding<T>,
{
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> Option<RgbHue<T>> {
        let sqrt_3: T = flt(1.73205081);

        if self.red == self.green && self.red == self.blue {
            None
        } else {
            Some(RgbHue::from_radians((sqrt_3 * (self.green - self.blue)).atan2(self.red * flt(2.0) - self.green - self.blue)))
        }
    }
}

impl<E, Wp, T> Default for Rgb<E, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        E: Encoding<T>,
{
    fn default() -> Rgb<E, Wp, T> {
        Rgb::with_wp(T::zero(), T::zero(), T::zero())
    }
}

impl<E, Wp, T> Add<Rgb<E, Wp, T>> for Rgb<E, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        E: Encoding<T>,
{
    type Output = Rgb<E, Wp, T>;

    fn add(self, other: Rgb<E, Wp, T>) -> Rgb<E, Wp, T> {
        Rgb {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
            white_point: PhantomData,
            encoding: PhantomData,
        }
    }
}

impl<E, Wp, T> Add<T> for Rgb<E, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        E: Encoding<T>,
{
    type Output = Rgb<E, Wp, T>;

    fn add(self, c: T) -> Rgb<E, Wp, T> {
        Rgb {
            red: self.red + c,
            green: self.green + c,
            blue: self.blue + c,
            white_point: PhantomData,
            encoding: PhantomData,
        }
    }
}

impl<E, Wp, T> Sub<Rgb<E, Wp, T>> for Rgb<E, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        E: Encoding<T>,
{
    type Output = Rgb<E, Wp, T>;

    fn sub(self, other: Rgb<E, Wp, T>) -> Rgb<E, Wp, T> {
        Rgb {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
            white_point: PhantomData,
            encoding: PhantomData,
        }
    }
}

impl<E, Wp, T> Sub<T> for Rgb<E, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        E: Encoding<T>,
{
    type Output = Rgb<E, Wp, T>;

    fn sub(self, c: T) -> Rgb<E, Wp, T> {
        Rgb {
            red: self.red - c,
            green: self.green - c,
            blue: self.blue - c,
            white_point: PhantomData,
            encoding: PhantomData,
        }
    }
}

impl<E, Wp, T> Mul<Rgb<E, Wp, T>> for Rgb<E, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        E: Encoding<T>,
{
    type Output = Rgb<E, Wp, T>;

    fn mul(self, other: Rgb<E, Wp, T>) -> Rgb<E, Wp, T> {
        Rgb {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
            white_point: PhantomData,
            encoding: PhantomData,
        }
    }
}

impl<E, Wp, T> Mul<T> for Rgb<E, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        E: Encoding<T>,
{
    type Output = Rgb<E, Wp, T>;

    fn mul(self, c: T) -> Rgb<E, Wp, T> {
        Rgb {
            red: self.red * c,
            green: self.green * c,
            blue: self.blue * c,
            white_point: PhantomData,
            encoding: PhantomData,
        }
    }
}

impl<E, Wp, T> Div<Rgb<E, Wp, T>> for Rgb<E, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        E: Encoding<T>,
{
    type Output = Rgb<E, Wp, T>;

    fn div(self, other: Rgb<E, Wp, T>) -> Rgb<E, Wp, T> {
        Rgb {
            red: self.red / other.red,
            green: self.green / other.green,
            blue: self.blue / other.blue,
            white_point: PhantomData,
            encoding: PhantomData,
        }
    }
}

impl<E, Wp, T> Div<T> for Rgb<E, Wp, T>
    where T: Float,
        Wp: WhitePoint<T>,
        E: Encoding<T>,
{
    type Output = Rgb<E, Wp, T>;

    fn div(self, c: T) -> Rgb<E, Wp, T> {
        Rgb {
            red: self.red / c,
            green: self.green / c,
            blue: self.blue / c,
            white_point: PhantomData,
            encoding: PhantomData,
        }
    }
}


#[cfg(test)]
mod test {
    use Rgb;

    #[test]
    fn ranges() {
        assert_ranges!{
            Rgb;
            limited {
                red: 0.0 => 1.0,
                green: 0.0 => 1.0,
                blue: 0.0 => 1.0
            }
            limited_min {}
            unlimited {}
        }
    }
}
