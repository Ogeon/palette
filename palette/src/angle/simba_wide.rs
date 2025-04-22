use ::simba::scalar::SupersetOf;
use ::simba::simd::{SimdPartialOrd, SimdRealField, WideF32x4, WideF32x8, WideF64x4};

use super::*;

macro_rules! impl_angle_simba_float {
    ($($ty: ident ($scalar_ty: ident)),+) => {
        $(

            impl HalfRotation for $ty {
                #[inline(always)]
                fn half_rotation() -> Self {
                    Self::from_subset(&(180.0 as $scalar_ty))
                }
            }

            impl FullRotation for $ty {
                #[inline(always)]
                fn full_rotation() -> Self {
                    Self::from_subset(&(360.0 as $scalar_ty))
                }
            }

            impl RealAngle for $ty {
                #[inline(always)]
                fn degrees_to_radians(self) -> Self {
                    let rads_per_degree = Self::simd_pi() / Self::from_subset(&(180.0 as $scalar_ty));
                    self * rads_per_degree
                }

                #[inline(always)]
                fn radians_to_degrees(self) -> Self {
                    let pis_in_180 = Self::from_subset(&(180.0 as $scalar_ty)) / $ty::simd_pi();
                    self * pis_in_180
                }
            }

            impl AngleEq for $ty {
                #[inline(always)]
                fn angle_eq(&self, other: &Self) -> Self::Mask {
                    self.normalize_unsigned_angle().simd_eq(other.normalize_unsigned_angle())
                }
            }

            impl SignedAngle for $ty {
                #[inline(always)]
                fn normalize_signed_angle(self) -> Self {
                    self - Round::ceil(((self + Self::from_subset(&(180.0 as $scalar_ty))) / Self::from_subset(&(360.0 as $scalar_ty))) - Self::from_subset(&(1.0 as $scalar_ty))) * Self::from_subset(&(360.0 as $scalar_ty))
                }
            }

            impl UnsignedAngle for $ty {
                #[inline(always)]
                fn normalize_unsigned_angle(self) -> Self {
                    self - (Round::floor(self / Self::from_subset(&(360.0 as $scalar_ty))) * Self::from_subset(&(360.0 as $scalar_ty)))
                }
            }
        )+
    };
}

impl_angle_simba_float!(WideF32x4(f32), WideF32x8(f32), WideF64x4(f64));

#[cfg(test)]
mod test {
    use std::cmp::Ordering;
    use crate::num::{IntoScalarArray};
    use super::*;

    #[test]
    fn test_half_rotation() {
        assert_eq!(WideF32x4::half_rotation().into_array().partial_cmp(&[180.0f32, 180.0f32, 180.0f32, 180.0f32]).unwrap(), Ordering::Equal);
        assert_eq!(WideF32x8::half_rotation().into_array().partial_cmp(&[180.0f32, 180.0f32, 180.0f32, 180.0f32, 180.0f32, 180.0f32, 180.0f32, 180.0f32]).unwrap(), Ordering::Equal);

        assert_eq!(WideF64x4::half_rotation().into_array().partial_cmp(&[180.0f64, 180.0f64, 180.0f64, 180.0f64]).unwrap(), Ordering::Equal);
    }

    #[test]
    fn test_full_rotation() {
        assert_eq!(WideF32x4::full_rotation().into_array().partial_cmp(&[360.0f32, 360.0f32, 360.0f32, 360.0f32]).unwrap(), Ordering::Equal);
        assert_eq!(WideF32x8::full_rotation().into_array().partial_cmp(&[360.0f32, 360.0f32, 360.0f32, 360.0f32, 360.0f32, 360.0f32, 360.0f32, 360.0f32]).unwrap(), Ordering::Equal);
        assert_eq!(WideF64x4::full_rotation().into_array().partial_cmp(&[360.0f64, 360.0f64, 360.0f64, 360.0f64]).unwrap(), Ordering::Equal);
    }

    #[test]
    fn test_degrees_to_radians() {
        assert_eq!(WideF32x4::half_rotation().degrees_to_radians(), WideF32x4::simd_pi());
        assert_eq!(WideF32x8::half_rotation().degrees_to_radians(), WideF32x8::simd_pi());
        assert_eq!(WideF64x4::half_rotation().degrees_to_radians(), WideF64x4::simd_pi());

        assert_eq!(WideF32x4::full_rotation().degrees_to_radians(), WideF32x4::simd_two_pi());
        assert_eq!(WideF32x8::full_rotation().degrees_to_radians(), WideF32x8::simd_two_pi());
        assert_eq!(WideF64x4::full_rotation().degrees_to_radians(), WideF64x4::simd_two_pi());
    }

    #[test]
    fn test_radians_to_degrees() {
        assert_eq!(WideF32x4::simd_pi().radians_to_degrees(), WideF32x4::half_rotation());
        assert_eq!(WideF32x8::simd_pi().radians_to_degrees(), WideF32x8::half_rotation());
        assert_eq!(WideF64x4::simd_pi().radians_to_degrees(), WideF64x4::half_rotation());

        assert_eq!(WideF32x4::simd_two_pi().radians_to_degrees(), WideF32x4::full_rotation());
        assert_eq!(WideF32x8::simd_two_pi().radians_to_degrees(), WideF32x8::full_rotation());
        assert_eq!(WideF64x4::simd_two_pi().radians_to_degrees(), WideF64x4::full_rotation());
    }

    #[test]
    fn test_normalize_signed_angle() {
        assert_eq!(
            WideF32x4::from_subset(&-365.0).normalize_signed_angle(),
            WideF32x4::from_subset(&-5.0)
        );
        assert_eq!(
            WideF32x8::from_subset(&-365.0).normalize_signed_angle(),
            WideF32x8::from_subset(&-5.0)
        );
        assert_eq!(
            WideF64x4::from_subset(&-365.0).normalize_signed_angle(),
            WideF64x4::from_subset(&-5.0)
        );

        assert_eq!(
            WideF32x4::from_subset(&-360.0).normalize_signed_angle(),
            WideF32x4::from_subset(&-0.0)
        );
        assert_eq!(
            WideF32x8::from_subset(&-360.0).normalize_signed_angle(),
            WideF32x8::from_subset(&-0.0)
        );
        assert_eq!(
            WideF64x4::from_subset(&-360.0).normalize_signed_angle(),
            WideF64x4::from_subset(&-0.0)
        );

        assert_eq!(
            WideF32x4::from_subset(&0.0).normalize_signed_angle(),
            WideF32x4::from_subset(&0.0)
        );
        assert_eq!(
            WideF32x8::from_subset(&0.0).normalize_signed_angle(),
            WideF32x8::from_subset(&0.0)
        );
        assert_eq!(
            WideF64x4::from_subset(&0.0).normalize_signed_angle(),
            WideF64x4::from_subset(&0.0)
        );

        assert_eq!(
            WideF32x4::from_subset(&360.0).normalize_signed_angle(),
            WideF32x4::from_subset(&0.0)
        );
        assert_eq!(
            WideF32x8::from_subset(&360.0).normalize_signed_angle(),
            WideF32x8::from_subset(&0.0)
        );
        assert_eq!(
            WideF64x4::from_subset(&360.0).normalize_signed_angle(),
            WideF64x4::from_subset(&0.0)
        );

        assert_eq!(
            WideF32x4::from_subset(&365.0).normalize_signed_angle(),
            WideF32x4::from_subset(&5.0)
        );
        assert_eq!(
            WideF32x8::from_subset(&365.0).normalize_signed_angle(),
            WideF32x8::from_subset(&5.0)
        );
        assert_eq!(
            WideF64x4::from_subset(&365.0).normalize_signed_angle(),
            WideF64x4::from_subset(&5.0)
        );
    }

    #[test]
    fn test_normalize_unsigned_angle() {
        assert_eq!(
            WideF32x4::from_subset(&-365.0).normalize_unsigned_angle(),
            WideF32x4::from_subset(&355.0)
        );
        assert_eq!(
            WideF32x8::from_subset(&-365.0).normalize_unsigned_angle(),
            WideF32x8::from_subset(&355.0)
        );
        assert_eq!(
            WideF64x4::from_subset(&-365.0).normalize_unsigned_angle(),
            WideF64x4::from_subset(&355.0)
        );

        assert_eq!(
            WideF32x4::from_subset(&-360.0).normalize_unsigned_angle(),
            WideF32x4::from_subset(&0.0)
        );
        assert_eq!(
            WideF32x8::from_subset(&-360.0).normalize_unsigned_angle(),
            WideF32x8::from_subset(&0.0)
        );
        assert_eq!(
            WideF64x4::from_subset(&-360.0).normalize_unsigned_angle(),
            WideF64x4::from_subset(&0.0)
        );

        assert_eq!(
            WideF32x4::from_subset(&0.0).normalize_unsigned_angle(),
            WideF32x4::from_subset(&0.0)
        );
        assert_eq!(
            WideF32x8::from_subset(&0.0).normalize_unsigned_angle(),
            WideF32x8::from_subset(&0.0)
        );
        assert_eq!(
            WideF64x4::from_subset(&0.0).normalize_unsigned_angle(),
            WideF64x4::from_subset(&0.0)
        );

        assert_eq!(
            WideF32x4::from_subset(&360.0).normalize_unsigned_angle(),
            WideF32x4::from_subset(&0.0)
        );
        assert_eq!(
            WideF32x8::from_subset(&360.0).normalize_unsigned_angle(),
            WideF32x8::from_subset(&0.0)
        );
        assert_eq!(
            WideF64x4::from_subset(&360.0).normalize_unsigned_angle(),
            WideF64x4::from_subset(&0.0)
        );

        assert_eq!(
            WideF32x4::from_subset(&365.0).normalize_unsigned_angle(),
            WideF32x4::from_subset(&5.0)
        );
        assert_eq!(
            WideF32x8::from_subset(&365.0).normalize_unsigned_angle(),
            WideF32x8::from_subset(&5.0)
        );
        assert_eq!(
            WideF64x4::from_subset(&365.0).normalize_unsigned_angle(),
            WideF64x4::from_subset(&5.0)
        );
    }
}