//! Defines the tristimulus values of the CIE Illuminants.
//!
//! White point is the reference white or target white as seen by a standard
//! observer under a standard illuminant. For example, photographs taken indoors
//! may be lit by incandescent lights, which are relatively orange compared to
//! daylight. Defining "white" as daylight will give unacceptable results when
//! attempting to color-correct a photograph taken with incandescent lighting.

use crate::{from_f64, FloatComponent, Xyz};

/// WhitePoint defines the Xyz color co-ordinates for a given white point.
///
/// A white point (often referred to as reference white or target white in
/// technical documents) is a set of tristimulus values or chromaticity
/// coordinates that serve to define the color "white" in image capture,
/// encoding, or reproduction.
///
/// Custom white points can be easily defined on an empty struct with the
/// tristimulus values and can be used in place of the ones defined in this
/// library.
pub trait WhitePoint: 'static {
    /// Get the Xyz chromaticity co-ordinates for the white point.
    fn get_xyz<Wp: WhitePoint, T: FloatComponent>() -> Xyz<Wp, T>;
}

/// CIE standard illuminant A
///
/// CIE standard illuminant A is intended to represent typical, domestic,
/// tungsten-filament lighting. Its relative spectral power distribution is that
/// of a Planckian radiator at a temperature of approximately 2856 K. Uses the
/// CIE 1932 2° Standard Observer
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct A;
impl WhitePoint for A {
    fn get_xyz<Wp: WhitePoint, T: FloatComponent>() -> Xyz<Wp, T> {
        Xyz::with_wp(from_f64(1.09850), T::one(), from_f64(0.35585))
    }
}
/// CIE standard illuminant B
///
/// CIE standard illuminant B represents noon sunlight, with a correlated color
/// temperature (CCT) of 4874 K Uses the CIE 1932 2° Standard Observer
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct B;
impl WhitePoint for B {
    fn get_xyz<Wp: WhitePoint, T: FloatComponent>() -> Xyz<Wp, T> {
        Xyz::with_wp(from_f64(0.99072), T::one(), from_f64(0.85223))
    }
}
/// CIE standard illuminant C
///
/// CIE standard illuminant C represents the average day light with a CCT of
/// 6774 K Uses the CIE 1932 2° Standard Observer
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct C;
impl WhitePoint for C {
    fn get_xyz<Wp: WhitePoint, T: FloatComponent>() -> Xyz<Wp, T> {
        Xyz::with_wp(from_f64(0.98074), T::one(), from_f64(1.18232))
    }
}
/// CIE D series standard illuminant - D50
///
/// D50 White Point is the natural daylight with a color temperature of around
/// 5000K for 2° Standard Observer.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct D50;
impl WhitePoint for D50 {
    fn get_xyz<Wp: WhitePoint, T: FloatComponent>() -> Xyz<Wp, T> {
        Xyz::with_wp(from_f64(0.96422), T::one(), from_f64(0.82521))
    }
}
/// CIE D series standard illuminant - D55
///
/// D55 White Point is the natural daylight with a color temperature of around
/// 5500K for 2° Standard Observer.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct D55;
impl WhitePoint for D55 {
    fn get_xyz<Wp: WhitePoint, T: FloatComponent>() -> Xyz<Wp, T> {
        Xyz::with_wp(from_f64(0.95682), T::one(), from_f64(0.92149))
    }
}
/// CIE D series standard illuminant - D65
///
/// D65 White Point is the natural daylight with a color temperature of 6500K
/// for 2° Standard Observer.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct D65;
impl WhitePoint for D65 {
    fn get_xyz<Wp: WhitePoint, T: FloatComponent>() -> Xyz<Wp, T> {
        Xyz::with_wp(from_f64(0.95047), T::one(), from_f64(1.08883))
    }
}
/// CIE D series standard illuminant - D75
///
/// D75 White Point is the natural daylight with a color temperature of around
/// 7500K for 2° Standard Observer.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct D75;
impl WhitePoint for D75 {
    fn get_xyz<Wp: WhitePoint, T: FloatComponent>() -> Xyz<Wp, T> {
        Xyz::with_wp(from_f64(0.94972), T::one(), from_f64(1.22638))
    }
}
/// CIE standard illuminant E
///
/// CIE standard illuminant E represents the equal energy radiator
/// Uses the CIE 1932 2° Standard Observer
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct E;
impl WhitePoint for E {
    fn get_xyz<Wp: WhitePoint, T: FloatComponent>() -> Xyz<Wp, T> {
        Xyz::with_wp(T::one(), T::one(), T::one())
    }
}
/// CIE fluorescent illuminant series - F2
///
/// F2 represents a semi-broadband fluorescent lamp for 2° Standard Observer.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct F2;
impl WhitePoint for F2 {
    fn get_xyz<Wp: WhitePoint, T: FloatComponent>() -> Xyz<Wp, T> {
        Xyz::with_wp(from_f64(0.99186), T::one(), from_f64(0.67393))
    }
}
/// CIE fluorescent illuminant series - F7
///
/// F7 represents a broadband fluorescent lamp for 2° Standard Observer.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct F7;
impl WhitePoint for F7 {
    fn get_xyz<Wp: WhitePoint, T: FloatComponent>() -> Xyz<Wp, T> {
        Xyz::with_wp(from_f64(0.95041), T::one(), from_f64(1.08747))
    }
}
/// CIE fluorescent illuminant series - F11
///
/// F11 represents a narrowband fluorescent lamp for 2° Standard Observer.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct F11;
impl WhitePoint for F11 {
    fn get_xyz<Wp: WhitePoint, T: FloatComponent>() -> Xyz<Wp, T> {
        Xyz::with_wp(from_f64(1.00962), T::one(), from_f64(0.64350))
    }
}
/// CIE D series standard illuminant - D50
///
/// D50 White Point is the natural daylight with a color temperature of around
/// 5000K for 10° Standard Observer.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct D50Degree10;
impl WhitePoint for D50Degree10 {
    fn get_xyz<Wp: WhitePoint, T: FloatComponent>() -> Xyz<Wp, T> {
        Xyz::with_wp(from_f64(0.9672), T::one(), from_f64(0.8143))
    }
}
/// CIE D series standard illuminant - D55
///
/// D55 White Point is the natural daylight with a color temperature of around
/// 5500K for 10° Standard Observer.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct D55Degree10;
impl WhitePoint for D55Degree10 {
    fn get_xyz<Wp: WhitePoint, T: FloatComponent>() -> Xyz<Wp, T> {
        Xyz::with_wp(from_f64(0.958), T::one(), from_f64(0.9093))
    }
}
/// CIE D series standard illuminant - D65
///
/// D65 White Point is the natural daylight with a color temperature of 6500K
/// for 10° Standard Observer.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct D65Degree10;
impl WhitePoint for D65Degree10 {
    fn get_xyz<Wp: WhitePoint, T: FloatComponent>() -> Xyz<Wp, T> {
        Xyz::with_wp(from_f64(0.9481), T::one(), from_f64(1.073))
    }
}
/// CIE D series standard illuminant - D75
///
/// D75 White Point is the natural daylight with a color temperature of around
/// 7500K for 10° Standard Observer.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct D75Degree10;
impl WhitePoint for D75Degree10 {
    fn get_xyz<Wp: WhitePoint, T: FloatComponent>() -> Xyz<Wp, T> {
        Xyz::with_wp(from_f64(0.94416), T::one(), from_f64(1.2064))
    }
}
