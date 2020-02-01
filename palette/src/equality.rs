use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use crate::float::Float;
use crate::white_point::WhitePoint;
use crate::{from_f64, FloatComponent, FromF64, Lab, LabHue, Lch, RgbHue, Xyz, Yxy};

macro_rules! impl_eq {
    (  $self_ty: ident , [$($element: ident),+]) => {
        impl<Wp, T> AbsDiffEq for $self_ty<Wp, T>
        where T: FloatComponent + AbsDiffEq,
            T::Epsilon: Copy + FloatComponent,
            Wp: WhitePoint + PartialEq
        {
            type Epsilon = T::Epsilon;

            fn default_epsilon() -> Self::Epsilon {
                T::default_epsilon()
            }

            fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
                $( self.$element.abs_diff_eq(&other.$element, epsilon) )&&+
            }
            fn abs_diff_ne(&self, other: &Self, epsilon: T::Epsilon) -> bool {
                $( self.$element.abs_diff_ne(&other.$element, epsilon) )||+
            }
        }

        impl<Wp, T> RelativeEq for $self_ty<Wp, T>
        where T: FloatComponent + RelativeEq,
            T::Epsilon: Copy + FloatComponent,
            Wp: WhitePoint + PartialEq
        {
            fn default_max_relative() -> T::Epsilon {
                T::default_max_relative()
            }

            fn relative_eq(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
                $( self.$element.relative_eq(&other.$element, epsilon, max_relative) )&&+
            }
            fn relative_ne(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
                $( self.$element.relative_ne(&other.$element, epsilon, max_relative) )||+
            }
        }

        impl<Wp, T> UlpsEq for $self_ty<Wp, T>
        where T: FloatComponent + UlpsEq,
            T::Epsilon: Copy + FloatComponent,
            Wp: WhitePoint + PartialEq
        {
            fn default_max_ulps() -> u32 {
                T::default_max_ulps()
            }

            fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
                $( self.$element.ulps_eq(&other.$element, epsilon, max_ulps) )&&+
            }
            fn ulps_ne(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
                $( self.$element.ulps_ne(&other.$element, epsilon, max_ulps) )||+
            }
        }
    }
}

impl_eq!(Xyz, [x, y, z]);
impl_eq!(Yxy, [y, x, luma]);
impl_eq!(Lab, [l, a, b]);
impl_eq!(Lch, [l, chroma, hue]);

// For hues, the difference is calculated and compared to zero. However due to
// the way floating point's work this is not so simple.
//
// Reference:
// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
//
// The recommendation is use 180 * epsilon as the epsilon and do not compare by
// ulps. Because of this we loose some precision for values close to 0.0.
macro_rules! impl_eq_hue {
    (  $self_ty: ident ) => {
        impl<T: Float + FromF64 + AbsDiffEq> AbsDiffEq for $self_ty<T>
        where
            T::Epsilon: Float + FromF64,
        {
            type Epsilon = T::Epsilon;

            fn default_epsilon() -> Self::Epsilon {
                T::default_epsilon() * from_f64(180.0)
            }

            fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
                let diff: T = (*self - *other).to_degrees();
                T::abs_diff_eq(&diff, &T::zero(), epsilon)
            }
            fn abs_diff_ne(&self, other: &Self, epsilon: T::Epsilon) -> bool {
                let diff: T = (*self - *other).to_degrees();
                T::abs_diff_ne(&diff, &T::zero(), epsilon)
            }
        }

        impl<T: Float + FromF64 + RelativeEq> RelativeEq for $self_ty<T>
        where
            T::Epsilon: Float + FromF64,
        {
            fn default_max_relative() -> Self::Epsilon {
                T::default_max_relative() * from_f64(180.0)
            }

            fn relative_eq(
                &self,
                other: &Self,
                epsilon: T::Epsilon,
                max_relative: T::Epsilon,
            ) -> bool {
                let diff: T = (*self - *other).to_degrees();
                T::relative_eq(&diff, &T::zero(), epsilon, max_relative)
            }
            fn relative_ne(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                let diff: T = (*self - *other).to_degrees();
                T::relative_ne(&diff, &T::zero(), epsilon, max_relative)
            }
        }

        impl<T: Float + FromF64 + UlpsEq> UlpsEq for $self_ty<T>
        where
            T::Epsilon: Float + FromF64,
        {
            fn default_max_ulps() -> u32 {
                T::default_max_ulps() * 180
            }

            fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
                let diff: T = (*self - *other).to_degrees();
                T::ulps_eq(&diff, &T::zero(), epsilon, max_ulps)
            }
            fn ulps_ne(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                let diff: T = (*self - *other).to_degrees();
                T::ulps_ne(&diff, &T::zero(), epsilon, max_ulps)
            }
        }
    };
}

impl_eq_hue!(LabHue);
impl_eq_hue!(RgbHue);
