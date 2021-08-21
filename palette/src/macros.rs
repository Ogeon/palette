#[cfg(test)]
macro_rules! raw_pixel_conversion_tests {
    ($name: ident <$($ty_param: path),+> : $($component: ident),+) => {
        #[test]
        fn convert_from_f32_array() {
            raw_pixel_conversion_tests!(@float_array_test f32, $name<$($ty_param),+>: $($component),+);
        }

        #[test]
        fn convert_from_f64_array() {
            raw_pixel_conversion_tests!(@float_array_test f64, $name<$($ty_param),+>: $($component),+);
        }

        #[test]
        fn convert_from_f32_slice() {
            raw_pixel_conversion_tests!(@float_slice_test f32, $name<$($ty_param),+>: $($component),+);
        }

        #[test]
        fn convert_from_f64_slice() {
            raw_pixel_conversion_tests!(@float_slice_test f64, $name<$($ty_param),+>: $($component),+);
        }
    };

    (@float_array_test $float: ty, $name: ident <$($ty_param: path),+> : $($component: ident),+) => {
        use crate::cast::ArrayCast;
        use crate::Alpha;

        let mut counter: $float = 0.0;
        $(
            counter += 0.1;
            let $component = counter;
        )+
        let alpha = counter + 0.1;

        let raw: <$name<$($ty_param,)+ $float> as ArrayCast>::Array = [$($component),+];
        let raw_plus_1: <Alpha<$name<$($ty_param,)+ $float>, $float> as ArrayCast>::Array = [
            $($component,)+
            alpha
        ];
        let color: $name<$($ty_param,)+ $float> = crate::cast::from_array(raw);

        let color_alpha: Alpha<$name<$($ty_param,)+ $float>, $float> = crate::cast::from_array(raw_plus_1);

        assert_eq!(color, $name::new($($component),+));

        assert_eq!(color_alpha, Alpha::<$name<$($ty_param,)+ $float>, $float>::new($($component,)+ alpha));
    };

    (@float_slice_test $float: ty, $name: ident <$($ty_param: path),+> : $($component: ident),+) => {
        use core::convert::{TryInto, TryFrom};
        use crate::Alpha;

        let mut counter: $float = 0.0;
        $(
            counter += 0.1;
            let $component = counter;
        )+
        let alpha = counter + 0.1;
        let extra = counter + 0.2;
        let raw: &[$float] = &[$($component),+];
        let raw_plus_1: &[$float] = &[
            $($component,)+
            alpha
        ];
        let raw_plus_2: &[$float] = &[
            $($component,)+
            alpha,
            extra
        ];
        let color: &$name<$($ty_param,)+ $float> = raw.try_into().unwrap();
        assert!(<&$name<$($ty_param,)+ $float>>::try_from(raw_plus_1).is_err());

        let color_alpha: &Alpha<$name<$($ty_param,)+ $float>, $float> = raw_plus_1.try_into().unwrap();
        assert!(<&Alpha<$name<$($ty_param,)+ $float>, $float>>::try_from(raw_plus_2).is_err());

        assert_eq!(color, &$name::new($($component),+));

        assert_eq!(color_alpha, &Alpha::<$name<$($ty_param,)+ $float>, $float>::new($($component,)+ alpha));
    };
}

#[cfg(test)]
macro_rules! raw_pixel_conversion_fail_tests {
    ($name: ident <$($ty_param: path),+> : $($component: ident),+) => {
        #[test]
        #[should_panic(expected = "TryFromSliceError")]
        fn convert_from_short_f32_slice() {
            raw_pixel_conversion_fail_tests!(@float_slice_test f32, $name<$($ty_param),+>);
        }

        #[test]
        #[should_panic(expected = "TryFromSliceError")]
        fn convert_from_short_f64_slice() {
            raw_pixel_conversion_fail_tests!(@float_slice_test f64, $name<$($ty_param),+>);
        }
    };

    (@float_slice_test $float: ty, $name: ident <$($ty_param: path),+>) => {
        use core::convert::TryInto;

        let raw: &[$float] = &[0.1];
        let _: &$name<$($ty_param,)+ $float> = raw.try_into().unwrap();
    };
}

#[cfg(all(test, feature = "random"))]
macro_rules! assert_uniform_distribution {
    ($bins:expr) => {{
        let bins = &$bins;

        for (i, &bin) in bins.iter().enumerate() {
            if bin < 5 {
                panic!("{}[{}] < 5: {:?}", stringify!($bins), i, bins);
            }
        }
        const P_LIMIT: f64 = 0.01; // Keeping it low to account for the RNG noise
        let p_value = crate::random_sampling::test_utils::uniform_distribution_test(bins);
        if p_value < P_LIMIT {
            panic!(
                "distribution of {} is not uniform enough (p-value {} < {}): {:?}",
                stringify!($bins),
                p_value,
                P_LIMIT,
                bins
            );
        }
    }};
}

#[cfg(all(test, feature = "random"))]
macro_rules! test_uniform_distribution {
    (
        $ty:path $(as $base_ty:path)? {
            $($component:ident: ($component_min:expr, $component_max:expr)),+$(,)?
        },
        min: $min:expr,
        max: $max:expr$(,)?
    ) => {
        #[cfg(feature = "random")]
        #[test]
        fn uniform_distribution_rng_gen() {
            use rand::Rng;

            const BINS: usize = crate::random_sampling::test_utils::BINS;
            const SAMPLES: usize = crate::random_sampling::test_utils::SAMPLES;

            $(let mut $component = [0; BINS];)+

            let mut rng = rand_mt::Mt::new(1234); // We want the same seed on every run to avoid random fails

            for _ in 0..SAMPLES {
                let color: $ty = rng.gen();
                $(let color: $base_ty = crate::convert::IntoColorUnclamped::into_color_unclamped(color);)?

                if $(color.$component < $component_min || color.$component > $component_max)||+ {
                    continue;
                }

                $({
                    let min: f32 = $component_min;
                    let max: f32 = $component_max;
                    let range = max - min;
                    let normalized = (color.$component - min) / range;
                    $component[((normalized * BINS as f32) as usize).min(BINS - 1)] += 1;
                })+
            }

            $(assert_uniform_distribution!($component);)+
        }

        #[cfg(feature = "random")]
        #[test]
        fn uniform_distribution_uniform_sample() {
            use rand::distributions::uniform::Uniform;
            use rand::Rng;

            const BINS: usize = crate::random_sampling::test_utils::BINS;
            const SAMPLES: usize = crate::random_sampling::test_utils::SAMPLES;

            $(let mut $component = [0; BINS];)+

            let mut rng = rand_mt::Mt::new(1234); // We want the same seed on every run to avoid random fails
            let uniform_sampler = Uniform::new($min, $max);

            for _ in 0..SAMPLES {
                let color: $ty = rng.sample(&uniform_sampler);
                $(let color: $base_ty = crate::convert::IntoColorUnclamped::into_color_unclamped(color);)?

                if $(color.$component < $component_min || color.$component > $component_max)||+ {
                    continue;
                }

                $({
                    let min: f32 = $component_min;
                    let max: f32 = $component_max;
                    let range = max - min;
                    let normalized = (color.$component - min) / range;
                    $component[((normalized * BINS as f32) as usize).min(BINS - 1)] += 1;
                })+
            }

            $(assert_uniform_distribution!($component);)+
        }

        #[cfg(feature = "random")]
        #[test]
        fn uniform_distribution_uniform_sample_inclusive() {
            use rand::distributions::uniform::Uniform;
            use rand::Rng;

            const BINS: usize = crate::random_sampling::test_utils::BINS;
            const SAMPLES: usize = crate::random_sampling::test_utils::SAMPLES;

            $(let mut $component = [0; BINS];)+

            let mut rng = rand_mt::Mt::new(1234); // We want the same seed on every run to avoid random fails
            let uniform_sampler = Uniform::new_inclusive($min, $max);

            for _ in 0..SAMPLES {
                let color: $ty = rng.sample(&uniform_sampler);
                $(let color: $base_ty = crate::convert::IntoColorUnclamped::into_color_unclamped(color);)?

                if $(color.$component < $component_min || color.$component > $component_max)||+ {
                    continue;
                }

                $({
                    let min: f32 = $component_min;
                    let max: f32 = $component_max;
                    let range = max - min;
                    let normalized = (color.$component - min) / range;
                    $component[((normalized * BINS as f32) as usize).min(BINS - 1)] += 1;
                })+
            }

            $(assert_uniform_distribution!($component);)+
        }
    };
}

macro_rules! impl_color_add {
    ($self_ty: ident < $phantom_ty: ident, $component_ty: ident > , [$($element: ident),+], $phantom: ident) => {
        impl<$phantom_ty, $component_ty> Add<Self> for $self_ty<$phantom_ty, $component_ty>
        where
            T: Add<Output=$component_ty>
        {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                $self_ty {
                    $( $element: self.$element + other.$element, )+
                    $phantom: PhantomData,
                }
            }
        }

        impl<$phantom_ty, $component_ty> Add<$component_ty> for $self_ty<$phantom_ty, $component_ty>
        where
            T: Add<Output=$component_ty> + Clone
        {
            type Output = Self;

            fn add(self, c: $component_ty) -> Self::Output {
                $self_ty {
                    $( $element: self.$element + c.clone(), )+
                    $phantom: PhantomData,
                }
            }
        }

        impl<$phantom_ty, $component_ty> AddAssign<Self> for $self_ty<$phantom_ty, $component_ty>
        where
            T: AddAssign,
        {
            fn add_assign(&mut self, other: Self) {
                $( self.$element += other.$element; )+
            }
        }

        impl<$phantom_ty, $component_ty> AddAssign<$component_ty> for $self_ty<$phantom_ty, $component_ty>
        where
            T:  AddAssign + Clone
        {
            fn add_assign(&mut self, c: $component_ty) {
                $( self.$element += c.clone(); )+
            }
        }
    };
    ($self_ty: ident < $component_ty: ident > , [$($element: ident),+]) => {
        impl<$component_ty> Add<Self> for $self_ty<$component_ty>
        where
            T: Add<Output=$component_ty>
        {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                $self_ty {
                    $( $element: self.$element + other.$element, )+
                }
            }
        }

        impl<$component_ty> Add<$component_ty> for $self_ty<$component_ty>
        where
            T: Add<Output=$component_ty> + Clone
        {
            type Output = Self;

            fn add(self, c: $component_ty) -> Self::Output {
                $self_ty {
                    $( $element: self.$element + c.clone(), )+
                }
            }
        }

        impl<$component_ty> AddAssign<Self> for $self_ty<$component_ty>
        where
            T: AddAssign,
        {
            fn add_assign(&mut self, other: Self) {
                $( self.$element += other.$element; )+
            }
        }

        impl<$component_ty> AddAssign<$component_ty> for $self_ty<$component_ty>
        where
            T:  AddAssign + Clone
        {
            fn add_assign(&mut self, c: $component_ty) {
                $( self.$element += c.clone(); )+
            }
        }
    };
}

/// Implement `Sub` and `SubAssign` traits for a color space.
///
/// Both scalars and color arithmetic are implemented.
macro_rules! impl_color_sub {
    ($self_ty: ident < $phantom_ty: ident, $component_ty: ident > , [$($element: ident),+], $phantom: ident) => {
        impl<$phantom_ty, $component_ty> Sub<Self> for $self_ty<$phantom_ty, $component_ty>
        where
            T: Sub<Output=$component_ty>
        {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                $self_ty {
                    $( $element: self.$element - other.$element, )+
                    $phantom: PhantomData,
                }
            }
        }

        impl<$phantom_ty, $component_ty> Sub<$component_ty> for $self_ty<$phantom_ty, $component_ty>
        where
            T: Sub<Output=$component_ty> + Clone
        {
            type Output = Self;

            fn sub(self, c: $component_ty) -> Self::Output {
                $self_ty {
                    $( $element: self.$element - c.clone(), )+
                    $phantom: PhantomData,
                }
            }
        }

        impl<$phantom_ty, $component_ty> SubAssign<Self> for $self_ty<$phantom_ty, $component_ty>
        where
            T: SubAssign,
        {
            fn sub_assign(&mut self, other: Self) {
                $( self.$element -= other.$element; )+
            }
        }

        impl<$phantom_ty, $component_ty> SubAssign<$component_ty> for $self_ty<$phantom_ty, $component_ty>
        where
            T:  SubAssign + Clone
        {
            fn sub_assign(&mut self, c: $component_ty) {
                $( self.$element -= c.clone(); )+
            }
        }
    };

    ($self_ty: ident < $component_ty: ident > , [$($element: ident),+]) => {
        impl<$component_ty> Sub<Self> for $self_ty<$component_ty>
        where
            T: Sub<Output=$component_ty>
        {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                $self_ty {
                    $( $element: self.$element - other.$element, )+
                }
            }
        }

        impl<$component_ty> Sub<$component_ty> for $self_ty<$component_ty>
        where
            T: Sub<Output=$component_ty> + Clone
        {
            type Output = Self;

            fn sub(self, c: $component_ty) -> Self::Output {
                $self_ty {
                    $( $element: self.$element - c.clone(), )+
                }
            }
        }

        impl<$component_ty> SubAssign<Self> for $self_ty<$component_ty>
        where
            T: SubAssign,
        {
            fn sub_assign(&mut self, other: Self) {
                $( self.$element -= other.$element; )+
            }
        }

        impl<$component_ty> SubAssign<$component_ty> for $self_ty<$component_ty>
        where
            T:  SubAssign + Clone
        {
            fn sub_assign(&mut self, c: $component_ty) {
                $( self.$element -= c.clone(); )+
            }
        }
    };
}

/// Implement `Mul` and `MulAssign` traits for a color space.
///
/// Both scalars and color arithmetic are implemented.
macro_rules! impl_color_mul {
    ($self_ty: ident < $phantom_ty: ident, $component_ty: ident > , [$($element: ident),+], $phantom: ident) => {
        impl<$phantom_ty, $component_ty> Mul<Self> for $self_ty<$phantom_ty, $component_ty>
        where
            T: Mul<Output=$component_ty>
        {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                $self_ty {
                    $( $element: self.$element * other.$element, )+
                    $phantom: PhantomData,
                }
            }
        }

        impl<$phantom_ty, $component_ty> Mul<$component_ty> for $self_ty<$phantom_ty, $component_ty>
        where
            T: Mul<Output=$component_ty> + Clone
        {
            type Output = Self;

            fn mul(self, c: $component_ty) -> Self::Output {
                $self_ty {
                    $( $element: self.$element * c.clone(), )+
                    $phantom: PhantomData,
                }
            }
        }

        impl<$phantom_ty, $component_ty> MulAssign<Self> for $self_ty<$phantom_ty, $component_ty>
        where
            T: MulAssign,
        {
            fn mul_assign(&mut self, other: Self) {
                $( self.$element *= other.$element; )+
            }
        }

        impl<$phantom_ty, $component_ty> MulAssign<$component_ty> for $self_ty<$phantom_ty, $component_ty>
        where
            T:  MulAssign + Clone
        {
            fn mul_assign(&mut self, c: $component_ty) {
                $( self.$element *= c.clone(); )+
            }
        }
    };
    ($self_ty: ident < $component_ty: ident > , [$($element: ident),+]) => {
        impl<$component_ty> Mul<Self> for $self_ty<$component_ty>
        where
            T: Mul<Output=$component_ty>
        {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                $self_ty {
                    $( $element: self.$element * other.$element, )+
                }
            }
        }

        impl<$component_ty> Mul<$component_ty> for $self_ty<$component_ty>
        where
            T: Mul<Output=$component_ty> + Clone
        {
            type Output = Self;

            fn mul(self, c: $component_ty) -> Self::Output {
                $self_ty {
                    $( $element: self.$element * c.clone(), )+
                }
            }
        }

        impl<$component_ty> MulAssign<Self> for $self_ty<$component_ty>
        where
            T: MulAssign,
        {
            fn mul_assign(&mut self, other: Self) {
                $( self.$element *= other.$element; )+
            }
        }

        impl<$component_ty> MulAssign<$component_ty> for $self_ty<$component_ty>
        where
            T:  MulAssign + Clone
        {
            fn mul_assign(&mut self, c: $component_ty) {
                $( self.$element *= c.clone(); )+
            }
        }
    };
}

/// Implement `Div` and `DivAssign` traits for a color space.
///
/// Both scalars and color arithmetic are implemented.
macro_rules! impl_color_div {
    ($self_ty: ident < $phantom_ty: ident, $component_ty: ident > , [$($element: ident),+], $phantom: ident) => {
        impl<$phantom_ty, $component_ty> Div<Self> for $self_ty<$phantom_ty, $component_ty>
        where
            T: Div<Output=$component_ty>
        {
            type Output = Self;

            fn div(self, other: Self) -> Self::Output {
                $self_ty {
                    $( $element: self.$element / other.$element, )+
                    $phantom: PhantomData,
                }
            }
        }

        impl<$phantom_ty, $component_ty> Div<$component_ty> for $self_ty<$phantom_ty, $component_ty>
        where
            T: Div<Output=$component_ty> + Clone
        {
            type Output = Self;

            fn div(self, c: $component_ty) -> Self::Output {
                $self_ty {
                    $( $element: self.$element / c.clone(), )+
                    $phantom: PhantomData,
                }
            }
        }

        impl<$phantom_ty, $component_ty> DivAssign<Self> for $self_ty<$phantom_ty, $component_ty>
        where
            T: DivAssign,
        {
            fn div_assign(&mut self, other: Self) {
                $( self.$element /= other.$element; )+
            }
        }

        impl<$phantom_ty, $component_ty> DivAssign<$component_ty> for $self_ty<$phantom_ty, $component_ty>
        where
            T:  DivAssign + Clone
        {
            fn div_assign(&mut self, c: $component_ty) {
                $( self.$element /= c.clone(); )+
            }
        }
    };
    ($self_ty: ident < $component_ty: ident > , [$($element: ident),+]) => {
        impl<$component_ty> Div<Self> for $self_ty<$component_ty>
        where
            T: Div<Output=$component_ty>
        {
            type Output = Self;

            fn div(self, other: Self) -> Self::Output {
                $self_ty {
                    $( $element: self.$element / other.$element, )+
                }
            }
        }

        impl<$component_ty> Div<$component_ty> for $self_ty<$component_ty>
        where
            T: Div<Output=$component_ty> + Clone
        {
            type Output = Self;

            fn div(self, c: $component_ty) -> Self::Output {
                $self_ty {
                    $( $element: self.$element / c.clone(), )+
                }
            }
        }

        impl<$component_ty> DivAssign<Self> for $self_ty<$component_ty>
        where
            T: DivAssign,
        {
            fn div_assign(&mut self, other: Self) {
                $( self.$element /= other.$element; )+
            }
        }

        impl<$component_ty> DivAssign<$component_ty> for $self_ty<$component_ty>
        where
            T:  DivAssign + Clone
        {
            fn div_assign(&mut self, c: $component_ty) {
                $( self.$element /= c.clone(); )+
            }
        }
    };
}

macro_rules! impl_array_casts {
    ($self_ty: ident < $($ty_param: ident),+ > $($rest: tt)*) => {
        impl_array_casts!([$($ty_param),+] $self_ty < $($ty_param),+ > $($rest)*);
    };
    ([$($ty_param: tt)+] $self_ty: ident < $($self_ty_param: ty),+ > , [$array_item: ty; $array_len: expr] $(, where $($where: tt)+)?) => {
        impl<$($ty_param)+> AsRef<[$array_item; $array_len]> for $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn as_ref(&self) -> &[$array_item; $array_len] {
                crate::cast::into_array_ref(self)
            }
        }

        impl<$($ty_param)+> AsRef<$self_ty<$($self_ty_param),+>> for [$array_item; $array_len]
        $(where $($where)+)?
        {
            #[inline]
            fn as_ref(&self) -> &$self_ty<$($self_ty_param),+> {
                crate::cast::from_array_ref(self)
            }
        }

        impl<$($ty_param)+> AsMut<[$array_item; $array_len]> for $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn as_mut(&mut self) -> &mut [$array_item; $array_len] {
                crate::cast::into_array_mut(self)
            }
        }

        impl<$($ty_param)+> AsMut<$self_ty<$($self_ty_param),+>> for [$array_item; $array_len]
        $(where $($where)+)?
        {
            #[inline]
            fn as_mut(&mut self) -> &mut $self_ty<$($self_ty_param),+> {
                crate::cast::from_array_mut(self)
            }
        }

        impl<$($ty_param)+> AsRef<[$array_item]> for $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn as_ref(&self) -> &[$array_item] {
                &*AsRef::<[$array_item; $array_len]>::as_ref(self)
            }
        }

        impl<$($ty_param)+> AsMut<[$array_item]> for $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn as_mut(&mut self) -> &mut [$array_item] {
                &mut *AsMut::<[$array_item; $array_len]>::as_mut(self)
            }
        }

        impl<$($ty_param)+> From<$self_ty<$($self_ty_param),+>> for [$array_item; $array_len]
        $(where $($where)+)?
        {
            #[inline]
            fn from(color: $self_ty<$($self_ty_param),+>) -> Self {
                crate::cast::into_array(color)
            }
        }

        impl<$($ty_param)+> From<[$array_item; $array_len]> for $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn from(array: [$array_item; $array_len]) -> Self {
                crate::cast::from_array(array)
            }
        }

        impl<'a, $($ty_param)+> From<&'a $self_ty<$($self_ty_param),+>> for &'a [$array_item; $array_len]
        $(where $($where)+)?
        {
            #[inline]
            fn from(color: &'a $self_ty<$($self_ty_param),+>) -> Self {
                color.as_ref()
            }
        }

        impl<'a, $($ty_param)+> From<&'a [$array_item; $array_len]> for &'a $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn from(array: &'a [$array_item; $array_len]) -> Self{
                array.as_ref()
            }
        }

        impl<'a, $($ty_param)+> From<&'a $self_ty<$($self_ty_param),+>> for &'a [$array_item]
        $(where $($where)+)?
        {
            #[inline]
            fn from(color: &'a $self_ty<$($self_ty_param),+>) -> Self {
                color.as_ref()
            }
        }

        impl<'a, $($ty_param)+> core::convert::TryFrom<&'a [$array_item]> for &'a $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            type Error = <&'a [$array_item; $array_len] as core::convert::TryFrom<&'a [$array_item]>>::Error;

            #[inline]
            fn try_from(slice: &'a [$array_item]) -> Result<Self, Self::Error> {
                use core::convert::TryInto;

                slice.try_into().map(crate::cast::from_array_ref)
            }
        }

        impl<'a, $($ty_param)+> From<&'a mut $self_ty<$($self_ty_param),+>> for &'a mut [$array_item; $array_len]
        $(where $($where)+)?
        {
            #[inline]
            fn from(color: &'a mut $self_ty<$($self_ty_param),+>) -> Self {
                color.as_mut()
            }
        }

        impl<'a, $($ty_param)+> From<&'a mut [$array_item; $array_len]> for &'a mut $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn from(array: &'a mut [$array_item; $array_len]) -> Self{
                array.as_mut()
            }
        }

        impl<'a, $($ty_param)+> From<&'a mut $self_ty<$($self_ty_param),+>> for &'a mut [$array_item]
        $(where $($where)+)?
        {
            #[inline]
            fn from(color: &'a mut $self_ty<$($self_ty_param),+>) -> Self {
                color.as_mut()
            }
        }

        impl<'a, $($ty_param)+> core::convert::TryFrom<&'a mut [$array_item]> for &'a mut $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            type Error = <&'a mut [$array_item; $array_len] as core::convert::TryFrom<&'a mut [$array_item]>>::Error;

            #[inline]
            fn try_from(slice: &'a mut [$array_item]) -> Result<Self, Self::Error> {
                use core::convert::TryInto;

                slice.try_into().map(crate::cast::from_array_mut)
            }
        }

        #[cfg(feature = "std")]
        impl<$($ty_param)+> From<Box<$self_ty<$($self_ty_param),+>>> for Box<[$array_item; $array_len]>
        $(where $($where)+)?
        {
            #[inline]
            fn from(color: Box<$self_ty<$($self_ty_param),+>>) -> Self {
                crate::cast::into_array_box(color)
            }
        }

        #[cfg(feature = "std")]
        impl<$($ty_param)+> From<Box<[$array_item; $array_len]>> for Box<$self_ty<$($self_ty_param),+>>
        $(where $($where)+)?
        {
            #[inline]
            fn from(array: Box<[$array_item; $array_len]>) -> Self{
                crate::cast::from_array_box(array)
            }
        }
    }
}

macro_rules! impl_uint_casts_self {
    ($self_ty: ident < $($ty_param: ident),+ > $($rest: tt)*) => {
        impl_uint_casts_self!([$($ty_param),+] $self_ty < $($ty_param),+ > $($rest)*);
    };
    ([$($ty_param: tt)+] $self_ty: ident < $($self_ty_param: ty),+ >, $uint: ty $(, where $($where: tt)+)?) => {
        impl<$($ty_param)+> AsRef<$uint> for $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn as_ref(&self) -> &$uint {
                crate::cast::into_uint_ref(self)
            }
        }

        impl<$($ty_param)+> AsMut<$uint> for $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn as_mut(&mut self) -> &mut $uint {
                crate::cast::into_uint_mut(self)
            }
        }

        impl<$($ty_param)+> From<$uint> for $self_ty<$($self_ty_param),+>
        $(where $($where)+)?
        {
            #[inline]
            fn from(uint: $uint) -> Self {
                crate::cast::from_uint(uint)
            }
        }

        impl<'a, $($ty_param)+> From<&'a $uint> for &'a $self_ty<$($self_ty_param),+>
        where
            $uint: AsRef<$self_ty<$($self_ty_param),+>> $(, $($where)+)?
        {
            #[inline]
            fn from(uint: &'a $uint) -> Self{
                uint.as_ref()
            }
        }

        impl<'a, $($ty_param)+> From<&'a mut $uint> for &'a mut $self_ty<$($self_ty_param),+>
        where
            $uint: AsMut<$self_ty<$($self_ty_param),+>> $(, $($where)+)?
        {
            #[inline]
            fn from(uint: &'a mut $uint) -> Self{
                uint.as_mut()
            }
        }
    }
}

macro_rules! impl_uint_casts_other {
    ($self_ty: ident < $($ty_param: ident),+ > $($rest: tt)*) => {
        impl_uint_casts_other!([$($ty_param),+] $self_ty < $($ty_param),+ > $($rest)*);
    };
    ([$($ty_param: ident),+] $self_ty: ident < $($self_ty_param: ty),+ >, $uint: ty $(, where $($where: tt)+)?) => {
        impl<$($ty_param)+> AsRef<$self_ty<$($self_ty_param),+>> for $uint
        $(where $($where)+)?
        {
            #[inline]
            fn as_ref(&self) -> &$self_ty<$($self_ty_param),+> {
                crate::cast::from_uint_ref(self)
            }
        }

        impl<$($ty_param)+> AsMut<$self_ty<$($self_ty_param),+>> for $uint
        $(where $($where)+)?
        {
            #[inline]
            fn as_mut(&mut self) -> &mut $self_ty<$($self_ty_param),+> {
                crate::cast::from_uint_mut(self)
            }
        }

        impl<$($ty_param)+> From<$self_ty<$($self_ty_param),+>> for $uint
        $(where $($where)+)?
        {
            #[inline]
            fn from(color: $self_ty<$($self_ty_param),+>) -> Self {
                crate::cast::into_uint(color)
            }
        }

        impl<'a, $($ty_param)+> From<&'a $self_ty<$($self_ty_param),+>> for &'a $uint
        $(where $($where)+)?
        {
            #[inline]
            fn from(color: &'a $self_ty<$($self_ty_param),+>) -> Self {
                color.as_ref()
            }
        }


        impl<'a, $($ty_param)+> From<&'a mut $self_ty<$($self_ty_param),+>> for &'a mut $uint
        $(where $($where)+)?
        {
            #[inline]
            fn from(color: &'a mut $self_ty<$($self_ty_param),+>) -> Self {
                color.as_mut()
            }
        }
    }
}
