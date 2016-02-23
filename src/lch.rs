use num::Float;

use std::ops::{Add, Sub};

use {Alpha, Limited, Mix, Shade, GetHue, FromColor, IntoColor, Hue, Xyz, Lab, Saturate, LabHue, clamp};
use ColorType;

///CIE L*C*h° with an alpha component. See the [`Lcha` implementation in `Alpha`](struct.Alpha.html#Lcha).
pub type Lcha<T = f32> = Alpha<Lch<T>>;

///CIE L*C*h°, a polar version of [CIE L*a*b*](struct.Lab.html).
///
///L*C*h° shares its range and perceptual uniformity with L*a*b*, but it's a
///cylindrical color space, like [HSL](struct.Hsl.html) and
///[HSV](struct.Hsv.html). This gives it the same ability to directly change
///the hue and colorfulness of a color, while preserving other visual aspects.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lch<T: Float = f32> {
    ///L* is the lightness of the color. 0.0 gives absolute black and 1.0
    ///give the brightest white.
    pub l: T,

    ///C* is the colorfulness of the color. It's similar to saturation. 0.0
    ///gives gray scale colors, and numbers around 1.0-1.41421356 gives fully
    ///saturated colors. The upper limit of 1.41421356 (or `sqrt(2.0)`) should
    ///include the whole L*a*b* space and some more.
    pub chroma: T,

    ///The hue of the color, in degrees. Decides if it's red, blue, purple,
    ///etc.
    pub hue: LabHue<T>,
}

impl<T: Float> Lch<T> {
    ///CIE L*C*h°.
    pub fn new(l: T, chroma: T, hue: LabHue<T>) -> Lch<T> {
        Lch {
            l: l,
            chroma: chroma,
            hue: hue,
        }
    }
}

///<span id="Lcha"></span>[`Lcha`](type.Lcha.html) implementations.
impl<T: Float> Alpha<Lch<T>> {
    ///CIE L*C*h° and transparency.
    pub fn new(l: T, chroma: T, hue: LabHue<T>, alpha: T) -> Lcha<T> {
        Alpha {
            color: Lch::new(l, chroma, hue),
            alpha: alpha,
        }
    }
}

impl<T: Float> ColorType for Lch<T> {
    type Scalar = T;
}

impl<T: Float> FromColor<T> for Lch<T> {
    fn from_xyz(xyz: Xyz<T>) -> Self {
        let lab: Lab<T> = xyz.into_lab();
        Self::from_lab(lab)
    }

    fn from_lab(lab: Lab<T>) -> Self {
        Lch {
            l: lab.l,
            chroma: (lab.a * lab.a + lab.b * lab.b).sqrt(),
            hue: lab.get_hue().unwrap_or(LabHue::from(T::zero())),
        }
    }

    fn from_lch(lch: Lch<T>) -> Self {
        lch
    }

}

impl<T: Float> Limited for Lch<T> {
    fn is_valid(&self) -> bool {
        self.l >= T::zero() && self.l <= T::one() &&
        self.chroma >= T::zero()
    }

    fn clamp(&self) -> Lch<T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.l = clamp(self.l, T::zero(), T::one());
        self.chroma = self.chroma.max(T::zero())
    }
}

impl<T: Float> Mix for Lch<T> {
    fn mix(&self, other: &Lch<T>, factor: T) -> Lch<T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).to_degrees();
        Lch {
            l: self.l + factor * (other.l - self.l),
            chroma: self.chroma + factor * (other.chroma - self.chroma),
            hue: self.hue + factor * hue_diff,
        }
    }
}

impl<T: Float> Shade for Lch<T> {
    fn lighten(&self, amount: T) -> Lch<T> {
        Lch {
            l: self.l + amount,
            chroma: self.chroma,
            hue: self.hue,
        }
    }
}

impl<T: Float> GetHue for Lch<T> {
    type Hue = LabHue<T>;

    fn get_hue(&self) -> Option<LabHue<T>> {
        if self.chroma <= T::zero() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<T: Float> Hue for Lch<T> {
    fn with_hue(&self, hue: LabHue<T>) -> Lch<T> {
        Lch {
            l: self.l,
            chroma: self.chroma,
            hue: hue,
        }
    }

    fn shift_hue(&self, amount: LabHue<T>) -> Lch<T> {
        Lch {
            l: self.l,
            chroma: self.chroma,
            hue: self.hue + amount,
        }
    }
}

impl<T: Float> Saturate for Lch<T> {
    fn saturate(&self, factor: T) -> Lch<T> {
        Lch {
            l: self.l,
            chroma: self.chroma * (T::one() + factor),
            hue: self.hue,
        }
    }
}

impl<T: Float> Default for Lch<T> {
    fn default() -> Lch<T> {
        Lch::new(T::zero(), T::zero(), LabHue::from(T::zero()))
    }
}

impl<T: Float> Add<Lch<T>> for Lch<T> {
    type Output = Lch<T>;

    fn add(self, other: Lch<T>) -> Lch<T> {
        Lch {
            l: self.l + other.l,
            chroma: self.chroma + other.chroma,
            hue: self.hue + other.hue,
        }
    }
}

impl<T: Float> Add<T> for Lch<T> {
    type Output = Lch<T>;

    fn add(self, c: T) -> Lch<T> {
        Lch {
            l: self.l + c,
            chroma: self.chroma + c,
            hue: self.hue + c,
        }
    }
}

impl<T: Float> Sub<Lch<T>> for Lch<T> {
    type Output = Lch<T>;

    fn sub(self, other: Lch<T>) -> Lch<T> {
        Lch {
            l: self.l - other.l,
            chroma: self.chroma - other.chroma,
            hue: self.hue - other.hue,
        }
    }
}

impl<T: Float> Sub<T> for Lch<T> {
    type Output = Lch<T>;

    fn sub(self, c: T) -> Lch<T> {
        Lch {
            l: self.l - c,
            chroma: self.chroma - c,
            hue: self.hue - c,
        }
    }
}

#[cfg(test)]
mod test {
    use Lch;

    #[test]
    fn ranges() {
        assert_ranges!{
            Lch;
            limited {
                l: 0.0 => 1.0
            }
            limited_min {
                chroma: 0.0 => 2.0
            }
            unlimited {
                hue: -360.0 => 360.0
            }
        }
    }
}
