use core::marker::PhantomData;

use crate::alpha::Alpha;
use crate::convert::FromColorUnclamped;
use crate::encoding::Linear;
use crate::float::Float;
use crate::luma::{Luma, LumaStandard};
use crate::rgb::{Rgb, RgbStandard, RgbSpace};
use crate::yuv::{DifferenceFn, YuvStandard};
use crate::{Component, FloatComponent, FromComponent, Pixel, Xyz};

/// Generic YUV.
///
/// YUV is an alternate representation for an RGB color space with a focus on separating luminance
/// from chroma components.
#[derive(Debug, PartialEq, FromColorUnclamped, Pixel)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
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

impl<S: YuvStandard, T: FloatComponent> Yuv<S, T> {
    /// Create a YUV color (in YCbCr order).
    pub fn new(luminance: T, blue_diff: T, red_diff: T) -> Yuv<S, T> {
        Yuv {
            luminance,
            red_diff,
            blue_diff,
            standard: PhantomData,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U>(self) -> Yuv<S, U>
    where
        U: FloatComponent + FromComponent<T>,
    {
        Yuv {
            luminance: U::from_component(self.luminance),
            red_diff: U::from_component(self.red_diff),
            blue_diff: U::from_component(self.blue_diff),
            standard: PhantomData,
        }
    }

    /// Convert from another component type.
    pub fn from_format<U>(color: Yuv<S, U>) -> Self
    where
        T: FromComponent<U>,
        U: FloatComponent,
    {
        color.into_format()
    }

    /// Convert to a `(luminance, red_diff, blue_diff)` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.luminance, self.red_diff, self.blue_diff)
    }

    /// Convert from a `(luminance, red_diff, blue_diff)` tuple.
    pub fn from_components((luminance, red_diff, blue_diff): (T, T, T)) -> Self {
        Self::new(luminance, red_diff, blue_diff)
    }

    fn from_rgb_internal<Sp>(rgb: Rgb<Linear<Sp>, T>) -> Self
    where
        T: FloatComponent,
        Sp: RgbSpace<WhitePoint = <S::RgbSpace as RgbSpace>::WhitePoint>,
    {
        let xyz = Xyz::<<S::RgbSpace as RgbSpace>::WhitePoint, T>::from_color_unclamped(rgb);
        let rgb = Rgb::<(S::RgbSpace, S::TransferFn), T>::from_color_unclamped(xyz);
        let weights = S::DifferenceFn::luminance::<T>();
        let luminance = weights[0]*rgb.red + weights[1]*rgb.green + weights[2]*rgb.blue;
        let blue_diff = S::DifferenceFn::normalize_blue(luminance - rgb.blue);
        let red_diff = S::DifferenceFn::normalize_red(luminance - rgb.red);

        Yuv {
            luminance,
            blue_diff,
            red_diff,
            standard: PhantomData,
        }
    }
}

impl<S: YuvStandard, T: FloatComponent, A: Component> Alpha<Yuv<S, T>, A> {
    /// Nonlinear RGB.
    pub fn new(luminance: T, blue_diff: T, red_diff: T, alpha: A) -> Self {
        Alpha {
            color: Yuv::new(luminance, blue_diff, red_diff),
            alpha,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U, B>(self) -> Alpha<Yuv<S, U>, B>
    where
        U: FloatComponent + FromComponent<T>,
        B: Component + FromComponent<A>,
    {
        Alpha::<Yuv<S, U>, B>::new(
            U::from_component(self.luminance),
            U::from_component(self.blue_diff),
            U::from_component(self.red_diff),
            B::from_component(self.alpha),
        )
    }

    /// Convert from another component type.
    pub fn from_format<U, B>(color: Alpha<Yuv<S, U>, B>) -> Self
    where
        T: FromComponent<U>,
        U: FloatComponent,
        A: FromComponent<B>,
        B: Component,
    {
        color.into_format()
    }

    /// Convert to a `(luminance, blue_diff, red_diff, alpha)` tuple.
    pub fn into_components(self) -> (T, T, T, A) {
        (self.luminance, self.blue_diff, self.red_diff, self.alpha)
    }

    /// Convert from a `(luminance, blue_diff, red_diff, alpha)` tuple.
    pub fn from_components((luminance, blue_diff, red_diff, alpha): (T, T, T, A)) -> Self {
        Self::new(luminance, blue_diff, red_diff, alpha)
    }
}


impl<S, T> Default for Yuv<S, T>
where
    T: FloatComponent,
    S: YuvStandard,
{
    fn default() -> Yuv<S, T> {
        Yuv::new(T::zero(), T::zero(), T::zero())
    }
}

impl<S, St, T> FromColorUnclamped<Luma<St, T>> for Yuv<S, T>
where
    S: YuvStandard,
    T: FloatComponent,
    St: LumaStandard<WhitePoint = <S::RgbSpace as RgbSpace>::WhitePoint>,
{
    fn from_color_unclamped(color: Luma<St, T>) -> Self {
        Yuv::from_rgb_internal::<S::RgbSpace>(Rgb::from_color_unclamped(color))
    }
}

impl<S, St, T> FromColorUnclamped<Rgb<St, T>> for Yuv<S, T>
where
    S: YuvStandard,
    T: FloatComponent,
    St: RgbStandard,
    St::Space: RgbSpace<WhitePoint = <S::RgbSpace as RgbSpace>::WhitePoint>,
{
    fn from_color_unclamped(color: Rgb<St, T>) -> Self {
        let linear: Rgb<Linear<St::Space>, T> = Rgb::from_color_unclamped(color);
        Yuv::from_rgb_internal(linear)
    }
}

#[cfg(test)]
mod test {
    use super::Yuv;
    use crate::encoding::itu::BT601_525;

    raw_pixel_conversion_tests!(Yuv<BT601_525>: luminance, blue_diff, red_diff);
    raw_pixel_conversion_fail_tests!(Yuv<BT601_525>: luminance, blue_diff, red_diff);
}
