use core::marker::PhantomData;

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use float::Float;

use encoding::Linear;
use luma::{Luma, LumaStandard};
use rgb::{Rgb, RgbSpace};
use yuv::{DifferenceFn, YuvStandard};
use {clamp};
use {Component, FromColor, Limited, Pixel};

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
        let rgb = Rgb::<(S::RgbSpace, S::TransferFn), T>::from_rgb(rgb);
        let weights = S::DifferenceFn::luminance::<T>();
        let luminance = weights[0]*rgb.red + weights[1]*rgb.green + weights[2]*rgb.blue;
        let blue_diff = S::DifferenceFn::normalize_blue(rgb.blue - luminance);
        let red_diff = S::DifferenceFn::normalize_red(rgb.red - luminance);

        Yuv {
            luminance,
            blue_diff,
            red_diff,
            standard: PhantomData,
        }
    }
}

impl<S, T> Limited for Yuv<S, T>
where
    S: YuvStandard,
    T: Float,
{
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn is_valid(&self) -> bool {
        let half = T::one()/(T::one() + T::one());
        self.luminance >= T::zero() && self.luminance <= T::one() &&
        self.red_diff >= -half && self.red_diff <= half &&
        self.blue_diff >= -half && self.blue_diff <= half
    }

    fn clamp(&self) -> Yuv<S, T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        let half = T::one()/(T::one() + T::one());
        self.luminance = clamp(self.luminance, T::zero(), T::one());
        self.red_diff = clamp(self.red_diff, -half, half);
        self.blue_diff = clamp(self.blue_diff, -half, half);
    }
}

impl<S, T> Default for Yuv<S, T>
where
    S: YuvStandard,
    T: Float,
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

impl<S, T> AbsDiffEq for Yuv<S, T>
where
    T: Float + AbsDiffEq,
    T::Epsilon: Copy,
    S: YuvStandard + PartialEq,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.luminance.abs_diff_eq(&other.luminance, epsilon) &&
            self.red_diff.abs_diff_eq(&other.red_diff, epsilon) &&
            self.blue_diff.abs_diff_eq(&other.blue_diff, epsilon)
    }
}

impl<S, T> RelativeEq for Yuv<S, T>
where
    T: Float + RelativeEq,
    T::Epsilon: Copy,
    S: YuvStandard + PartialEq,
{
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.luminance.relative_eq(&other.luminance, epsilon, max_relative) &&
            self.red_diff.relative_eq(&other.red_diff, epsilon, max_relative) &&
            self.blue_diff.relative_eq(&other.blue_diff, epsilon, max_relative)
    }
}

impl<S, T> UlpsEq for Yuv<S, T>
where
    T: Float + UlpsEq,
    T::Epsilon: Copy,
    S: YuvStandard + PartialEq,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.luminance.ulps_eq(&other.luminance, epsilon, max_ulps) &&
            self.red_diff.ulps_eq(&other.red_diff, epsilon, max_ulps) &&
            self.blue_diff.ulps_eq(&other.blue_diff, epsilon, max_ulps)
    }
}

#[cfg(test)]
mod tests {
    use super::{Yuv};

    use encoding::itu::{BT601_525, BT601_625, BT709};
    use rgb::Rgb;
    use yuv::DifferenceFn;

    #[test]
    fn ranges() {
        assert_ranges!{
            Yuv<BT709, f64>;
            limited {
                luminance: 0.0 => 1.0,
                red_diff: -0.5 => 0.5,
                blue_diff: -0.5 => 0.5
            }
            limited_min {}
            unlimited {}
        }
    }

    #[test]
    fn bt601_data_sheets() {
        macro_rules! assert_yuv_eq_rgb {
            ($st:ty, ($y:expr, $u:expr, $v:expr), ($r:expr, $g:expr, $b:expr)) => {{
                assert_eq!(
                    Yuv::<$st>::new($y, <$st as DifferenceFn>::normalize_blue($u), <$st as DifferenceFn>::normalize_red($v)),
                    Yuv::<$st>::from(Rgb::<$st>::new($r, $g, $b)));
            }};

            ($st:ty, ($y:expr, $u:expr, $v:expr), ($r:expr, $g:expr, $b:expr), ulps) => {{
                assert_ulps_eq!(
                    Yuv::<$st>::new($y, <$st as DifferenceFn>::normalize_blue($u), <$st as DifferenceFn>::normalize_red($v)),
                    Yuv::<$st>::from(Rgb::<$st>::new($r, $g, $b)));
            }};
        };

        assert_yuv_eq_rgb!(BT601_525, (0.0, 0.0, 0.0), (0.0, 0.0, 0.0));
        assert_yuv_eq_rgb!(BT601_525, (1.0, 0.0, 0.0), (1.0, 1.0, 1.0));
        assert_yuv_eq_rgb!(BT601_625, (0.0, 0.0, 0.0), (0.0, 0.0, 0.0));
        assert_yuv_eq_rgb!(BT601_625, (1.0, 0.0, 0.0), (1.0, 1.0, 1.0));

        // Table from https://www.itu.int/rec/R-REC-BT.601-7-201103-I page 3
        // blue difference and red difference swapped compared to document.
        assert_yuv_eq_rgb!(BT601_525, (0.299, -0.299, 0.701), (1.0, 0.0, 0.0));
        assert_yuv_eq_rgb!(BT601_625, (0.299, -0.299, 0.701), (1.0, 0.0, 0.0));
        assert_yuv_eq_rgb!(BT601_525, (0.587, -0.587, -0.587), (0.0, 1.0, 0.0));
        assert_yuv_eq_rgb!(BT601_625, (0.587, -0.587, -0.587), (0.0, 1.0, 0.0));
        assert_yuv_eq_rgb!(BT601_525, (0.114, 0.886, -0.114), (0.0, 0.0, 1.0));
        assert_yuv_eq_rgb!(BT601_625, (0.114, 0.886, -0.114), (0.0, 0.0, 1.0));

        // These involve floating point computation. Check for ulps.
        assert_yuv_eq_rgb!(BT601_525, (0.886, -0.886, 0.114), (1.0, 1.0, 0.0), ulps);
        assert_yuv_eq_rgb!(BT601_625, (0.886, -0.886, 0.114), (1.0, 1.0, 0.0), ulps);
        assert_yuv_eq_rgb!(BT601_525, (0.701, 0.299, -0.701), (0.0, 1.0, 1.0), ulps);
        assert_yuv_eq_rgb!(BT601_625, (0.701, 0.299, -0.701), (0.0, 1.0, 1.0), ulps);
        assert_yuv_eq_rgb!(BT601_525, (0.413, 0.587, 0.587), (1.0, 0.0, 1.0), ulps);
        assert_yuv_eq_rgb!(BT601_625, (0.413, 0.587, 0.587), (1.0, 0.0, 1.0), ulps);
    }

    #[test]
    fn bt709_baseline() {
        // Otherwise we trust the table tests from the other encodings and the hardcoded constants.
        assert_eq!(
            Yuv::<BT709>::new(0.0, 0.0, 0.0),
            Yuv::<BT709>::from(Rgb::<BT709>::new(0.0, 0.0, 0.0)));

        // Note: not exactly equal as the specification has factors *slightly* different from the
        // exact luminance values of the primaries (derived from weight point and primary
        // coordinates). This means that floating point actually does some calcuation here.
        assert_abs_diff_eq!(
            Yuv::<BT709>::new(1.0, 0.0, 0.0),
            Yuv::<BT709>::from(Rgb::<BT709>::new(1.0, 1.0, 1.0)),
            epsilon = 1.0e-4); // > 12 bit accuracy
    }
}
