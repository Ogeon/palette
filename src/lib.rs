//!A library that makes linear color calculations and conversion easy and
//!accessible for anyone. It provides both precision tools that lets you work
//!in exactly the color space you want to, as well as a general color type
//!that abstracts away some of the technical details.
//!
//!# Linear?
//!
//!Colors in, for example, images are often "gamma corrected" or stored in
//!sRGB format as a compression method and to prevent banding. This is also a
//!bit of a legacy from the ages of the CRT monitors, where the output from
//!the electron guns was nonlinear. The problem is that these formats doesn't
//!represent the actual intensities, and the compression has to be reverted to
//!make sure that any operations on the colors are accurate. This library uses
//!a completely linear work flow, and comes with the tools for transitioning
//!between linear and non-linear RGB.


#![doc(html_root_url = "http://ogeon.github.io/docs/palette/master/")]

#![cfg_attr(feature = "strict", deny(missing_docs))]
#![cfg_attr(feature = "strict", deny(warnings))]

#[cfg(test)]
#[macro_use]
extern crate approx;
extern crate num;

use num::traits::Float;

pub use gradient::Gradient;
pub use rgb::{Rgb, RgbPixel};
pub use luma::Luma;
pub use xyz::Xyz;
pub use lab::Lab;
pub use lch::Lch;
pub use hsv::Hsv;
pub use hsl::Hsl;

pub use hues::{LabHue, RgbHue};

macro_rules! from_color {
    (to $to:ident from $($from:ident),+) => (
        impl<T:Float> From<Color<T>> for $to<T> {
            fn from(color: Color<T>) -> $to<T> {
                match color {
                    Color::$to(c) => c,
                    $(Color::$from(c) => c.into(),)+
                }
            }
        }
    )
}

//Helper macro for approximate component wise comparison. Most color spaces
//are in roughly the same ranges, so this epsilon should be alright.
#[cfg(test)]
macro_rules! assert_approx_eq {
    ($a:ident, $b:ident, [$($components:ident),+]) => ({
        $(
            let a: f32 = $a.$components.into();
            let b: f32 = $b.$components.into();
            assert_relative_eq!(a, b, epsilon = 0.0001);
        )+
        assert_relative_eq!($a.alpha, $b.alpha, epsilon = 0.0001);
    })
}

pub mod gradient;

mod rgb;
mod luma;
mod xyz;
mod lab;
mod lch;
mod hsv;
mod hsl;

mod hues;

mod tristimulus;


///A generic color type.
///
///The `Color` may belong to any color space and it may change
///depending on which operation is performed. That makes it immune to
///the "without conversion" rule of the operations it supports. The
///color spaces are selected as follows:
///
/// * `Mix`: RGB for no particular reason, except that it's intuitive.
/// * `Shade`: CIE L*a*b* for its luminance component.
/// * `Hue` and `GetHue`: CIE L*C*h° for its hue component and how it preserves the apparent lightness.
/// * `Saturate`: CIE L*C*h° for its chromaticity component and how it preserves the apparent lightness.
///
///It's not recommended to use `Color` when full control is necessary,
///but it can easily be converted to a fixed color space in those
///cases.
pub enum Color<T: Float> {













    #[doc = r"Linear luminance."]
    Luma(Luma<T>),




    #[doc = r"Linear RGB."]
    Rgb(Rgb<T>),















    #[doc = r"CIE 1931 XYZ."]
    Xyz(Xyz<T>),


    #[doc = r"CIE L*a*b* (CIELAB)."]
    Lab(Lab<T>),


    #[doc = r"CIE L*C*h°, a polar version of CIE L*a*b*."]
    Lch(Lch<T>),


    #[doc = r"Linear HSV, a cylindrical version of RGB."]
    Hsv(Hsv<T>),


    #[doc = r"Linear HSL, a cylindrical version of RGB."]
    Hsl(Hsl<T>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::std::fmt::Debug + Float> ::std::fmt::Debug for Color<T> {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&Color::Luma(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("Luma");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
            (&Color::Rgb(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("Rgb");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
            (&Color::Xyz(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("Xyz");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
            (&Color::Lab(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("Lab");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
            (&Color::Lch(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("Lch");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
            (&Color::Hsv(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("Hsv");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
            (&Color::Hsl(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("Hsl");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::std::marker::Copy + Float> ::std::marker::Copy for Color<T> { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::std::clone::Clone + Float> ::std::clone::Clone for Color<T> {
    #[inline]
    fn clone(&self) -> Color<T> {
        match (&*self,) {
            (&Color::Luma(ref __self_0),) =>
            Color::Luma(::std::clone::Clone::clone(&(*__self_0))),
            (&Color::Rgb(ref __self_0),) =>
            Color::Rgb(::std::clone::Clone::clone(&(*__self_0))),
            (&Color::Xyz(ref __self_0),) =>
            Color::Xyz(::std::clone::Clone::clone(&(*__self_0))),
            (&Color::Lab(ref __self_0),) =>
            Color::Lab(::std::clone::Clone::clone(&(*__self_0))),
            (&Color::Lch(ref __self_0),) =>
            Color::Lch(::std::clone::Clone::clone(&(*__self_0))),
            (&Color::Hsv(ref __self_0),) =>
            Color::Hsv(::std::clone::Clone::clone(&(*__self_0))),
            (&Color::Hsl(ref __self_0),) =>
            Color::Hsl(::std::clone::Clone::clone(&(*__self_0))),
        }
    }
}
impl <T: Float> Color<T> {
    #[doc = r"Linear luminance."]
    pub fn y(luma: T) -> Color<T> { Color::Luma(Luma::y(luma)) }
    #[doc = r"Linear luminance with transparency."]
    pub fn ya(luma: T, alpha: T) -> Color<T> {
        Color::Luma(Luma::ya(luma, alpha))
    }
    #[doc = r"Linear luminance from an 8 bit value."]
    pub fn y8(luma: u8) -> Color<T> { Color::Luma(Luma::y8(luma)) }
    #[doc = r"Linear luminance and transparency from 8 bit values."]
    pub fn ya8(luma: u8, alpha: u8) -> Color<T> {
        Color::Luma(Luma::ya8(luma, alpha))
    }
}
impl <T: Float> Color<T> {
    #[doc = r"Linear RGB."]
    pub fn linear_rgb(red: T, green: T, blue: T) -> Color<T> {
        Color::Rgb(Rgb::linear_rgb(red, green, blue))
    }
    #[doc = r"Linear RGB and transparency."]
    pub fn linear_rgba(red: T, green: T, blue: T, alpha: T) -> Color<T> {
        Color::Rgb(Rgb::linear_rgba(red, green, blue, alpha))
    }
    #[doc = r"Linear RGB from 8 bit values."]
    pub fn linear_rgb8(red: u8, green: u8, blue: u8) -> Color<T> {
        Color::Rgb(Rgb::linear_rgb8(red, green, blue))
    }
    #[doc = r"Linear RGB and transparency from 8 bit values."]
    pub fn linear_rgba8(red: u8, green: u8, blue: u8, alpha: u8) -> Color<T> {
        Color::Rgb(Rgb::linear_rgba8(red, green, blue, alpha))
    }
    #[doc = r"Linear RGB from a linear pixel value."]
    pub fn linear_pixel<P: RgbPixel<T>>(pixel: &P) -> Color<T> {
        Color::Rgb(Rgb::linear_pixel(pixel))
    }
    #[doc = r"Linear RGB from sRGB."]
    pub fn srgb(red: T, green: T, blue: T) -> Color<T> {
        Color::Rgb(Rgb::srgb(red, green, blue))
    }
    #[doc = r"Linear RGB from sRGB with transparency."]
    pub fn srgba(red: T, green: T, blue: T, alpha: T) -> Color<T> {
        Color::Rgb(Rgb::srgba(red, green, blue, alpha))
    }
    #[doc = r"Linear RGB from 8 bit sRGB."]
    pub fn srgb8(red: u8, green: u8, blue: u8) -> Color<T> {
        Color::Rgb(Rgb::srgb8(red, green, blue))
    }
    #[doc = r"Linear RGB from 8 bit sRGB with transparency."]
    pub fn srgba8(red: u8, green: u8, blue: u8, alpha: u8) -> Color<T> {
        Color::Rgb(Rgb::srgba8(red, green, blue, alpha))
    }
    #[doc = r"Linear RGB from an sRGB pixel value."]
    pub fn srgb_pixel<P: RgbPixel<T>>(pixel: &P) -> Color<T> {
        Color::Rgb(Rgb::srgb_pixel(pixel))
    }
    #[doc = r"Linear RGB from gamma corrected RGB."]
    pub fn gamma_rgb(red: T, green: T, blue: T, gamma: T) -> Color<T> {
        Color::Rgb(Rgb::gamma_rgb(red, green, blue, gamma))
    }
    #[doc = r"Linear RGB from gamma corrected RGB with transparency."]
    pub fn gamma_rgba(red: T, green: T, blue: T, alpha: T, gamma: T)
     -> Color<T> {
        Color::Rgb(Rgb::gamma_rgba(red, green, blue, alpha, gamma))
    }
    #[doc = r"Linear RGB from 8 bit gamma corrected RGB."]
    pub fn gamma_rgb8(red: u8, green: u8, blue: u8, gamma: T) -> Color<T> {
        Color::Rgb(Rgb::gamma_rgb8(red, green, blue, gamma))
    }
    #[doc = r"Linear RGB from 8 bit gamma corrected RGB with transparency."]
    pub fn gamma_rgba8(red: u8, green: u8, blue: u8, alpha: u8, gamma: T)
     -> Color<T> {
        Color::Rgb(Rgb::gamma_rgba8(red, green, blue, alpha, gamma))
    }
    #[doc = r"Linear RGB from a gamma corrected pixel value."]
    pub fn gamma_pixel<P: RgbPixel<T>>(pixel: &P, gamma: T) -> Color<T> {
        Color::Rgb(Rgb::gamma_pixel(pixel, gamma))
    }
}
impl <T: Float> Color<T> {
    #[doc = r"CIE XYZ."]
    pub fn xyz(x: T, y: T, z: T) -> Color<T> { Color::Xyz(Xyz::xyz(x, y, z)) }
    #[doc = r"CIE XYZ and transparency."]
    pub fn xyza(x: T, y: T, z: T, alpha: T) -> Color<T> {
        Color::Xyz(Xyz::xyza(x, y, z, alpha))
    }
}
impl <T: Float> Color<T> {
    #[doc = r"CIE L*a*b*."]
    pub fn lab(l: T, a: T, b: T) -> Color<T> { Color::Lab(Lab::lab(l, a, b)) }
    #[doc = r"CIE L*a*b* and transparency."]
    pub fn laba(l: T, a: T, b: T, alpha: T) -> Color<T> {
        Color::Lab(Lab::laba(l, a, b, alpha))
    }
}
impl <T: Float> Color<T> {
    #[doc = r"CIE L*C*h°."]
    pub fn lch(l: T, chroma: T, hue: LabHue<T>) -> Color<T> {
        Color::Lch(Lch::lch(l, chroma, hue))
    }
    #[doc = r"CIE L*C*h° and transparency."]
    pub fn lcha(l: T, chroma: T, hue: LabHue<T>, alpha: T) -> Color<T> {
        Color::Lch(Lch::lcha(l, chroma, hue, alpha))
    }
}
impl <T: Float> Color<T> {
    #[doc = r"Linear HSV."]
    pub fn hsv(hue: RgbHue<T>, saturation: T, value: T) -> Color<T> {
        Color::Hsv(Hsv::hsv(hue, saturation, value))
    }
    #[doc = r"Linear HSV and transparency."]
    pub fn hsva(hue: RgbHue<T>, saturation: T, value: T, alpha: T)
     -> Color<T> {
        Color::Hsv(Hsv::hsva(hue, saturation, value, alpha))
    }
}
impl<T: Float> Color<T> {
    #[doc = r"Linear HSL."]
    pub fn hsl(hue: RgbHue<T>, saturation: T, lightness: T) -> Color<T> {
        Color::Hsl(Hsl::hsl(hue, saturation, lightness))
    }
    #[doc =
          r"Linear HSL and transparency."]
    pub fn hsla(hue: RgbHue<T>, saturation: T, lightness: T, alpha: T)
     -> Color<T> {
        Color::Hsl(Hsl::hsla(hue, saturation, lightness, alpha))
    }
}
impl<T: Float> Mix<T> for Color<T> {
    fn mix(&self, other: &Color<T>, factor: T) -> Color<T> {
        Rgb::from(*self).mix(&Rgb::from(*other), factor).into()
    }
}
impl<T: Float> Shade<T> for Color<T> {
    fn lighten(&self, amount: T) -> Color<T> {
        Lab::from(*self).lighten(amount).into()
    }
}
impl <T: Float> GetHue for Color<T> {
    type
    Hue
    =
    LabHue<T>;
    fn get_hue(&self) -> Option<LabHue<T>> { Lch::from(*self).get_hue() }
}
impl <T: Float> Hue for Color<T> {
    fn with_hue(&self, hue: LabHue<T>) -> Color<T> {
        Lch::from(*self).with_hue(hue).into()
    }
    fn shift_hue(&self, amount: LabHue<T>) -> Color<T> {
        Lch::from(*self).shift_hue(amount).into()
    }
}
impl<T: Float> Saturate<T> for Color<T> {
    fn saturate(&self, factor: T) -> Color<T> {
        Lch::from(*self).saturate(factor).into()
    }
}
impl <T: Float> From<Luma<T>> for Color<T> {
    fn from(color: Luma<T>) -> Color<T> { Color::Luma(color) }
}
impl <T: Float> From<Rgb<T>> for Color<T> {
    fn from(color: Rgb<T>) -> Color<T> { Color::Rgb(color) }
}
impl <T: Float> From<Xyz<T>> for Color<T> {
    fn from(color: Xyz<T>) -> Color<T> { Color::Xyz(color) }
}
impl <T: Float> From<Lab<T>> for Color<T> {
    fn from(color: Lab<T>) -> Color<T> { Color::Lab(color) }
}
impl <T: Float> From<Lch<T>> for Color<T> {
    fn from(color: Lch<T>) -> Color<T> { Color::Lch(color) }
}
impl <T: Float> From<Hsv<T>> for Color<T> {
    fn from(color: Hsv<T>) -> Color<T> { Color::Hsv(color) }
}
impl <T: Float> From<Hsl<T>> for Color<T> {
    fn from(color: Hsl<T>) -> Color<T> { Color::Hsl(color) }
}


fn clamp<T:Float>(v: T, min: T, max: T) -> T {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

///Common functionality for color spaces.
pub trait ColorSpace {
    ///Check if the color's components are within the expected ranges.
    fn is_valid(&self) -> bool;

    ///Return a new color where the components has been clamped to the nearest
    ///valid values.
    fn clamp(&self) -> Self;

    ///Clamp the color's components to the nearest valid values.
    fn clamp_self(&mut self);
}

///A trait for linear color interpolation.
///
///```
///use palette::{Rgb, Mix};
///
///let a = Rgb::linear_rgb(0.0, 0.5, 1.0);
///let b = Rgb::linear_rgb(1.0, 0.5, 0.0);
///
///assert_eq!(a.mix(&b, 0.0), a);
///assert_eq!(a.mix(&b, 0.5), Rgb::linear_rgb(0.5, 0.5, 0.5));
///assert_eq!(a.mix(&b, 1.0), b);
///```
pub trait Mix<T:Float> {
    ///Mix the color with an other color, by `factor`.
    ///
    ///`factor` sould be between `0.0` and `1.0`, where `0.0` will result in
    ///the same color as `self` and `1.0` will result in the same color as
    ///`other`.
    fn mix(&self, other: &Self, factor: T) -> Self;
}

///The `Shade` trait allows a color to be lightened or darkened.
///
///```
///use palette::{Rgb, Shade};
///
///let a = Rgb::linear_rgb(0.4, 0.4, 0.4);
///let b = Rgb::linear_rgb(0.6, 0.6, 0.6);
///
///assert_eq!(a.lighten(0.1), b.darken(0.1));
///```
pub trait Shade<T:Float>: Sized {
    ///Lighten the color by `amount`.
    fn lighten(&self, amount: T) -> Self;

    ///Darken the color by `amount`.
    fn darken(&self, amount: T) -> Self {
        self.lighten(-amount)
    }
}

///A trait for colors where a hue may be calculated.
///
///```
///use palette::{Rgb, GetHue};
///
///let red = Rgb::linear_rgb(1.0, 0.0, 0.0);
///let green = Rgb::linear_rgb(0.0, 1.0, 0.0);
///let blue = Rgb::linear_rgb(0.0, 0.0, 1.0);
///let gray = Rgb::linear_rgb(0.5, 0.5, 0.5);
///
///assert_eq!(red.get_hue(), Some(0.0.into()));
///assert_eq!(green.get_hue(), Some(120.0.into()));
///assert_eq!(blue.get_hue(), Some(240.0.into()));
///assert_eq!(gray.get_hue(), None);
///```
pub trait GetHue {
    ///The kind of hue unit this color space uses.
    ///
    ///The hue is most commonly calculated as an angle around a color circle
    ///and may not always be uniform between color spaces. It's therefore not
    ///recommended to take one type of hue and apply it to a color space that
    ///expects an other.
    type Hue;

    ///Calculate a hue if possible.
    ///
    ///Colors in the gray scale has no well defined hue and should preferably
    ///return `None`.
    fn get_hue(&self) -> Option<Self::Hue>;
}

///A trait for colors where the hue can be manipulated without conversion.
pub trait Hue: GetHue {
    ///Return a new copy of `self`, but with a specific hue.
    fn with_hue(&self, hue: Self::Hue) -> Self;

    ///Return a new copy of `self`, but with the hue shifted by `amount`.
    fn shift_hue(&self, amount: Self::Hue) -> Self;
}

///A trait for colors where the saturation (or chroma) can be manipulated
///without conversion.
///
///```
///use palette::{Hsv, Saturate};
///
///let a = Hsv::hsv(0.0.into(), 0.25, 1.0);
///let b = Hsv::hsv(0.0.into(), 1.0, 1.0);
///
///assert_eq!(a.saturate(1.0), b.desaturate(0.5));
///```
pub trait Saturate<T: Float>: Sized {
    ///Increase the saturation by `factor`.
    fn saturate(&self, factor: T) -> Self;

    ///Decrease the saturation by `factor`.
    fn desaturate(&self, factor: T) -> Self {
        self.saturate(-factor)
    }
}
