//! YUV types, spaces and standards.
use float::Float;

use encoding::{TransferFn};
use rgb::RgbSpace;
use {Component};

mod quant;
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
    /// The luminance weights correspond closely to the `Y` components of the `yxy`
    /// parameterization of the color space primaries. However, they may add up to a value smaller
    /// than `1` to represent colors appearing brighter than the white point i.e. offer a larger
    /// dynamic range than otherwise possible.
    fn luminance<T: Float>() -> [T; 3];

    /// Normalize the difference of luminance and blue channel.
    fn normalize_blue<T: Float>(denorm: T) -> T;

    /// Denormalize the difference of luminance and blue channel.
    fn denormalize_blue<T: Float>(norm: T) -> T;

    /// Normalize the difference of luminance and red channel.
    fn normalize_red<T: Float>(denorm: T) -> T;

    /// Denormalize the difference of luminance and red channel.
    fn denormalize_red<T: Float>(norm: T) -> T;
}

/// A digital encoding of a YUV color model.
///
/// This is not a mere type conversion. Instead, it is a standardized encoding depending on the bit
/// length of the output symbols. This also ensures that the symbol space is not completely
/// exhausted by color information and therefore keeps some headroom in the produced digital
/// signal.
///
/// While the difference conversion is mostly performed in an analog signal space free of
/// quantization errors, the final digital output is quantized to some number of bits defined in
/// individual standards.
///
// TODO: See https://github.com/Ogeon/palette/issues/121
// The direct conversion of digitally quantized, gamma pre-corrected RGB is also possible. This
// yields minor differences compared to a conversion to analog signals and quantization. A strict
// integer arithmetic quantization is available as well where performance concerns make the
// floating point conversion less reasonable. Note that for Rec.601 there is an extensive
// standardized table of integer coefficients for the conversion depending on the required accuracy
// (8-16 bits) of the intermediates.
pub trait QuantizationFn {
    /// The quantized integer representation of the color value.
    type Output: Component;

    /// Quantize an analog yuv pixel.
    fn quantize_yuv<F: Component + Float>(yuv: [F; 3]) -> [Self::Output; 3];

    /// Quantize an rgb value.
    fn quantize_rgb<F: Component + Float>(rgb: [F; 3]) -> [Self::Output; 3];
}

impl<R: RgbSpace, T: TransferFn, D: DifferenceFn> YuvStandard for (R, T, D) {
    type RgbSpace = R;
    type TransferFn = T;
    type DifferenceFn = D;
}
