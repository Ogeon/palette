macro_rules! _impl_increase_value_trait {
    (
        $trait: ident :: {$method: ident, $method_fixed: ident},
        $assign_trait: ident :: {$assign_method: ident, $assign_method_fixed: ident},
        $ty: ident <$($ty_param: ident),*>
        increase {$($component: ident => [$get_min: expr, $get_max: expr]),+}
        other {$($other_component: ident),*}
        $(phantom: $phantom: ident)?
        $(where $($where: tt)+)?
    ) => {
        impl<$($ty_param,)* T> $trait for $ty<$($ty_param,)* T>
        where
            T: Real + Zero + MinMax + Arithmetics + PartialOrd + Clone,
            $($($where)+)?
        {
            type Scalar = T;

            #[inline]
            fn $method(self, factor: T) -> Self {
                $(
                    let difference = if factor >= T::zero() {
                       $get_max - &self.$component
                    } else {
                        self.$component.clone()
                    };

                    let $component = difference.max(T::zero()) * &factor;
                )+

                $ty {
                    $($other_component: self.$other_component,)*
                    $($component: clamp(self.$component + $component, $get_min, $get_max),)+
                    $($phantom: PhantomData,)?
                }
            }

            #[inline]
            fn $method_fixed(self, amount: T) -> Self {
                $ty {
                    $($other_component: self.$other_component,)*
                    $($component: clamp(self.$component + $get_max * &amount, $get_min, $get_max),)+
                    $($phantom: PhantomData,)?
                }
            }
        }

        impl<$($ty_param,)* T> $assign_trait for $ty<$($ty_param,)* T>
        where
            T: Real + Zero + MinMax + AddAssign + Arithmetics + PartialOrd + Clone,
            $($($where)+)?
        {
            type Scalar = T;

            #[inline]
            fn $assign_method(&mut self, factor: T) {
                $(
                    let difference = if factor >= T::zero() {
                        $get_max - &self.$component
                    } else {
                        self.$component.clone()
                    };

                    self.$component += difference.max(T::zero()) * &factor;
                    clamp_assign(&mut self.$component, $get_min, $get_max);
                )+
            }

            #[inline]
            fn $assign_method_fixed(&mut self, amount: T) {
                $(
                    self.$component += $get_max * &amount;
                    clamp_assign(&mut self.$component, $get_min, $get_max);
                )+
            }
        }
    };
}

macro_rules! impl_lighten {
    ($ty: ident increase $($input: tt)+) => {
        impl_lighten!($ty<> increase $($input)+);
    };
    ($($input: tt)+) => {
        _impl_increase_value_trait!(
            Lighten::{lighten, lighten_fixed},
            LightenAssign::{lighten_assign, lighten_fixed_assign},
            $($input)+
        );
    };
}

macro_rules! impl_saturate {
    ($ty: ident increase $($input: tt)+) => {
        impl_saturate!($ty<> increase $($input)+);
    };
    ($($input: tt)+) => {
        _impl_increase_value_trait!(
            Saturate::{saturate, saturate_fixed},
            SaturateAssign::{saturate_assign, saturate_fixed_assign},
            $($input)+
        );
    };
}
