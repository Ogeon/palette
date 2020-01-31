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

macro_rules! convert_float_to_uint {
    ($float: ident; direct ($($direct_target: ident),+); $(via $temporary: ident ($($target: ident),+);)*) => {
        $(
            impl IntoComponent<$direct_target> for $float {
                #[inline]
                fn into_component(self) -> $direct_target {
                    let max = $direct_target::max_intensity() as $float;
                    let scaled = self * max;
                    clamp(scaled.round(), 0.0, max) as $direct_target
                }
            }
        )+

        $(
            $(
                impl IntoComponent<$target> for $float {
                    #[inline]
                    fn into_component(self) -> $target {
                        let max = $target::max_intensity() as $temporary;
                        let scaled = self as $temporary * max;
                        clamp(scaled.round(), 0.0, max) as $target
                    }
                }
            )+
        )*
    };
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
        self as f64
    }
}
convert_float_to_uint!(f32; direct (u8, u16); via f64 (u32, u64, u128););

impl IntoComponent<f32> for f64 {
    #[inline]
    fn into_component(self) -> f32 {
        self as f32
    }
}
convert_float_to_uint!(f64; direct (u8, u16, u32, u64, u128););

convert_uint_to_float!(u8; via f32 (f32); via f64 (f64););
convert_uint_to_uint!(u8; via f32 (u16); via f64 (u32, u64, u128););

convert_uint_to_float!(u16; via f32 (f32); via f64 (f64););
convert_uint_to_uint!(u16; via f32 (u8); via f64 (u32, u64, u128););

convert_uint_to_float!(u32; via f64 (f32, f64););
convert_uint_to_uint!(u32; via f64 (u8, u16, u64, u128););

convert_uint_to_float!(u64; via f64 (f32, f64););
convert_uint_to_uint!(u64; via f64 (u8, u16, u32, u128););

convert_uint_to_float!(u128; via f64 (f32, f64););
convert_uint_to_uint!(u128; via f64 (u8, u16, u32, u64););
