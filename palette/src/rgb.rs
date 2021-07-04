//! RGB types, spaces and standards.
//!
//! # Linear And Non-linear RGB
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
//! But, even when colors *are* 'linear', there is yet more to explore.
//!
//! The most common way that colors are defined, especially for computer
//! storage, is in terms of so-called *tristimulus values*, meaning that all
//! colors can be represented as a vector of three values.
//! The reason colors can generally be stored as only a three-dimensional
//! vector, and not an *N*-dimensional one, where *N* is some number of possible
//! wavelengths of light, is because our eyes contain only three types of cones.
//! Each of these cones has its own sensitivity curve in response to the
//! wavelengths of visible light, giving us three "dimensions" of sensitivity to color.
//! These cones are often called the L, M, and S (for long, medium, and short)
//! cones, and their sensitivity curves *roughly* position them as most
//! sensitive to "red", "green", and "blue" parts of the spectrum. As such, we
//! can choose only three values to represent any possible color that a human is
//! able to see. An interesting consequence of this is that humans can see two
//! different objects which are emitting *completely different actual light
//! spectra* as the *exact same perceptual color* so long as those wavelengths,
//! when transformed by the sensitivity curves of our cones, end up resulting in
//! the same L, M, and S values sent to our brains.
//!
//! A **color space** (which simply refers to a set of standards by which we map
//! a set of arbitrary values to real-world colors) which uses tristimulus
//! values is often defined in terms of
//!
//!  1. Its **primaries**
//!  2. Its **reference white** or **white point**
//!
//! The **primaries** together represent the total *gamut* (i.e. displayable
//! range of colors) of that color space. The **white point** defines a
//! concrete tristimulus value that corresponds to a real, physical white
//! reflecting object being lit by a known light source and observed by the
//! 'standard observer' (i.e. a standardized model of human color perception).
//!
//! The informal "RGB" color space is such a tristimulus color space, since it
//! is defined by three values, but it is underspecified since we don't know
//! which primaries are being used (i.e. how exactly are the canonical "red",
//! "green", and "blue" defined?), nor its white point. In most cases, when
//! people talk about "RGB" or "Linear RGB" colors, what they are *actually*
//! talking about is the "Linear sRGB" color space, which uses the primaries and
//! white point defined in the sRGB standard, but which *does not* have the
//! (non-linear) sRGB *transfer function* applied.
//!
//! Palette takes these details into account and encodes them as type
//! parameters, with sRGB as the default. The goal is to make it easy to use
//! colors correctly and still allow advanced users a high degree of
//! flexibility.

use crate::encoding::{self, Gamma, Linear, TransferFn};
use crate::white_point::WhitePoint;
use crate::{Component, FloatComponent, FromComponent, Yxy};

pub use self::packed::{channels, Packed, RgbChannels};
pub use self::rgb::{FromHexError, Rgb, Rgba};

mod packed;
mod rgb;

/// Non-linear sRGB.
pub type Srgb<T = f32> = Rgb<encoding::Srgb, T>;
/// Non-linear sRGB with an alpha component.
pub type Srgba<T = f32> = Rgba<encoding::Srgb, T>;

/// Linear sRGB.
#[doc(alias = "linear")]
pub type LinSrgb<T = f32> = Rgb<Linear<encoding::Srgb>, T>;
/// Linear sRGB with an alpha component.
#[doc(alias = "linear")]
pub type LinSrgba<T = f32> = Rgba<Linear<encoding::Srgb>, T>;

/// Gamma 2.2 encoded sRGB.
pub type GammaSrgb<T = f32> = Rgb<Gamma<encoding::Srgb>, T>;
/// Gamma 2.2 encoded sRGB with an alpha component.
pub type GammaSrgba<T = f32> = Rgba<Gamma<encoding::Srgb>, T>;

/// An RGB space and a transfer function.
pub trait RgbStandard: 'static {
    /// The RGB color space.
    type Space: RgbSpace;

    /// The transfer function for the color components.
    type TransferFn: TransferFn;
}

impl<S: RgbSpace, T: TransferFn> RgbStandard for (S, T) {
    type Space = S;
    type TransferFn = T;
}

impl<P: Primaries, W: WhitePoint, T: TransferFn> RgbStandard for (P, W, T) {
    type Space = (P, W);
    type TransferFn = T;
}

/// A set of primaries and a white point.
pub trait RgbSpace: 'static {
    /// The primaries of the RGB color space.
    type Primaries: Primaries;

    /// The white point of the RGB color space.
    type WhitePoint: WhitePoint;
}

impl<P: Primaries, W: WhitePoint> RgbSpace for (P, W) {
    type Primaries = P;
    type WhitePoint = W;
}

/// Represents the red, green and blue primaries of an RGB space.
pub trait Primaries: 'static {
    /// Primary red.
    fn red<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T>;
    /// Primary green.
    fn green<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T>;
    /// Primary blue.
    fn blue<Wp: WhitePoint, T: FloatComponent>() -> Yxy<Wp, T>;
}

impl<T, U> From<LinSrgb<T>> for Srgb<U>
where
    T: FloatComponent,
    U: Component + FromComponent<T>,
{
    fn from(lin_srgb: LinSrgb<T>) -> Self {
        let non_lin = Srgb::<T>::from_linear(lin_srgb);
        non_lin.into_format()
    }
}

impl<T, U> From<Srgb<T>> for LinSrgb<U>
where
    T: FloatComponent,
    U: Component + FromComponent<T>,
{
    fn from(srgb: Srgb<T>) -> Self {
        srgb.into_linear().into_format()
    }
}

impl<T, U> From<LinSrgb<T>> for Srgba<U>
where
    T: FloatComponent,
    U: Component + FromComponent<T>,
{
    fn from(lin_srgb: LinSrgb<T>) -> Self {
        let non_lin = Srgb::<T>::from_linear(lin_srgb);
        let new_fmt = Srgb::<U>::from_format(non_lin);
        new_fmt.into()
    }
}

impl<T, U> From<LinSrgba<T>> for Srgba<U>
where
    T: FloatComponent,
    U: Component + FromComponent<T>,
{
    fn from(lin_srgba: LinSrgba<T>) -> Self {
        let non_lin = Srgba::<T>::from_linear(lin_srgba);
        non_lin.into_format()
    }
}

impl<T, U> From<Srgb<T>> for LinSrgba<U>
where
    T: FloatComponent,
    U: Component + FromComponent<T>,
{
    fn from(srgb: Srgb<T>) -> Self {
        srgb.into_linear().into_format().into()
    }
}

impl<T, U> From<Srgba<T>> for LinSrgba<U>
where
    T: FloatComponent,
    U: Component + FromComponent<T>,
{
    fn from(srgba: Srgba<T>) -> Self {
        srgba.into_linear().into_format()
    }
}
