#[cfg(test)]
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

#[cfg(test)]
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
