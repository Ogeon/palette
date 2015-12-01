use std::f32::consts::PI;
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
        pub struct $name(f32);

        impl $name {
            ///Create a new hue from radians, instead of degrees.
            pub fn from_radians(radians: f32) -> $name {
                $name(radians * 180.0 / PI)
            }

            ///Convert the hue to radians.
            pub fn to_radians(self) -> f32 {
                normalize_angle(self.0) * PI / 180.0
            }
        }

        impl From<f32> for $name {
            fn from(degrees: f32) -> $name {
                $name(degrees)
            }
        }

        impl Into<f32> for $name {
            fn into(self) -> f32 {
                normalize_angle(self.0)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &$name) -> bool {
                let hue_s: f32 = (*self).into();
                let hue_o: f32 = (*other).into();
                hue_s.eq(&hue_o)
            }
        }

        impl PartialEq<f32> for $name {
            fn eq(&self, other: &f32) -> bool {
                let hue: f32 = (*self).into();
                hue.eq(&normalize_angle(*other))
            }
        }

        impl PartialEq<$name> for f32 {
            fn eq(&self, other: &$name) -> bool {
                other.eq(self)
            }
        }

        impl Add<$name> for $name {
            type Output = $name;

            fn add(self, other: $name) -> $name {
                $name(self.0 + other.0)
            }
        }

        impl Add<f32> for $name {
            type Output = $name;

            fn add(self, other: f32) -> $name {
                $name(self.0 + other)
            }
        }

        impl Sub<$name> for $name {
            type Output = $name;

            fn sub(self, other: $name) -> $name {
                $name(self.0 - other.0)
            }
        }

        impl Sub<f32> for $name {
            type Output = $name;

            fn sub(self, other: f32) -> $name {
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

fn normalize_angle(mut deg: f32) -> f32 {
    while deg > 180.0 {
        deg -= 360.0;
    }

    while deg <= -180.0 {
        deg += 360.0;
    }

    deg
}
