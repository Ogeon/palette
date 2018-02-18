use num_traits::Float;
use approx::ApproxEq;

use {cast, Lab, LabHue, Lch, Luma, RgbHue, Xyz, Yxy};
use white_point::WhitePoint;

macro_rules! impl_eq {
    (  $self_ty: ident , [$($element: ident),+]) => {
        impl<Wp, T> ApproxEq for $self_ty<Wp, T>
        where T: Float + ApproxEq,
            T::Epsilon: Copy + Float,
            Wp: WhitePoint
        {
            type Epsilon = <T as ApproxEq>::Epsilon;

            fn default_epsilon() -> Self::Epsilon {
                T::default_epsilon()
            }
            fn default_max_relative() -> Self::Epsilon {
                T::default_max_relative()
            }
            fn default_max_ulps() -> u32 {
                T::default_max_ulps()
            }
            fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
                $( self.$element.relative_eq(&other.$element, epsilon, max_relative) )&&+
            }
            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool{
                $( self.$element.ulps_eq(&other.$element, epsilon, max_ulps) )&&+
            }

            fn relative_ne(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
                $( self.$element.relative_ne(&other.$element, epsilon, max_relative) )&&+
            }
            fn ulps_ne(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                $( self.$element.ulps_ne(&other.$element, epsilon, max_ulps) )&&+
            }
        }
    }
}

impl_eq!(Xyz, [x, y, z]);
impl_eq!(Yxy, [y, x, luma]);
impl_eq!(Lab, [l, a, b]);
impl_eq!(Luma, [luma]);
impl_eq!(Lch, [l, chroma, hue]);

// For hues diffence is calculated and compared to zero. However due to the way floating point's
// work this is not so simple
// reference
// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
//
// The recommendation is use 180 * epsilon as the epsilon and do not compare by ulps.
// Because of this we loose some precision for values close to 0.0.
macro_rules! impl_eq_hue {
    (  $self_ty: ident ) => {
        impl<T: Float + ApproxEq> ApproxEq for $self_ty<T>
        where <T as ApproxEq>::Epsilon: Float
        {
            type Epsilon = <T as ApproxEq>::Epsilon;

            fn default_epsilon() -> Self::Epsilon {
                T::default_epsilon() * cast(180.0)
            }
            fn default_max_relative() -> Self::Epsilon {
                T::default_max_relative() * cast(180.0)
            }
            fn default_max_ulps() -> u32 {
                T::default_max_ulps() * 180
            }
            fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
                let diff: T = (*self - *other).to_degrees();
                T::relative_eq(&diff, &T::zero(), epsilon, max_relative)
            }
            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool{
                let diff: T = (*self - *other).to_degrees();
                T::ulps_eq(&diff, &T::zero(), epsilon, max_ulps)
            }

            fn relative_ne(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
                let diff: T = (*self - *other).to_degrees();
                T::relative_ne(&diff, &T::zero(), epsilon, max_relative)
            }
            fn ulps_ne(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                let diff: T = (*self - *other).to_degrees();
                T::ulps_ne(&diff, &T::zero(), epsilon, max_ulps)
            }
        }

    }
}

impl_eq_hue!(LabHue);
impl_eq_hue!(RgbHue);
