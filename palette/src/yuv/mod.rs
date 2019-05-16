//! YUV types, spaces and standards.
use float::Float;

use encoding::{TransferFn};
use rgb::RgbSpace;
use {Component};

mod yuv;

/// A YUV standard with digital quantization function.
pub trait YuvStandard {
    /// Underlying color space of the RGB signal.
    type RgbSpace: RgbSpace;

    /// The transfer function from linear RGB space.
    type TransferFn: TransferFn;

    /// The normalized color difference space.
    type Differences: Differences;
}

/// Gives the YUV space values of each primary.
pub trait Differences {
    /// The weights of the luminance transform.
    ///
    /// The linear transform is assumed to happen after the opto-electric transfer function is
    /// applied to each color value. This is true for all ITU-R standards. Nevertheless, A
    /// different form of encoding exists, called YcCbcCbr or constant luminance, which calculates
    /// the luminance value from the linear RGB values instead to optimize the accuracy of its
    /// result.
    fn luminance<T: Float>() -> [T; 3];

    /// Normalize the difference of luminance and blue channel.
    fn norm_blue<T: Float>(denorm: T) -> T;

    /// Denormalize the difference of luminance and blue channel.
    fn denorm_blue<T: Float>(norm: T) -> T;

    /// Normalize the difference of luminance and red channel.
    fn norm_red<T: Float>(denorm: T) -> T;

    /// Denormalize the difference of luminance and red channel.
    fn denorm_red<T: Float>(norm: T) -> T;
}

/// A digital encoding of a YUV color model.
///
/// While the difference conversion is mostly performed in an analog signal space free of
/// quantization errors, the final digital output is quantized to some number of bits defined in
/// individual standards.
///
/// The direct conversion of digitally quantized, gamma pre-corrected RGB is also possible. This
/// yields minor differences compared to a conversion to analog signals and quantization. A strict
/// integer arithmetic quantization is available as well where performance concerns make the
/// floating point conversion less reasonable.
pub trait QuantizationFn<Y: YuvStandard> {
    type Output: Component;

    /// Quantize an analog yuv pixel.
    fn quantize<F: Component + Float>(yuv: [F; 3]) -> [Self::Output; 3];

    /// Quantize from an rgb value directly.
    fn quantize_direct<F: Component + Float>(rgb: [F; 3]) -> [Self::Output; 3];

    /// Transfer a quantized color value.
    fn quantized_rgb(rgb: [Self::Output; 3]) -> [Self::Output; 3];
}

impl<R: RgbSpace, T: TransferFn, D: Differences> YuvStandard for (R, T, D) {
    type RgbSpace = R;
    type TransferFn = T;
    type Differences = D;
}
