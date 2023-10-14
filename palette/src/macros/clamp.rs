macro_rules! impl_is_within_bounds {
    (
        $ty: ident
        {$($component: ident => [$get_min: expr, $get_max: expr]),+}
        $(where $($where: tt)+)?
    ) => {
        // add empty generics brackets
        impl_is_within_bounds!($ty<> {$($component => [$get_min, $get_max]),+} $(where $($where)+)?);
    };
    (
        $ty: ident <$($ty_param: ident),*>
        {$($component: ident => [$get_min: expr, $get_max: expr]),+}
        $(where $($where: tt)+)?
    ) => {
        impl<$($ty_param,)* T> crate::IsWithinBounds for $ty<$($ty_param,)* T>
        where
            T: crate::num::PartialCmp,
            T::Mask: core::ops::BitAnd<Output = T::Mask>,
            $($($where)+)?
        {
            #[inline]
            fn is_within_bounds(&self) -> T::Mask {
                $(
                    self.$component.gt_eq(&$get_min)
                    & Option::from($get_max).map_or(crate::BoolMask::from_bool(true), |max|self.$component.lt_eq(&max))
                )&+
            }
        }
    };
}

macro_rules! impl_is_within_bounds_hwb {
    (
        $ty: ident
        $(where $($where: tt)+)?
    ) => {
        // add empty generics brackets
        impl_is_within_bounds_hwb!($ty<> $(where $($where)+)?);
    };
    (
        $ty: ident <$($ty_param: ident),*>
        $(where $($where: tt)+)?
    ) => {
        impl<$($ty_param,)* T> crate::IsWithinBounds for $ty<$($ty_param,)* T>
        where
            T: crate::num::PartialCmp + core::ops::Add<Output = T> + Clone,
            T::Mask: core::ops::BitAnd<Output = T::Mask>,
            $($($where)+)?
        {
            #[inline]
            fn is_within_bounds(&self) -> T::Mask {
                self.blackness.gt_eq(&Self::min_blackness()) & self.blackness.lt_eq(&Self::max_blackness()) &
                self.whiteness.gt_eq(&Self::min_whiteness()) & self.whiteness.lt_eq(&Self::max_blackness()) &
                (self.whiteness.clone() + self.blackness.clone()).lt_eq(&T::max_intensity())
            }
        }
    };
}
