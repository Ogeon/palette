macro_rules! impl_premultiply {
    ($ty: ident {$($component: ident),+} $(phantom: $phantom: ident)? $(where $($where: tt)+)?) => {
        impl_premultiply!($ty<> {$($component),+} $(phantom: $phantom)? $(where $($where)+)?);
    };
    ($ty: ident <$($ty_param: ident),*> {$($component: ident),+} $(phantom: $phantom: ident)? $(where $($where: tt)+)?) => {
        impl<$($ty_param,)* T> Premultiply for $ty<$($ty_param,)* T>
        where
            T: Real + Stimulus + Zero + IsValidDivisor + Mul<T, Output = T> + Div<T, Output = T> + Clone,
            T::Mask: LazySelect<T> + Clone,
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
                let PreAlpha {
                    color: $ty { $($component,)+ .. },
                    alpha,
                } = premultiplied;

                let is_valid_divisor = alpha.is_valid_divisor();

                let color = Self {
                    $(
                        $component: lazy_select! {
                            if is_valid_divisor.clone() => $component / alpha.clone(),
                            else => T::zero()
                        },
                    )+
                    $($phantom: PhantomData,)?
                };

                (color, alpha)
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
