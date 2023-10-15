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

macro_rules! _clamp_value {
    ($value: expr, $min: expr) => {
        crate::clamp_min($value, $min)
    };
    ($value: expr, $min: expr, $max: expr) => {
        crate::clamp($value, $min, $max)
    };
    (@assign $value: expr, $min: expr) => {
        crate::clamp_min_assign($value, $min)
    };
    (@assign $value: expr, $min: expr, $max: expr) => {
        crate::clamp_assign($value, $min, $max)
    };
}

macro_rules! impl_clamp {
    (
        $ty: ident
        {$($component: ident => [$get_min: expr $(, $get_max: expr)?]),+}
        $(other {$($other: ident),+})?
        $(where $($where: tt)+)?
    ) => {
        // add empty generics brackets
        impl_clamp!($ty<> {$($component => [$get_min $(, $get_max)?]),+} $(other {$($other),+})? $(where $($where)+)?);
    };
    (
        $ty: ident <$($ty_param: ident),*>
        {$($component: ident => [$get_min: expr $(, $get_max: expr)?]),+}
        $(other {$($other: ident),+})?
        $(where $($where: tt)+)?
    ) => {
        impl<$($ty_param,)* T> crate::Clamp for $ty<$($ty_param,)* T>
        where
            T: crate::num::Clamp,
            $($($where)+)?
        {
            #[inline]
            fn clamp(self) -> Self {
                Self {
                    $($component: _clamp_value!(self.$component, $get_min $(, $get_max)?),)+
                    $($($other: self.$other,)+)?
                }
            }
        }

        impl<$($ty_param,)* T> crate::ClampAssign for $ty<$($ty_param,)* T>
        where
            T: crate::num::ClampAssign,
            $($($where)+)?
        {
            #[inline]
            fn clamp_assign(&mut self) {
                $(_clamp_value!(@assign &mut self.$component, $get_min $(, $get_max)?);)+
            }
        }
    };
}

macro_rules! impl_clamp_hwb {
    (
        $ty: ident
        $(phantom: $phantom: ident)?
        $(where $($where: tt)+)?
    ) => {
        // add empty generics brackets
        impl_clamp_hwb!($ty<> $(phantom: $phantom)? $(where $($where)+)?);
    };
    (
        $ty: ident <$($ty_param: ident),*>
        $(phantom: $phantom: ident)?
        $(where $($where: tt)+)?
    ) => {
        impl<$($ty_param,)* T> crate::Clamp for $ty<$($ty_param,)* T>
        where
            T: crate::num::One
                + crate::num::Clamp
                + crate::num::PartialCmp
                + core::ops::Add<Output = T>
                + core::ops::DivAssign
                + Clone,
            T::Mask: crate::bool_mask::Select<T>,
            $($($where)+)?
        {
            #[inline]
            fn clamp(self) -> Self {
                let mut whiteness = crate::clamp_min(self.whiteness.clone(), Self::min_whiteness());
                let mut blackness = crate::clamp_min(self.blackness.clone(), Self::min_blackness());

                let sum = self.blackness + self.whiteness;
                let divisor = sum.gt(&T::max_intensity()).select(sum, T::one());
                whiteness /= divisor.clone();
                blackness /= divisor;

                Self {hue: self.hue, whiteness, blackness $(, $phantom: self.$phantom)?}
            }
        }

        impl<$($ty_param,)* T> crate::ClampAssign for $ty<$($ty_param,)* T>
        where
            T: crate::num::One
                + crate::num::ClampAssign
                + crate::num::PartialCmp
                + core::ops::Add<Output = T>
                + core::ops::DivAssign
                + Clone,
            T::Mask: crate::bool_mask::Select<T>,
            $($($where)+)?
        {
            #[inline]
            fn clamp_assign(&mut self) {
                crate::clamp_min_assign(&mut self.whiteness, Self::min_whiteness());
                crate::clamp_min_assign(&mut self.blackness, Self::min_blackness());

                let sum = self.blackness.clone() + self.whiteness.clone();
                let divisor = sum.gt(&T::max_intensity()).select(sum, T::one());
                self.whiteness /= divisor.clone();
                self.blackness /= divisor;
            }
        }
    };
}
