use super::QuantizationFn;
use {clamp, cast, Component, Float};

/// Shared 8-bit quantization functions.
pub struct QuantU8;

impl QuantU8 {
    fn quantize_yuv<F: Component + Float>([y, u, v]: [F; 3]) -> [u8; 3] {
        let y = y*cast(219.) + cast(16.);
        let u = u*cast(224.) + cast(128.);
        let v = v*cast(224.) + cast(128.);
        [int_u8(y), int_u8(u), int_u8(v)]
    }

    fn quantize_rgb<F: Component + Float>([r, g, b]: [F; 3]) -> [u8; 3] {
        let r = r*cast(219.) + cast(16.);
        let g = g*cast(219.) + cast(16.);
        let b = b*cast(219.) + cast(16.);
        [int_u8(r), int_u8(g), int_u8(b)]
    }
}

impl QuantizationFn for QuantU8 {
    type Output = u8;

    fn quantize_yuv<F: Component + Float>(yuv: [F; 3]) -> [u8; 3] {
        Self::quantize_yuv(yuv)
    }

    fn quantize_rgb<F: Component + Float>(rgb: [F; 3]) -> [u8; 3] {
        Self::quantize_rgb(rgb)
    }
}

/// Round to 8-bit integer in valid signal range.
fn int_u8<F: Component + Float>(value: F) -> u8 {
    // Note: signal level below 1 and the level 255 and above are reserved.
    clamp(value.round(), cast(1.), cast(254.)).convert()
}

// TODO: 10bit quantization. Already here as a reference for encoding to 10 bit quantized values.
#[allow(unused)]
fn to_u10<F: Component + Float>(value: F) -> u16 {
    // Representation of 254.75 with 2 fractional bits.
    const UPPER_MAX: u16 = 4*254 + 3; // = 1019
    // Representation of 1.0 with 2 fraction bits.
    const LOWER_MIN: u16 = 4;

    // Add two fractional bits.
    let value = value*cast(4.);

    // Note: signal level below 1 and the level 255 and above are reserved.
    clamp(value.round(), cast(LOWER_MIN), cast(UPPER_MAX)).convert()
    // Final division is only conceptual, output has 2 fractional bits.
}
