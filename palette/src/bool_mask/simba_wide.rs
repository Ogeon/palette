use ::simba::simd::{
    SimdBool, SimdValue, WideBoolF32x4, WideBoolF32x8, WideBoolF64x4, WideF32x4, WideF32x8,
    WideF64x4,
};

use super::{BoolMask, HasBoolMask, LazySelect, Select};

macro_rules! impl_simba_bool_mask {
    ($($ty: ident: ($bool: ident,$uint: ident, $lanes: expr)),+) => {
        $(

            impl HasBoolMask for $ty {
               type Mask = $bool;
            }

            impl HasBoolMask for $bool {
               type Mask = Self;
            }


            impl BoolMask for $bool {
                #[inline(always)]
                fn from_bool(value: bool) -> Self {
                    Self::splat(value)
                }

                #[inline(always)]
                fn is_true(&self) -> bool {
                    self.0.all()
                }

                #[inline(always)]
                fn is_false(&self) -> bool{
                    self.0.none()
                }
            }


            impl<T> Select<T>  for $bool where T: HasBoolMask<Mask = Self> + SimdValue<SimdBool = Self>, {
                #[inline(always)]
                fn select(self, a: T, b: T) -> T {
                    SimdValue::select(a, self, b)
                }
            }


            impl<T> LazySelect<T>  for $bool where T: HasBoolMask<Mask = Self> + SimdValue<SimdBool = Self>, {
                #[inline(always)]
                fn lazy_select<A, B>(self, a: A, b: B) -> T
                where
                    A: FnOnce() -> T,
                    B: FnOnce() -> T,
                {
                    self.if_else(a, b)
                }
            }
        )+
    };
}

impl_simba_bool_mask!(
    WideF32x4: (WideBoolF32x4, u32, 4),
    WideF32x8: (WideBoolF32x8, u32, 8),
    WideF64x4: (WideBoolF64x4, u64, 4)
);

#[cfg(test)]
mod test {
    use ::simba::simd::{WideBoolF32x4, WideBoolF32x8, WideBoolF64x4};

    use crate::bool_mask::BoolMask;

    #[test]
    fn from_true() {
        assert!(WideBoolF32x4::from_bool(true).is_true());
        assert!(!WideBoolF32x4::from_bool(true).is_false());

        assert!(WideBoolF32x8::from_bool(true).is_true());
        assert!(!WideBoolF32x8::from_bool(true).is_false());

        assert!(WideBoolF64x4::from_bool(true).is_true());
        assert!(!WideBoolF64x4::from_bool(true).is_false());
    }

    #[test]
    fn from_false() {
        assert!(WideBoolF32x4::from_bool(false).is_false());
        assert!(!WideBoolF32x4::from_bool(false).is_true());

        assert!(WideBoolF32x8::from_bool(false).is_false());
        assert!(!WideBoolF32x8::from_bool(false).is_true());

        assert!(WideBoolF64x4::from_bool(false).is_false());
        assert!(!WideBoolF64x4::from_bool(false).is_true());
    }
}
