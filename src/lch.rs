use std::ops::{Add, Sub};

use {Color, ColorSpace, Mix, Shade, GetHue, Hue, Rgb, Luma, Xyz, Lab, Hsv, Hsl, Saturate, LabHue,
     clamp};

///CIE L*C*h°, a polar version of [CIE L*a*b*](struct.Lab.html), with an alpha
///component.
///
///L*C*h° shares its range and perceptual uniformity with L*a*b*, but it's a
///cylindrical color space, like [HSL](struct.Hsl.html) and
///[HSV](struct.Hsv.html). This gives it the same ability to directly change
///the hue and colorfulness of a color, while preserving other visual aspects.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lch<T> {
    ///L* is the lightness of the color. T::zero()gives absolute black and T::one()
    ///give the brightest white.
    pub l: T,

    ///C* is the colorfulness of the color. It's similar to saturation. 0.0
    ///gives gray scale colors, and numbers around T::one()-1.41421356 gives fully
    ///saturated colors. The upper limit of 1.41421356 (or `sqrt(2.0)`) should
    ///include the whole L*a*b* space and some more.
    pub chroma: T,

    ///The hue of the color, in degrees. Decides if it's red, blue, purple,
    ///etc.
    pub hue: LabHue,

    ///The transparency of the color. T::zero()is completely transparent and T::one()is
    ///completely opaque.
    pub alpha: T,
}

impl<T: Float> Lch<T> {
    ///CIE L*C*h°.
    pub fn lch(l: T, chroma: T, hue: LabHue) -> Lch<T> {
        Lch {
            l: l,
            chroma: chroma,
            hue: hue,
            alpha: T::one(),
        }
    }

    ///CIE L*C*h° and transparency.
    pub fn lcha(l: T, chroma: T, hue: LabHue, alpha: T) -> Lch<T> {
        Lch {
            l: l,
            chroma: chroma,
            hue: hue,
            alpha: alpha,
        }
    }
}

impl<T: Float> ColorSpace for Lch<T> {
    fn is_valid(&self) -> bool {
        self.l >= T::zero() && self.l <= T::one() && self.chroma >= T::zero() &&
        self.chroma <= T::from(1.41421356).unwrap() && self.alpha >= T::zero() &&
        self.alpha <= T::one()
    }

    fn clamp(&self) -> Lch<T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.l = clamp(self.l, T::zero(), T::one());
        self.chroma = clamp(self.chroma, T::zero(), T::from(1.41421356).unwrap()); //should include all of L*a*b*, but will also overshoot...
        self.alpha = clamp(self.alpha, T::zero(), T::one());
    }
}

impl<T: Float> Mix for Lch<T> {
    fn mix(&self, other: &Lch<T>, factor: T) -> Lch<T> {
        let factor = clamp(factor, T::zero(), T::one());
        let hue_diff: T = (other.hue - self.hue).into();
        Lch {
            l: self.l + factor * (other.l - self.l),
            chroma: self.chroma + factor * (other.chroma - self.chroma),
            hue: self.hue + factor * hue_diff,
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl<T: Float> Shade for Lch<T> {
    fn lighten(&self, amount: T) -> Lch<T> {
        Lch {
            l: self.l + amount,
            chroma: self.chroma,
            hue: self.hue,
            alpha: self.alpha,
        }
    }
}

impl<T: Float> GetHue for Lch<T> {
    type Hue = LabHue;

    fn get_hue(&self) -> Option<LabHue> {
        if self.chroma <= T::zero() {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl<T: Float> Hue for Lch<T> {
    fn with_hue(&self, hue: LabHue) -> Lch<T> {
        Lch {
            l: self.l,
            chroma: self.chroma,
            hue: hue,
            alpha: self.alpha,
        }
    }

    fn shift_hue(&self, amount: LabHue) -> Lch<T> {
        Lch {
            l: self.l,
            chroma: self.chroma,
            hue: self.hue + amount,
            alpha: self.alpha,
        }
    }
}

impl<T: Float> Saturate for Lch<T> {
    fn saturate(&self, factor: T) -> Lch<T> {
        Lch {
            l: self.l,
            chroma: self.chroma * (1.0 + factor),
            hue: self.hue,
            alpha: self.alpha,
        }
    }
}

impl<T: Float> Default for Lch<T> {
    fn default() -> Lch<T> {
        Lch::<T>::lch(T::zero(), T::zero(), T::zero())
    }
}

impl Add<Lch> for Lch {
    type Output = Lch;

    fn add(self, other: Lch) -> Lch {
        Lch {
            l: self.l + other.l,
            chroma: self.chroma + other.chroma,
            hue: self.hue + other.hue,
            alpha: self.alpha + other.alpha,
        }
    }
}

impl Add<f32> for Lch {
    type Output = Lch;

    fn add(self, c: f32) -> Lch {
        Lch {
            l: self.l + c,
            chroma: self.chroma + c,
            hue: self.hue + c,
            alpha: self.alpha + c,
        }
    }
}

impl Sub<Lch> for Lch {
    type Output = Lch;

    fn sub(self, other: Lch) -> Lch {
        Lch {
            l: self.l - other.l,
            chroma: self.chroma - other.chroma,
            hue: self.hue - other.hue,
            alpha: self.alpha - other.alpha,
        }
    }
}

impl Sub<f32> for Lch {
    type Output = Lch;

    fn sub(self, c: f32) -> Lch {
        Lch {
            l: self.l - c,
            chroma: self.chroma - c,
            hue: self.hue - c,
            alpha: self.alpha - c,
        }
    }
}

from_color!(to Lch from Rgb, Luma, Xyz, Lab, Hsv, Hsl);

impl<T: Float> From<Lab<T>> for Lch<T> {
    fn from(lab: Lab<T>) -> Lch<T> {
        Lch {
            l: lab.l,
            chroma: (lab.a * lab.a + lab.b * lab.b).sqrt(),
            hue: lab.get_hue().unwrap_or(T::zero()),
            alpha: lab.alpha,
        }
    }
}

impl<T: Float> From<Rgb<T>> for Lch<T> {
    fn from(rgb: Rgb<T>) -> Lch<T> {
        Lab::from(rgb).into()
    }
}

impl<T: Float> From<Luma<T>> for Lch<T> {
    fn from(luma: Luma<T>) -> Lch<T> {
        Lab::from(luma).into()
    }
}

impl<T: Float> From<Xyz<T>> for Lch<T> {
    fn from(xyz: Xyz<T>) -> Lch<T> {
        Lab::from(xyz).into()
    }
}

impl<T: Float> From<Hsv<T>> for Lch<T> {
    fn from(hsv: Hsv) -> Lch<T> {
        Lab::from(hsv).into()
    }
}

impl<T: Float> From<Hsl<T>> for Lch<T> {
    fn from(hsl: Hsl) -> Lch<T> {
        Lab::from(hsl).into()
    }
}
