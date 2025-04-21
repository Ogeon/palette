use ::wide::{f32x4, f32x8, f64x2, f64x4};

use super::*;

macro_rules! impl_wide_float {
    ($($ty: ident { array: [$scalar: ident; $n: expr], powf: $pow_self: ident, }),+) => {
        $(
            impl One for $ty {
                #[inline]
                fn one() -> Self {
                    $ty::ONE
                }
            }

            impl Powf for $ty {
                #[inline]
                fn powf(self, exp: Self) -> Self {
                    $ty::$pow_self(self, exp)
                }
            }

            impl Powi for $ty {
                #[inline]
                fn powi(mut self, mut exp: i32) -> Self {
                    if exp < 0 {
                        exp = exp.wrapping_neg();
                        self = self.recip();
                    }

                    Powu::powu(self, exp as u32)
                }
            }

            impl Powu for $ty {
                #[inline]
                fn powu(self, exp: u32) -> Self {
                    pow(self, exp)
                }
            }
        )+
    };
}

impl_wide_float!(
    f32x4 {
        array: [f32; 4],
        powf: pow_f32x4,
    },
    f32x8 {
        array: [f32; 8],
        powf: pow_f32x8,
    },
    f64x2 {
        array: [f64; 2],
        powf: pow_f64x2,
    },
    f64x4 {
        array: [f64; 4],
        powf: pow_f64x4,
    }
);

impl Recip for f32x4 {
    #[inline]
    fn recip(self) -> Self {
        f32x4::recip(self)
    }
}

impl Recip for f32x8 {
    #[inline]
    fn recip(self) -> Self {
        f32x8::recip(self)
    }
}

impl Recip for f64x2 {
    #[inline]
    fn recip(self) -> Self {
        f64x2::ONE / self
    }
}

impl Recip for f64x4 {
    #[inline]
    fn recip(self) -> Self {
        f64x4::ONE / self
    }
}
