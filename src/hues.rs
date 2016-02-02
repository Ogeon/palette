use num::Float;

use std::f64::consts::PI;
use std::cmp::PartialEq;
use std::ops::{Add, Sub};

use flt;

macro_rules! make_hues {
    ($($(#[$doc:meta])+ struct $name:ident;)+) => ($(
        $(#[$doc])+
        ///
        ///The hue is a circular type, where `0` and `360` is the same, and
        ///it's normalized to `(-180, 180]` when it's converted to a linear
        ///number (like `f32`). This makes many calculations easier, but may
        ///also have some surprising effects if it's expected to act as a
        ///linear number.
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name<T:Float = f32>(T);

        impl<T:Float> $name<T> {
            ///Create a new hue from radians, instead of degrees.
            pub fn from_radians(radians: T) -> $name<T> {
                $name(radians * flt(180.0) / flt(PI))
            }

            ///Get the hue as degrees, in the range `(-180, 180]`.
            pub fn to_degrees(self) -> T {
                normalize_angle(self.0)
            }

            ///Convert the hue to radians, in the range `(-π, π]`.
            pub fn to_radians(self) -> T {
                normalize_angle(self.0) * flt(PI) / flt(180.0)
            }

            ///Convert the hue to positive degrees, in the range `[0, 360)`.
            pub fn to_positive_degrees(self) -> T {
                normalize_angle_positive(self.0)
            }

            ///Convert the hue to positive radians, in the range `[0, 2π)`.
            pub fn to_positive_radians(self) -> T {
                normalize_angle_positive(self.0) * flt(PI) / flt(180.0)
            }
        }

        impl<T:Float> From<T> for $name<T> {
            fn from(degrees: T) -> $name<T> {
                $name(degrees)
            }
        }

        impl Into<f64> for $name<f64> {
            fn into(self) -> f64 {
                normalize_angle(self.0)
            }
        }

        impl Into<f32> for $name<f32> {
            fn into(self) -> f32 {
                normalize_angle(self.0)
            }
        }
        impl Into<f32> for $name<f64> {
            fn into(self) -> f32 {
                normalize_angle(self.0) as f32
            }
        }

        impl<T:Float> PartialEq for $name<T> {
            fn eq(&self, other: &$name<T>) -> bool {
                let hue_s: T = (*self).to_degrees();
                let hue_o: T = (*other).to_degrees();
                hue_s.eq(&hue_o)
            }
        }

        impl<T:Float> PartialEq<T> for $name<T> {
            fn eq(&self, other: &T) -> bool {
                let hue: T = (*self).to_degrees();
                hue.eq(&normalize_angle(*other))
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
    let c180 = flt(180.0);
    let c360 = flt(360.0);
    while deg > c180 {
        deg = deg - c360;
    }

    while deg <= -c180 {
        deg = deg + c360;
    }

    deg
}

fn normalize_angle_positive<T: Float>(mut deg: T) -> T {
    let c360 = flt(360.0);
    while deg >= c360 {
        deg = deg - c360;
    }

    while deg < T::zero() {
        deg = deg + c360;
    }

    deg
}

#[cfg(test)]
mod test {
    use RgbHue;

    #[test]
    fn float_conversion() {
        for i in -180..180 {
            let hue = RgbHue::from(4.0 * i as f32);

            let degs = hue.to_degrees();
            assert!(degs > -180.0 && degs <= 180.0);

            let pos_degs = hue.to_positive_degrees();
            assert!(pos_degs >= 0.0 && pos_degs < 360.0);

            assert_eq!(RgbHue::from(degs), RgbHue::from(pos_degs));
        }
    }
}
