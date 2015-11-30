use std::f32::consts::PI;

use {Color, Rgb, Luma, Xyz, Mix, Shade, GetHue, clamp};

use tristimulus::{X_N, Y_N, Z_N};

///The CIE L*a*b* (CIELAB) color space with an alpha component.
#[derive(Clone, Debug, PartialEq)]
pub struct Lab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
    pub alpha: f32,
}

impl Lab {
    pub fn lab(l: f32, a: f32, b: f32) -> Lab {
        Lab {
            l: l,
            a: a,
            b: b,
            alpha: 1.0,
        }
    }

    pub fn laba(l: f32, a: f32, b: f32, alpha: f32) -> Lab {
        Lab {
            l: l,
            a: a,
            b: b,
            alpha: alpha,
        }
    }
}

impl Mix for Lab {
    fn mix(&self, other: &Lab, factor: f32) -> Lab {
        let factor = clamp(factor, 0.0, 1.0);

        Lab {
            l: self.l + factor * (other.l - self.l),
            a: self.a + factor * (other.a - self.a),
            b: self.b + factor * (other.b - self.b),
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl Shade for Lab {
    fn lighten(&self, amount: f32) -> Lab {
        Lab {
            l: (self.l + amount * 100.0).max(0.0),
            a: self.a,
            b: self.b,
            alpha: self.alpha,
        }
    }
}

impl GetHue for Lab {
    fn get_hue(&self) -> Option<f32> {
        if self.a == 0.0 && self.b == 0.0 {
            None
        } else {
            Some(self.b.atan2(self.a) * 180.0 / PI)
        }
    }
}

impl Default for Lab {
    fn default() -> Lab {
        Lab::lab(0.0, 0.0, 0.0)
    }
}
from_color!(to Lab from Rgb, Luma, Xyz);

impl From<Xyz> for Lab {
    fn from(xyz: Xyz) -> Lab {
        Lab {
            l: 116.0 * f(xyz.y / Y_N) - 16.0,
            a: 500.0 * (f(xyz.x / X_N) - f(xyz.y / Y_N)),
            b: 200.0 * (f(xyz.y / Y_N) - f(xyz.z / Z_N)),
            alpha: xyz.alpha,
        }
    }
}

impl From<Rgb> for Lab {
    fn from(rgb: Rgb) -> Lab {
        Xyz::from(rgb).into()
    }
}

impl From<Luma> for Lab {
    fn from(luma: Luma) -> Lab {
        Xyz::from(luma).into()
    }
}

fn f(t: f32) -> f32 {
    //(6/29)^3
    const C_6_O_29_P_3: f32 = 0.00885645167;
    //(29/6)^2
    const C_29_O_6_P_2: f32 = 23.3611111111;

    if t > C_6_O_29_P_3 {
        t.powf(1.0 / 3.0)
    } else {
        (1.0 / 3.0) * C_29_O_6_P_2 * t + (4.0 / 29.0)
    }
}

#[cfg(test)]
mod test {
    use super::Lab;
    use ::Rgb;

    #[test]
    fn red() {
        let a = Lab::from(Rgb::rgb(1.0, 0.0, 0.0));
        let b = Lab::lab(53.23288, 80.10933, 67.22006);
        assert_eq!(a, b);
    }

    #[test]
    fn green() {
        let a = Lab::from(Rgb::rgb(0.0, 1.0, 0.0));
        let b = Lab::lab(87.73704, -86.184654, 83.18117);
        assert_eq!(a, b);
    }

    #[test]
    fn blue() {
        let a = Lab::from(Rgb::rgb(0.0, 0.0, 1.0));
        let b = Lab::lab(32.302586, 79.19668, -107.863686);
        assert_eq!(a, b);
    }
}
