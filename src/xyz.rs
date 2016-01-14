use std::ops::{Add, Sub, Mul, Div};

use {Color, Rgb, Luma, Lab, Lch, Hsv, Hsl, ColorSpace, Mix, Shade, clamp};

use tristimulus::{X_N, Y_N, Z_N};

///The CIE 1931 XYZ color space with an alpha component.
///
///XYZ links the perceived colors to their wavelengths and simply makes it
///possible to describe the way we see colors as numbers. It's often used when
///converting from one color space to an other, and requires a standard
///illuminant and a standard observer to be defined.
///
///Conversions and operations on this color space assumes the CIE Standard
///Illuminant D65 as the white point, and the 2° standard colorimetric
///observer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Xyz {
    ///X is the scale of what can be seen as a response curve for the cone
    ///cells in the human eye. It goes from 0.0 to 1.0.
    pub x: f32,

    ///Y is the luminance of the color, where 0.0 is black and 1.0 is white.
    pub y: f32,

    ///Z is the scale of what can be seen as the blue stimulation. It goes
    ///from 0.0 to 1.0.
    pub z: f32,

    ///The transparency of the color. 0.0 is completely transparent and 1.0 is
    ///completely opaque.
    pub alpha: f32,
}

impl Xyz {
    ///CIE XYZ.
    pub fn xyz(x: f32, y: f32, z: f32) -> Xyz {
        Xyz {
            x: x,
            y: y,
            z: z,
            alpha: 1.0,
        }
    }

    ///CIE XYZ and transparency.
    pub fn xyza(x: f32, y: f32, z: f32, alpha: f32) -> Xyz {
        Xyz {
            x: x,
            y: y,
            z: z,
            alpha: alpha,
        }
    }
}

impl ColorSpace for Xyz {
    fn is_valid(&self) -> bool {
        self.x >= 0.0 && self.x <= 1.0 &&
        self.y >= 0.0 && self.y <= 1.0 &&
        self.z >= 0.0 && self.z <= 1.0 &&
        self.alpha >= 0.0 && self.alpha <= 1.0
    }

    fn clamp(&self) -> Xyz {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.x = clamp(self.x, 0.0, 1.0);
        self.y = clamp(self.y, 0.0, 1.0);
        self.z = clamp(self.z, 0.0, 1.0);
        self.alpha = clamp(self.alpha, 0.0, 1.0);
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
            y: self.y + amount,
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

impl Add<Xyz> for Xyz {
    type Output = Xyz;

    fn add(self, other: Xyz) -> Xyz {
        Xyz {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            alpha: self.alpha + other.alpha,
        }
    }
}

impl Add<f32> for Xyz {
    type Output = Xyz;

    fn add(self, c: f32) -> Xyz {
        Xyz {
            x: self.x + c,
            y: self.y + c,
            z: self.z + c,
            alpha: self.alpha + c,
        }
    }
}

impl Sub<Xyz> for Xyz {
    type Output = Xyz;

    fn sub(self, other: Xyz) -> Xyz {
        Xyz {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            alpha: self.alpha - other.alpha,
        }
    }
}

impl Sub<f32> for Xyz {
    type Output = Xyz;

    fn sub(self, c: f32) -> Xyz {
        Xyz {
            x: self.x - c,
            y: self.y - c,
            z: self.z - c,
            alpha: self.alpha - c,
        }
    }
}

impl Mul<Xyz> for Xyz {
    type Output = Xyz;

    fn mul(self, other: Xyz) -> Xyz {
        Xyz {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            alpha: self.alpha * other.alpha,
        }
    }
}

impl Mul<f32> for Xyz {
    type Output = Xyz;

    fn mul(self, c: f32) -> Xyz {
        Xyz {
            x: self.x * c,
            y: self.y * c,
            z: self.z * c,
            alpha: self.alpha * c,
        }
    }
}

impl Div<Xyz> for Xyz {
    type Output = Xyz;

    fn div(self, other: Xyz) -> Xyz {
        Xyz {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            alpha: self.alpha / other.alpha,
        }
    }
}

impl Div<f32> for Xyz {
    type Output = Xyz;

    fn div(self, c: f32) -> Xyz {
        Xyz {
            x: self.x / c,
            y: self.y / c,
            z: self.z / c,
            alpha: self.alpha / c,
        }
    }
}

from_color!(to Xyz from Rgb, Luma, Lab, Lch, Hsv, Hsl);

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
            x: X_N * f_inv((1.0 / 116.0) * (lab.l * 100.0 + 16.0) + (1.0 / 500.0) * lab.a * 128.0),
            y: Y_N * f_inv((1.0 / 116.0) * (lab.l * 100.0 + 16.0)),
            z: Z_N * f_inv((1.0 / 116.0) * (lab.l * 100.0 + 16.0) - (1.0 / 200.0) * lab.b * 128.0),
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

impl From<Hsl> for Xyz {
    fn from(hsl: Hsl) -> Xyz {
        Rgb::from(hsl).into()
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
        let a = Xyz::from(Rgb::linear_rgb(1.0, 0.0, 0.0));
        let b = Xyz::xyz(0.41240, 0.21260, 0.01930);
        assert_approx_eq!(a, b, [x, y, z]);
    }

    #[test]
    fn green() {
        let a = Xyz::from(Rgb::linear_rgb(0.0, 1.0, 0.0));
        let b = Xyz::xyz(0.35760, 0.71520, 0.11920);
        assert_approx_eq!(a, b, [x, y, z]);
    }

    #[test]
    fn blue() {
        let a = Xyz::from(Rgb::linear_rgb(0.0, 0.0, 1.0));
        let b = Xyz::xyz(0.18050, 0.07220, 0.95050);
        assert_approx_eq!(a, b, [x, y, z]);
    }
}
