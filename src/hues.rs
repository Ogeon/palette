use num::traits::Float;

use std::f64::consts::PI;
use std::cmp::PartialEq;
use std::ops::{Add, Sub};

macro_rules! make_hues {
    ($($(#[$doc:meta])+ struct $name:ident;)+) => ($(
        $(#[$doc])+
        ///
        ///The hue is a circular type, where `0` and `360` is the same, and
        ///it's normalized to `(-180, +180]` when it's converted to a linear
        ///number (like `f32`). This makes many calculations easier, but may
        ///also have some surprising effects if it's expected to act as a
        ///linear number.
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name<T:Float>(T);

        impl<T:Float> $name<T> {
            ///Create a new hue from radians, instead of degrees.
            pub fn from_radians<T:Float>(radians: T) -> $name {
                $name(radians * T::from(180.0).unwrap() / PI)
            }

            ///Convert the hue to radians.
            pub fn to_radians<T: Float>(self) -> T {
                normalize_angle(self.0) * PI / T::from(180.0).unwrap()
            }
        }

        impl<T:Float> From<T> for $name<T> {
            fn from(degrees: f32) -> $name<T> {
                $name(degrees)
            }
        }

        impl<T:Float> Into<T> for $name<T> {
            fn into(self) -> T {
                normalize_angle(self.0)
            }
        }

        impl<T:Float> PartialEq for $name<T> {
            fn eq(&self, other: &$name<T>) -> bool {
                let hue_s: T = (*self).into();
                let hue_o: T = (*other).into();
                hue_s.eq(&hue_o)
            }
        }

        impl<T:Float> PartialEq for $name<T> {
            fn eq(&self, other: &T) -> bool {
                let hue: T = (*self).into();
                hue.eq(&normalize_angle(*other))
            }
        }

        impl<T:Float> PartialEq<$name<T>> for T {
            fn eq(&self, other: &$name<T>) -> bool {
                other.eq(self)
            }
        }

        impl<T:Float> Add<$name<T>> for $name<T> {
            type Output = $name<T>;

            fn add(self, other: $name<T>) -> $name<T> {
                $name(self.0 + other.0)
            }
        }

        impl<T:Float> Add<T> for $name<T> {
            type Output = $name<T>;

            fn add(self, other: T) -> $name<T> {
                $name(self.0 + other)
            }
        }

        impl<T:Float> Sub<$name<T>> for $name<T> {
            type Output = $name<T>;

            fn sub(self, other: $name<T>) -> $name<T> {
                $name(self.0 - other.0)
            }
        }

        impl<T:Float> Sub<T> for $name<T> {
            type Output = $name<T>;

            fn sub(self, other: T) -> $name<T> {
                $name(self.0 - other)
            }
        }
    )+)
}

make_hues! {
    ///A hue type for the CIE L*a*b* family of color spaces.
    ///
    ///It's measured in degrees and it's based on the four physiological
    ///elementary colors _red_, _yellow_, _green_ and _blue_. This makes it
    ///different from the hue of RGB based color spaces.
    struct LabHue;

    ///A hue type for the RGB family of color spaces.
    ///
    ///It's measured in degrees and uses the three additive primaries _red_,
    ///_green_ and _blue_.
    struct RgbHue;
}

fn normalize_angle<T: Float>(mut deg: T) -> T {
    while deg > T::from(180.0).unwrap() {
        deg -= T::from(360.0).unwrap();
    }

    while deg <= -T::from(180.0).unwrap() {
        deg += T::from(360.0).unwrap();
    }

    deg
}
