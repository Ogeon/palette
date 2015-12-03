use {Color, ColorSpace, Mix, Shade, GetHue, Hue, Rgb, Luma, Xyz, Lab, Hsv, Hsl, Saturate, LabHue, clamp};

///CIE L*C*h°, a polar version of CIE L*a*b*, with an alpha component.
#[derive(Clone, Debug, PartialEq)]
pub struct Lch {
    pub l: f32,
    pub chroma: f32,
    pub hue: LabHue,
    pub alpha: f32,
}

impl Lch {
    ///CIE L*C*h°.
    pub fn lch(l: f32, chroma: f32, hue: LabHue) -> Lch {
        Lch {
            l: l,
            chroma: chroma,
            hue: hue,
            alpha: 1.0
        }
    }

    ///CIE L*C*h° and transparency.
    pub fn lcha(l: f32, chroma: f32, hue: LabHue, alpha: f32) -> Lch {
        Lch {
            l: l,
            chroma: chroma,
            hue: hue,
            alpha: alpha
        }
    }
}

impl ColorSpace for Lch {
    fn is_valid(&self) -> bool {
        self.l >= 0.0 && self.l <= 100.0 &&
        self.chroma >= 0.0 && self.chroma <= 182.0 && //should include all of L*a*b*, but will also overshoot...
        self.alpha >= 0.0 && self.alpha <= 1.0
    }

    fn clamp(&self) -> Lch {
        let mut c = self.clone();
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.l = clamp(self.l, 0.0, 100.0);
        self.chroma = clamp(self.chroma, 0.0, 182.0); //should include all of L*a*b*, but will also overshoot...
        self.alpha = clamp(self.alpha, 0.0, 1.0);
    }
}

impl Mix for Lch {
    fn mix(&self, other: &Lch, factor: f32) -> Lch {
        let factor = clamp(factor, 0.0, 1.0);
        let hue_diff: f32 = (other.hue - self.hue).into();
        Lch {
            l: self.l + factor * (other.l - self.l),
            chroma: self.chroma + factor * (other.chroma - self.chroma),
            hue: self.hue + factor * hue_diff,
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl Shade for Lch {
    fn lighten(&self, amount: f32) -> Lch {
        Lch {
            l: self.l + amount * 100.0,
            chroma: self.chroma,
            hue: self.hue,
            alpha: self.alpha,
        }
    }
}

impl GetHue for Lch {
    type Hue = LabHue;

    fn get_hue(&self) -> Option<LabHue> {
        if self.chroma <= 0.0 {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl Hue for Lch {
    fn with_hue(&self, hue: LabHue) -> Lch {
        Lch {
            l: self.l,
            chroma: self.chroma,
            hue: hue,
            alpha: self.alpha,
        }
    }

    fn shift_hue(&self, amount: LabHue) -> Lch {
        Lch {
            l: self.l,
            chroma: self.chroma,
            hue: self.hue + amount,
            alpha: self.alpha,
        }
    }
}

impl Saturate for Lch {
    fn saturate(&self, factor: f32) -> Lch {
        Lch {
            l: self.l,
            chroma: self.chroma * (1.0 + factor),
            hue: self.hue,
            alpha: self.alpha,
        }
    }
}

impl Default for Lch {
    fn default() -> Lch {
        Lch::lch(0.0, 0.0, 0.0.into())
    }
}

from_color!(to Lch from Rgb, Luma, Xyz, Lab, Hsv, Hsl);

impl From<Lab> for Lch {
    fn from(lab: Lab) -> Lch {
        Lch {
            l: lab.l,
            chroma: (lab.a * lab.a + lab.b * lab.b).sqrt(),
            hue: lab.get_hue().unwrap_or(0.0.into()),
            alpha: lab.alpha,
        }
    }
}

impl From<Rgb> for Lch {
    fn from(rgb: Rgb) -> Lch {
        Lab::from(rgb).into()
    }
}

impl From<Luma> for Lch {
    fn from(luma: Luma) -> Lch {
        Lab::from(luma).into()
    }
}

impl From<Xyz> for Lch {
    fn from(xyz: Xyz) -> Lch {
        Lab::from(xyz).into()
    }
}

impl From<Hsv> for Lch {
    fn from(hsv: Hsv) -> Lch {
        Lab::from(hsv).into()
    }
}

impl From<Hsl> for Lch {
    fn from(hsl: Hsl) -> Lch {
        Lab::from(hsl).into()
    }
}
