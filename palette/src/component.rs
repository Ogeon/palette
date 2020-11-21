use num_traits::Zero;

use crate::float::Float;
use crate::{clamp, FromF64};

/// Common trait for color components.
pub trait Component: Copy + Zero + PartialOrd {
    /// The highest displayable value this component type can reach. Higher
    /// values are allowed, but they may be lowered to this before
    /// converting to another format.
    fn max_intensity() -> Self;
}

/// Common trait for floating point color components.
pub trait FloatComponent: Component + Float + FromF64 {}

impl<T: Component + Float + FromF64> FloatComponent for T {}

macro_rules! impl_float_components {
    ($($ty: ident),+) => {
        $(
            impl Component for $ty {
                fn max_intensity() -> Self {
                    1.0
                }
            }
        )*
    };
}

impl_float_components!(f32, f64);

macro_rules! impl_uint_components {
    ($($ty: ident),+) => {
        $(
            impl Component for $ty {
                fn max_intensity() -> Self {
                    core::$ty::MAX
                }
            }
        )*
    };
}

impl_uint_components!(u8, u16, u32, u64, u128);

/// Converts from a color component type, while performing the appropriate
/// scaling, rounding and clamping.
///
/// ```
/// use palette::FromComponent;
///
/// // Scales the value up to u8::MAX while converting.
/// let u8_component = u8::from_component(1.0f32);
/// assert_eq!(u8_component, 255);
/// ```
pub trait FromComponent<T: Component> {
    /// Converts `other` into `Self`, while performing the appropriate scaling,
    /// rounding and clamping.
    fn from_component(other: T) -> Self;
}

impl<T: Component, U: IntoComponent<T> + Component> FromComponent<U> for T {
    #[inline]
    fn from_component(other: U) -> T {
        other.into_component()
    }
}

/// Converts into a color component type, while performing the appropriate
/// scaling, rounding and clamping.
///
/// ```
/// use palette::IntoComponent;
///
/// // Scales the value up to u8::MAX while converting.
/// let u8_component: u8 = 1.0f32.into_component();
/// assert_eq!(u8_component, 255);
/// ```
pub trait IntoComponent<T: Component> {
    /// Converts `self` into `T`, while performing the appropriate scaling,
    /// rounding and clamping.
    fn into_component(self) -> T;
}

impl<T: Component> IntoComponent<T> for T {
    #[inline]
    fn into_component(self) -> T {
        self
    }
}

// C23 = 2^23, in f32
// C52 = 2^52, in f64
const C23: u32 = 0x4b00_0000;
const C52: u64 = 0x4330_0000_0000_0000;

// Float to uint conversion with rounding to nearest even number. Formula
// follows the form (x_f32 + C23_f32) - C23_u32, where x is the component. From
// Hacker's Delight, p. 378-380.
// Works on the range of [-0.25, 2^23] for f32, [-0.25, 2^52] for f64.
//
// Special cases:
// NaN -> uint::MAX
// inf -> uint::MAX
// -inf -> 0
// Greater than 2^23 for f64, 2^52 for f64 -> uint::MAX
macro_rules! convert_float_to_uint {
    ($float: ident; direct ($($direct_target: ident),+); $(via $temporary: ident ($($target: ident),+);)*) => {
        $(
            impl IntoComponent<$direct_target> for $float {
                #[inline]
                fn into_component(self) -> $direct_target {
                    let max = $direct_target::max_intensity() as $float;
                    let scaled = (self * max).min(max);
                    let f = scaled + f32::from_bits(C23);
                    (f.to_bits().saturating_sub(C23)) as $direct_target
                }
            }
        )+

        $(
            $(
                impl IntoComponent<$target> for $float {
                    #[inline]
                    fn into_component(self) -> $target {
                        let max = $target::max_intensity() as $temporary;
                        let scaled = (self as $temporary * max).min(max);
                        let f = scaled + f64::from_bits(C52);
                        (f.to_bits().saturating_sub(C52)) as  $target
                    }
                }
            )+
        )*
    };
}

// Double to uint conversion with rounding to nearest even number. Formula
// follows the form (x_f64 + C52_f64) - C52_u64, where x is the component.
macro_rules! convert_double_to_uint {
    ($double: ident; direct ($($direct_target: ident),+);) => {
        $(
            impl IntoComponent<$direct_target> for $double {
                #[inline]
                fn into_component(self) -> $direct_target {
                    let max = $direct_target::max_intensity() as $double;
                    let scaled = (self * max).min(max);
                    let f = scaled + f64::from_bits(C52);
                    (f.to_bits().saturating_sub(C52)) as $direct_target
                }
            }
        )+
    };
}

// Uint to float conversion with the formula (x_u32 + C23_u32) - C23_f32, where
// x is the component. We convert the component to f32 then multiply it by the
// reciprocal of the float representation max value for u8.
// Works on the range of [0, 2^23] for f32, [0, 2^52 - 1] for f64.
impl IntoComponent<f32> for u8 {
    #[inline]
    fn into_component(self) -> f32 {
        let comp_u = u32::from(self) + C23;
        let comp_f = f32::from_bits(comp_u) - f32::from_bits(C23);
        let max_u = u32::from(core::u8::MAX) + C23;
        let max_f = (f32::from_bits(max_u) - f32::from_bits(C23)).recip();
        comp_f * max_f
    }
}

// Uint to f64 conversion with the formula (x_u64 + C23_u64) - C23_f64.
impl IntoComponent<f64> for u8 {
    #[inline]
    fn into_component(self) -> f64 {
        let comp_u = u64::from(self) + C52;
        let comp_f = f64::from_bits(comp_u) - f64::from_bits(C52);
        let max_u = u64::from(core::u8::MAX) + C52;
        let max_f = (f64::from_bits(max_u) - f64::from_bits(C52)).recip();
        comp_f * max_f
    }
}

macro_rules! convert_uint_to_float {
    ($uint: ident; $(via $temporary: ident ($($target: ident),+);)*) => {
        $(
            $(
                impl IntoComponent<$target> for $uint {
                    #[inline]
                    fn into_component(self) -> $target {
                        let max = $uint::max_intensity() as $temporary;
                        let scaled = self as $temporary / max;
                        scaled as $target
                    }
                }
            )+
        )*
    };
}

macro_rules! convert_uint_to_uint {
    ($uint: ident; $(via $temporary: ident ($($target: ident),+);)*) => {
        $(
            $(
                impl IntoComponent<$target> for $uint {
                    #[inline]
                    fn into_component(self) -> $target {
                        let target_max = $target::max_intensity() as $temporary;
                        let own_max = $uint::max_intensity() as $temporary;
                        let scaled = (self as $temporary / own_max) * target_max;
                        clamp(scaled.round(), 0.0, target_max) as $target
                    }
                }
            )+
        )*
    };
}

impl IntoComponent<f64> for f32 {
    #[inline]
    fn into_component(self) -> f64 {
        f64::from(self)
    }
}
convert_float_to_uint!(f32; direct (u8, u16); via f64 (u32, u64, u128););

impl IntoComponent<f32> for f64 {
    #[inline]
    fn into_component(self) -> f32 {
        self as f32
    }
}
convert_double_to_uint!(f64; direct (u8, u16, u32, u64, u128););

convert_uint_to_uint!(u8; via f32 (u16); via f64 (u32, u64, u128););

convert_uint_to_float!(u16; via f32 (f32); via f64 (f64););
convert_uint_to_uint!(u16; via f32 (u8); via f64 (u32, u64, u128););

convert_uint_to_float!(u32; via f64 (f32, f64););
convert_uint_to_uint!(u32; via f64 (u8, u16, u64, u128););

convert_uint_to_float!(u64; via f64 (f32, f64););
convert_uint_to_uint!(u64; via f64 (u8, u16, u32, u128););

convert_uint_to_float!(u128; via f64 (f32, f64););
convert_uint_to_uint!(u128; via f64 (u8, u16, u32, u64););

#[cfg(test)]
mod test {
    use crate::IntoComponent;
    use approx::assert_relative_eq;

    #[test]
    fn float_to_uint() {
        let data = vec![
            -800.0,
            -0.3,
            0.0,
            0.005,
            0.024983,
            0.01,
            0.15,
            0.3,
            0.5,
            0.6,
            0.7,
            0.8,
            0.8444,
            0.9,
            0.955,
            0.999,
            1.0,
            1.4,
            f32::from_bits(0x4b44_0000),
            std::f32::MAX,
            std::f32::MIN,
            std::f32::NAN,
            std::f32::INFINITY,
            std::f32::NEG_INFINITY,
        ];

        let expected = vec![
            0u8, 0, 0, 1, 6, 3, 38, 76, 128, 153, 178, 204, 215, 230, 244, 255, 255, 255, 255, 255,
            0, 255, 255, 0,
        ];

        for (d, e) in data.into_iter().zip(expected) {
            assert_eq!(IntoComponent::<u8>::into_component(d), e);
        }
    }

    #[test]
    fn double_to_uint() {
        let data = vec![
            -800.0,
            -0.3,
            0.0,
            0.005,
            0.024983,
            0.01,
            0.15,
            0.3,
            0.5,
            0.6,
            0.7,
            0.8,
            0.8444,
            0.9,
            0.955,
            0.999,
            1.0,
            1.4,
            f64::from_bits(0x4334_0000_0000_0000),
            std::f64::MAX,
            std::f64::MIN,
            std::f64::NAN,
            std::f64::INFINITY,
            std::f64::NEG_INFINITY,
        ];

        let expected = vec![
            0u8, 0, 0, 1, 6, 3, 38, 76, 128, 153, 178, 204, 215, 230, 244, 255, 255, 255, 255, 255,
            0, 255, 255, 0,
        ];

        for (d, e) in data.into_iter().zip(expected) {
            assert_eq!(IntoComponent::<u8>::into_component(d), e);
        }
    }

    #[test]
    fn uint_to_float() {
        fn into_component_old(n: u8) -> f32 {
            let max = core::u8::MAX as f32;
            let scaled = n as f32 / max;
            scaled as f32
        }

        for n in (0..=255).step_by(5) {
            assert_relative_eq!(
                IntoComponent::<f32>::into_component(n),
                into_component_old(n)
            )
        }
    }

    #[test]
    fn uint_to_double() {
        fn into_component_old(n: u8) -> f64 {
            let max = core::u8::MAX as f64;
            let scaled = n as f64 / max;
            scaled as f64
        }

        for n in (0..=255).step_by(5) {
            assert_relative_eq!(
                IntoComponent::<f64>::into_component(n),
                into_component_old(n)
            )
        }
    }
}
