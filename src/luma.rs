use {Color, Rgb, Xyz, Lab, Lch, Hsv, Hsl, ColorSpace, Mix, Shade, clamp};

///Linear luminance with an alpha component.
///
///Luma is a purely gray scale color space, which is included more for
///completeness than anything else, and represents how bright a color is
///perceived to be. It's basically the `Y` component of [CIE
///XYZ](struct.Xyz.html). The lack of any form of hue representation limits
///the set of operations that can be performed on it.
#[derive(Clone, Debug, PartialEq)]
pub struct Luma {
    ///The lightness of the color. 0.0 is black and 1.0 is white.
    pub luma: f32,

    ///The transparency of the color. 0.0 is completely transparent and 1.0 is
    ///completely opaque.
    pub alpha: f32,
}

impl Luma {
    ///Linear luminance.
    pub fn y(luma: f32) -> Luma {
        Luma {
            luma: luma,
            alpha: 0.0,
        }
    }

    ///Linear luminance with transparency.
    pub fn ya(luma: f32, alpha: f32) -> Luma {
        Luma {
            luma: luma,
            alpha: alpha,
        }
    }

    ///Linear luminance from an 8 bit value.
    pub fn y8(luma: u8) -> Luma {
        Luma {
            luma: luma as f32 / 255.0,
            alpha: 0.0,
        }
    }

    ///Linear luminance and transparency from 8 bit values.
    pub fn ya8(luma: u8, alpha: u8) -> Luma {
        Luma {
            luma: luma as f32 / 255.0,
            alpha: alpha as f32 / 255.0,
        }
    }
}

impl ColorSpace for Luma {
    fn is_valid(&self) -> bool {
        self.luma >= 0.0 && self.luma <= 1.0 &&
        self.alpha >= 0.0 && self.alpha <= 1.0
    }

    fn clamp(&self) -> Luma {
        let mut c = self.clone();
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
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

from_color!(to Luma from Rgb, Xyz, Lab, Lch, Hsv, Hsl);

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

impl From<Hsl> for Luma {
    fn from(hsl: Hsl) -> Luma {
        Rgb::from(hsl).into()
    }
}
