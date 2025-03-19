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
