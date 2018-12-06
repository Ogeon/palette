//! A library that makes linear color calculations and conversion easy and
//! accessible for anyone. It uses the type system to enforce correctness and
//! to avoid mistakes, such as mixing incompatible color types.
//!
//! # It's Never "Just RGB"
//!
//! Colors in, for example, images are often "gamma corrected" or stored in
//! sRGB format as a compression method and to prevent banding. This is also a
//! bit of a legacy from the ages of the CRT monitors, where the output from
//! the electron gun was nonlinear. The problem is that these formats doesn't
//! represent the actual intensities, and the compression has to be reverted to
//! make sure that any operations on the colors are accurate. This library uses
//! a completely linear work flow, and comes with the tools for transitioning
//! between linear and non-linear RGB.
//!
//! Adding to that, there are more than one kind of non-linear RGB. Ironically
//! enough, this turns RGB into one of the most complex color spaces.
//!
//! For example, this does not work:
//!
//! ```rust
//! // An alias for Rgb<Srgb>, which is what most pictures store.
//! use palette::Srgb;
//!
//! let orangeish = Srgb::new(1.0, 0.6, 0.0);
//! let blueish = Srgb::new(0.0, 0.2, 1.0);
//! // let whateve_it_becomes = orangeish + blueish;
//! ```
//!
//! Instead, they have to be made linear before adding:
//!
//! ```rust
//! // An alias for Rgb<Srgb>, which is what most pictures store.
//! use palette::{Pixel, Srgb};
//!
//! let orangeish = Srgb::new(1.0, 0.6, 0.0).into_linear();
//! let blueish = Srgb::new(0.0, 0.2, 1.0).into_linear();
//! let whateve_it_becomes = orangeish + blueish;
//!
//! // Encode the result back into sRGB and create a byte array
//! let pixel: [u8; 3] = Srgb::from_linear(whateve_it_becomes)
//!     .into_format()
//!     .into_raw();
//! ```
//!
//! # Transparency
//!
//! There are many cases where pixel transparency is important, but there are
//! also many cases where it becomes a dead weight, if it's always stored
//! together with the color, but not used. Palette has therefore adopted a
//! structure where the transparency component (alpha) is attachable using the
//! [`Alpha`](struct.Alpha.html) type, instead of having copies of each color
//! space.
//!
//! This approach comes with the extra benefit of allowing operations to
//! selectively affect the alpha component:
//!
//! ```rust
//! use palette::{LinSrgb, LinSrgba};
//!
//! let mut c1 = LinSrgba::new(1.0, 0.5, 0.5, 0.8);
//! let c2 = LinSrgb::new(0.5, 1.0, 1.0);
//!
//! c1.color = c1.color * c2; //Leave the alpha as it is
//! c1.blue += 0.2; //The color components can easily be accessed
//! c1 = c1 * 0.5; //Scale both the color and the alpha
//! ```
//!
//! # A Basic Workflow
//!
//! The overall workflow can be divided into three steps, where the first and
//! last may be taken care of by other parts of the application:
//!
//! ```text
//! Decoding -> Processing -> Encoding
//! ```
//!
//! ## 1. Decoding
//!
//! Find out what the source format is and convert it to a linear color space.
//! There may be a specification, such as when working with SVG or CSS.
//!
//! When working with RGB or gray scale (luma):
//!
//! * If you are asking your user to enter an RGB value, you are in a gray zone
//! where it depends on the context. It's usually safe to assume sRGB, but
//! sometimes it's already linear.
//!
//! * If you are decoding an image, there may be some meta data that gives you
//! the necessary details. Otherwise it's most commonly sRGB. Usually you
//! will end up with a slice or vector with RGB bytes, which can easily be
//! converted to Palette colors:
//!
//! ```rust
//! # let mut image_buffer: Vec<u8> = vec![];
//! use palette::{Srgb, Pixel};
//!
//! // This works for any (even non-RGB) color type that can have the
//! // buffer element type as component.
//! let color_buffer: &mut [Srgb<u8>] = Pixel::from_raw_slice_mut(&mut image_buffer);
//! ```
//!
//! * If you are getting your colors from the GPU, in a game or other graphical
//! application, or if they are otherwise generated by the application, then
//! chances are that they are already linear. Still, make sure to check that
//! they are not being encoded somewhere.
//!
//! When working with other colors:
//!
//! * For HSL, HSV, HWB: Check if they are based on any other color space than
//! sRGB, such as Adobe or Apple RGB.
//!
//! * For any of the CIE color spaces, check for a specification of white point
//! and light source. These are necessary for converting to RGB and other
//! colors, that depend on perception and "viewing devices". Common defaults
//! are the D65 light source and the sRGB white point. The Palette defaults
//! should take you far.
//!
//! ## 2. Processing
//!
//! When your color has been decoded into some Palette type, it's ready for
//! processing. This includes things like blending, hue shifting, darkening and
//! conversion to other formats. Just make sure that your non-linear RGB is
//! made linear first (`my_srgb.into_linear()`), to make the operations
//! available.
//!
//! Different color spaced have different capabilities, pros and cons. You may
//! have to experiment a bit (or look at the example programs) to find out what
//! gives the desired result.
//!
//! ## 3. Encoding
//!
//! When the desired processing is done, it's time to encode the colors back
//! into some image format. The same rules applies as for the decoding, but the
//! process reversed.
//!

// Keep the standard library when running tests, too
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

#![doc(html_root_url = "https://docs.rs/palette/0.4.1/palette/")]
#![cfg_attr(feature = "strict", deny(missing_docs))]
#![cfg_attr(feature = "strict", deny(warnings))]

#[cfg(any(feature = "std", test))]
extern crate core;

#[cfg_attr(test, macro_use)]
extern crate approx;

#[macro_use]
extern crate palette_derive;

extern crate num_traits;

#[cfg(feature = "phf")]
extern crate phf;

#[cfg(feature = "serializing")]
#[macro_use]
extern crate serde;
#[cfg(all(test, feature = "serializing"))]
extern crate serde_json;

use num_traits::{ToPrimitive, Zero};
use float::Float;

use luma::Luma;

#[doc(hidden)]
pub use palette_derive::*;

pub use alpha::Alpha;
pub use blend::Blend;
#[cfg(feature = "std")]
pub use gradient::Gradient;

pub use hsl::{Hsl, Hsla};
pub use hsv::{Hsv, Hsva};
pub use hwb::{Hwb, Hwba};
pub use lab::{Lab, Laba};
pub use lch::{Lch, Lcha};
pub use luma::{GammaLuma, GammaLumaa, LinLuma, LinLumaa, SrgbLuma, SrgbLumaa};
pub use rgb::{GammaSrgb, GammaSrgba, LinSrgb, LinSrgba, Srgb, Srgba};
pub use xyz::{Xyz, Xyza};
pub use yxy::{Yxy, Yxya};

pub use convert::{ConvertFrom, ConvertInto, OutOfBounds, FromColor, IntoColor};
pub use encoding::pixel::Pixel;
pub use hues::{LabHue, RgbHue};
pub use matrix::Mat3;

//Helper macro for checking ranges and clamping.
#[cfg(test)]
macro_rules! assert_ranges {
    (@make_tuple $first:pat, $next:ident,) => (($first, $next));

    (@make_tuple $first:pat, $next:ident, $($rest:ident,)*) => (
        assert_ranges!(@make_tuple ($first, $next), $($rest,)*)
    );

    (
        $ty:ident < $($ty_params:ty),+ >;
        limited {$($limited:ident: $limited_from:expr => $limited_to:expr),+}
        limited_min {$($limited_min:ident: $limited_min_from:expr => $limited_min_to:expr),*}
        unlimited {$($unlimited:ident: $unlimited_from:expr => $unlimited_to:expr),*}
    ) => (
        {
            use core::iter::repeat;
            use Limited;

            {
                print!("checking below limits ... ");
                $(
                    let from = $limited_from;
                    let to = $limited_to;
                    let diff = to - from;
                    let $limited = (1..11).map(|i| from - (i as f64 / 10.0) * diff);
                )+

                $(
                    let from = $limited_min_from;
                    let to = $limited_min_to;
                    let diff = to - from;
                    let $limited_min = (1..11).map(|i| from - (i as f64 / 10.0) * diff);
                )*

                $(
                    let from = $unlimited_from;
                    let to = $unlimited_to;
                    let diff = to - from;
                    let $unlimited = (1..11).map(|i| from - (i as f64 / 10.0) * diff);
                )*

                for assert_ranges!(@make_tuple (), $($limited,)+ $($limited_min,)* $($unlimited,)* ) in repeat(()) $(.zip($limited))+ $(.zip($limited_min))* $(.zip($unlimited))* {
                    let c: $ty<$($ty_params),+> = $ty {
                        $($limited: $limited.into(),)+
                        $($limited_min: $limited_min.into(),)*
                        $($unlimited: $unlimited.into(),)*
                        ..$ty::default() //This prevents exhaustiveness checking
                    };
                    let clamped = c.clamp();
                    let expected: $ty<$($ty_params),+> = $ty {
                        $($limited: $limited_from.into(),)+
                        $($limited_min: $limited_min_from.into(),)*
                        $($unlimited: $unlimited.into(),)*
                        ..$ty::default() //This prevents exhaustiveness checking
                    };

                    assert!(!c.is_valid());
                    assert_relative_eq!(clamped, expected);
                }

                println!("ok")
            }

            {
                print!("checking within limits ... ");
                $(
                    let from = $limited_from;
                    let to = $limited_to;
                    let diff = to - from;
                    let $limited = (0..11).map(|i| from + (i as f64 / 10.0) * diff);
                )+

                $(
                    let from = $limited_min_from;
                    let to = $limited_min_to;
                    let diff = to - from;
                    let $limited_min = (0..11).map(|i| from + (i as f64 / 10.0) * diff);
                )*

                $(
                    let from = $unlimited_from;
                    let to = $unlimited_to;
                    let diff = to - from;
                    let $unlimited = (0..11).map(|i| from + (i as f64 / 10.0) * diff);
                )*

                for assert_ranges!(@make_tuple (), $($limited,)+ $($limited_min,)* $($unlimited,)* ) in repeat(()) $(.zip($limited))+ $(.zip($limited_min))* $(.zip($unlimited))* {
                    let c: $ty<$($ty_params),+> = $ty {
                        $($limited: $limited.into(),)+
                        $($limited_min: $limited_min.into(),)*
                        $($unlimited: $unlimited.into(),)*
                        ..$ty::default() //This prevents exhaustiveness checking
                    };
                    let clamped = c.clamp();

                    assert!(c.is_valid());
                    assert_relative_eq!(clamped, c);
                }

                println!("ok")
            }

            {
                print!("checking above limits ... ");
                $(
                    let from = $limited_from;
                    let to = $limited_to;
                    let diff = to - from;
                    let $limited = (1..11).map(|i| to + (i as f64 / 10.0) * diff);
                )+

                $(
                    let from = $limited_min_from;
                    let to = $limited_min_to;
                    let diff = to - from;
                    let $limited_min = (1..11).map(|i| to + (i as f64 / 10.0) * diff);
                )*

                $(
                    let from = $unlimited_from;
                    let to = $unlimited_to;
                    let diff = to - from;
                    let $unlimited = (1..11).map(|i| to + (i as f64 / 10.0) * diff);
                )*

                for assert_ranges!(@make_tuple (), $($limited,)+ $($limited_min,)* $($unlimited,)* ) in repeat(()) $(.zip($limited))+ $(.zip($limited_min))* $(.zip($unlimited))* {
                    let c: $ty<$($ty_params),+> = $ty {
                        $($limited: $limited.into(),)+
                        $($limited_min: $limited_min.into(),)*
                        $($unlimited: $unlimited.into(),)*
                        ..$ty::default() //This prevents exhaustiveness checking
                    };
                    let clamped = c.clamp();
                    let expected: $ty<$($ty_params),+> = $ty {
                        $($limited: $limited_to.into(),)+
                        $($limited_min: $limited_min.into(),)*
                        $($unlimited: $unlimited.into(),)*
                        ..$ty::default() //This prevents exhaustiveness checking
                    };

                    assert!(!c.is_valid());
                    assert_relative_eq!(clamped, expected);
                }

                println!("ok")
            }
        }
    );
}


#[macro_use]
mod macros;

pub mod blend;
#[cfg(feature = "std")]
pub mod gradient;

#[cfg(feature = "named")]
pub mod named;

mod alpha;
mod hsl;
mod hsv;
mod hwb;
mod lab;
mod lch;
pub mod luma;
pub mod rgb;
mod xyz;
mod yxy;

mod hues;

pub mod chromatic_adaptation;
mod convert;
pub mod encoding;
mod equality;
mod matrix;
pub mod white_point;

pub mod float;

fn clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

///A trait for clamping and checking if colors are within their ranges.
pub trait Limited {
    ///Check if the color's components are within the expected ranges.
    fn is_valid(&self) -> bool;

    ///Return a new color where the components has been clamped to the nearest
    ///valid values.
    fn clamp(&self) -> Self;

    ///Clamp the color's components to the nearest valid values.
    fn clamp_self(&mut self);
}

/// A trait for linear color interpolation.
///
/// ```
/// use palette::{LinSrgb, Mix};
///
/// let a = LinSrgb::new(0.0, 0.5, 1.0);
/// let b = LinSrgb::new(1.0, 0.5, 0.0);
///
/// assert_eq!(a.mix(&b, 0.0), a);
/// assert_eq!(a.mix(&b, 0.5), LinSrgb::new(0.5, 0.5, 0.5));
/// assert_eq!(a.mix(&b, 1.0), b);
/// ```
pub trait Mix {
    ///The type of the mixing factor.
    type Scalar: Float;

    ///Mix the color with an other color, by `factor`.
    ///
    ///`factor` sould be between `0.0` and `1.0`, where `0.0` will result in
    ///the same color as `self` and `1.0` will result in the same color as
    ///`other`.
    fn mix(&self, other: &Self, factor: Self::Scalar) -> Self;
}

/// The `Shade` trait allows a color to be lightened or darkened.
///
/// ```
/// use palette::{LinSrgb, Shade};
///
/// let a = LinSrgb::new(0.4, 0.4, 0.4);
/// let b = LinSrgb::new(0.6, 0.6, 0.6);
///
/// assert_eq!(a.lighten(0.1), b.darken(0.1));
/// ```
pub trait Shade: Sized {
    ///The type of the lighten/darken amount.
    type Scalar: Float;

    ///Lighten the color by `amount`.
    fn lighten(&self, amount: Self::Scalar) -> Self;

    ///Darken the color by `amount`.
    fn darken(&self, amount: Self::Scalar) -> Self {
        self.lighten(-amount)
    }
}

/// A trait for colors where a hue may be calculated.
///
/// ```
/// use palette::{GetHue, LinSrgb};
///
/// let red = LinSrgb::new(1.0f32, 0.0, 0.0);
/// let green = LinSrgb::new(0.0f32, 1.0, 0.0);
/// let blue = LinSrgb::new(0.0f32, 0.0, 1.0);
/// let gray = LinSrgb::new(0.5f32, 0.5, 0.5);
///
/// assert_eq!(red.get_hue(), Some(0.0.into()));
/// assert_eq!(green.get_hue(), Some(120.0.into()));
/// assert_eq!(blue.get_hue(), Some(240.0.into()));
/// assert_eq!(gray.get_hue(), None);
/// ```
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
    fn with_hue<H: Into<Self::Hue>>(&self, hue: H) -> Self;

    ///Return a new copy of `self`, but with the hue shifted by `amount`.
    fn shift_hue<H: Into<Self::Hue>>(&self, amount: H) -> Self;
}

/// A trait for colors where the saturation (or chroma) can be manipulated
/// without conversion.
///
/// ```
/// use palette::{Hsv, Saturate};
///
/// let a = Hsv::new(0.0, 0.25, 1.0);
/// let b = Hsv::new(0.0, 1.0, 1.0);
///
/// assert_eq!(a.saturate(1.0), b.desaturate(0.5));
/// ```
pub trait Saturate: Sized {
    ///The type of the (de)saturation factor.
    type Scalar: Float;

    ///Increase the saturation by `factor`.
    fn saturate(&self, factor: Self::Scalar) -> Self;

    ///Decrease the saturation by `factor`.
    fn desaturate(&self, factor: Self::Scalar) -> Self {
        self.saturate(-factor)
    }
}

///Perform a unary or binary operation on each component of a color.
pub trait ComponentWise {
    ///The scalar type for color components.
    type Scalar;

    ///Perform a binary operation on this and an other color.
    fn component_wise<F: FnMut(Self::Scalar, Self::Scalar) -> Self::Scalar>(
        &self,
        other: &Self,
        f: F,
    ) -> Self;

    ///Perform a unary operation on this color.
    fn component_wise_self<F: FnMut(Self::Scalar) -> Self::Scalar>(&self, f: F) -> Self;
}

/// Common trait for color components.
pub trait Component: Copy + Zero + PartialOrd {
    /// The highest displayable value this component type can reach. Higher
    /// values are allowed, but they may be lowered to this before
    /// converting to another format.
    fn max_intensity() -> Self;
}

/// Trait for converting between color components; component-specific version
/// of std::convert::From.
pub trait FromComponent<T: Component>: Component {
    fn from_component(T) -> Self;
}

/// Trait for converting between color components; component-specific version
/// of std::convert::Into.
pub trait IntoComponent<T: Component>: Component {
    /// Convert into another color component type, including scaling.
    fn into_component(self) -> T;
}

impl<T, U> IntoComponent<U> for T
where
    T: Component,
    U: FromComponent<T>,
{
    fn into_component(self) -> U {
        U::from_component(self)
    }
}


// Conversions between floats
impl FromComponent<f32> for f64 {
    fn from_component(float: f32) -> f64 {
        float.into()
    }
}

impl FromComponent<f64> for f32 {
    fn from_component(float: f64) -> f32 {
        float as f32
    }
}


// Conversions from float to integer
impl FromComponent<f32> for u8 {
    fn from_component(float: f32) -> u8 {
        let max: f32 = u8::max_value().into();
        let scaled = (float * max).round();
        let clamped = clamp(scaled, 0.0, max);
        clamped as u8
    }
}

impl FromComponent<f32> for u16 {
    fn from_component(float: f32) -> u16 {
        let max: f32 = u16::max_value().into();
        let scaled = (float * max).round();
        let clamped = clamp(scaled, 0.0, max);
        clamped as u16
    }
}

// impl FromComponent<f32> for u32 {
//     fn from_component(float: f32) -> u32 {
//         let max: f64 = u32::max_value().into();
//         let scaled = (f64::from(float) * max).round();
//         let clamped = clamp(scaled, 0.0, max);
//         clamped as u32
//     }
// }

impl FromComponent<f64> for u8 {
    fn from_component(float: f64) -> u8 {
        let max: f64 = u8::max_value().into();
        let scaled = (float * max).round();
        let clamped = clamp(scaled, 0.0, max);
        clamped as u8
    }
}

impl FromComponent<f64> for u16 {
    fn from_component(float: f64) -> u16 {
        let max: f64 = u16::max_value().into();
        let scaled = (float * max).round();
        let clamped = clamp(scaled, 0.0, max);
        clamped as u16
    }
}

impl FromComponent<f64> for u32 {
    fn from_component(float: f64) -> u32 {
        let max: f64 = u32::max_value().into();
        let scaled = (float * max).round();
        let clamped = clamp(scaled, 0.0, max);
        clamped as u32
    }
}


// Conversions from int to float
impl FromComponent<u8> for f32 {
    fn from_component(int: u8) -> f32 {
        f32::from(int) / f32::from(u8::max_value())
    }
}

impl FromComponent<u8> for f64 {
    fn from_component(int: u8) -> f64 {
        f64::from(int) / f64::from(u8::max_value())
    }
}

impl FromComponent<u16> for f32 {
    fn from_component(int: u16) -> f32 {
        f32::from(int) / f32::from(u16::max_value())
    }
}

impl FromComponent<u16> for f64 {
    fn from_component(int: u16) -> f64 {
        f64::from(int) / f64::from(u16::max_value())
    }
}

impl FromComponent<u32> for f32 {
    fn from_component(int: u32) -> f32 {
        f64::from_component(int) as f32
    }
}

impl FromComponent<u32> for f64 {
    fn from_component(int: u32) -> f64 {
        f64::from(int) / f64::from(u32::max_value())
    }
}


// Conversions from smaller int to larger int
//
// Each of the larger max values is evenly divisible by the smaller ones,
// so no floating point operations are needed.
impl FromComponent<u8> for u16 {
    fn from_component(int: u8) -> u16 {
        u16::from(int) * (u16::max_value() / u16::from(u8::max_value()))
    }
}

impl FromComponent<u8> for u32 {
    fn from_component(int: u8) -> u32 {
        u32::from(int) * (u32::max_value() / u32::from(u8::max_value()))
    }
}

impl FromComponent<u16> for u32 {
    fn from_component(int: u16) -> u32 {
        u32::from(int) * (u32::max_value() / u32::from(u16::max_value()))
    }
}

// Conversions from larger int to smaller int
//
// We want to divide by the (integer) quotient of the max values, while
// rounding to nearest intstead of down. To do so, we add half the divisior
// (rounded down) to the dividend, then divide rounding down.
impl FromComponent<u16> for u8 {
    fn from_component(int: u16) -> u8 {
        let divisor = u16::max_value() / u16::from(u8::max_value());
        let dividend = int.saturating_add(divisor / 2);
        (dividend / divisor) as u8
    }
}

impl FromComponent<u32> for u8 {
    fn from_component(int: u32) -> u8 {
        let divisor = u32::max_value() / u32::from(u8::max_value());
        let dividend = int.saturating_add(divisor / 2);
        (dividend / divisor) as u8
    }
}

impl FromComponent<u32> for u16 {
    fn from_component(int: u32) -> u16 {
        let divisor = u32::max_value() / u32::from(u16::max_value());
        let dividend = int.saturating_add(divisor / 2);
        (dividend / divisor) as u16
    }
}

impl Component for f32 {
    fn max_intensity() -> Self {
        1.0
    }
}

impl Component for f64 {
    fn max_intensity() -> Self {
        1.0
    }
}

impl Component for u8 {
    fn max_intensity() -> Self {
        core::u8::MAX
    }
}

impl Component for u16 {
    fn max_intensity() -> Self {
        core::u16::MAX
    }
}

impl Component for u32 {
    fn max_intensity() -> Self {
        core::u32::MAX
    }
}

impl Component for u64 {
    fn max_intensity() -> Self {
        core::u64::MAX
    }
}

/// A convenience function to convert a constant number to Float Type
#[inline]
fn cast<T: num_traits::NumCast, P: ToPrimitive>(prim: P) -> T {
    num_traits::NumCast::from(prim).unwrap()
}

#[cfg(test)]
mod test {
    use FromComponent;

    #[test]
    fn f64_from_f32_1() {
        assert_eq!(f64::from_component(0.0f32), 0.0f64);
    }

    #[test]
    fn f64_from_f32_2() {
        assert_eq!(f64::from_component(1.0f32), 1.0f64);
    }

    #[test]
    fn f32_from_f64_1() {
        assert_eq!(f32::from_component(0.0f64), 0.0f32);
    }

    #[test]
    fn f32_from_f64_2() {
        assert_eq!(f32::from_component(1.0f64), 1.0f32);
    }

    #[test]
    fn u8_from_f32_1() {
        assert_eq!(u8::from_component(254.499f32 / 255.0f32), 254u8);
    }

    #[test]
    fn u8_from_f32_2() {
        assert_eq!(u8::from_component(254.501f32 / 255.0f32), 255u8);
    }

   #[test]
    fn u16_from_f32_1() {
        assert_eq!(u16::from_component(65_534.49f32 / 65_535.0f32), 65_534u16);
    }

    #[test]
    fn u16_from_f32_2() {
        assert_eq!(u16::from_component(65_534.51f32 / 65_535.0f32), 65_535u16);
    }

   #[test]
    fn u8_from_f64_1() {
        assert_eq!(u8::from_component(254.49999f64 / 255.0f64), 254u8);
    }

    #[test]
    fn u8_from_f64_2() {
        assert_eq!(u8::from_component(254.50001f64 / 255.0f64), 255u8);
    }

   #[test]
    fn u16_from_f64_1() {
        assert_eq!(u16::from_component(65_534.49999f64 / 65_535.0f64), 65_534u16);
    }

    #[test]
    fn u16_from_f64_2() {
        assert_eq!(u16::from_component(65_534.50001f64 / 65_535.0f64), 65_535u16);
    }

   #[test]
    fn u32_from_f64_1() {
        assert_eq!(u32::from_component(4_294_967_294.49f64 / 4_294_967_295.0f64), 4_294_967_294u32);
    }

    #[test]
    fn u32_from_f64_2() {
        assert_eq!(u32::from_component(4_294_967_294.51f64 / 4_294_967_295.0f64), 4_294_967_295u32);
    }

    #[test]
    fn f32_from_u8_1() {
        assert_eq!(f32::from_component(0u8), 0.0f32);
    }

    #[test]
    fn f32_from_u8_2() {
        assert_eq!(f32::from_component(255u8), 1.0f32);
    }

    #[test]
    fn f64_from_u8_1() {
        assert_eq!(f64::from_component(0u8), 0.0f64);
    }

    #[test]
    fn f64_from_u8_2() {
        assert_eq!(f64::from_component(255u8), 1.0f64);
    }

    #[test]
    fn f32_from_u16_1() {
        assert_eq!(f32::from_component(0u16), 0.0f32);
    }

    #[test]
    fn f32_from_u16_2() {
        assert_eq!(f32::from_component(65_535u16), 1.0f32);
    }

    #[test]
    fn f64_from_u16_1() {
        assert_eq!(f64::from_component(0u16), 0.0f64);
    }

    #[test]
    fn f64_from_u16_2() {
        assert_eq!(f64::from_component(65_535u16), 1.0f64);
    }

    #[test]
    fn f32_from_u32_1() {
        assert_eq!(f32::from_component(0u32), 0.0f32);
    }

    #[test]
    fn f32_from_u32_2() {
        assert_eq!(f32::from_component(4_294_967_295u32), 1.0f32);
    }

    #[test]
    fn f64_from_u32_1() {
        assert_eq!(f64::from_component(0u32), 0.0f64);
    }

    #[test]
    fn f64_from_u32_2() {
        assert_eq!(f64::from_component(4_294_967_295u32), 1.0f64);
    }

    #[test]
    fn u16_from_u8_1() {
        assert_eq!(u16::from_component(0xFEu8), 0xFEFEu16);
    }

    #[test]
    fn u16_from_u8_2() {
        assert_eq!(u16::from_component(0xFFu8), 0xFFFFu16);
    }

    #[test]
    fn u32_from_u8_1() {
        assert_eq!(u32::from_component(0xFEu8), 0xFEFEFEFEu32);
    }

    #[test]
    fn u32_from_u8_2() {
        assert_eq!(u32::from_component(0xFFu8), 0xFFFFFFFFu32);
    }

    #[test]
    fn u32_from_u16_1() {
        assert_eq!(u32::from_component(0xFFFEu16), 0xFFFEFFFEu32);
    }

    #[test]
    fn u32_from_u16_2() {
        assert_eq!(u32::from_component(0xFFFFu16), 0xFFFFFFFFu32);
    }

    #[test]
    fn u8_from_u16_1() {
        assert_eq!(u8::from_component(0xFF7Eu16), 0xFEu8);
    }

    #[test]
    fn u8_from_u16_2() {
        assert_eq!(u8::from_component(0xFF7Fu16), 0xFFu8);
    }

    #[test]
    fn u8_from_u32_1() {
        assert_eq!(u8::from_component(0xFF7F7F7Eu32), 0xFEu8);
    }

    #[test]
    fn u8_from_u32_2() {
        assert_eq!(u8::from_component(0xFF7F7F7Fu32), 0xFFu8);
    }

    #[test]
    fn u16_from_u32_1() {
        assert_eq!(u16::from_component(0xFFFF7FFEu32), 0xFFFEu16);
    }

    #[test]
    fn u16_from_u32_2() {
        assert_eq!(u16::from_component(0xFFFF7FFFu32), 0xFFFFu16);
    }

}
