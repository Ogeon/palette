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
        use crate::Pixel;
        use crate::Alpha;

        let mut counter: $float = 0.0;
        $(
            counter += 0.1;
            let $component = counter;
        )+
        let alpha = counter + 0.1;

        let raw: [$float; <$name<$($ty_param,)+ $float> as Pixel<$float>>::CHANNELS] = [$($component),+];
        let raw_plus_1: [$float; <$name<$($ty_param,)+ $float> as Pixel<$float>>::CHANNELS + 1] = [
            $($component,)+
            alpha
        ];
        let color: $name<$($ty_param,)+ $float> = *$name::from_raw(&raw);
        let color_long: $name<$($ty_param,)+ $float> = *$name::from_raw(&raw_plus_1);

        let color_alpha: Alpha<$name<$($ty_param,)+ $float>, $float> = *Alpha::<$name<$($ty_param,)+ $float>, $float>::from_raw(&raw_plus_1);

        assert_eq!(color, $name::new($($component),+));
        assert_eq!(color_long, $name::new($($component),+));

        assert_eq!(color_alpha, Alpha::<$name<$($ty_param,)+ $float>, $float>::new($($component,)+ alpha));
    };

    (@float_slice_test $float: ty, $name: ident <$($ty_param: path),+> : $($component: ident),+) => {
        use crate::Pixel;
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
        let color: $name<$($ty_param,)+ $float> = *$name::from_raw(raw);
        let color_long: $name<$($ty_param,)+ $float> = *$name::from_raw(raw_plus_1);

        let color_alpha: Alpha<$name<$($ty_param,)+ $float>, $float> = *Alpha::<$name<$($ty_param,)+ $float>, $float>::from_raw(raw_plus_1);
        let color_alpha_long: Alpha<$name<$($ty_param,)+ $float>, $float> = *Alpha::<$name<$($ty_param,)+ $float>, $float>::from_raw(raw_plus_2);

        assert_eq!(color, $name::new($($component),+));
        assert_eq!(color_long, $name::new($($component),+));

        assert_eq!(color_alpha, Alpha::<$name<$($ty_param,)+ $float>, $float>::new($($component,)+ alpha));
        assert_eq!(color_alpha_long, Alpha::<$name<$($ty_param,)+ $float>, $float>::new($($component,)+ alpha));
    };
}

#[cfg(test)]
macro_rules! raw_pixel_conversion_fail_tests {
    ($name: ident <$($ty_param: path),+> : $($component: ident),+) => {
        #[test]
        #[should_panic(expected = "not enough color channels")]
        fn convert_from_short_f32_array() {
            raw_pixel_conversion_fail_tests!(@float_array_test f32, $name<$($ty_param),+>);
        }

        #[test]
        #[should_panic(expected = "not enough color channels")]
        fn convert_from_short_f64_array() {
            raw_pixel_conversion_fail_tests!(@float_array_test f64, $name<$($ty_param),+>);
        }

        #[test]
        #[should_panic(expected = "not enough color channels")]
        fn convert_from_short_f32_slice() {
            raw_pixel_conversion_fail_tests!(@float_slice_test f32, $name<$($ty_param),+>);
        }

        #[test]
        #[should_panic(expected = "not enough color channels")]
        fn convert_from_short_f64_slice() {
            raw_pixel_conversion_fail_tests!(@float_slice_test f64, $name<$($ty_param),+>);
        }
    };

    (@float_array_test $float: ty, $name: ident <$($ty_param: path),+>) => {
        use crate::Pixel;
        let raw: [$float; 1] = [0.1];
        let _: $name<$($ty_param,)+ $float> = *$name::from_raw(&raw);
    };

    (@float_slice_test $float: ty, $name: ident <$($ty_param: path),+>) => {
        use crate::Pixel;
        let raw: &[$float] = &[0.1];
        let _: $name<$($ty_param,)+ $float> = *$name::from_raw(raw);
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
                    let min = $component_min;
                    let range = $component_max - min;
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
                    let min = $component_min;
                    let range = $component_max - min;
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
                    let min = $component_min;
                    let range = $component_max - min;
                    let normalized = (color.$component - min) / range;
                    $component[((normalized * BINS as f32) as usize).min(BINS - 1)] += 1;
                })+
            }

            $(assert_uniform_distribution!($component);)+
        }
    };
}

macro_rules! impl_color_add {
    ($self_ty: ident , [$($element: ident),+], $phantom: ident) => {
	impl<Wp, T> Add<$self_ty<Wp, T>> for $self_ty<Wp, T>
	where
	    T: FloatComponent,
	    Wp: WhitePoint,
	{
	    type Output = $self_ty<Wp, T>;

	    fn add(self, other: $self_ty<Wp, T>) -> Self::Output {
		$self_ty {
		    $( $element: self.$element + other.$element ),+,
		    $phantom: PhantomData,
		}
	    }
	}
	impl<Wp, T> Add<T> for $self_ty<Wp, T>
	where
	    T: FloatComponent,
	    Wp: WhitePoint,
	{
	    type Output = $self_ty<Wp, T>;

	    fn add(self, c: T) -> Self::Output {
		$self_ty {
		    $( $element: self.$element + c ),+,
		    $phantom: PhantomData,
		}
	    }
	}

	impl<Wp, T> AddAssign<$self_ty<Wp, T>> for $self_ty<Wp, T>
	where
	    T: FloatComponent + AddAssign,
	    Wp: WhitePoint,
	{
	    fn add_assign(&mut self, other: $self_ty<Wp, T>) {
		$( self.$element += other.$element );+
	    }
	}

	impl<Wp, T> AddAssign<T> for $self_ty<Wp, T>
	where
	    T: FloatComponent + AddAssign,
	    Wp: WhitePoint,
	{
	    fn add_assign(&mut self, c: T) {
		$( self.$element += c );+
	    }
	}
    }
}

/// Implement `Sub` and `SubAssign` traits for a color space.
///
/// Both scalars and color arithmetic are implemented.
macro_rules! impl_color_sub {
    ($self_ty: ident , [$($element: ident),+], $phantom: ident) => {

	impl<Wp, T> Sub<$self_ty<Wp, T>> for $self_ty<Wp, T>
	where
	    T: FloatComponent,
	    Wp: WhitePoint,
	{
	    type Output = $self_ty<Wp, T>;

	    fn sub(self, other: $self_ty<Wp, T>) -> Self::Output {
		$self_ty {
		    $( $element: self.$element - other.$element ),+,
		    $phantom: PhantomData,
		}
	    }
	}

	impl<Wp, T> Sub<T> for $self_ty<Wp, T>
	where
	    T: FloatComponent,
	    Wp: WhitePoint,
	{
	    type Output = $self_ty<Wp, T>;

	    fn sub(self, c: T) -> Self::Output {
		$self_ty {
		    $( $element: self.$element - c ),+,
		    $phantom: PhantomData,
		}
	    }
	}

	impl<Wp, T> SubAssign<$self_ty<Wp, T>> for $self_ty<Wp, T>
	where
	    T: FloatComponent + SubAssign,
	    Wp: WhitePoint,
	{
	    fn sub_assign(&mut self, other: $self_ty<Wp, T>) {
		$( self.$element -= other.$element; )+
	    }
	}

	impl<Wp, T> SubAssign<T> for $self_ty<Wp, T>
	where
	    T: FloatComponent + SubAssign,
	    Wp: WhitePoint,
	{
	    fn sub_assign(&mut self, c: T) {
		$( self.$element -= c; )+
	    }
	}
    }
}
