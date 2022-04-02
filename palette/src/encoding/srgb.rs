//! The sRGB standard.

use crate::{
    bool_mask::LazySelect,
    encoding::TransferFn,
    luma::LumaStandard,
    num::{Arithmetics, One, PartialCmp, Powf, Real, Recip},
    rgb::{Primaries, RgbSpace, RgbStandard},
    white_point::{Any, WhitePoint, D65},
    Yxy,
};

/// The sRGB color space.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Srgb;

impl<T: Real> Primaries<T> for Srgb {
    fn red() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.6400),
            T::from_f64(0.3300),
            T::from_f64(0.212656),
        )
    }
    fn green() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.3000),
            T::from_f64(0.6000),
            T::from_f64(0.715158),
        )
    }
    fn blue() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.1500),
            T::from_f64(0.0600),
            T::from_f64(0.072186),
        )
    }
}

impl<T> RgbSpace<T> for Srgb
where
    Srgb: Primaries<T>,
    D65: WhitePoint<T>,
{
    type Primaries = Srgb;
    type WhitePoint = D65;
}

impl<T> RgbStandard<T> for Srgb
where
    Srgb: RgbSpace<T> + TransferFn<T>,
{
    type Space = Srgb;
    type TransferFn = Srgb;
}

impl<T> LumaStandard<T> for Srgb
where
    D65: WhitePoint<T>,
    Srgb: TransferFn<T>,
{
    type WhitePoint = D65;
    type TransferFn = Srgb;
}

impl<T> TransferFn<T> for Srgb
where
    T: Real + One + Powf + Recip + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    fn into_linear(x: T) -> T {
        // Recip call shows performance benefits in benchmarks for this function
        lazy_select! {
            if x.lt_eq(&T::from_f64(0.04045)) => T::from_f64(12.92).recip() * &x,
            else => ((T::from_f64(0.055) + &x) * T::from_f64(1.055).recip()).powf(T::from_f64(2.4)),
        }
    }

    fn from_linear(x: T) -> T {
        lazy_select! {
            if x.lt_eq(&T::from_f64(0.0031308)) => T::from_f64(12.92) * &x,
            else => x.clone().powf(T::one() / T::from_f64(2.4)) * T::from_f64(1.055) - T::from_f64(0.055),
        }
    }
}
