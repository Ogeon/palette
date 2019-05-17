use core::marker::PhantomData;

use crate::float::Float;

use crate::encoding::Linear;
use crate::convert::FromColorUnclamped;
use crate::luma::{Luma, LumaStandard};
use crate::rgb::{Rgb, RgbSpace};
use crate::yuv::{YuvStandard, Differences};
use crate::{FloatComponent, Pixel, Xyz};

/// Generic YUV.
///
/// YUV is an alternate representation for an RGB color space with a focus on separating luminance
/// from chroma components.
#[derive(Debug, PartialEq, FromColorUnclamped, Pixel)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    // rgb_standard = "S::RgbSpace",
    white_point = "<S::RgbSpace as RgbSpace>::WhitePoint",
    component = "T",
    skip_derives(Luma, Rgb)
)]
#[repr(C)]
pub struct Yuv<S: YuvStandard, T: Float = f32> {
    /// The lumnance signal where `0.0f` is no light and `1.0f` means a maximum displayable amount
    /// of all colors.
    pub luminance: T,

    /// The difference between overall brightness and the blue signal. This is centered around
    /// `0.0f` but its maximum absolute value depends on the standard.
    pub blue_diff: T,

    /// The difference between overall brightness and the red signal. This is centered around
    /// `0.0f` but its maximum absolute value depends on the standard.
    pub red_diff: T,

    /// The kind of YUV standard.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub standard: PhantomData<S>,
}

impl<S: YuvStandard, T: Float> Copy for Yuv<S, T> {}

impl<S: YuvStandard, T: Float> Clone for Yuv<S, T> {
    fn clone(&self) -> Yuv<S, T> {
        *self
    }
}

impl<S: YuvStandard, T: Float> Yuv<S, T> {
    /// Create an YUV color (in YCbCr order)
    pub fn new(luminance: T, blue_diff: T, red_diff: T) -> Yuv<S, T> {
        Yuv {
            luminance,
            red_diff,
            blue_diff,
            standard: PhantomData,
        }
    }

    fn from_rgb_internal<Sp>(rgb: Rgb<Linear<Sp>, T>) -> Self
    where
        T: FloatComponent,
        Sp: RgbSpace<WhitePoint = <S::RgbSpace as RgbSpace>::WhitePoint>,
    {
        let xyz = Xyz::<<S::RgbSpace as RgbSpace>::WhitePoint, T>::from(rgb);
        let rgb = Rgb::<(S::RgbSpace, S::TransferFn), T>::from(xyz);
        let weights = S::Differences::luminance::<T>();
        let luminance = weights[0]*rgb.red + weights[1]*rgb.green + weights[2]*rgb.blue;
        let blue_diff = S::Differences::norm_blue(luminance - rgb.blue);
        let red_diff = S::Differences::norm_red(luminance - rgb.red);

        Yuv {
            luminance,
            blue_diff,
            red_diff,
            standard: PhantomData,
        }
    }
}


impl<S, T> Default for Yuv<S, T>
where
    T: Float,
    S: YuvStandard,
{
    fn default() -> Yuv<S, T> {
        Yuv::new(T::zero(), T::zero(), T::zero())
    }
}

impl<S, T, St> From<Luma<St, T>> for Yuv<S, T>
where
    S: YuvStandard,
    T: FloatComponent,
    St: LumaStandard<WhitePoint = <S::RgbSpace as RgbSpace>::WhitePoint>,
{
    fn from(color: Luma<St, T>) -> Self {
        Yuv::new(color.luma, T::zero(), T::zero())
    }
}
