//! A library that makes linear color calculations and conversion easy and
//! accessible for anyone. It uses the type system to enforce correctness and
//! to avoid mistakes, such as mixing incompatible color types.
//!
//! # It's Never "Just RGB"
//!
//! Colors in images are often "gamma corrected", or converted using some
//! non-linear transfer function into a format like sRGB before being stored or
//! displayed. This is done as a compression method and to prevent banding; it's
//! also a bit of a legacy from the ages of the CRT monitors, where the output
//! from the electron gun was non-linear. The problem is that these formats are
//! *non-linear color spaces*, which means that many operations that you may
//! want to perform on colors (addition, subtraction, multiplication, linear
//! interpolation, etc.) will work unexpectedly when performed in such a
//! non-linear color space. Thus, the compression has to be reverted to restore
//! linearity and ensure that many operations on the colors behave as expected.
//!
//! For example, this does not work:
//!
//! ```rust,compile_fail
//! // An alias for Rgb<Srgb, T>, which is what most pictures store.
//! use palette::Srgb;
//!
//! let orangeish = Srgb::new(1.0, 0.6, 0.0);
//! let blueish = Srgb::new(0.0, 0.2, 1.0);
//! let whatever_it_becomes = orangeish + blueish; // Does not compile
//! ```
//!
//! Instead, they have to be made linear before adding:
//!
//! ```rust
//! // An alias for Rgb<Srgb, T>, which is what most pictures store.
//! use palette::Srgb;
//!
//! let orangeish = Srgb::new(1.0f32, 0.6, 0.0).into_linear();
//! let blueish = Srgb::new(0.0, 0.2, 1.0).into_linear();
//! let whatever_it_becomes = orangeish + blueish;
//!
//! // Encode the result back into sRGB and create a byte array
//! let pixel: [u8; 3] = Srgb::from_linear(whatever_it_becomes).into();
//! ```
//!
//! See the [rgb] module for a deeper dive into RGB and (non-)linearity.
//!
//! # Color Spaces
//!
//! "RGB" and other tristimulus based spaces like CIE Xyz are probably the most
//! widely known color spaces. These spaces are great when you want to perform
//! physically based math on color (like in a 2D or 3D rendering program) but
//! there are also color spaces that are not defined in terms of tristimulus
//! values.
//!
//! You have probably used a color picker with a rainbow wheel and a brightness
//! slider. That may have been an HSV or an HSL color picker, where the color is
//! encoded as hue, saturation and brightness/lightness. Even though these
//! spaces are defined using 3 values, they *aren't* based on tristimulus
//! values, since those three values don't have a direct relation to human
//! vision (i.e. our L, M, and S cones).
//! Such color spaces are excellent when it comes to humans intuitively
//! selecting color values, and as such are the go-to choice when this
//! interaction is needed. They can then be converted into other color spaces
//! to perform modifications to them.
//!
//! There's also a group of color spaces that are designed to be perceptually
//! uniform, meaning that the perceptual change is equal to the numerical
//! change. An example of this is the CIE L\*a\*b\* color space. These color
//! spaces are excellent when you want to "blend" between colors in a
//! *perceptually pleasing* manner rather than based on *physical light intensity*.
//!
//! Selecting the proper color space can have a big impact on how the resulting
//! image looks (as illustrated by some of the programs in `examples`), and
//! Palette makes the conversion between them as easy as a call to `from_color`
//! or `into_color`.
//!
//! This example takes an sRGB color, converts it to CIE L\*C\*h°, a color space
//! similar to the colloquial HSL/HSV color spaces, shifts its hue by 180° and
//! converts it back to RGB:
//!
//! ```
//! use palette::{FromColor, ShiftHue, IntoColor, Lch, Srgb};
//!
//! let lch_color: Lch = Srgb::new(0.8, 0.2, 0.1).into_color();
//! let new_color = Srgb::from_color(lch_color.shift_hue(180.0));
//! ```
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
//! use palette::{Srgb, cast};
//!
//! // This works for any color type (even non-RGB) that can have the
//! // buffer element type as component.
//! let color_buffer: &mut [Srgb<u8>] = cast::from_component_slice_mut(&mut image_buffer);
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
//! Oftentimes, pixel data is stored in a plain array or slice such as a `[u8;
//! 3]`. The [`cast`] module allows for easy conversion between Palette colors
//! and other crates or systems. `from` can be used to convert a `[u8;3]` into a
//! Palette color, `into_format` converts from  `Srgb<u8>` to `Srgb<f32>`, and
//! finally `into` converts from a Palette color back to a `[u8;3]`.
//!
//! ```rust
//! use approx::assert_relative_eq;
//! use palette::Srgb;
//!
//! let buffer = [255, 0, 255];
//! let srgb = Srgb::from(buffer);
//! assert_eq!(srgb, Srgb::<u8>::new(255u8, 0, 255));
//!
//! let srgb_float: Srgb<f32> = srgb.into_format();
//! assert_relative_eq!(srgb_float, Srgb::new(1.0, 0.0, 1.0));
//!
//! let array: [u8; 3] = srgb_float.into_format().into();
//! assert_eq!(array, buffer);
//! ```

// Keep the standard library when running tests, too
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![doc(html_root_url = "https://docs.rs/palette/0.6.1/")]
#![warn(missing_docs)]

#[cfg(any(feature = "std", test))]
extern crate core;

#[cfg(feature = "approx")]
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

use core::ops::{BitAndAssign, Neg};

use bool_mask::{BoolMask, HasBoolMask};
use luma::Luma;

pub use alpha::{Alpha, WithAlpha};
#[cfg(feature = "std")]
pub use gradient::Gradient;

pub use hsl::{Hsl, Hsla};
pub use hsluv::{Hsluv, Hsluva};
pub use hsv::{Hsv, Hsva};
pub use hwb::{Hwb, Hwba};
pub use lab::{Lab, Laba};
pub use lch::{Lch, Lcha};
pub use lchuv::{Lchuv, Lchuva};
pub use luma::{GammaLuma, GammaLumaa, LinLuma, LinLumaa, SrgbLuma, SrgbLumaa};
pub use luv::{Luv, Luva};
pub use oklab::{Oklab, Oklaba};
pub use oklch::{Oklch, Oklcha};
pub use rgb::{GammaSrgb, GammaSrgba, LinSrgb, LinSrgba, Srgb, Srgba};
pub use xyz::{Xyz, Xyza};
pub use yxy::{Yxy, Yxya};

pub use color_difference::ColorDifference;
pub use convert::{FromColor, FromColorMut, FromColorMutGuard, IntoColor, IntoColorMut};
pub use hues::{LabHue, LuvHue, OklabHue, RgbHue};
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
        clamped {$($clamped:ident: $clamped_from:expr => $clamped_to:expr),+}
        clamped_min {$($clamped_min:ident: $clamped_min_from:expr => $clamped_min_to:expr),*}
        unclamped {$($unclamped:ident: $unclamped_from:expr => $unclamped_to:expr),*}
    ) => (
        {
            use core::iter::repeat;
            use crate::{Clamp, IsWithinBounds};

            {
                print!("checking below clamp bounds... ");
                $(
                    let from = $clamped_from;
                    let to = $clamped_to;
                    let diff = to - from;
                    let $clamped = (1..11).map(|i| from - (i as f64 / 10.0) * diff);
                )+

                $(
                    let from = $clamped_min_from;
                    let to = $clamped_min_to;
                    let diff = to - from;
                    let $clamped_min = (1..11).map(|i| from - (i as f64 / 10.0) * diff);
                )*

                $(
                    let from = $unclamped_from;
                    let to = $unclamped_to;
                    let diff = to - from;
                    let $unclamped = (1..11).map(|i| from - (i as f64 / 10.0) * diff);
                )*

                for assert_ranges!(@make_tuple (), $($clamped,)+ $($clamped_min,)* $($unclamped,)* ) in repeat(()) $(.zip($clamped))+ $(.zip($clamped_min))* $(.zip($unclamped))* {
                    let color: $ty<$($ty_params),+> = $ty {
                        $($clamped: $clamped.into(),)+
                        $($clamped_min: $clamped_min.into(),)*
                        $($unclamped: $unclamped.into(),)*
                        ..$ty::default() //This prevents exhaustiveness checking
                    };

                    let clamped = color.clamp();

                    let expected: $ty<$($ty_params),+> = $ty {
                        $($clamped: $clamped_from.into(),)+
                        $($clamped_min: $clamped_min_from.into(),)*
                        $($unclamped: $unclamped.into(),)*
                        ..$ty::default() //This prevents exhaustiveness checking
                    };

                    assert!(!color.is_within_bounds());
                    assert_relative_eq!(clamped, expected);
                }

                println!("ok")
            }

            {
                print!("checking within clamp bounds... ");
                $(
                    let from = $clamped_from;
                    let to = $clamped_to;
                    let diff = to - from;
                    let $clamped = (0..11).map(|i| from + (i as f64 / 10.0) * diff);
                )+

                $(
                    let from = $clamped_min_from;
                    let to = $clamped_min_to;
                    let diff = to - from;
                    let $clamped_min = (0..11).map(|i| from + (i as f64 / 10.0) * diff);
                )*

                $(
                    let from = $unclamped_from;
                    let to = $unclamped_to;
                    let diff = to - from;
                    let $unclamped = (0..11).map(|i| from + (i as f64 / 10.0) * diff);
                )*

                for assert_ranges!(@make_tuple (), $($clamped,)+ $($clamped_min,)* $($unclamped,)* ) in repeat(()) $(.zip($clamped))+ $(.zip($clamped_min))* $(.zip($unclamped))* {
                    let color: $ty<$($ty_params),+> = $ty {
                        $($clamped: $clamped.into(),)+
                        $($clamped_min: $clamped_min.into(),)*
                        $($unclamped: $unclamped.into(),)*
                        ..$ty::default() //This prevents exhaustiveness checking
                    };

                    let clamped = color.clamp();

                    assert!(color.is_within_bounds());
                    assert_relative_eq!(clamped, color);
                }

                println!("ok")
            }

            {
                print!("checking above clamp bounds... ");
                $(
                    let from = $clamped_from;
                    let to = $clamped_to;
                    let diff = to - from;
                    let $clamped = (1..11).map(|i| to + (i as f64 / 10.0) * diff);
                )+

                $(
                    let from = $clamped_min_from;
                    let to = $clamped_min_to;
                    let diff = to - from;
                    let $clamped_min = (1..11).map(|i| to + (i as f64 / 10.0) * diff);
                )*

                $(
                    let from = $unclamped_from;
                    let to = $unclamped_to;
                    let diff = to - from;
                    let $unclamped = (1..11).map(|i| to + (i as f64 / 10.0) * diff);
                )*

                for assert_ranges!(@make_tuple (), $($clamped,)+ $($clamped_min,)* $($unclamped,)* ) in repeat(()) $(.zip($clamped))+ $(.zip($clamped_min))* $(.zip($unclamped))* {
                    let color: $ty<$($ty_params),+> = $ty {
                        $($clamped: $clamped.into(),)+
                        $($clamped_min: $clamped_min.into(),)*
                        $($unclamped: $unclamped.into(),)*
                        ..$ty::default() //This prevents exhaustiveness checking
                    };

                    let clamped = color.clamp();

                    let expected: $ty<$($ty_params),+> = $ty {
                        $($clamped: $clamped_to.into(),)+
                        $($clamped_min: $clamped_min.into(),)*
                        $($unclamped: $unclamped.into(),)*
                        ..$ty::default() //This prevents exhaustiveness checking
                    };

                    assert!(!color.is_within_bounds());
                    assert_relative_eq!(clamped, expected);
                }

                println!("ok")
            }
        }
    );
}

#[macro_use]
mod macros;

#[cfg(feature = "std")]
pub mod gradient;

#[cfg(feature = "named")]
pub mod named;

#[cfg(feature = "random")]
mod random_sampling;

mod alpha;
pub mod angle;
pub mod blend;
pub mod bool_mask;
pub mod cast;
pub mod chromatic_adaptation;
mod color_difference;
pub mod convert;
pub mod encoding;
mod hsl;
mod hsluv;
mod hsv;
mod hues;
mod hwb;
mod lab;
mod lch;
mod lchuv;
pub mod luma;
mod luv;
mod luv_bounds;
pub mod num;
mod oklab;
mod oklch;
mod relative_contrast;
pub mod rgb;
pub mod stimulus;
pub mod white_point;
mod xyz;
mod yxy;

#[doc(hidden)]
pub mod matrix;

#[inline]
fn clamp<T: num::Clamp>(value: T, min: T, max: T) -> T {
    value.clamp(min, max)
}

#[inline]
fn clamp_assign<T: num::ClampAssign>(value: &mut T, min: T, max: T) {
    value.clamp_assign(min, max);
}

#[inline]
fn clamp_min<T: num::Clamp>(value: T, min: T) -> T {
    value.clamp_min(min)
}

#[inline]
fn clamp_min_assign<T: num::ClampAssign>(value: &mut T, min: T) {
    value.clamp_min_assign(min);
}

/// Checks if color components are within their expected range bounds.
///
/// A color with out-of-bounds components may be clamped with [`Clamp`] or
/// [`ClampAssign`].
///
/// ```
/// use palette::{Srgb, IsWithinBounds};
/// let a = Srgb::new(0.4f32, 0.3, 0.8);
/// let b = Srgb::new(1.2f32, 0.3, 0.8);
/// let c = Srgb::new(-0.6f32, 0.3, 0.8);
///
/// assert!(a.is_within_bounds());
/// assert!(!b.is_within_bounds());
/// assert!(!c.is_within_bounds());
/// ```
///
/// `IsWithinBounds` is also implemented for `[T]`:
///
/// ```
/// use palette::{Srgb, IsWithinBounds};
///
/// let my_vec = vec![Srgb::new(0.4f32, 0.3, 0.8), Srgb::new(0.8, 0.5, 0.1)];
/// let my_array = [Srgb::new(0.4f32, 0.3, 0.8), Srgb::new(1.3, 0.5, -3.0)];
/// let my_slice = &[Srgb::new(0.4f32, 0.3, 0.8), Srgb::new(1.2, 0.3, 0.8)];
///
/// assert!(my_vec.is_within_bounds());
/// assert!(!my_array.is_within_bounds());
/// assert!(!my_slice.is_within_bounds());
/// ```
pub trait IsWithinBounds: HasBoolMask {
    /// Check if the color's components are within the expected range bounds.
    ///
    /// ```
    /// use palette::{Srgb, IsWithinBounds};
    /// assert!(Srgb::new(0.8f32, 0.5, 0.2).is_within_bounds());
    /// assert!(!Srgb::new(1.3f32, 0.5, -3.0).is_within_bounds());
    /// ```
    fn is_within_bounds(&self) -> Self::Mask;
}

impl<T> IsWithinBounds for [T]
where
    T: IsWithinBounds,
    T::Mask: BoolMask + BitAndAssign,
{
    #[inline]
    fn is_within_bounds(&self) -> Self::Mask {
        let mut result = Self::Mask::from_bool(true);

        for item in self {
            result &= item.is_within_bounds();

            if result.is_false() {
                break;
            }
        }

        result
    }
}

/// An operator for restricting a color's components to their expected ranges.
///
/// [`IsWithinBounds`] can be used to check if the components are within their
/// range bounds.
///
/// See also [`ClampAssign`].
///
/// ```
/// use palette::{Srgb, IsWithinBounds, Clamp};
///
/// let unclamped = Srgb::new(1.3f32, 0.5, -3.0);
/// assert!(!unclamped.is_within_bounds());
///
/// let clamped = unclamped.clamp();
/// assert!(clamped.is_within_bounds());
/// assert_eq!(clamped, Srgb::new(1.0, 0.5, 0.0));
/// ```
pub trait Clamp {
    /// Return a new color where out-of-bounds components have been changed to
    /// the nearest valid values.
    ///
    /// ```
    /// use palette::{Srgb, Clamp};
    /// assert_eq!(Srgb::new(1.3, 0.5, -3.0).clamp(), Srgb::new(1.0, 0.5, 0.0));
    /// ```
    #[must_use]
    fn clamp(self) -> Self;
}

/// An assigning operator for restricting a color's components to their expected
/// ranges.
///
/// [`IsWithinBounds`] can be used to check if the components are within their
/// range bounds.
///
/// See also [`Clamp`].
///
/// ```
/// use palette::{Srgb, IsWithinBounds, ClampAssign};
///
/// let mut color = Srgb::new(1.3f32, 0.5, -3.0);
/// assert!(!color.is_within_bounds());
///
/// color.clamp_assign();
/// assert!(color.is_within_bounds());
/// assert_eq!(color, Srgb::new(1.0, 0.5, 0.0));
/// ```
///
/// `ClampAssign` is also implemented for `[T]`:
///
/// ```
/// use palette::{Srgb, ClampAssign};
///
/// let mut my_vec = vec![Srgb::new(0.4, 0.3, 0.8), Srgb::new(1.3, 0.5, -3.0)];
/// let mut my_array = [Srgb::new(0.4, 0.3, 0.8), Srgb::new(1.3, 0.5, -3.0)];
/// let mut my_slice = &mut [Srgb::new(0.4, 0.3, 0.8), Srgb::new(1.2, 0.3, 0.8)];
///
/// my_vec.clamp_assign();
/// my_array.clamp_assign();
/// my_slice.clamp_assign();
/// ```
pub trait ClampAssign {
    /// Changes out-of-bounds components to the nearest valid values.
    ///
    /// ```
    /// use palette::{Srgb, ClampAssign};
    ///
    /// let mut color = Srgb::new(1.3, 0.5, -3.0);
    /// color.clamp_assign();
    /// assert_eq!(color, Srgb::new(1.0, 0.5, 0.0));
    /// ```
    fn clamp_assign(&mut self);
}

impl<T> ClampAssign for [T]
where
    T: ClampAssign,
{
    #[inline]
    fn clamp_assign(&mut self) {
        self.iter_mut().for_each(T::clamp_assign);
    }
}

/// Linear color interpolation of two colors.
///
/// See also [`MixAssign`].
///
/// ```
/// use approx::assert_relative_eq;
/// use palette::{LinSrgb, Mix};
///
/// let a = LinSrgb::new(0.0, 0.5, 1.0);
/// let b = LinSrgb::new(1.0, 0.5, 0.0);
///
/// assert_relative_eq!(a.mix(b, 0.0), a);
/// assert_relative_eq!(a.mix(b, 0.5), LinSrgb::new(0.5, 0.5, 0.5));
/// assert_relative_eq!(a.mix(b, 1.0), b);
/// ```
pub trait Mix {
    /// The type of the mixing factor.
    type Scalar;

    /// Mix the color with an other color, by `factor`.
    ///
    /// `factor` should be between `0.0` and `1.0`, where `0.0` will result in
    /// the same color as `self` and `1.0` will result in the same color as
    /// `other`.
    #[must_use]
    fn mix(self, other: Self, factor: Self::Scalar) -> Self;
}

/// Assigning linear color interpolation of two colors.
///
/// See also [`Mix`].
///
/// ```
/// use approx::assert_relative_eq;
/// use palette::{LinSrgb, MixAssign};
///
/// let mut a = LinSrgb::new(0.0, 0.5, 1.0);
/// let b = LinSrgb::new(1.0, 0.5, 0.0);
///
/// a.mix_assign(b, 0.5);
/// assert_relative_eq!(a, LinSrgb::new(0.5, 0.5, 0.5));
/// ```
pub trait MixAssign {
    /// The type of the mixing factor.
    type Scalar;

    /// Mix the color with an other color, by `factor`.
    ///
    /// `factor` should be between `0.0` and `1.0`, where `0.0` will result in
    /// the same color as `self` and `1.0` will result in the same color as
    /// `other`.
    fn mix_assign(&mut self, other: Self, factor: Self::Scalar);
}

/// Operators for lightening a color.
///
/// The trait's functions are split into two groups of functions: relative and
/// fixed/absolute.
///
/// The relative function, [`lighten`](Lighten::lighten), scales the lightness
/// towards the maximum lightness value. This means that for a color with 50%
/// lightness, if `lighten(0.5)` is applied to it, the color will scale halfway
/// to the maximum value of 100% resulting in a new lightness value of 75%.
///
/// The fixed or absolute function, [`lighten_fixed`](Lighten::lighten_fixed),
/// increase the lightness value by an amount that is independent of the current
/// lightness of the color. So for a color with 50% lightness, if
/// `lighten_fixed(0.5)` is applied to it, the color will have 50% lightness
/// added to its lightness value resulting in a new value of 100%.
///
/// See also [`LightenAssign`], [`Darken`] and [`DarkenAssign`].
pub trait Lighten {
    /// The type of the lighten modifier.
    type Scalar;

    /// Scale the color towards the maximum lightness by `factor`, a value
    /// ranging from `0.0` to `1.0`.
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use palette::{Hsl, Lighten};
    ///
    /// let color = Hsl::new_srgb(0.0, 1.0, 0.5);
    /// assert_relative_eq!(color.lighten(0.5).lightness, 0.75);
    /// ```
    #[must_use]
    fn lighten(self, factor: Self::Scalar) -> Self;

    /// Lighten the color by `amount`, a value ranging from `0.0` to `1.0`.
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use palette::{Hsl, Lighten};
    ///
    /// let color = Hsl::new_srgb(0.0, 1.0, 0.4);
    /// assert_relative_eq!(color.lighten_fixed(0.2).lightness, 0.6);
    /// ```
    #[must_use]
    fn lighten_fixed(self, amount: Self::Scalar) -> Self;
}

/// Assigning operators for lightening a color.
///
/// The trait's functions are split into two groups of functions: relative and
/// fixed/absolute.
///
/// The relative function, [`lighten_assign`](LightenAssign::lighten_assign),
/// scales the lightness towards the maximum lightness value. This means that
/// for a color with 50% lightness, if `lighten_assign(0.5)` is applied to it,
/// the color will scale halfway to the maximum value of 100% resulting in a new
/// lightness value of 75%.
///
/// The fixed or absolute function,
/// [`lighten_fixed_assign`](LightenAssign::lighten_fixed_assign), increase the
/// lightness value by an amount that is independent of the current lightness of
/// the color. So for a color with 50% lightness, if `lighten_fixed_assign(0.5)`
/// is applied to it, the color will have 50% lightness added to its lightness
/// value resulting in a new value of 100%.
///
/// `LightenAssign` is also implemented for `[T]`:
///
/// ```
/// use palette::{Hsl, LightenAssign};
///
/// let mut my_vec = vec![Hsl::new_srgb(104.0, 0.3, 0.8), Hsl::new_srgb(113.0, 0.5, 0.8)];
/// let mut my_array = [Hsl::new_srgb(104.0, 0.3, 0.8), Hsl::new_srgb(113.0, 0.5, 0.8)];
/// let mut my_slice = &mut [Hsl::new_srgb(104.0, 0.3, 0.8), Hsl::new_srgb(112.0, 0.5, 0.8)];
///
/// my_vec.lighten_assign(0.5);
/// my_array.lighten_assign(0.5);
/// my_slice.lighten_assign(0.5);
/// ```
///
/// See also [`Lighten`], [`Darken`] and [`DarkenAssign`].
pub trait LightenAssign {
    /// The type of the lighten modifier.
    type Scalar;

    /// Scale the color towards the maximum lightness by `factor`, a value
    /// ranging from `0.0` to `1.0`.
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use palette::{Hsl, LightenAssign};
    ///
    /// let mut color = Hsl::new_srgb(0.0, 1.0, 0.5);
    /// color.lighten_assign(0.5);
    /// assert_relative_eq!(color.lightness, 0.75);
    /// ```
    fn lighten_assign(&mut self, factor: Self::Scalar);

    /// Lighten the color by `amount`, a value ranging from `0.0` to `1.0`.
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use palette::{Hsl, LightenAssign};
    ///
    /// let mut color = Hsl::new_srgb(0.0, 1.0, 0.4);
    /// color.lighten_fixed_assign(0.2);
    /// assert_relative_eq!(color.lightness, 0.6);
    /// ```
    fn lighten_fixed_assign(&mut self, amount: Self::Scalar);
}

impl<T> LightenAssign for [T]
where
    T: LightenAssign,
    T::Scalar: Clone,
{
    type Scalar = T::Scalar;

    #[inline]
    fn lighten_assign(&mut self, factor: Self::Scalar) {
        for color in self {
            color.lighten_assign(factor.clone());
        }
    }

    #[inline]
    fn lighten_fixed_assign(&mut self, amount: Self::Scalar) {
        for color in self {
            color.lighten_fixed_assign(amount.clone());
        }
    }
}

/// Operators for darkening a color;
///
/// The trait's functions are split into two groups of functions: relative and
/// fixed/absolute.
///
/// The relative function, [`darken`](Darken::darken), scales the lightness
/// towards the minimum lightness value. This means that for a color with 50%
/// lightness, if `darken(0.5)` is applied to it, the color will scale halfway
/// to the minimum value of 0% resulting in a new lightness value of 25%.
///
/// The fixed or absolute function, [`darken_fixed`](Darken::darken_fixed),
/// decreases the lightness value by an amount that is independent of the
/// current lightness of the color. So for a color with 50% lightness, if
/// `darken_fixed(0.5)` is applied to it, the color will have 50% lightness
/// removed from its lightness value resulting in a new value of 0%.
///
/// See also [`DarkenAssign`], [`Lighten`] and [`LightenAssign`].
pub trait Darken {
    /// The type of the darken modifier.
    type Scalar;

    /// Scale the color towards the minimum lightness by `factor`, a value
    /// ranging from `0.0` to `1.0`.
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use palette::{Hsv, Darken};
    ///
    /// let color = Hsv::new_srgb(0.0, 1.0, 0.5);
    /// assert_relative_eq!(color.darken(0.5).value, 0.25);
    /// ```
    #[must_use]
    fn darken(self, factor: Self::Scalar) -> Self;

    /// Darken the color by `amount`, a value ranging from `0.0` to `1.0`.
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use palette::{Hsv, Darken};
    ///
    /// let color = Hsv::new_srgb(0.0, 1.0, 0.4);
    /// assert_relative_eq!(color.darken_fixed(0.2).value, 0.2);
    /// ```
    #[must_use]
    fn darken_fixed(self, amount: Self::Scalar) -> Self;
}

impl<T> Darken for T
where
    T: Lighten,
    T::Scalar: Neg<Output = T::Scalar>,
{
    type Scalar = T::Scalar;

    #[inline]
    fn darken(self, factor: Self::Scalar) -> Self {
        self.lighten(-factor)
    }

    #[inline]
    fn darken_fixed(self, amount: Self::Scalar) -> Self {
        self.lighten_fixed(-amount)
    }
}

/// Assigning operators for darkening a color;
///
/// The trait's functions are split into two groups of functions: relative and
/// fixed/absolute.
///
/// The relative function, [`darken_assign`](DarkenAssign::darken_assign),
/// scales the lightness towards the minimum lightness value. This means that
/// for a color with 50% lightness, if `darken_assign(0.5)` is applied to it,
/// the color will scale halfway to the minimum value of 0% resulting in a new
/// lightness value of 25%.
///
/// The fixed or absolute function,
/// [`darken_fixed_assign`](DarkenAssign::darken_fixed_assign), decreases the
/// lightness value by an amount that is independent of the current lightness of
/// the color. So for a color with 50% lightness, if `darken_fixed_assign(0.5)`
/// is applied to it, the color will have 50% lightness removed from its
/// lightness value resulting in a new value of 0%.
///
/// `DarkenAssign` is also implemented for `[T]`:
///
/// ```
/// use palette::{Hsl, DarkenAssign};
///
/// let mut my_vec = vec![Hsl::new_srgb(104.0, 0.3, 0.8), Hsl::new_srgb(113.0, 0.5, 0.8)];
/// let mut my_array = [Hsl::new_srgb(104.0, 0.3, 0.8), Hsl::new_srgb(113.0, 0.5, 0.8)];
/// let mut my_slice = &mut [Hsl::new_srgb(104.0, 0.3, 0.8), Hsl::new_srgb(112.0, 0.5, 0.8)];
///
/// my_vec.darken_assign(0.5);
/// my_array.darken_assign(0.5);
/// my_slice.darken_assign(0.5);
/// ```
///
/// See also [`Darken`], [`Lighten`] and [`LightenAssign`].
pub trait DarkenAssign {
    /// The type of the darken modifier.
    type Scalar;

    /// Scale the color towards the minimum lightness by `factor`, a value
    /// ranging from `0.0` to `1.0`.
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use palette::{Hsv, DarkenAssign};
    ///
    /// let mut color = Hsv::new_srgb(0.0, 1.0, 0.5);
    /// color.darken_assign(0.5);
    /// assert_relative_eq!(color.value, 0.25);
    /// ```
    fn darken_assign(&mut self, factor: Self::Scalar);

    /// Darken the color by `amount`, a value ranging from `0.0` to `1.0`.
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use palette::{Hsv, DarkenAssign};
    ///
    /// let mut color = Hsv::new_srgb(0.0, 1.0, 0.4);
    /// color.darken_fixed_assign(0.2);
    /// assert_relative_eq!(color.value, 0.2);
    /// ```
    fn darken_fixed_assign(&mut self, amount: Self::Scalar);
}

impl<T> DarkenAssign for T
where
    T: LightenAssign + ?Sized,
    T::Scalar: Neg<Output = T::Scalar>,
{
    type Scalar = T::Scalar;

    #[inline]
    fn darken_assign(&mut self, factor: Self::Scalar) {
        self.lighten_assign(-factor);
    }

    #[inline]
    fn darken_fixed_assign(&mut self, amount: Self::Scalar) {
        self.lighten_fixed_assign(-amount);
    }
}

/// A trait for colors where a hue may be calculated.
///
/// See also [`WithHue`], [`SetHue`], [`ShiftHue`] and [`ShiftHueAssign`].
///
/// ```
/// use approx::assert_relative_eq;
/// use palette::{GetHue, LinSrgb};
///
/// let red = LinSrgb::new(1.0f32, 0.0, 0.0);
/// let green = LinSrgb::new(0.0f32, 1.0, 0.0);
/// let blue = LinSrgb::new(0.0f32, 0.0, 1.0);
/// let gray = LinSrgb::new(0.5f32, 0.5, 0.5);
///
/// assert_relative_eq!(red.get_hue(), 0.0.into());
/// assert_relative_eq!(green.get_hue(), 120.0.into());
/// assert_relative_eq!(blue.get_hue(), 240.0.into());
/// assert_relative_eq!(gray.get_hue(), 0.0.into());
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
    /// return `0`.
    #[must_use]
    fn get_hue(&self) -> Self::Hue;
}

/// Change the hue of a color to a specific value.
///
/// See also [`SetHue`], [`GetHue`], [`ShiftHue`] and [`ShiftHueAssign`].
///
/// ```
/// use palette::{Hsl, WithHue};
///
/// let green = Hsl::new_srgb(120.0, 1.0, 0.5);
/// let blue = green.with_hue(240.0);
/// assert_eq!(blue, Hsl::new_srgb(240.0, 1.0, 0.5));
/// ```
pub trait WithHue<H> {
    /// Return a copy of `self` with a specific hue.
    #[must_use]
    fn with_hue(self, hue: H) -> Self;
}

/// Change the hue of a color to a specific value without moving.
///
/// See also [`WithHue`], [`GetHue`], [`ShiftHue`] and [`ShiftHueAssign`].
///
/// ```
/// use palette::{Hsl, SetHue};
///
/// let mut color = Hsl::new_srgb(120.0, 1.0, 0.5);
/// color.set_hue(240.0);
/// assert_eq!(color, Hsl::new_srgb(240.0, 1.0, 0.5));
/// ```
///
/// `SetHue` is also implemented for `[T]`:
///
/// ```
/// use palette::{Hsl, SetHue};
///
/// let mut my_vec = vec![Hsl::new_srgb(104.0, 0.3, 0.8), Hsl::new_srgb(113.0, 0.5, 0.8)];
/// let mut my_array = [Hsl::new_srgb(104.0, 0.3, 0.8), Hsl::new_srgb(113.0, 0.5, 0.8)];
/// let mut my_slice = &mut [Hsl::new_srgb(104.0, 0.3, 0.8), Hsl::new_srgb(112.0, 0.5, 0.8)];
///
/// my_vec.set_hue(120.0);
/// my_array.set_hue(120.0);
/// my_slice.set_hue(120.0);
/// ```
pub trait SetHue<H> {
    /// Change the hue to a specific value.
    fn set_hue(&mut self, hue: H);
}

impl<T, H> SetHue<H> for [T]
where
    T: SetHue<H>,
    H: Clone,
{
    fn set_hue(&mut self, hue: H) {
        for color in self {
            color.set_hue(hue.clone());
        }
    }
}

/// Operator for increasing or decreasing the hue by an amount.
///
/// See also [`ShiftHueAssign`], [`WithHue`], [`SetHue`] and [`GetHue`].
///
/// ```
/// use palette::{Hsl, ShiftHue};
///
/// let green = Hsl::new_srgb(120.0, 1.0, 0.5);
/// let blue = green.shift_hue(120.0);
/// assert_eq!(blue, Hsl::new_srgb(240.0, 1.0, 0.5));
/// ```
pub trait ShiftHue {
    /// The type of the hue modifier.
    type Scalar;

    /// Return a copy of `self` with the hue shifted by `amount`.
    #[must_use]
    fn shift_hue(self, amount: Self::Scalar) -> Self;
}

/// Assigning operator for increasing or decreasing the hue by an amount.
///
/// See also [`ShiftHue`], [`WithHue`], [`SetHue`] and [`GetHue`].
///
/// ```
/// use palette::{Hsl, ShiftHueAssign};
///
/// let mut color = Hsl::new_srgb(120.0, 1.0, 0.5);
/// color.shift_hue_assign(120.0);
/// assert_eq!(color, Hsl::new_srgb(240.0, 1.0, 0.5));
/// ```
///
/// `ShiftHueAssign` is also implemented for `[T]`:
///
/// ```
/// use palette::{Hsl, ShiftHueAssign};
///
/// let mut my_vec = vec![Hsl::new_srgb(104.0, 0.3, 0.8), Hsl::new_srgb(113.0, 0.5, 0.8)];
/// let mut my_array = [Hsl::new_srgb(104.0, 0.3, 0.8), Hsl::new_srgb(113.0, 0.5, 0.8)];
/// let mut my_slice = &mut [Hsl::new_srgb(104.0, 0.3, 0.8), Hsl::new_srgb(112.0, 0.5, 0.8)];
///
/// my_vec.shift_hue_assign(120.0);
/// my_array.shift_hue_assign(120.0);
/// my_slice.shift_hue_assign(120.0);
/// ```
pub trait ShiftHueAssign {
    /// The type of the hue modifier.
    type Scalar;

    /// Shifts the hue by `amount`.
    fn shift_hue_assign(&mut self, amount: Self::Scalar);
}

impl<T> ShiftHueAssign for [T]
where
    T: ShiftHueAssign,
    T::Scalar: Clone,
{
    type Scalar = T::Scalar;

    fn shift_hue_assign(&mut self, amount: Self::Scalar) {
        for color in self {
            color.shift_hue_assign(amount.clone());
        }
    }
}

/// Operator for increasing the saturation (or chroma) of a color.
///
/// The trait's functions are split into two groups of functions: relative and
/// fixed/absolute.
///
/// The relative function, [`saturate`](Saturate::saturate), scales the
/// saturation towards the maximum saturation value. This means that for a color
/// with 50% saturation, if `saturate(0.5)` is applied to it, the color will
/// scale halfway to the maximum value of 100% resulting in a new saturation
/// value of 75%.
///
/// The fixed or absolute function,
/// [`saturate_fixed`](Saturate::saturate_fixed), increases the saturation by an
/// amount that is independent of the current saturation of the color. So for a
/// color with 50% saturation, if `saturate_fixed(0.5)` is applied to it, the
/// color will have 50% saturation added to its saturation value resulting in a
/// new value of 100%.
///
/// See also [`SaturateAssign`], [`Desaturate`] and [`DesaturateAssign`].
///
/// ```
/// use approx::assert_relative_eq;
/// use palette::{Hsv, Saturate};
///
/// let a = Hsv::new_srgb(0.0, 0.5, 1.0);
///
/// assert_relative_eq!(a.saturate(0.5).saturation, 0.75);
/// assert_relative_eq!(a.saturate_fixed(0.5).saturation, 1.0);
/// ```
pub trait Saturate {
    /// The type of the saturation modifier.
    type Scalar;

    /// Scale the color towards the maximum saturation by `factor`, a value
    /// ranging from `0.0` to `1.0`.
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use palette::{Hsl, Saturate};
    ///
    /// let color = Hsl::new_srgb(0.0, 0.5, 0.5);
    /// assert_relative_eq!(color.saturate(0.5).saturation, 0.75);
    /// ```
    #[must_use]
    fn saturate(self, factor: Self::Scalar) -> Self;

    /// Increase the saturation by `amount`, a value ranging from `0.0` to
    /// `1.0`.
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use palette::{Hsl, Saturate};
    ///
    /// let color = Hsl::new_srgb(0.0, 0.4, 0.5);
    /// assert_relative_eq!(color.saturate_fixed(0.2).saturation, 0.6);
    /// ```
    #[must_use]
    fn saturate_fixed(self, amount: Self::Scalar) -> Self;
}

/// Assigning operator for increasing the saturation (or chroma) of a color.
///
/// The trait's functions are split into two groups of functions: relative and
/// fixed/absolute.
///
/// The relative function, [`saturate_assign`](SaturateAssign::saturate_assign),
/// scales the saturation towards the maximum saturation value. This means that
/// for a color with 50% saturation, if `saturate_assign(0.5)` is applied to it,
/// the color will scale halfway to the maximum value of 100% resulting in a new
/// saturation value of 75%.
///
/// The fixed or absolute function,
/// [`saturate_fixed_assign`](SaturateAssign::saturate_fixed_assign), increases
/// the saturation by an amount that is independent of the current saturation of
/// the color. So for a color with 50% saturation, if
/// `saturate_fixed_assign(0.5)` is applied to it, the color will have 50%
/// saturation added to its saturation value resulting in a new value of 100%.
///
/// See also [`Saturate`], [`Desaturate`] and [`DesaturateAssign`].
///
/// ```
/// use approx::assert_relative_eq;
/// use palette::{Hsv, SaturateAssign};
///
/// let mut relative = Hsv::new_srgb(0.0, 0.5, 1.0);
/// relative.saturate_assign(0.5);
///
/// let mut fixed = Hsv::new_srgb(0.0, 0.5, 1.0);
/// fixed.saturate_fixed_assign(0.5);
///
/// assert_relative_eq!(relative.saturation, 0.75);
/// assert_relative_eq!(fixed.saturation, 1.0);
/// ```
///
/// `SaturateAssign` is also implemented for `[T]`:
///
/// ```
/// use palette::{Hsl, SaturateAssign};
///
/// let mut my_vec = vec![Hsl::new_srgb(104.0, 0.3, 0.8), Hsl::new_srgb(113.0, 0.5, 0.8)];
/// let mut my_array = [Hsl::new_srgb(104.0, 0.3, 0.8), Hsl::new_srgb(113.0, 0.5, 0.8)];
/// let mut my_slice = &mut [Hsl::new_srgb(104.0, 0.3, 0.8), Hsl::new_srgb(112.0, 0.5, 0.8)];
///
/// my_vec.saturate_assign(0.5);
/// my_array.saturate_assign(0.5);
/// my_slice.saturate_assign(0.5);
/// ```
pub trait SaturateAssign {
    /// The type of the saturation modifier.
    type Scalar;

    /// Scale the color towards the maximum saturation by `factor`, a value
    /// ranging from `0.0` to `1.0`.
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use palette::{Hsl, SaturateAssign};
    ///
    /// let mut color = Hsl::new_srgb(0.0, 0.5, 0.5);
    /// color.saturate_assign(0.5);
    /// assert_relative_eq!(color.saturation, 0.75);
    /// ```
    fn saturate_assign(&mut self, factor: Self::Scalar);

    /// Increase the saturation by `amount`, a value ranging from `0.0` to
    /// `1.0`.
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use palette::{Hsl, SaturateAssign};
    ///
    /// let mut color = Hsl::new_srgb(0.0, 0.4, 0.5);
    /// color.saturate_fixed_assign(0.2);
    /// assert_relative_eq!(color.saturation, 0.6);
    /// ```
    fn saturate_fixed_assign(&mut self, amount: Self::Scalar);
}

impl<T> SaturateAssign for [T]
where
    T: SaturateAssign,
    T::Scalar: Clone,
{
    type Scalar = T::Scalar;

    fn saturate_assign(&mut self, factor: Self::Scalar) {
        for color in self {
            color.saturate_assign(factor.clone());
        }
    }

    fn saturate_fixed_assign(&mut self, amount: Self::Scalar) {
        for color in self {
            color.saturate_fixed_assign(amount.clone());
        }
    }
}

/// Operator for decreasing the saturation (or chroma) of a color.
///
/// The trait's functions are split into two groups of functions: relative and
/// fixed/absolute.
///
/// The relative function, [`desaturate`](Desaturate::desaturate), scales the
/// saturation towards the minimum saturation value. This means that for a color
/// with 50% saturation, if `desaturate(0.5)` is applied to it, the color will
/// scale halfway to the minimum value of 0% resulting in a new saturation value
/// of 25%.
///
/// The fixed or absolute function,
/// [`desaturate_fixed`](Desaturate::desaturate_fixed), decreases the saturation
/// by an amount that is independent of the current saturation of the color. So
/// for a color with 50% saturation, if `desaturate_fixed(0.5)` is applied to
/// it, the color will have 50% saturation removed from its saturation value
/// resulting in a new value of 0%.
///
/// See also [`DesaturateAssign`], [`Saturate`] and [`SaturateAssign`].
///
/// ```
/// use approx::assert_relative_eq;
/// use palette::{Hsv, Desaturate};
///
/// let a = Hsv::new_srgb(0.0, 0.5, 1.0);
///
/// assert_relative_eq!(a.desaturate(0.5).saturation, 0.25);
/// assert_relative_eq!(a.desaturate_fixed(0.5).saturation, 0.0);
/// ```
pub trait Desaturate {
    /// The type of the desaturation modifier.
    type Scalar;

    /// Scale the color towards the minimum saturation by `factor`, a value
    /// ranging from `0.0` to `1.0`.
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use palette::{Hsv, Desaturate};
    ///
    /// let color = Hsv::new_srgb(0.0, 0.5, 0.5);
    /// assert_relative_eq!(color.desaturate(0.5).saturation, 0.25);
    /// ```
    #[must_use]
    fn desaturate(self, factor: Self::Scalar) -> Self;

    /// Increase the saturation by `amount`, a value ranging from `0.0` to
    /// `1.0`.
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use palette::{Hsv, Desaturate};
    ///
    /// let color = Hsv::new_srgb(0.0, 0.4, 0.5);
    /// assert_relative_eq!(color.desaturate_fixed(0.2).saturation, 0.2);
    /// ```
    #[must_use]
    fn desaturate_fixed(self, amount: Self::Scalar) -> Self;
}

impl<T> Desaturate for T
where
    T: Saturate,
    T::Scalar: Neg<Output = T::Scalar>,
{
    type Scalar = T::Scalar;

    #[inline]
    fn desaturate(self, factor: Self::Scalar) -> Self {
        self.saturate(-factor)
    }

    #[inline]
    fn desaturate_fixed(self, amount: Self::Scalar) -> Self {
        self.saturate_fixed(-amount)
    }
}

/// Assigning operator for decreasing the saturation (or chroma) of a color.
///
/// The trait's functions are split into two groups of functions: relative and
/// fixed/absolute.
///
/// The relative function,
/// [`desaturate_assign`](DesaturateAssign::desaturate_assign), scales the
/// saturation towards the minimum saturation value. This means that for a color
/// with 50% saturation, if `desaturate_assign(0.5)` is applied to it, the color
/// will scale halfway to the minimum value of 0% resulting in a new saturation
/// value of 25%.
///
/// The fixed or absolute function,
/// [`desaturate_fixed_assign`](DesaturateAssign::desaturate_fixed_assign),
/// decreases the saturation by an amount that is independent of the current
/// saturation of the color. So for a color with 50% saturation, if
/// `desaturate_fixed_assign(0.5)` is applied to it, the color will have 50%
/// saturation removed from its saturation value resulting in a new value of 0%.
///
/// See also [`Desaturate`], [`Saturate`] and [`SaturateAssign`].
///
/// ```
/// use approx::assert_relative_eq;
/// use palette::{Hsv, DesaturateAssign};
///
/// let mut relative = Hsv::new_srgb(0.0, 0.5, 1.0);
/// relative.desaturate_assign(0.5);
///
/// let mut fixed = Hsv::new_srgb(0.0, 0.5, 1.0);
/// fixed.desaturate_fixed_assign(0.5);
///
/// assert_relative_eq!(relative.saturation, 0.25);
/// assert_relative_eq!(fixed.saturation, 0.0);
/// ```
///
/// `DesaturateAssign` is also implemented for `[T]`:
///
/// ```
/// use palette::{Hsl, DesaturateAssign};
///
/// let mut my_vec = vec![Hsl::new_srgb(104.0, 0.3, 0.8), Hsl::new_srgb(113.0, 0.5, 0.8)];
/// let mut my_array = [Hsl::new_srgb(104.0, 0.3, 0.8), Hsl::new_srgb(113.0, 0.5, 0.8)];
/// let mut my_slice = &mut [Hsl::new_srgb(104.0, 0.3, 0.8), Hsl::new_srgb(112.0, 0.5, 0.8)];
///
/// my_vec.desaturate_assign(0.5);
/// my_array.desaturate_assign(0.5);
/// my_slice.desaturate_assign(0.5);
/// ```
pub trait DesaturateAssign {
    /// The type of the desaturation modifier.
    type Scalar;

    /// Scale the color towards the minimum saturation by `factor`, a value
    /// ranging from `0.0` to `1.0`.
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use palette::{Hsv, DesaturateAssign};
    ///
    /// let mut color = Hsv::new_srgb(0.0, 0.5, 0.5);
    /// color.desaturate_assign(0.5);
    /// assert_relative_eq!(color.saturation, 0.25);
    /// ```
    fn desaturate_assign(&mut self, factor: Self::Scalar);

    /// Increase the saturation by `amount`, a value ranging from `0.0` to
    /// `1.0`.
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use palette::{Hsv, DesaturateAssign};
    ///
    /// let mut color = Hsv::new_srgb(0.0, 0.4, 0.5);
    /// color.desaturate_fixed_assign(0.2);
    /// assert_relative_eq!(color.saturation, 0.2);
    /// ```
    fn desaturate_fixed_assign(&mut self, amount: Self::Scalar);
}

impl<T> DesaturateAssign for T
where
    T: SaturateAssign + ?Sized,
    T::Scalar: Neg<Output = T::Scalar>,
{
    type Scalar = T::Scalar;

    #[inline]
    fn desaturate_assign(&mut self, factor: Self::Scalar) {
        self.saturate_assign(-factor);
    }

    #[inline]
    fn desaturate_fixed_assign(&mut self, amount: Self::Scalar) {
        self.saturate_fixed_assign(-amount);
    }
}

/// Extension trait for fixed size arrays.
///
/// ## Safety
///
/// * `Item` must be the type of the array's items (eg: `T` in `[T; N]`).
/// * `LENGTH` must be the length of the array (eg: `N` in `[T; N]`).
pub unsafe trait ArrayExt {
    /// The type of the array's items.
    type Item;

    /// The number of items in the array.
    const LENGTH: usize;
}

unsafe impl<T, const N: usize> ArrayExt for [T; N] {
    type Item = T;

    const LENGTH: usize = N;
}

/// Temporary helper trait for getting an array type of size `N + 1`.
///
/// ## Safety
///
/// * `Next` must have the same item type as `Self`.
/// * `Next` must be one item longer than `Self`.
pub unsafe trait NextArray {
    /// An array of size `N + 1`.
    type Next: ArrayExt;
}

macro_rules! impl_next_array {
    ($length: expr) => {};
    ($length: expr, $next_length: expr $(, $rest: expr)*) => {
        unsafe impl<T> NextArray for [T; $length] {
            type Next = [T; $next_length];
        }

        impl_next_array!($next_length $(, $rest)*);
    };
}

impl_next_array!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17);

#[cfg(doctest)]
macro_rules! doctest {
    ($str: expr, $name: ident) => {
        #[doc = $str]
        mod $name {}
    };
}

// Makes doctest run tests on README.md.
#[cfg(doctest)]
doctest!(include_str!("../README.md"), readme);
