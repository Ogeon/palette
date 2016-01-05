use {Color, Rgb, Luma, Xyz, Lch, Hsv, Hsl, ColorSpace, Mix, Shade, GetHue, LabHue, clamp};

use tristimulus::{X_N, Y_N, Z_N};

///The CIE L*a*b* (CIELAB) color space with an alpha component.
///
///CIE L*a*b* is a device independent color space which includes all
///perceivable colors. It's sometimes used to convert between other color
///spaces, because of its ability to represent all of their colors, and
///sometimes in color manipulation, because of its perceptual uniformity. This
///means that the perceptual difference between two colors is equal to their
///numerical difference.
///
///The parameters of L*a*b* are quite different, compared to many other color
///spaces, so manipulating them manually can be unintuitive.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lab {
    ///L* is the lightness of the color. 0.0 gives absolute black and 100.0
    ///give the brightest white.
    pub l: f32,

    ///a* goes from red at -127.0 to green at 128.0.
    pub a: f32,

    ///b* goes from yellow at -127.0 to blue at 128.0.
    pub b: f32,

    ///The transparency of the color. 0.0 is completely transparent and 1.0 is
    ///completely opaque.
    pub alpha: f32,
}

impl Lab {
    ///CIE L*a*b*.
    pub fn lab(l: f32, a: f32, b: f32) -> Lab {
        Lab {
            l: l,
            a: a,
            b: b,
            alpha: 1.0,
        }
    }

    ///CIE L*a*b* and transparency.
    pub fn laba(l: f32, a: f32, b: f32, alpha: f32) -> Lab {
        Lab {
            l: l,
            a: a,
            b: b,
            alpha: alpha,
        }
    }
}

impl ColorSpace for Lab {
    fn is_valid(&self) -> bool {
        self.l >= 0.0 && self.l <= 100.0 &&
        self.a >= -127.0 && self.a <= 128.0 &&
        self.b >= -127.0 && self.b <= 128.0 &&
        self.alpha >= 0.0 && self.alpha <= 1.0
    }

    fn clamp(&self) -> Lab {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.l = clamp(self.l, 0.0, 100.0);
        self.a = clamp(self.a, -127.0, 128.0);
        self.b = clamp(self.b, -127.0, 128.0);
        self.alpha = clamp(self.alpha, 0.0, 1.0);
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
            l: self.l + amount * 100.0,
            a: self.a,
            b: self.b,
            alpha: self.alpha,
        }
    }
}

impl GetHue for Lab {
    type Hue = LabHue;

    fn get_hue(&self) -> Option<LabHue> {
        if self.a == 0.0 && self.b == 0.0 {
            None
        } else {
            Some(LabHue::from_radians(self.b.atan2(self.a)))
        }
    }
}

impl Default for Lab {
    fn default() -> Lab {
        Lab::lab(0.0, 0.0, 0.0)
    }
}

from_color!(to Lab from Rgb, Luma, Xyz, Lch, Hsv, Hsl);

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

impl From<Lch> for Lab {
    fn from(lch: Lch) -> Lab {
        Lab {
            l: lch.l,
            a: lch.chroma.max(0.0) * lch.hue.to_radians().cos(),
            b: lch.chroma.max(0.0) * lch.hue.to_radians().sin(),
            alpha: lch.alpha,
        }
    }
}

impl From<Hsv> for Lab {
    fn from(hsv: Hsv) -> Lab {
        Xyz::from(hsv).into()
    }
}

impl From<Hsl> for Lab {
    fn from(hsl: Hsl) -> Lab {
        Xyz::from(hsl).into()
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
        assert_approx_eq!(a, b, [l, a, b]);
    }

    #[test]
    fn green() {
        let a = Lab::from(Rgb::rgb(0.0, 1.0, 0.0));
        let b = Lab::lab(87.73704, -86.184654, 83.18117);
        assert_approx_eq!(a, b, [l, a, b]);
    }

    #[test]
    fn blue() {
        let a = Lab::from(Rgb::rgb(0.0, 0.0, 1.0));
        let b = Lab::lab(32.302586, 79.19668, -107.863686);
        assert_approx_eq!(a, b, [l, a, b]);
    }
}
