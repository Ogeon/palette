use num::Float;

use std::ops::{Add, Sub};

use {Alpha, Xyz, Hsv, Limited, Mix, Shade, GetHue, Hue, RgbHue, FromColor, clamp};

///Linear HWB with an alpha component. See the [`Hwba` implementation in `Alpha`](struct.Alpha.html#Hwba).
pub type Hwba<T = f32> = Alpha<Hwb<T>, T>;

///Linear HWB color space.
///
///HWB is a cylindrical version of [RGB](struct.Rgb.html) and it's very
///closely related to [HSV](struct.Hsv.html).  It describes colors with a starting hue,
///then a degree of whiteness and blackness to mix into that base hue.
///
///It is very intuitive for humans to use and many color-pickers are based on the HWB color system
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hwb<T: Float = f32> {
    ///The hue of the color, in degrees. Decides if it's red, blue, purple,
    ///etc. Same as the hue for HSL and HSV.
    pub hue: RgbHue<T>,

    ///The whiteness of the color. It specifies the amount white to mix into the hue.
    ///It varies from 0 to 1, with 1 being always full white and 0
    ///always being the color shade (a mixture of a pure hue with black) chosen with the other two
    ///controls.
    pub whiteness: T,

    ///The blackness of the color. It specifies the amount black to mix into the hue.
    ///It varies from 0 to 1, with 1 being always full black and 0 always
    ///being the color tint (a mixture of a pure hue with white) chosen with the other two
    //controls.
    pub blackness: T,
}

impl<T: Float> Hwb<T> {
    ///Linear HWB.
    pub fn new(hue: RgbHue<T>, whiteness: T, blackness: T) -> Hwb<T> {
        Hwb {
            hue: hue,
            whiteness: whiteness,
            blackness: blackness,
        }
    }
}

///<span id="Hwba"></span>[`Hwba`](type.Hwba.html) implementations.
impl<T: Float> Alpha<Hwb<T>, T> {
    ///Linear HSV and transparency.
    pub fn new(hue: RgbHue<T>, whiteness: T, blackness: T, alpha: T) -> Hwba<T> {
        Alpha {
            color: Hwb::new(hue, whiteness, blackness),
            alpha: alpha,
        }
    }
}

impl<T: Float> FromColor<T> for Hwb<T> {
    fn from_xyz(xyz: Xyz<T>) -> Self {
        let hsv: Hsv<T> = Hsv::from_xyz(xyz);
        Self::from_hsv(hsv)
    }

    fn from_hsv(hsv: Hsv<T>) -> Self {
        Hwb {
            hue: hsv.hue,
            whiteness: (T::one() - hsv.saturation) * hsv.value,
            blackness: (T::one() - hsv.value),
        }
    }

    fn from_hwb(hwb: Hwb<T>) -> Self {
        hwb
    }

}

impl<T: Float> Limited for Hwb<T> {
    fn is_valid(&self) -> bool {
        self.blackness >= T::zero() && self.blackness <= T::one() &&
        self.whiteness >= T::zero() && self.whiteness <= T::one() &&
        (self.whiteness + self.blackness) <= T::one()
    }

    fn clamp(&self) -> Hwb<T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.whiteness = self.whiteness.max(T::zero());
        self.blackness = self.blackness.max(T::zero());
        let sum = self.blackness + self.whiteness;
        if sum > T::one() {
            self.whiteness = self.whiteness / sum;
            self.blackness = self.blackness / sum;
        }
    }
}

impl<T: Float> Mix for Hwb<T> {
    type Scalar = T;

    fn mix(&self, other: &Hwb<T>, factor: T) -> Hwb<T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();

        Hwb {
            hue: self.hue + factor * hue_diff,
            whiteness: self.whiteness + factor * (other.whiteness - self.whiteness),
            blackness: self.blackness + factor * (other.blackness - self.blackness),
        }
    }
}

impl<T: Float> Shade for Hwb<T> {
    type Scalar = T;

    fn lighten(&self, amount: T) -> Hwb<T> {
        Hwb {
            hue: self.hue,
            whiteness: self.whiteness + amount,
            blackness: self.blackness - amount,
        }
    }
}

impl<T: Float> GetHue for Hwb<T> {
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> Option<RgbHue<T>> {
        if self.whiteness + self.blackness >= T::one() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<T: Float> Hue for Hwb<T> {
    fn with_hue(&self, hue: RgbHue<T>) -> Hwb<T> {
        Hwb {
            hue: hue,
            whiteness: self.whiteness,
            blackness: self.blackness,
        }
    }

    fn shift_hue(&self, amount: RgbHue<T>) -> Hwb<T> {
        Hwb {
            hue: self.hue + amount,
            whiteness: self.whiteness,
            blackness: self.blackness,
        }
    }
}

impl<T: Float> Default for Hwb<T> {
    fn default() -> Hwb<T> {
        Hwb::new(RgbHue::from(T::zero()), T::zero(), T::one())
    }
}

impl<T: Float> Add<Hwb<T>> for Hwb<T> {
    type Output = Hwb<T>;

    fn add(self, other: Hwb<T>) -> Hwb<T> {
        Hwb {
            hue: self.hue + other.hue,
            whiteness: self.whiteness + other.whiteness,
            blackness: self.blackness + other.blackness,
        }
    }
}

impl<T: Float> Add<T> for Hwb<T> {
    type Output = Hwb<T>;

    fn add(self, c: T) -> Hwb<T> {
        Hwb {
            hue: self.hue + c,
            whiteness: self.whiteness + c,
            blackness: self.blackness + c,
        }
    }
}

impl<T: Float> Sub<Hwb<T>> for Hwb<T> {
    type Output = Hwb<T>;

    fn sub(self, other: Hwb<T>) -> Hwb<T> {
        Hwb {
            hue: self.hue - other.hue,
            whiteness: self.whiteness - other.whiteness,
            blackness: self.blackness - other.blackness,
        }
    }
}

impl<T: Float> Sub<T> for Hwb<T> {
    type Output = Hwb<T>;

    fn sub(self, c: T) -> Hwb<T> {
        Hwb {
            hue: self.hue - c,
            whiteness: self.whiteness - c,
            blackness: self.blackness - c,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Hwb;
    use ::{Rgb, Limited};

    #[test]
    fn red() {
        let a = Hwb::from(Rgb::new(1.0, 0.0, 0.0));
        let b = Hwb::new(0.0.into(), 0.0, 0.0);
        assert_relative_eq!(a, b);
    }

    #[test]
    fn orange() {
        let a = Hwb::from(Rgb::new(1.0, 0.5, 0.0));
        let b = Hwb::new(30.0.into(), 0.0, 0.0);
        assert_relative_eq!(a, b);
    }

    #[test]
    fn green() {
        let a = Hwb::from(Rgb::new(0.0, 1.0, 0.0));
        let b = Hwb::new(120.0.into(), 0.0, 0.0);
        assert_relative_eq!(a, b);
    }

    #[test]
    fn blue() {
        let a = Hwb::from(Rgb::new(0.0, 0.0, 1.0));
        let b = Hwb::new(240.0.into(), 0.0, 0.0);
        assert_relative_eq!(a, b);
    }

    #[test]
    fn purple() {
        let a = Hwb::from(Rgb::new(0.5, 0.0, 1.0));
        let b = Hwb::new(270.0.into(), 0.0, 0.0);
        assert_relative_eq!(a, b);
    }

    #[test]
    fn clamp_invalid() {
        let expected = Hwb { hue: (240.0).into(), whiteness: 0.0, blackness: 0.0 };

        let a = Hwb { hue: (240.0).into(), whiteness: -3.0, blackness: -4.0 };
        let calc_a = a.clamp();
        assert_relative_eq!(expected, calc_a);

    }

    #[test]
    fn clamp_none() {
        let expected = Hwb { hue: (240.0).into(), whiteness: 0.3, blackness: 0.7 };

        let a = Hwb { hue: (240.0).into(), whiteness: 0.3, blackness: 0.7 };
        let calc_a = a.clamp();
        assert_relative_eq!(expected, calc_a);
    }
    #[test]
    fn clamp_over_one() {
        let expected = Hwb { hue: (240.0).into(), whiteness: 0.2, blackness: 0.8};

        let a = Hwb { hue: (240.0).into(), whiteness: 5.0, blackness: 20.0 };
        let calc_a = a.clamp();
        assert_relative_eq!(expected, calc_a);

    }
    #[test]
    fn clamp_under_one() {
        let expected = Hwb { hue: (240.0).into(), whiteness: 0.3, blackness: 0.1};

        let a = Hwb { hue: (240.0).into(), whiteness: 0.3, blackness: 0.1 };
        let calc_a = a.clamp();
        assert_relative_eq!(expected, calc_a);
    }
}
