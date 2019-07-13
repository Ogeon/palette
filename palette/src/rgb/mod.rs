//!RGB types, spaces and standards.

use alpha::Alpha;
use float::Float;
use core::any::Any;

use {Component, Yxy};
use white_point::WhitePoint;

use encoding::{Linear, TransferFn};

pub use self::rgb::{Rgb, Rgba};

//mod linear;
mod rgb;

///Nonlinear sRGB.
pub type Srgb<T = f32> = Rgb<::encoding::Srgb, T>;
///Nonlinear sRGB with an alpha component.
pub type Srgba<T = f32> = Rgba<::encoding::Srgb, T>;

///Linear sRGB.
pub type LinSrgb<T = f32> = Rgb<Linear<::encoding::Srgb>, T>;
///Linear sRGB with an alpha component.
pub type LinSrgba<T = f32> = Rgba<Linear<::encoding::Srgb>, T>;

/// Gamma 2.2 encoded sRGB.
pub type GammaSrgb<T = f32> = Rgb<::encoding::Gamma<::encoding::Srgb>, T>;
/// Gamma 2.2 encoded sRGB with an alpha component.
pub type GammaSrgba<T = f32> = Rgba<::encoding::Gamma<::encoding::Srgb>, T>;

///An RGB space and a transfer function.
pub trait RgbStandard {
    ///The RGB color space.
    type Space: RgbSpace;

    ///The transfer function for the color components.
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

///A set of primaries and a white point.
pub trait RgbSpace {
    ///The primaries of the RGB color space.
    type Primaries: Primaries;

    ///The white point of the RGB color space.
    type WhitePoint: WhitePoint;
}

impl<P: Primaries, W: WhitePoint> RgbSpace for (P, W) {
    type Primaries = P;
    type WhitePoint = W;
}

///Represents the red, green and blue primaries of an RGB space.
pub trait Primaries: Any {
    ///Primary red.
    fn red<Wp: WhitePoint, T: Component + Float>() -> Yxy<Wp, T>;
    ///Primary green.
    fn green<Wp: WhitePoint, T: Component + Float>() -> Yxy<Wp, T>;
    ///Primary blue.
    fn blue<Wp: WhitePoint, T: Component + Float>() -> Yxy<Wp, T>;
}

impl<T, U> From<LinSrgb<T>> for Srgb<U>
where
    T: Component + Float,
    U: Component,
{
    fn from(lin_srgb: LinSrgb<T>) -> Self {
        let non_lin = Srgb::<T>::from_linear(lin_srgb);
        non_lin.into_format()
    }
}

impl<T, U> From<Srgb<T>> for LinSrgb<U>
where
    T: Component + Float,
    U: Component,
{
    fn from(srgb: Srgb<T>) -> Self {
        srgb.into_linear()
            .into_format()
    }
}

impl<T, U> From<LinSrgb<T>> for Srgba<U>
where
    T: Component + Float,
    U: Component,
{
    fn from(lin_srgb: LinSrgb<T>) -> Self {
        let color = lin_srgb.into();
        let alpha = U::max_intensity();
        Alpha { color, alpha }
    }
}

impl<T, U> From<LinSrgba<T>> for Srgba<U>
where
    T: Component + Float,
    U: Component,
{
    fn from(lin_srgba: LinSrgba<T>) -> Self {
        let (r, g, b, a) = lin_srgba.into();
        let color = LinSrgb::new(r, g, b).into();
        let alpha = a.convert();
        Alpha { color, alpha }
    }
}

impl<T, U> From<Srgb<T>> for LinSrgba<U>
where
    T: Component + Float,
    U: Component,
{
    fn from(srgb: Srgb<T>) -> Self {
        let color = srgb.into();
        let alpha = U::max_intensity();
        Alpha { color, alpha }
    }
}

impl<T, U> From<Srgba<T>> for LinSrgba<U>
where
    T: Component + Float,
    U: Component,
{
    fn from(srgba: Srgba<T>) -> Self {
        let (r, g, b, a) = srgba.into();
        let color = Srgb::new(r, g, b).into();
        let alpha = a.convert();
        Alpha { color, alpha }
    }
}
