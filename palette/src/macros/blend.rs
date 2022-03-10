macro_rules! impl_premultiply {
    ($ty: ident $(where $($where: tt)+)?) => {
        impl_premultiply!($ty<> $(where $($where)+)?);
    };
    ($ty: ident <$($ty_param: ident),*> $(where $($where: tt)+)?) => {
        impl<$($ty_param,)* T> Premultiply for $ty<$($ty_param,)* T>
        where
            Self: Mul<T, Output = Self> + Div<T, Output = Self> + Default,
            T: Real + Stimulus + IsValidDivisor + Clone,
            $($($where)+)?
        {
            type Scalar = T;

            #[inline]
            fn premultiply(self, alpha: T) -> PreAlpha<Self> {
                PreAlpha {
                    color: self * alpha.clone(),
                    alpha
                }
            }

            #[inline]
            fn unpremultiply(premultiplied: PreAlpha<Self>) -> (Self, T) {
                let color = if premultiplied.alpha.is_valid_divisor() {
                    premultiplied.color / premultiplied.alpha.clone()
                } else {
                    Self::default()
                };

                (color, premultiplied.alpha)
            }
        }

        impl<$($ty_param,)* T> From<PreAlpha<Self>> for $ty<$($ty_param,)* T>
        where
            Self: Premultiply<Scalar = T>,
        {
            fn from(premultiplied: PreAlpha<Self>) -> Self {
                Self::unpremultiply(premultiplied).0
            }
        }
    };
}
