use super::QuantizationFn;
use crate::{clamp, from_f64, FloatComponent};
use num_traits::NumCast;

/// Shared 8-bit quantization functions.
pub struct QuantU8;

impl QuantU8 {
    fn quantize_yuv<F: FloatComponent>([y, u, v]: [F; 3]) -> [u8; 3] {
        let y = y*from_f64(219.) + from_f64(16.);
        let u = u*from_f64(224.) + from_f64(128.);
        let v = v*from_f64(224.) + from_f64(128.);
        [int_u8(y), int_u8(u), int_u8(v)]
    }

    fn quantize_rgb<F: FloatComponent>([r, g, b]: [F; 3]) -> [u8; 3] {
        let r = r*from_f64(219.) + from_f64(16.);
        let g = g*from_f64(219.) + from_f64(16.);
        let b = b*from_f64(219.) + from_f64(16.);
        [int_u8(r), int_u8(g), int_u8(b)]
    }
}

impl QuantizationFn for QuantU8 {
    type Output = u8;

    fn quantize_yuv<F: FloatComponent>(yuv: [F; 3]) -> [u8; 3] {
        Self::quantize_yuv(yuv)
    }

    fn quantize_rgb<F: FloatComponent>(rgb: [F; 3]) -> [u8; 3] {
        Self::quantize_rgb(rgb)
    }
}

/// Round to 8-bit integer in valid signal range.
fn int_u8<F: FloatComponent>(value: F) -> u8 {
    // Note: signal level below 1 and the level 255 and above are reserved.
    let value: F = clamp(value.round(), from_f64(1.), from_f64(254.));
    NumCast::from(value).unwrap()
}

// TODO: 10bit quantization. Already here as a reference for encoding to 10 bit quantized values.
#[allow(unused)]
fn to_u10<F: FloatComponent>(value: F) -> u16 {
    // Representation of 254.75 with 2 fractional bits.
    const UPPER_MAX: f64 = (4*254 + 3) as f64; // = 1019
    // Representation of 1.0 with 2 fraction bits.
    const LOWER_MIN: f64 = 4.0;

    // Add two fractional bits.
    let value = value*from_f64(4.);

    // Note: signal level below 1 and the level 255 and above are reserved.
    let value: F = clamp(value.round(), from_f64(LOWER_MIN), from_f64(UPPER_MAX));
    // Final division is only conceptual, output has 2 fractional bits.
    NumCast::from(value).unwrap()
}
