macro_rules! impl_euclidean_distance {
    (
        $ty: ident
        {$($component: ident),+}
        $(where $($where: tt)+)?
    ) => {
        // add empty generics brackets
        impl_euclidean_distance!($ty<> {$($component),+} $(where $($where)+)?);
    };
    (
        $ty: ident <$($ty_param: ident),*>
        {$($component: ident),+}
        $(where $($where: tt)+)?
    ) => {
        impl<$($ty_param,)* T> crate::color_difference::EuclideanDistance for $ty<$($ty_param,)* T>
        where
            T: self::num::Real + core::ops::Sub<T, Output=T> + core::ops::Add<T, Output=T> + core::ops::Mul<T, Output=T> + Clone,
            $($($where)+)?
        {
            type Scalar = T;

            #[inline]
            fn distance_squared(self, other: Self) -> Self::Scalar {
                let difference = self - other;
                let differece_squared = difference.clone() * difference;

                strip_plus!($(+ differece_squared.$component)+)
            }
        }
    };
}
