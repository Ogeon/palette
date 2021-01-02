//! A quantized color representation.
use core::marker::PhantomData;

use crate::yuv::{QuantizationFn, YuvStandard};

/// A quantized YUV color value.
///
/// Quantization is the process of encoding the analog signal into digital levels. The thus encoded
/// signal does not necessarily cover all possible levels and may leave some *headroom* in the low
/// and high ends of the available code symbols.
#[derive(Debug, PartialEq, Pixel)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
)]
#[repr(C)]
pub struct YCbCr<S: YuvStandard, Q: QuantizationFn> {
    /// The lumnance signal where `0.0f` is no light and `1.0f` means a maximum displayable amount
    /// of all colors.
    pub luminance: Q::Output,

    /// The difference between overall brightness and the blue signal. This is centered around
    /// `0.0f` but its maximum absolute value depends on the standard.
    pub blue_diff: Q::Output,

    /// The difference between overall brightness and the red signal. This is centered around
    /// `0.0f` but its maximum absolute value depends on the standard.
    pub red_diff: Q::Output,

    /// The kind of YUV standard.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub standard: PhantomData<S>,
}
