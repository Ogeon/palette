use core::marker::PhantomData;

use float::Float;

use encoding::Linear;
use luma::{Luma, LumaStandard};
use rgb::{Rgb, RgbSpace};
use yuv::{DifferenceFn, YuvStandard};
use {Component, Pixel};
use {Xyz};

/// Generic YUV.
///
/// YUV is an alternate representation for an RGB color space with a focus on separating luminance
/// from chroma components.
#[derive(Debug, PartialEq, FromColor, Pixel)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette_internal]
#[palette_rgb_space = "S::RgbSpace"]
#[palette_white_point = "<S::RgbSpace as RgbSpace>::WhitePoint"]
#[palette_component = "T"]
#[palette_manual_from(Luma, Rgb = "from_rgb_internal")]
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
    #[palette_unsafe_zero_sized]
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
        T: Component,
        Sp: RgbSpace<WhitePoint = <S::RgbSpace as RgbSpace>::WhitePoint>,
    {
        let xyz = Xyz::<<S::RgbSpace as RgbSpace>::WhitePoint, T>::from(rgb);
        let rgb = Rgb::<(S::RgbSpace, S::TransferFn), T>::from(xyz);
        let weights = S::DifferenceFn::luminance::<T>();
        let luminance = weights[0]*rgb.red + weights[1]*rgb.green + weights[2]*rgb.blue;
        let blue_diff = S::DifferenceFn::denormalize_blue(luminance - rgb.blue);
        let red_diff = S::DifferenceFn::denormalize_red(luminance - rgb.red);

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
    T: Component + Float,
    St: LumaStandard<WhitePoint = <S::RgbSpace as RgbSpace>::WhitePoint>,
{
    fn from(color: Luma<St, T>) -> Self {
        Yuv::new(color.luma, T::zero(), T::zero())
    }
}
