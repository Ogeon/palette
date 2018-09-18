use float::Float;

use core::f64::consts::PI;
use core::cmp::PartialEq;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use cast;

macro_rules! make_hues {
    ($($(#[$doc:meta])+ struct $name:ident;)+) => ($(
        $(#[$doc])+
        ///
        /// The hue is a circular type, where `0` and `360` is the same, and
        /// it's normalized to `(-180, 180]` when it's converted to a linear
        /// number (like `f32`). This makes many calculations easier, but may
        /// also have some surprising effects if it's expected to act as a
        /// linear number.
        #[derive(Clone, Copy, Debug, Default)]
        #[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
        #[repr(C)]
        pub struct $name<T: Float = f32>(T);

        impl<T: Float> $name<T> {
            /// Create a new hue from degrees.
            #[inline]
            pub fn from_degrees(degrees: T) -> $name<T> {
                $name(degrees)
            }

            /// Create a new hue from radians, instead of degrees.
            #[inline]
            pub fn from_radians(radians: T) -> $name<T> {
                $name(radians * cast(180.0) / cast(PI))
            }

            /// Get the hue as degrees, in the range `(-180, 180]`.
            #[inline]
            pub fn to_degrees(self) -> T {
                normalize_angle(self.0)
            }

            /// Convert the hue to radians, in the range `(-π, π]`.
            #[inline]
            pub fn to_radians(self) -> T {
                normalize_angle(self.0) * cast(PI) / cast(180.0)
            }

            /// Convert the hue to positive degrees, in the range `[0, 360)`.
            #[inline]
            pub fn to_positive_degrees(self) -> T {
                normalize_angle_positive(self.0)
            }

            /// Convert the hue to positive radians, in the range `[0, 2π)`.
            #[inline]
            pub fn to_positive_radians(self) -> T {
                normalize_angle_positive(self.0) * cast(PI) / cast(180.0)
            }

            /// Get the internal representation, without normalizing it.
            #[inline]
            pub fn to_raw_degrees(self) -> T {
                self.0
            }

            /// Get the internal representation as radians, without normalizing it.
            #[inline]
            pub fn to_raw_radians(self) -> T {
                self.0 * cast(PI) / cast(180.0)
            }
        }

        impl<T: Float> From<T> for $name<T> {
            #[inline]
            fn from(degrees: T) -> $name<T> {
                $name(degrees)
            }
        }

        impl Into<f64> for $name<f64> {
            #[inline]
            fn into(self) -> f64 {
                normalize_angle(self.0)
            }
        }

        impl Into<f32> for $name<f32> {
            #[inline]
            fn into(self) -> f32 {
                normalize_angle(self.0)
            }
        }
        impl Into<f32> for $name<f64> {
            #[inline]
            fn into(self) -> f32 {
                normalize_angle(self.0) as f32
            }
        }

        impl<T: Float> PartialEq for $name<T> {
            #[inline]
            fn eq(&self, other: &$name<T>) -> bool {
                let hue_s: T = (*self).to_degrees();
                let hue_o: T = (*other).to_degrees();
                hue_s.eq(&hue_o)
            }
        }

        impl<T: Float> PartialEq<T> for $name<T> {
            #[inline]
            fn eq(&self, other: &T) -> bool {
                let hue: T = (*self).to_degrees();
                hue.eq(&normalize_angle(*other))
            }
        }

        impl<T: Float> Add<$name<T>> for $name<T> {
            type Output = $name<T>;

            #[inline]
            fn add(self, other: $name<T>) -> $name<T> {
                $name(self.0 + other.0)
            }
        }

        impl<T: Float> Add<T> for $name<T> {
            type Output = $name<T>;

            #[inline]
            fn add(self, other: T) -> $name<T> {
                $name(self.0 + other)
            }
        }

        impl Add<$name<f32>> for f32 {
            type Output = $name<f32>;

            #[inline]
            fn add(self, other: $name<f32>) -> $name<f32> {
                $name(self + other.0)
            }
        }

        impl Add<$name<f64>> for f64 {
            type Output = $name<f64>;

            #[inline]
            fn add(self, other: $name<f64>) -> $name<f64> {
                $name(self + other.0)
            }
        }

        impl<T: Float + AddAssign> AddAssign<$name<T>> for $name<T> {
            #[inline]
            fn add_assign(&mut self, other: $name<T>) {
                self.0 += other.0;
            }
        }

        impl<T: Float + AddAssign> AddAssign<T> for $name<T> {
            #[inline]
            fn add_assign(&mut self, other: T) {
                self.0 += other;
            }
        }

        impl AddAssign<$name<f32>> for f32 {
            #[inline]
            fn add_assign(&mut self, other: $name<f32>) {
                *self += other.0;
            }
        }

        impl AddAssign<$name<f64>> for f64 {
            #[inline]
            fn add_assign(&mut self, other: $name<f64>){
                *self += other.0;
            }
        }

        impl<T: Float> Sub<$name<T>> for $name<T> {
            type Output = $name<T>;

            #[inline]
            fn sub(self, other: $name<T>) -> $name<T> {
                $name(self.0 - other.0)
            }
        }

        impl<T: Float> Sub<T> for $name<T> {
            type Output = $name<T>;

            #[inline]
            fn sub(self, other: T) -> $name<T> {
                $name(self.0 - other)
            }
        }

        impl Sub<$name<f32>> for f32 {
            type Output = $name<f32>;

            #[inline]
            fn sub(self, other: $name<f32>) -> $name<f32> {
                $name(self - other.0)
            }
        }

        impl Sub<$name<f64>> for f64 {
            type Output = $name<f64>;

            #[inline]
            fn sub(self, other: $name<f64>) -> $name<f64> {
                $name(self - other.0)
            }
        }
        
        impl<T: Float + SubAssign> SubAssign<$name<T>> for $name<T> {
            #[inline]
            fn sub_assign(&mut self, other: $name<T>) {
                self.0 -= other.0;
            }
        }

        impl<T: Float + SubAssign> SubAssign<T> for $name<T> {
            #[inline]
            fn sub_assign(&mut self, other: T) {
                self.0 -= other;
            }
        }

        impl SubAssign<$name<f32>> for f32 {
            #[inline]
            fn sub_assign(&mut self, other: $name<f32>) {
                *self -= other.0;
            }
        }

        impl SubAssign<$name<f64>> for f64 {
            #[inline]
            fn sub_assign(&mut self, other: $name<f64>){
                *self -= other.0;
            }
        }
    )+)
}

make_hues! {
    /// A hue type for the CIE L\*a\*b\* family of color spaces.
    ///
    /// It's measured in degrees and it's based on the four physiological
    /// elementary colors _red_, _yellow_, _green_ and _blue_. This makes it
    /// different from the hue of RGB based color spaces.
    struct LabHue;

    /// A hue type for the RGB family of color spaces.
    ///
    /// It's measured in degrees and uses the three additive primaries _red_,
    /// _green_ and _blue_.
    struct RgbHue;
}

#[inline]
fn normalize_angle<T: Float>(deg: T) -> T {
    let c360 = cast(360.0);
    let c180 = cast(180.0);
    deg - (((deg + c180) / c360) - T::one()).ceil() * c360
}

#[inline]
fn normalize_angle_positive<T: Float>(deg: T) -> T {
    let c360 = cast(360.0);
    deg - ((deg / c360).floor() * c360)
}

#[cfg(test)]
mod test {
    use RgbHue;
    use super::{normalize_angle, normalize_angle_positive};

    #[test]
    fn normalize_angle_0_360() {
        let inp = [
            -1000.0_f32,
            -900.0,
            -360.5,
            -360.0,
            -359.5,
            -240.0,
            -180.5,
            -180.0,
            -179.5,
            -90.0,
            -0.5,
            0.0,
            0.5,
            90.0,
            179.5,
            180.0,
            180.5,
            240.0,
            359.5,
            360.0,
            360.5,
            900.0,
            1000.0,
        ];

        let expected = [
            80.0_f32, 180.0, 359.5, 0.0, 0.5, 120.0, 179.5, 180.0, 180.5, 270.0, 359.5, 0.0, 0.5,
            90.0, 179.5, 180.0, 180.5, 240.0, 359.5, 0.0, 0.5, 180.0, 280.0,
        ];

        let result: Vec<f32> = inp.iter().map(|x| normalize_angle_positive(*x)).collect();
        for (res, exp) in result.iter().zip(expected.iter()) {
            assert_eq!(res, exp);
        }
    }

    #[test]
    fn normalize_angle_180_180() {
        let inp = [
            -1000.0_f32,
            -900.0,
            -360.5,
            -360.0,
            -359.5,
            -240.0,
            -180.5,
            -180.0,
            -179.5,
            -90.0,
            -0.5,
            0.0,
            0.5,
            90.0,
            179.5,
            180.0,
            180.5,
            240.0,
            359.5,
            360.0,
            360.5,
            900.0,
            1000.0,
        ];

        let expected = [
            80.0, 180.0, -0.5, 0.0, 0.5, 120.0, 179.5, 180.0, -179.5, -90.0, -0.5, 0.0, 0.5, 90.0,
            179.5, 180.0, -179.5, -120.0, -0.5, 0.0, 0.5, 180.0, -80.0,
        ];

        let result: Vec<f32> = inp.iter().map(|x| normalize_angle(*x)).collect();
        for (res, exp) in result.iter().zip(expected.iter()) {
            assert_eq!(res, exp);
        }
    }

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

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&RgbHue::from_degrees(10.2)).unwrap();

        assert_eq!(serialized, "10.2");
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: RgbHue = ::serde_json::from_str("10.2").unwrap();

        assert_eq!(deserialized, RgbHue::from_degrees(10.2));
    }
}
