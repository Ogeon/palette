#[cfg(test)]
macro_rules! raw_pixel_conversion_tests {
    ($name: ident <$($ty_param: ident),+> : $($component: ident),+) => {
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

    (@float_array_test $float: ty, $name: ident <$($ty_param: ident),+> : $($component: ident),+) => {
        use ::Pixel;
        use ::Alpha;

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

    (@float_slice_test $float: ty, $name: ident <$($ty_param: ident),+> : $($component: ident),+) => {
        use ::Pixel;
        use ::Alpha;

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
    ($name: ident <$($ty_param: ident),+> : $($component: ident),+) => {
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

    (@float_array_test $float: ty, $name: ident <$($ty_param: ident),+>) => {
        use ::Pixel;
        let raw: [$float; 1] = [0.1];
        let _: $name<$($ty_param,)+ $float> = *$name::from_raw(&raw);
    };

    (@float_slice_test $float: ty, $name: ident <$($ty_param: ident),+>) => {
        use ::Pixel;
        let raw: &[$float] = &[0.1];
        let _: $name<$($ty_param,)+ $float> = *$name::from_raw(raw);
    };
}
