//! RGB types, spaces and standards.

use core::any::Any;

use crate::encoding::{self, Gamma, Linear, TransferFn};
use crate::white_point::WhitePoint;
use crate::{Component, FloatComponent, FromComponent, Yxy};

pub use self::packed::{channels, Packed, RgbChannels};
pub use self::rgb::{Rgb, Rgba};

mod packed;
mod rgb;

/// Nonlinear sRGB.
pub type Srgb<T = f32> = Rgb<encoding::Srgb, T>;
/// Nonlinear sRGB with an alpha component.
pub type Srgba<T = f32> = Rgba<encoding::Srgb, T>;

/// Linear sRGB.
pub type LinSrgb<T = f32> = Rgb<Linear<encoding::Srgb>, T>;
/// Linear sRGB with an alpha component.
pub type LinSrgba<T = f32> = Rgba<Linear<encoding::Srgb>, T>;

/// Gamma 2.2 encoded sRGB.
pub type GammaSrgb<T = f32> = Rgb<Gamma<encoding::Srgb>, T>;
/// Gamma 2.2 encoded sRGB with an alpha component.
pub type GammaSrgba<T = f32> = Rgba<Gamma<encoding::Srgb>, T>;

/// An RGB space and a transfer function.
pub trait RgbStandard {
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
pub trait RgbSpace {
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
pub trait Primaries: Any {
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
