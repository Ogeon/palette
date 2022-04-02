macro_rules! impl_mix {
    ($ty: ident $(where $($where: tt)+)?) => {
        impl_mix!($ty<> $(where $($where)+)?);
    };
    ($ty: ident <$($ty_param: ident),*> $(where $($where: tt)+)?) => {
        impl<$($ty_param,)* T> Mix for $ty<$($ty_param,)* T>
        where
            T: Real + Zero + One + Arithmetics + num::Clamp + Clone,
            $($($where)+)?
        {
            type Scalar = T;

            #[inline]
            fn mix(self, other: Self, factor: T) -> Self {
                let factor = clamp(factor, T::zero(), T::one());
                self.clone() + (other - self) * factor
            }
        }

        impl<$($ty_param,)* T> MixAssign for $ty<$($ty_param,)* T>
        where
            T: Real + Zero + One + AddAssign + Arithmetics + num::Clamp + Clone,
            $($($where)+)?
        {
            type Scalar = T;

            #[inline]
            fn mix_assign(&mut self, other: Self, factor: T) {
                let factor = clamp(factor, T::zero(), T::one());
                *self += (other - self.clone()) * factor;
            }
        }
    };
}

macro_rules! impl_mix_hue {
    ($ty: ident {$($other_field: ident),*} $(phantom: $phantom: ident)?) => {
        impl_mix_hue!($ty<> {$($other_field),*} $(phantom: $phantom)?);
    };
    ($ty: ident <$($ty_param: ident),*> {$($other_field: ident),*} $(phantom: $phantom: ident)?) => {
        impl<$($ty_param,)* T> Mix for $ty<$($ty_param,)* T>
        where
            T: RealAngle + SignedAngle + Zero + One + num::Clamp + Arithmetics + Clone,
        {
            type Scalar = T;

            #[inline]
            fn mix(self, other: Self, factor: T) -> Self {
                let factor = clamp(factor, T::zero(), T::one());
                let hue = (other.hue - self.hue.clone()).into_degrees();
                $(
                    let $other_field = other.$other_field - &self.$other_field;
                )*

                $ty {
                    $(
                        $other_field: self.$other_field + $other_field * &factor,
                    )*
                    hue: self.hue + hue * factor,
                    $($phantom: PhantomData)?
                }
            }
        }

        impl<$($ty_param,)* T> MixAssign for $ty<$($ty_param,)* T>
        where
            T: RealAngle + SignedAngle + Zero + One + num::Clamp + AddAssign + Arithmetics + Clone,
        {
            type Scalar = T;

            #[inline]
            fn mix_assign(&mut self, other: Self, factor: T) {
                let factor = clamp(factor, T::zero(), T::one());
                let hue = (other.hue - self.hue.clone()).into_degrees();
                $(
                    let $other_field = other.$other_field - &self.$other_field;
                )*

                $(
                    self.$other_field += $other_field * &factor;
                )*
                self.hue += hue * factor;
            }
        }
    };
}
