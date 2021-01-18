//! YUV types, spaces and standards.
use crate::encoding::{TransferFn};
use crate::rgb::RgbSpace;
use crate::FloatComponent;

mod yuv;

/// A YUV standard for analog signal conversion.
///
/// In precise terms, YUV identifies an analog encoding of color signal while YCbCr is the digital,
/// quantized version of that signal.
pub trait YuvStandard {
    /// Underlying color space of the RGB signal.
    type RgbSpace: RgbSpace;

    /// The transfer function from linear RGB space.
    type TransferFn: TransferFn;

    /// The normalized color difference space.
    type DifferenceFn: DifferenceFn;
}

/// Gives the YUV space values of each primary.
pub trait DifferenceFn {
    /// The weights of the luminance transform.
    ///
    /// The linear transform is assumed to happen after the opto-electric transfer function is
    /// applied to each color value. This is true for all ITU-R standards. Nevertheless, A
    /// different form of encoding exists, called YcCbcCbr or constant luminance, which calculates
    /// the luminance value from the linear RGB values instead to optimize the accuracy of its
    /// result.
    ///
    /// The luminance weights correspond closely to the `Y` components of the `Yxy`.
    /// parameterization of the color space primaries. However, they may add up to a value smaller
    /// than `1` to represent colors appearing brighter than the white point i.e. offer a larger
    /// dynamic range than otherwise possible.
    fn luminance<T: FloatComponent>() -> [T; 3];

    /// Normalize the difference of luminance and blue channel.
    fn normalize_blue<T: FloatComponent>(denorm: T) -> T;

    /// Denormalize the difference of luminance and blue channel.
    fn denormalize_blue<T: FloatComponent>(norm: T) -> T;

    /// Normalize the difference of luminance and red channel.
    fn normalize_red<T: FloatComponent>(denorm: T) -> T;

    /// Denormalize the difference of luminance and red channel.
    fn denormalize_red<T: FloatComponent>(norm: T) -> T;
}

impl<R: RgbSpace, T: TransferFn, D: DifferenceFn> YuvStandard for (R, T, D) {
    type RgbSpace = R;
    type TransferFn = T;
    type DifferenceFn = D;
}
