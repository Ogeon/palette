use num_traits::Float;

use std::marker::PhantomData;

use {Color, Alpha, LinRgb, LinRgba, clamp, flt};

use pixel::RgbPixel;
use white_point::{WhitePoint, D65};

///An sRGB encoded color.
///
///sRGB is a common kind of gamma encoding, but it doesn't exactly follow the
///power-law, as in [`GammaRgb`](struct.GammaRgb.html). It's perhaps the most
///common color space for monitors and on the Internet, so it's usually safe
///to assume that an image or pixel with unknown gamma is sRGB encoded.
///
///```
/// #[macro_use] extern crate approx;
/// # extern crate palette;
/// # use palette::LinRgb;
/// # use palette::pixel::Srgb;
///
/// # fn main() {
/// let c: LinRgb = Srgb::new(0.5, 0.3, 0.1).into();
/// let (r, g, b) = Srgb::from(c).to_pixel();
/// assert_relative_eq!(0.5f32, r);
/// assert_relative_eq!(0.3f32, g);
/// assert_relative_eq!(0.1f32, b);
/// # }
///```
#[derive(Debug, PartialEq)]
pub struct Srgb<Wp = D65, T = f32>
    where T: Float,
        Wp: WhitePoint
{
    ///The red component, where 0.0 is no red light and 1.0 is the
    ///highest displayable amount.
    pub red: T,

    ///The green component, where 0.0 is no red light and 1.0 is the
    ///highest displayable amount.
    pub green: T,

    ///The blue component, where 0.0 is no red light and 1.0 is the
    ///highest displayable amount.
    pub blue: T,

    ///The transparency of the color. 0.0 is completely transparent and 1.0 is
    ///completely opaque.
    pub alpha: T,

    ///The white point associated with the color's illuminant and observer.
///D65 for 2 degree observer is used by default.
pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Copy for Srgb<Wp, T>
    where T: Float,
        Wp: WhitePoint
{}

impl<Wp, T> Clone for Srgb<Wp, T>
    where T: Float,
        Wp: WhitePoint
{
    fn clone(&self) -> Srgb<Wp, T> { *self }
}

impl<T> Srgb<D65, T>
    where T: Float,
{
    ///Create a new opaque sRGB encoded color.
    pub fn new(red: T, green: T, blue: T) -> Srgb<D65, T> {
        Srgb::with_alpha(red, green, blue, T::one())
    }

    ///Create a new sRGB encoded color with transparency.
    pub fn with_alpha(red: T, green: T, blue: T, alpha: T) -> Srgb<D65, T> {
        Srgb {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha,
            white_point: PhantomData,
        }
    }

    ///Create a new opaque sRGB encoded color from `u8` values.
    pub fn new_u8(red: u8, green: u8, blue: u8) -> Srgb<D65, T> {
        Srgb::with_alpha_u8(red, green, blue, 255)
    }

    ///Create a new sRGB encoded color, with transparency, from `u8` values.
    pub fn with_alpha_u8(red: u8, green: u8, blue: u8, alpha: u8) -> Srgb<D65, T> {
        Srgb {
            red: flt::<T,_>(red) / flt(255.0),
            green: flt::<T,_>(green) / flt(255.0),
            blue: flt::<T,_>(blue) / flt(255.0),
            alpha: flt::<T,_>(alpha) / flt(255.0),
            white_point: PhantomData,
        }
    }

    ///Convert linear color components to sRGB encoding.
    pub fn from_linear<C: Into<LinRgba<::rgb::standards::Srgb, T>>>(color: C) -> Srgb<D65, T> {
        let rgb = color.into();
        Srgb {
            red: to_srgb(rgb.red),
            green: to_srgb(rgb.green),
            blue: to_srgb(rgb.blue),
            alpha: rgb.alpha,
            white_point: PhantomData,
        }
    }

    ///Decode this color to a linear representation.
    pub fn to_linear(&self) -> LinRgba<::rgb::standards::Srgb, T> {
        Alpha {
            color: LinRgb::<::rgb::standards::Srgb, T>::with_wp(
                from_srgb(self.red),
                from_srgb(self.green),
                from_srgb(self.blue),
            ),
            alpha: self.alpha,
        }
    }

    ///Shortcut to convert a linear color to an sRGB encoded pixel.
    pub fn linear_to_pixel<C: Into<LinRgba<::rgb::standards::Srgb, T>>, P: RgbPixel<T>>(color: C) -> P {
        Srgb::from_linear(color).to_pixel()
    }
}

impl<Wp, T> Srgb<Wp, T>
    where T: Float,
        Wp: WhitePoint
{
    ///Create a new opaque sRGB encoded color.
    pub fn with_wp(red: T, green: T, blue: T) -> Srgb<Wp, T> {
        Srgb::with_wp_alpha(red, green, blue, T::one())
    }

    ///Create a new sRGB encoded color with transparency.
    pub fn with_wp_alpha(red: T, green: T, blue: T, alpha: T) -> Srgb<Wp, T> {
        Srgb {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha,
            white_point: PhantomData,
        }
    }

    ///Create a new opaque sRGB encoded color from `u8` values.
    pub fn with_wp_u8(red: u8, green: u8, blue: u8) -> Srgb<Wp, T> {
        Srgb::with_wp_alpha_u8(red, green, blue, 255)
    }

    ///Create a new sRGB encoded color, with transparency, from `u8` values.
    pub fn with_wp_alpha_u8(red: u8, green: u8, blue: u8, alpha: u8) -> Srgb<Wp, T> {
        Srgb {
            red: flt::<T,_>(red) / flt(255.0),
            green: flt::<T,_>(green) / flt(255.0),
            blue: flt::<T,_>(blue) / flt(255.0),
            alpha: flt::<T,_>(alpha) / flt(255.0),
            white_point: PhantomData,
        }
    }

    ///Create a new sRGB encoded color from a pixel value.
    pub fn from_pixel<P: RgbPixel<T>>(pixel: &P) -> Srgb<Wp, T> {
        let (r, g, b, a) = pixel.to_rgba();
        Srgb::with_wp_alpha(r, g, b, a)
    }

    ///Transform this color into a pixel representation.
    pub fn to_pixel<P: RgbPixel<T>>(&self) -> P {
        P::from_rgba(
            clamp(self.red, T::zero(), T::one()),
            clamp(self.green, T::zero(), T::one()),
            clamp(self.blue, T::zero(), T::one()),
            clamp(self.alpha, T::zero(), T::one()),
        )
    }
}

impl<T> From<LinRgb<::rgb::standards::Srgb, T>> for Srgb<D65, T>
    where T: Float,
{
    fn from(rgb: LinRgb<::rgb::standards::Srgb, T>) -> Srgb<D65, T> {
        LinRgba::from(rgb).into()
    }
}

impl<T> From<LinRgba<::rgb::standards::Srgb, T>> for Srgb<D65, T>
    where T: Float,
{
    fn from(rgba: LinRgba<::rgb::standards::Srgb, T>) -> Srgb<D65, T> {
        Srgb::from_linear(rgba)
    }
}

impl<T> From<Color<::rgb::standards::Srgb, T>> for Srgb<D65, T>
    where T: Float,
{
    fn from(color: Color<::rgb::standards::Srgb, T>) -> Srgb<D65, T> {
        LinRgb::from(color).into()
    }
}

fn from_srgb<T: Float>(x: T) -> T {
    if x <= flt(0.04045) {
        x / flt(12.92)
    } else {
        ((x + flt(0.055)) / flt(1.055)).powf(flt(2.4))
    }
}

fn to_srgb<T: Float>(x: T) -> T {
    if x <= flt(0.0031308) {
        x * flt(12.92)
    } else {
        x.powf(T::one() / flt(2.4)) * flt(1.055)  - flt(0.055)
    }
}
