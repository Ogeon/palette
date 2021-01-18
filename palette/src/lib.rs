//! A library that makes linear color calculations and conversion easy and
//! accessible for anyone. It uses the type system to enforce correctness and
//! to avoid mistakes, such as mixing incompatible color types.
//!
//! # It's Never "Just RGB"
//!
//! Colors in, for example, images, are often "gamma corrected", or converted
//! using some non-linear transfer function into a format like sRGB before being
//! stored or displayed. This is done as a compression method and to prevent banding,
//! and is also a bit of a legacy from the ages of the CRT monitors, where the
//! output from the electron gun was nonlinear. The problem is that these formats
//! are *non-linear color spaces*, which means that many operations that you may want
//! to perform on colors (addition, subtraction, multiplication, linear interpolation,
//! etc.) will work unexpectedly when performed in such a non-linear color space. As
//! such, the compression has to be reverted to restore linearity and make sure that
//! many operations on the colors are accurate.
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
//! But, even when colors *are* 'linear', there is yet more to explore.
//!
//! The most common way that colors are defined, especially for computer
//! storage, is in terms of so-called *tristimulus values*, meaning that
//! all colors are defined as a vector of three values which may represent
//! any color. The reason colors can generally be stored as only a three
//! dimensional vector, and not an *n* dimensional one, where *n* is some
//! number of possible frequencies of light, is because our eyes contain
//! only three types of cones. Each of these cones have different sensitivity
//! curves to different wavelengths of light, giving us three "dimensions"
//! of sensitivity to color. These cones are often called the S, M, and L
//! (for small, medium, and large) cones, and their sensitivity curves
//! *roughly* position them as most sensitive to "red", "green", and "blue"
//! parts of the spectrum. As such, we can choose only three values to
//! represent any possible color that a human is able to see. An interesting
//! consequence of this is that humans can see two different objects which
//! are emitting *completely different actual light spectra* as the *exact
//! same perceptual color* so long as those wavelengths, when transformed
//! by the sensitivity curves of our cones, end up resulting in the same
//! S, M, and L values sent to our brains.
//!
//! A **color space** (which simply refers to a set of standards by which
//! we map a set of arbitrary values to real-world colors) which uses
//! tristimulus values is often defined in terms of
//!
//! 1. Its **primaries**
//! 2. Its **reference white** or **white point**
//!
//! The **primaries** together represent the total *gamut* (i.e. displayable
//! range of colors) of that color space, while the **white point** defines
//! which concrete tristimulus value corresponds to a real, physical white
//! reflecting object being lit by a known light source and observed by the
//! 'standard observer' (i.e. a standardized model of human color perception).
//!
//! The informal "RGB" color space is such a tristimulus color space, since
//! it is defined by three values, but it is underspecified since we don't
//! know which primaries are being used (i.e. how exactly are the canonical
//! "red", "green", and "blue" defined?), nor its white point. In most cases,
//! when people talk about "RGB" or "Linear RGB" colors, what they are
//! *actually* talking about is the "Linear sRGB" color space, which uses the
//! primaries and white point defined in the sRGB standard, but which *does
//! not* have the (non-linear) sRGB *transfer function* applied.
//!
//! This library takes these things into account, and attempts to provide an
//! interface which will let those who don't care so much about the intricacies
//! of color still use colors correctly, while also allowing the advanced user
//! a high degree of flexibility in how they use it.
//!
//! # Transparency
//!
//! There are many cases where pixel transparency is important, but there are
//! also many cases where it becomes a dead weight, if it's always stored
//! together with the color, but not used. Palette has therefore adopted a
//! structure where the transparency component (alpha) is attachable using the
//! [`Alpha`](crate::Alpha) type, instead of having copies of each color
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
//! # Working with Raw Data
//!
//! Oftentimes, pixel data is stored in a raw buffer such as a `[u8; 3]`. The
//! [`Pixel`](crate::encoding::pixel::Pixel) trait allows for easy interoperation between
//! Palette colors and other crates or systems. `from_raw` can be used to
//! convert into a Palette color, `into_format` converts from  `Srgb<u8>` to
//! `Srgb<f32>`, and finally `into_raw` to convert from a Palette color back to
//! a `[u8;3]`.
//!
//! ```rust
//! use approx::assert_relative_eq;
//! use palette::{Srgb, Pixel};
//!
//! let buffer = [255, 0, 255];
//! let raw = Srgb::from_raw(&buffer);
//! assert_eq!(raw, &Srgb::<u8>::new(255u8, 0, 255));
//!
//! let raw_float: Srgb<f32> = raw.into_format();
//! assert_relative_eq!(raw_float, Srgb::new(1.0, 0.0, 1.0));
//!
//! let raw: [u8; 3] = Srgb::into_raw(raw_float.into_format());
//! assert_eq!(raw, buffer);
//! ```

// Keep the standard library when running tests, too
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![doc(html_root_url = "https://docs.rs/palette/0.5.0/palette/")]
#![warn(missing_docs)]

#[cfg(any(feature = "std", test))]
extern crate core;

#[cfg_attr(test, macro_use)]
extern crate approx;

#[macro_use]
extern crate palette_derive;

#[cfg(feature = "phf")]
extern crate phf;

#[cfg(feature = "serializing")]
#[macro_use]
extern crate serde;
#[cfg(all(test, feature = "serializing"))]
extern crate serde_json;

use float::Float;

use luma::Luma;

pub use alpha::{Alpha, WithAlpha};
pub use blend::Blend;
#[cfg(feature = "std")]
pub use gradient::Gradient;

pub use hsl::{Hsl, Hsla};
pub use hsv::{Hsv, Hsva};
pub use hwb::{Hwb, Hwba};
pub use lab::{Lab, Laba};
pub use lch::{Lch, Lcha};
pub use luma::{GammaLuma, GammaLumaa, LinLuma, LinLumaa, SrgbLuma, SrgbLumaa};
pub use rgb::{GammaSrgb, GammaSrgba, LinSrgb, LinSrgba, Packed, RgbChannels, Srgb, Srgba};
pub use xyz::{Xyz, Xyza};
pub use yxy::{Yxy, Yxya};
pub use yuv::{Yuv, Yuva};

pub use color_difference::ColorDifference;
pub use component::*;
pub use convert::{FromColor, IntoColor};
pub use encoding::pixel::Pixel;
pub use hues::{LabHue, RgbHue};
pub use matrix::Mat3;
pub use relative_contrast::{contrast_ratio, RelativeContrast};

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
            use crate::Limited;

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

#[cfg(feature = "random")]
mod random_sampling;

mod alpha;
mod hsl;
mod hsv;
mod hwb;
mod lab;
mod lch;
pub mod luma;
pub mod rgb;
mod xyz;
pub mod yuv;
mod yxy;

mod hues;

pub mod chromatic_adaptation;
mod color_difference;
mod component;
pub mod convert;
pub mod encoding;
mod equality;
mod relative_contrast;
pub mod white_point;

pub mod float;

#[doc(hidden)]
pub mod matrix;

fn clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

/// A trait for clamping and checking if colors are within their ranges.
pub trait Limited {
    /// Check if the color's components are within the expected ranges.
    fn is_valid(&self) -> bool;

    /// Return a new color where the components has been clamped to the nearest
    /// valid values.
    fn clamp(&self) -> Self;

    /// Clamp the color's components to the nearest valid values.
    fn clamp_self(&mut self);
}

/// A trait for linear color interpolation.
///
/// ```
/// use approx::assert_relative_eq;
///
/// use palette::{LinSrgb, Mix};
///
/// let a = LinSrgb::new(0.0, 0.5, 1.0);
/// let b = LinSrgb::new(1.0, 0.5, 0.0);
///
/// assert_relative_eq!(a.mix(&b, 0.0), a);
/// assert_relative_eq!(a.mix(&b, 0.5), LinSrgb::new(0.5, 0.5, 0.5));
/// assert_relative_eq!(a.mix(&b, 1.0), b);
/// ```
pub trait Mix {
    /// The type of the mixing factor.
    type Scalar: Float;

    /// Mix the color with an other color, by `factor`.
    ///
    /// `factor` sould be between `0.0` and `1.0`, where `0.0` will result in
    /// the same color as `self` and `1.0` will result in the same color as
    /// `other`.
    fn mix(&self, other: &Self, factor: Self::Scalar) -> Self;
}

/// The `Shade` trait allows a color to be lightened or darkened.
///
/// ```
/// use approx::assert_relative_eq;
///
/// use palette::{LinSrgb, Shade};
///
/// let a = LinSrgb::new(0.4, 0.4, 0.4);
/// let b = LinSrgb::new(0.6, 0.6, 0.6);
///
/// assert_relative_eq!(a.lighten(0.1), b.darken(0.1));
/// ```
pub trait Shade: Sized {
    /// The type of the lighten/darken amount.
    type Scalar: Float;

    /// Lighten the color by `amount`.
    fn lighten(&self, amount: Self::Scalar) -> Self;

    /// Darken the color by `amount`.
    fn darken(&self, amount: Self::Scalar) -> Self {
        self.lighten(-amount)
    }
}

/// A trait for colors where a hue may be calculated.
///
/// ```
/// use approx::assert_relative_eq;
///
/// use palette::{GetHue, LinSrgb};
///
/// let red = LinSrgb::new(1.0f32, 0.0, 0.0);
/// let green = LinSrgb::new(0.0f32, 1.0, 0.0);
/// let blue = LinSrgb::new(0.0f32, 0.0, 1.0);
/// let gray = LinSrgb::new(0.5f32, 0.5, 0.5);
///
/// assert_relative_eq!(red.get_hue().unwrap(), 0.0.into());
/// assert_relative_eq!(green.get_hue().unwrap(), 120.0.into());
/// assert_relative_eq!(blue.get_hue().unwrap(), 240.0.into());
/// assert_eq!(gray.get_hue(), None);
/// ```
pub trait GetHue {
    /// The kind of hue unit this color space uses.
    ///
    /// The hue is most commonly calculated as an angle around a color circle
    /// and may not always be uniform between color spaces. It's therefore not
    /// recommended to take one type of hue and apply it to a color space that
    /// expects an other.
    type Hue;

    /// Calculate a hue if possible.
    ///
    /// Colors in the gray scale has no well defined hue and should preferably
    /// return `None`.
    fn get_hue(&self) -> Option<Self::Hue>;
}

/// A trait for colors where the hue can be manipulated without conversion.
pub trait Hue: GetHue {
    /// Return a new copy of `self`, but with a specific hue.
    fn with_hue<H: Into<Self::Hue>>(&self, hue: H) -> Self;

    /// Return a new copy of `self`, but with the hue shifted by `amount`.
    fn shift_hue<H: Into<Self::Hue>>(&self, amount: H) -> Self;
}

/// A trait for colors where the saturation (or chroma) can be manipulated
/// without conversion.
///
/// ```
/// use approx::assert_relative_eq;
///
/// use palette::{Hsv, Saturate};
///
/// let a = Hsv::new(0.0, 0.25, 1.0);
/// let b = Hsv::new(0.0, 1.0, 1.0);
///
/// assert_relative_eq!(a.saturate(1.0), b.desaturate(0.5));
/// ```
pub trait Saturate: Sized {
    /// The type of the (de)saturation factor.
    type Scalar: Float;

    /// Increase the saturation by `factor`.
    fn saturate(&self, factor: Self::Scalar) -> Self;

    /// Decrease the saturation by `factor`.
    fn desaturate(&self, factor: Self::Scalar) -> Self {
        self.saturate(-factor)
    }
}

/// Perform a unary or binary operation on each component of a color.
pub trait ComponentWise {
    /// The scalar type for color components.
    type Scalar;

    /// Perform a binary operation on this and an other color.
    fn component_wise<F: FnMut(Self::Scalar, Self::Scalar) -> Self::Scalar>(
        &self,
        other: &Self,
        f: F,
    ) -> Self;

    /// Perform a unary operation on this color.
    fn component_wise_self<F: FnMut(Self::Scalar) -> Self::Scalar>(&self, f: F) -> Self;
}

/// A trait for infallible conversion from `f64`. The conversion may be lossy.
pub trait FromF64 {
    /// Creates a value from an `f64` constant.
    fn from_f64(c: f64) -> Self;
}

impl FromF64 for f32 {
    #[inline]
    fn from_f64(c: f64) -> Self {
        c as f32
    }
}

impl FromF64 for f64 {
    #[inline]
    fn from_f64(c: f64) -> Self {
        c
    }
}

/// A convenience function to convert a constant number to Float Type
#[inline]
fn from_f64<T: FromF64>(c: f64) -> T {
    T::from_f64(c)
}
