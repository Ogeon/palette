use {Color, Rgb, Xyz, Lab, Lch, Hsv, Mix, Shade, clamp};

///Linear luminance with an alpha component.
#[derive(Clone, Debug, PartialEq)]
pub struct Luma {
    pub luma: f32,
    pub alpha: f32,
}

impl Luma {
    pub fn y(luma: f32) -> Luma {
        Luma {
            luma: luma,
            alpha: 0.0,
        }
    }

    pub fn ya(luma: f32, alpha: f32) -> Luma {
        Luma {
            luma: luma,
            alpha: alpha,
        }
    }

    pub fn y8(luma: u8) -> Luma {
        Luma {
            luma: luma as f32 / 255.0,
            alpha: 0.0,
        }
    }

    pub fn ya8(luma: u8, alpha: u8) -> Luma {
        Luma {
            luma: luma as f32 / 255.0,
            alpha: alpha as f32 / 255.0,
        }
    }

    ///Return a new luminance value with all channels clamped to `[0.0, 1.0]`.
    pub fn clamp(&self) -> Luma {
        Luma {
            luma: clamp(self.luma, 0.0, 1.0),
            alpha: clamp(self.alpha, 0.0, 1.0),
        }
    }

    ///Clamp all channels to `[0.0, 1.0]`.
    pub fn clamp_self(&mut self) {
        self.luma = clamp(self.luma, 0.0, 1.0);
        self.alpha = clamp(self.alpha, 0.0, 1.0);
    }
}

impl Mix for Luma {
    fn mix(&self, other: &Luma, factor: f32) -> Luma {
        let factor = clamp(factor, 0.0, 1.0);

        Luma {
            luma: self.luma + factor * (other.luma - self.luma),
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl Shade for Luma {
    fn lighten(&self, amount: f32) -> Luma {
        Luma {
            luma: (self.luma + amount).max(0.0),
            alpha: self.alpha,
        }
    }
}

impl Default for Luma {
    fn default() -> Luma {
        Luma::y(0.0)
    }
}

from_color!(to Luma from Rgb, Xyz, Lab, Lch, Hsv);

impl From<Rgb> for Luma {
    fn from(rgb: Rgb) -> Luma {
        Luma {
            luma: rgb.red * 0.2126 + rgb.green * 0.7152 + rgb.blue * 0.0722,
            alpha: rgb.alpha
        }
    }
}

impl From<Xyz> for Luma {
    fn from(xyz: Xyz) -> Luma {
        Luma {
            luma: xyz.y,
            alpha: xyz.alpha
        }
    }
}

impl From<Lab> for Luma {
    fn from(lab: Lab) -> Luma {
        Xyz::from(lab).into()
    }
}

impl From<Lch> for Luma {
    fn from(lch: Lch) -> Luma {
        Xyz::from(lch).into()
    }
}

impl From<Hsv> for Luma {
    fn from(hsv: Hsv) -> Luma {
        Rgb::from(hsv).into()
    }
}
