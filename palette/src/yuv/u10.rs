//! Internal module to define quantization levels.

/// A 10-bit fixed point refinement of a u8.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct U10(u16);

impl U10 {
    /// Convert a u8.
    ///
    /// Since the additional two-bits represent fractional parts of the number the provided value
    /// is the integer part of the resulting number.
    pub fn convert_u8(val: u8) -> Self {
        U10(u16::from(val) << 2)
    }

    /// Retrieve the integer part of the represented number.
    pub fn trunc(self) -> u8 {
        (self.0 >> 2) as u8
    }

    /// Retrieve the fractional part of the represented number.
    pub fn frac(self) -> u8 {
        (self.0 & 0x3) as u8
    }

    /// Reinterpret a `u16` as a `U10` if possible.
    ///
    /// If the number fits within the range of 10-bits (0–1024) then reinterpret as such, with the
    /// last two bits being the fractional part and first eight the integral one.
    pub fn from_fractional_u16(val: u16) -> Option<Self> {
        if val < 1024 {
            Some(U10(val))
        } else {
            None
        }
    }

    /// Reinterpret as 10-bit fixed point by clamping the value.
    pub fn from_fractional_u16_clamped(val: u16) -> Self {
        U10(val.min(1023))
    }
}
