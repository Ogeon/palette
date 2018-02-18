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

        let mut counter: $float = 0.0;
        $(
            counter += 0.1;
            let $component = counter;
        )+
        let raw: [$float; <$name<$($ty_param,)+ $float> as Pixel<$float>>::CHANNELS] = [$($component),+];
        let color: $name<$($ty_param,)+ $float> = *$name::from_raw(&raw);

        assert_eq!(color, $name::new($($component),+));
    };

    (@float_slice_test $float: ty, $name: ident <$($ty_param: ident),+> : $($component: ident),+) => {
        use ::Pixel;

        let mut counter: $float = 0.0;
        $(
            counter += 0.1;
            let $component = counter;
        )+
        let raw: &[$float] = &[$($component),+];
        let color: $name<$($ty_param,)+ $float> = *$name::from_raw(raw);

        assert_eq!(color, $name::new($($component),+));
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
