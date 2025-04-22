use ::simba::scalar::SupersetOf;
use ::simba::simd::{
    SimdComplexField, SimdPartialOrd, SimdRealField, WideBoolF32x4, WideBoolF32x8, WideBoolF64x4,
    WideF32x4, WideF32x8, WideF64x4,
};

use super::*;

macro_rules! impl_simba_float {
    ($($ty: ident ($bool: ident, $scalar: ty, $lanes: expr)),+) => {
        $(
            impl Real for $ty {
                #[inline(always)]
                fn from_f64(n: f64) -> Self {
                   Self::from_subset(&n)
                }
            }


            impl Zero for $ty {
                #[inline(always)]
                fn zero() -> Self {
                     Self::ZERO
                }
            }


            impl One for $ty {
                #[inline(always)]
                fn one() -> Self {
                    Self::ONE
                }
            }


            impl MinMax for $ty {
                #[inline(always)]
                fn min(self, other: Self) -> Self {
                    self.simd_min(other)
                }

                #[inline(always)]
                fn max(self, other: Self) -> Self {
                    self.simd_max(other)
                }

                #[inline(always)]
                fn min_max(self, other: Self) -> (Self, Self){
                    (self.simd_min(other), self.simd_max(other))
                }
            }


            impl Clamp for $ty {
                #[inline(always)]
                fn clamp(self, min: Self, max: Self) -> Self {
                    self.simd_clamp(min, max)
                }

                #[inline(always)]
                fn clamp_min(self, min: Self) -> Self{
                    self.simd_max(min)
                }


                #[inline(always)]
                fn clamp_max(self, max: Self) -> Self{
                    self.simd_min(max)
                }
            }


            impl ClampAssign for $ty {
                #[inline(always)]
                fn clamp_assign(&mut self, min: Self, max: Self) {
                    *self = self.simd_clamp(min, max);
                }

                #[inline(always)]
                fn clamp_min_assign(&mut self, min: Self){
                    *self = self.simd_max(min);
                }


                #[inline(always)]
                fn clamp_max_assign(&mut self, max: Self){
                    *self = self.simd_min(max);
                }
            }


            impl PartialCmp for $ty {
                #[inline(always)]
                fn lt(&self, other: &Self) -> Self::Mask{
                    self.simd_lt(*other)
                }
                #[inline(always)]
                fn lt_eq(&self, other: &Self) -> Self::Mask {
                    self.simd_le(*other)
                }
                #[inline(always)]
                fn eq(&self, other: &Self) -> Self::Mask {
                    self.simd_eq(*other)
                }
                #[inline(always)]
                fn neq(&self, other: &Self) -> Self::Mask {
                    self.simd_ne(*other)
                }
                #[inline(always)]
                fn gt_eq(&self, other: &Self) -> Self::Mask {
                    self.simd_ge(*other)
                }
                #[inline(always)]
                fn gt(&self, other: &Self) -> Self::Mask {
                    self.simd_gt(*other)
                }
            }


            impl Abs for $ty {
                #[inline(always)]
                fn abs(self) -> Self {
                    self.simd_abs()
                }
            }


            impl Cbrt for $ty {
                #[inline(always)]
                fn cbrt(self) -> Self {
                    self.simd_cbrt()
                }
            }


            impl Exp for $ty {
                #[inline(always)]
                fn exp(self) -> Self {
                    self.simd_exp()
                }
            }


            impl Ln for $ty {
                #[inline(always)]
                fn ln(self) -> Self {
                    self.simd_ln()
                }
            }


            impl Powf for $ty {
                #[inline(always)]
                fn powf(self, exp: Self) -> Self {
                    self.simd_powf(exp)
                }
            }


            impl Powi for $ty {
                #[inline(always)]
                fn powi(self, exp: i32) -> Self {
                    self.simd_powi(exp)
                }
            }


            impl Recip for $ty {
                #[inline(always)]
                fn recip(self) -> Self {
                    self.simd_recip()
                }
            }


            impl Signum for $ty {
                #[inline(always)]
                fn signum(self) -> Self {
                    self.simd_signum()
                }
            }



            impl Round for $ty {
                #[inline(always)]
                fn round(self) -> Self {
                    self.simd_round()
                }

                #[inline(always)]
                fn floor(self) -> Self {
                    self.simd_floor()
                }

                #[inline(always)]
                fn ceil(self) -> Self {
                    self.simd_ceil()
                }
            }


            impl IsValidDivisor for $ty {
                #[inline(always)]
                fn is_valid_divisor(&self) -> Self::Mask {
                    self.simd_ne(Self::from_subset(&(0.0 as $scalar))) & self.simd_ne(Self::from_subset(&(-0.0  as $scalar))) & $bool(self.0.is_finite())
                }
            }


            impl MulAdd for $ty {
                #[inline(always)]
                fn mul_add(self, m: Self, a: Self) -> Self {
                    self.simd_mul_add(m, a)
                }
            }


            impl MulSub for $ty {
                #[inline(always)]
                fn mul_sub(self, m: Self, s: Self) -> Self {
                    self.simd_mul_add(m, -s)
                }
            }


            impl FromScalar for $ty {
                type Scalar = $scalar;

                #[inline(always)]
                fn from_scalar(scalar: Self::Scalar) -> Self {
                    Self::from_subset(&scalar)
                }
            }


            impl FromScalarArray<$lanes> for $ty {
                #[inline(always)]
                fn from_array(scalars: [Self::Scalar; $lanes]) -> Self {
                    Self::from(scalars)
                }
            }


            impl IntoScalarArray<$lanes> for $ty {
                #[inline(always)]
                fn into_array(self) ->  [Self::Scalar; $lanes] {
                    self.into()
                }
            }


            impl Hypot for $ty {
                #[inline(always)]
                fn hypot(self, other: Self) -> Self {
                    self.simd_hypot(other)
                }
            }


            impl Sqrt for $ty {
                #[inline(always)]
                fn sqrt(self) -> Self {
                    self.simd_sqrt()
                }
            }


            impl Trigonometry for $ty {
                #[inline(always)]
                fn sin(self) -> Self {
                    self.simd_sin()
                }

                #[inline(always)]
                fn cos(self) -> Self {
                    self.simd_cos()
                }

                #[inline(always)]
                fn sin_cos(self) -> (Self, Self) {
                    (self.simd_sin(), self.simd_cos())
                }

                #[inline(always)]
                fn tan(self) -> Self {
                    self.simd_tan()
                }

                #[inline(always)]
                fn asin(self) -> Self {
                    self.simd_asin()
                }

                #[inline(always)]
                fn acos(self) -> Self {
                    self.simd_acos()
                }


                #[inline(always)]
                fn atan(self) -> Self {
                    self.simd_atan()
                }


                #[inline(always)]
                fn atan2(self, other: Self) -> Self {
                    self.simd_atan2(other)
                }
            }
        )+
    };
}

impl_simba_float!(
    WideF32x4(WideBoolF32x4, f32, 4),
    WideF32x8(WideBoolF32x8, f32, 8),
    WideF64x4(WideBoolF64x4, f64, 4)
);
