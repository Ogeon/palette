use num::traits::Float;

use std::f64::consts::PI;
use std::cmp::PartialEq;
use std::ops::{Add, Sub};




#[doc = r"A hue type for the CIE L*a*b* family of color spaces."]
#[doc = r""]
#[doc = r"It's measured in degrees and it's based on the four physiological"]
#[doc = r"elementary colors _red_, _yellow_, _green_ and _blue_. This makes it"]
#[doc = r"different from the hue of RGB based color spaces."]
///
///The hue is a circular type, where `0` and `360` is the same, and
///it's normalized to `(-180, +180]` when it's converted to a linear
///number (like `f32`). This makes many calculations easier, but may
///also have some surprising effects if it's expected to act as a
///linear number.
#[derive(Clone, Copy, Debug, Default)]
pub struct LabHue<T: Float>(T);

impl<T: Float> LabHue<T> {
    ///Create a new hue from radians, instead of degrees.
    pub fn from_radians(radians: T) -> LabHue<T> {
        LabHue(radians * T::from(180.0).unwrap() / T::from(PI).unwrap())
    }
    ///Convert the hue to radians.
    pub fn to_radians(self) -> T {
        normalize_angle(self.0) * T::from(PI).unwrap() / T::from(180.0).unwrap()
    }
    ///Returns the hue value as a f32 or f64 as needed
    pub fn to_float(self) -> T {
        normalize_angle(self.0)
    }
}
impl<T: Float> From<T> for LabHue<T> {
    fn from(degrees: T) -> LabHue<T> {
        LabHue(degrees)
    }
}

impl<T: Float> Into<f64> for LabHue<T> {
    fn into(self) -> f64 {
        normalize_angle(self.0).to_f64().unwrap()
    }
}
impl<T: Float> Into<f32> for LabHue<T> {
    fn into(self) -> f32 {
        normalize_angle(self.0).to_f32().unwrap()
    }
}


impl<T: Float> PartialEq for LabHue<T> {
    fn eq(&self, other: &LabHue<T>) -> bool {
        let hue_s: T = self.to_float();
        let hue_o: T = other.to_float();
        hue_s.eq(&hue_o)
    }
}
impl<T: Float> PartialEq<T> for LabHue<T> {
    fn eq(&self, other: &T) -> bool {
        let hue: T = self.to_float();
        hue.eq(&normalize_angle(*other))
    }
}

impl<T: Float> Add<LabHue<T>> for LabHue<T> {
    type Output = LabHue<T>;
    fn add(self, other: LabHue<T>) -> LabHue<T> {
        LabHue(self.0 + other.0)
    }
}
impl<T: Float> Add<T> for LabHue<T> {
    type Output = LabHue<T>;
    fn add(self, other: T) -> LabHue<T> {
        LabHue(self.0 + other)
    }
}
impl<T: Float> Sub<LabHue<T>> for LabHue<T> {
    type Output = LabHue<T>;
    fn sub(self, other: LabHue<T>) -> LabHue<T> {
        LabHue(self.0 - other.0)
    }
}
impl<T: Float> Sub<T> for LabHue<T> {
    type Output = LabHue<T>;
    fn sub(self, other: T) -> LabHue<T> {
        LabHue(self.0 - other)
    }
}
#[doc = r"A hue type for the RGB family of color spaces."]
#[doc = r""]
#[doc = r"It's measured in degrees and uses the three additive primaries _red_,"]
#[doc = r"_green_ and _blue_."]
///
///The hue is a circular type, where `0` and `360` is the same, and
///it's normalized to `(-180, +180]` when it's converted to a linear
///number (like `f32`). This makes many calculations easier, but may
///also have some surprising effects if it's expected to act as a
///linear number.
#[derive(Clone, Copy, Debug, Default)]
pub struct RgbHue<T: Float = f32>(T);

impl<T: Float> RgbHue<T> {
    ///Create a new hue from radians, instead of degrees.
    pub fn from_radians(radians: T) -> RgbHue<T> {
        RgbHue(radians * T::from(180.0).unwrap() / T::from(PI).unwrap())
    }
    ///Convert the hue to radians.
    pub fn to_radians(self) -> T {
        normalize_angle(self.0) * T::from(PI).unwrap() / T::from(180.0).unwrap()
    }
    ///Returns the hue value as a f32 or f64 as needed
    pub fn to_float(self) -> T {
        normalize_angle(self.0)
    }
}
impl<T: Float> From<T> for RgbHue<T> {
    fn from(degrees: T) -> RgbHue<T> {
        RgbHue(degrees)
    }
}
impl<T: Float> Into<f64> for RgbHue<T> {
    fn into(self) -> f64 {
        normalize_angle(self.0).to_f64().unwrap()
    }
}
impl<T: Float> Into<f32> for RgbHue<T> {
    fn into(self) -> f32 {
        normalize_angle(self.0).to_f32().unwrap()
    }
}

impl<T: Float> PartialEq for RgbHue<T> {
    fn eq(&self, other: &RgbHue<T>) -> bool {
        let hue_s: T = self.to_float();
        let hue_o: T = other.to_float();
        hue_s.eq(&hue_o)
    }
}
impl<T: Float> PartialEq<T> for RgbHue<T> {
    fn eq(&self, other: &T) -> bool {
        let hue: T = self.to_float();
        hue.eq(&normalize_angle(*other))
    }
}

impl<T: Float> Add<RgbHue<T>> for RgbHue<T> {
    type Output = RgbHue<T>;
    fn add(self, other: RgbHue<T>) -> RgbHue<T> {
        RgbHue(self.0 + other.0)
    }
}
impl<T: Float> Add<T> for RgbHue<T> {
    type Output = RgbHue<T>;
    fn add(self, other: T) -> RgbHue<T> {
        RgbHue(self.0 + other)
    }
}
impl<T: Float> Sub<RgbHue<T>> for RgbHue<T> {
    type Output = RgbHue<T>;
    fn sub(self, other: RgbHue<T>) -> RgbHue<T> {
        RgbHue(self.0 - other.0)
    }
}
impl<T: Float> Sub<T> for RgbHue<T> {
    type Output = RgbHue<T>;
    fn sub(self, other: T) -> RgbHue<T> {
        RgbHue(self.0 - other)
    }
}



fn normalize_angle<T: Float>(mut deg: T) -> T {
    while deg > T::from(180.0).unwrap() {
        deg = deg - T::from(360.0).unwrap();
    }

    while deg <= -T::from(180.0).unwrap() {
        deg = deg - T::from(360.0).unwrap();
    }

    deg
}
