use {Color, Rgb, Luma, Lab, Lch, Hsv, Mix, Shade, clamp};

use tristimulus::{X_N, Y_N, Z_N};

///The CIE 1931 XYZ color space with an alpha component.
///
///Conversions and operations on this color space assumes the CIE Standard
///Illuminant D65 as the white point, and the 2° standard colorimetric
///observer.
#[derive(Clone, Debug, PartialEq)]
pub struct Xyz {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub alpha: f32,
}

impl Xyz {
    pub fn xyz(x: f32, y: f32, z: f32) -> Xyz {
        Xyz {
            x: x,
            y: y,
            z: z,
            alpha: 1.0,
        }
    }

    pub fn xyza(x: f32, y: f32, z: f32, alpha: f32) -> Xyz {
        Xyz {
            x: x,
            y: y,
            z: z,
            alpha: alpha,
        }
    }
}

impl Mix for Xyz {
    fn mix(&self, other: &Xyz, factor: f32) -> Xyz {
        let factor = clamp(factor, 0.0, 1.0);

        Xyz {
            x: self.x + factor * (other.x - self.x),
            y: self.y + factor * (other.y - self.y),
            z: self.z + factor * (other.z - self.z),
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl Shade for Xyz {
    fn lighten(&self, amount: f32) -> Xyz {
        Xyz {
            x: self.x,
            y: (self.y + amount).max(0.0),
            z: self.z,
            alpha: self.alpha,
        }
    }
}

impl Default for Xyz {
    fn default() -> Xyz {
        Xyz::xyz(0.0, 0.0, 0.0)
    }
}

from_color!(to Xyz from Rgb, Luma, Lab, Lch, Hsv);

impl From<Rgb> for Xyz {
    fn from(rgb: Rgb) -> Xyz {
        Xyz {
            x: rgb.red * 0.4124 + rgb.green * 0.3576 + rgb.blue * 0.1805,
            y: rgb.red * 0.2126 + rgb.green * 0.7152 + rgb.blue * 0.0722,
            z: rgb.red * 0.0193 + rgb.green * 0.1192 + rgb.blue * 0.9505,
            alpha: rgb.alpha,
        }
    }
}

impl From<Luma> for Xyz {
    fn from(luma: Luma) -> Xyz {
        Xyz {
            x: 0.0,
            y: luma.luma,
            z: 0.0,
            alpha: luma.alpha,
        }
    }
}

impl From<Lab> for Xyz {
    fn from(lab: Lab) -> Xyz {
        Xyz {
            x: X_N * f_inv((1.0 / 116.0) * (lab.l + 16.0) + (1.0 / 500.0) * lab.a),
            y: Y_N * f_inv((1.0 / 116.0) * (lab.l + 16.0)),
            z: Z_N * f_inv((1.0 / 116.0) * (lab.l + 16.0) - (1.0 / 200.0) * lab.b),
            alpha: lab.alpha,
        }
    }
}

impl From<Lch> for Xyz {
    fn from(lch: Lch) -> Xyz {
        Lab::from(lch).into()
    }
}

impl From<Hsv> for Xyz {
    fn from(hsv: Hsv) -> Xyz {
        Rgb::from(hsv).into()
    }
}


fn f_inv(t: f32) -> f32 {
    //(6/29)^2
    const C_6_O_29_P_2: f32 = 0.04280618311;

    if t > 6.0 / 29.0 {
        t * t * t
    } else {
        3.0 * C_6_O_29_P_2 * (t - (4.0 / 29.0))
    }
}

#[cfg(test)]
mod test {
    use super::Xyz;
    use ::Rgb;

    #[test]
    fn red() {
        let a = Xyz::from(Rgb::rgb(1.0, 0.0, 0.0));
        let b = Xyz::xyz(0.41240, 0.21260, 0.01930);
        assert_eq!(a, b);
    }

    #[test]
    fn green() {
        let a = Xyz::from(Rgb::rgb(0.0, 1.0, 0.0));
        let b = Xyz::xyz(0.35760, 0.71520, 0.11920);
        assert_eq!(a, b);
    }

    #[test]
    fn blue() {
        let a = Xyz::from(Rgb::rgb(0.0, 0.0, 1.0));
        let b = Xyz::xyz(0.18050, 0.07220, 0.95050);
        assert_eq!(a, b);
    }
}
