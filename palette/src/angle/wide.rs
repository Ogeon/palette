use ::wide::{f32x4, f32x8, f64x2, f64x4, CmpEq};

use super::*;

macro_rules! impl_angle_wide_float {
    ($($ty: ident),+) => {
        $(
            impl HalfRotation for $ty {
                #[inline]
                fn half_rotation() -> Self {
                    $ty::splat(180.0)
                }
            }

            impl FullRotation for $ty {
                #[inline]
                fn full_rotation() -> Self {
                    $ty::splat(360.0)
                }
            }

            impl RealAngle for $ty {
                #[inline]
                fn degrees_to_radians(self) -> Self {
                    self.to_radians()
                }

                #[inline]
                fn radians_to_degrees(self) -> Self {
                    self.to_degrees()
                }
            }

            impl AngleEq for $ty {
                #[inline]
                fn angle_eq(&self, other: &Self) -> Self {
                    self.normalize_unsigned_angle().cmp_eq(other.normalize_unsigned_angle())
                }
            }

            impl SignedAngle for $ty {
                #[inline]
                fn normalize_signed_angle(self) -> Self {
                    self - Round::ceil(((self + 180.0) / 360.0) - 1.0) * 360.0
                }
            }

            impl UnsignedAngle for $ty {
                #[inline]
                fn normalize_unsigned_angle(self) -> Self {
                    use crate::{num::PartialCmp, bool_mask::Select};

                    let normalized = self - (Round::floor(self / 360.0) * 360.0);

                    // Check for issues with very small numbers
                    (normalized.gt_eq(&Self::splat(0.0)) & normalized.lt(&Self::splat(360.0))).select(
                        normalized,
                        Self::splat(0.0)
                    )
                }
            }
        )+
    };
}

impl_angle_wide_float!(f32x4, f32x8, f64x2, f64x4);

#[cfg(test)]
mod test {
    use wide::f64x2;

    use crate::{angle::UnsignedAngle, bool_mask::BoolMask, num::PartialCmp};

    #[test]
    fn normalize_unsigned_angle() {
        // Regression test for https://github.com/Ogeon/palette/issues/473.
        let hue = f64x2::new([-1e-30_f64, -5e-324_f64]);
        let normalized = hue.normalize_unsigned_angle();
        assert!(normalized.lt(&f64x2::splat(360.0)).is_true());
        assert!(normalized.gt_eq(&f64x2::splat(0.0)).is_true());
    }
}
