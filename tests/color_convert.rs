#[macro_use]
extern crate approx;
extern crate rustc_serialize;
extern crate csv;
extern crate palette;


macro_rules! assert_color_eq {
    ($a:expr, $b:expr, [$($components:ident),+]) => ({
        $(
            let a: f32 = $a.$components.into();
            let b: f32 = $b.$components.into();
            assert_relative_eq!(a, b, epsilon = 0.05);
        )+
    })
}

// Check if the hue diff equal to zero
macro_rules! assert_color_hue_eq {
    ($a:expr, $b:expr, [$($components:ident),+], $eps:expr) => ({
        $(
            let out = ($a.$components - $b.$components).into();
            assert_relative_eq!(out, 0.0, epsilon = $eps);
        )+
    })
}

mod convert;
