macro_rules! impl_is_within_bounds {
    (
        $ty: ident
        {$($component: ident => [$get_min: expr, $get_max: expr]),+}
        $(where $($where: tt)+)?
    ) => {
        impl_is_within_bounds!($ty<> {$($component => [$get_min, $get_max]),+} $(where $($where)+)?);
    };
    (
        $ty: ident <$($ty_param: ident),*>
        {$($component: ident => [$get_min: expr, $get_max: expr]),+}
        $(where $($where: tt)+)?
    ) => {
        impl<$($ty_param,)* T> IsWithinBounds for $ty<$($ty_param,)* T>
        where
            T: PartialCmp,
            T::Mask: BitAnd<Output = T::Mask>,
            $($($where)+)?
        {
            #[inline]
            fn is_within_bounds(&self) -> T::Mask {
                $(self.$component.gt_eq(&$get_min) & self.$component.lt_eq(&$get_max))&+
            }
        }
    };
}
