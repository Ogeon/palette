//! Types for the CIE CAM16 color appearance model.

use crate::{
    angle::{RealAngle, SignedAngle},
    bool_mask::LazySelect,
    num::{Abs, Arithmetics, One, PartialCmp, Powf, Real, Signum, Sqrt, Trigonometry, Zero},
    white_point::{self},
    Xyz,
};

pub use full::*;
pub use parameters::*;
pub use partial::*;
pub use ucs_jab::*;
pub use ucs_jmh::*;

mod full;
mod math;
mod parameters;
mod partial;
mod ucs_jab;
mod ucs_jmh;

/// Converts a color to CAM16, using a set of parameters.
pub trait IntoCam16<Wp, T> {
    /// Convert `self` into CAM16, with `parameters` that describe the viewing
    /// conditions.
    fn into_cam16(self, parameters: BakedParameters<Wp, T>) -> Cam16<Wp, T>;
}

impl<Wp, T> IntoCam16<Wp, T> for Xyz<Wp, T>
where
    T: Real + Arithmetics + Powf + Sqrt + Abs + Signum + Trigonometry + RealAngle + Clone,
{
    fn into_cam16(self, parameters: BakedParameters<Wp, T>) -> Cam16<Wp, T> {
        math::xyz_to_cam16(self.with_white_point(), parameters.inner).with_white_point()
    }
}

/// Converts CAM16 to a color, using a set of parameters.
pub trait FromCam16<Wp, T>: Sized {
    /// Convert `cam16` into `Self`, with `parameters` that describe the viewing
    /// conditions.
    fn from_cam16(cam16: Cam16<Wp, T>, parameters: BakedParameters<Wp, T>) -> Self {
        Self::from_partial_cam16(DynPartialCam16::from(cam16), parameters)
    }

    /// Convert the partially specified `cam16` into `Self`, with `parameters`
    /// that describe the viewing conditions.
    fn from_partial_cam16<L, C>(
        cam16: PartialCam16<Wp, T, L, C>,
        parameters: BakedParameters<Wp, T>,
    ) -> Self
    where
        L: Cam16Luminance<T>,
        C: Cam16Chromaticity<T>;
}

impl<Wp, T> FromCam16<Wp, T> for Xyz<Wp, T>
where
    T: Real
        + One
        + Zero
        + Sqrt
        + Powf
        + Abs
        + Signum
        + Arithmetics
        + Trigonometry
        + RealAngle
        + SignedAngle
        + PartialCmp
        + Clone,
    T::Mask: LazySelect<Xyz<white_point::Any, T>>,
{
    fn from_partial_cam16<L, C>(
        cam16: PartialCam16<Wp, T, L, C>,
        parameters: BakedParameters<Wp, T>,
    ) -> Self
    where
        L: Cam16Luminance<T>,
        C: Cam16Chromaticity<T>,
    {
        math::cam16_to_xyz(cam16.into_dynamic().with_white_point(), parameters.inner)
            .with_white_point()
    }
}
