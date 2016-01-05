use {Color, Rgb, Luma, Xyz, Lab, Lch, Hsv, ColorSpace, Mix, Shade, GetHue, Hue, Saturate, RgbHue, clamp};

///Linear HSL color space with an alpha component.
///
///The HSL color space can be seen as a cylindrical version of
///[RGB](struct.Rgb.html), where the `hue` is the angle around the color
///cylinder, the `saturation` is the distance from the center, and the
///`lightness` is the height from the bottom. Its composition makes it
///especially good for operations like changing green to red, making a color
///more gray, or making it darker.
///
///See [HSV](struct.Hsv.html) for a very similar color space, with brightness instead of lightness.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsl {
    ///The hue of the color, in degrees. Decides if it's red, blue, purple,
    ///etc.
	pub hue: RgbHue,

    ///The colorfulness of the color. 0.0 gives gray scale colors and 1.0 will
    ///give absolutely clear colors.
	pub saturation: f32,

    ///Decides how light the color will look. 0.0 will be black, 0.5 will give
    ///a clear color, and 1.0 will give white.
	pub lightness: f32,

    ///The transparency of the color. 0.0 is completely transparent and 1.0 is
    ///completely opaque.
	pub alpha: f32,
}

impl Hsl {
    ///Linear HSL.
	pub fn hsl(hue: RgbHue, saturation: f32, lightness: f32) -> Hsl {
		Hsl {
			hue: hue,
			saturation: saturation,
			lightness: lightness,
			alpha: 1.0,
		}
	}

	///Linear HSL and transparency.
	pub fn hsla(hue: RgbHue, saturation: f32, lightness: f32, alpha: f32) -> Hsl {
		Hsl {
			hue: hue,
			saturation: saturation,
			lightness: lightness,
			alpha: alpha,
		}
	}
}

impl ColorSpace for Hsl {
    fn is_valid(&self) -> bool {
        self.saturation >= 0.0 && self.saturation <= 1.0 &&
        self.lightness >= 0.0 && self.lightness <= 1.0 &&
        self.alpha >= 0.0 && self.alpha <= 1.0
    }

    fn clamp(&self) -> Hsl {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.saturation = clamp(self.saturation, 0.0, 1.0);
        self.lightness = clamp(self.lightness, 0.0, 1.0);
        self.alpha = clamp(self.alpha, 0.0, 1.0);
    }
}

impl Mix for Hsl {
	fn mix(&self, other: &Hsl, factor: f32) -> Hsl {
        let factor = clamp(factor, 0.0, 1.0);
        let hue_diff: f32 = (other.hue - self.hue).into();

        Hsl {
            hue: self.hue + factor * hue_diff,
            saturation: self.saturation + factor * (other.saturation - self.saturation),
            lightness: self.lightness + factor * (other.lightness - self.lightness),
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl Shade for Hsl {
    fn lighten(&self, amount: f32) -> Hsl {
        Hsl {
            hue: self.hue,
            saturation: self.saturation,
            lightness: self.lightness + amount,
            alpha: self.alpha,
        }
    }
}

impl GetHue for Hsl {
    type Hue = RgbHue;

    fn get_hue(&self) -> Option<RgbHue> {
        if self.saturation <= 0.0 {
            None
        } else {
            Some(self.hue)
        }
    }
}

impl Hue for Hsl {
    fn with_hue(&self, hue: RgbHue) -> Hsl {
        Hsl {
            hue: hue,
            saturation: self.saturation,
            lightness: self.lightness,
            alpha: self.alpha,
        }
    }

    fn shift_hue(&self, amount: RgbHue) -> Hsl {
        Hsl {
            hue: self.hue + amount,
            saturation: self.saturation,
            lightness: self.lightness,
            alpha: self.alpha,
        }
    }
}

impl Saturate for Hsl {
    fn saturate(&self, factor: f32) -> Hsl {
        Hsl {
            hue: self.hue,
            saturation: self.saturation * (1.0 + factor),
            lightness: self.lightness,
            alpha: self.alpha,
        }
    }
}

impl Default for Hsl {
	fn default() -> Hsl {
		Hsl::hsl(0.0.into(), 0.0, 0.0)
	}
}

from_color!(to Hsl from Rgb, Luma, Xyz, Lab, Lch, Hsv);

impl From<Rgb> for Hsl {
	fn from(rgb: Rgb) -> Hsl {
		enum Channel { Red, Green, Blue };

        let val_min = rgb.red.min(rgb.green).min(rgb.blue);
        let mut val_max = rgb.red;
        let mut chan_max = Channel::Red;

        if rgb.green > val_max {
            chan_max = Channel::Green;
            val_max = rgb.green;
        }

        if rgb.blue > val_max {
            chan_max = Channel::Blue;
            val_max = rgb.blue;
        }

        let diff = val_max - val_min;
        let lightness = (val_min + val_max) / 2.0;

        let hue = if diff == 0.0 {
            0.0
        } else {
            60.0 * match chan_max {
                Channel::Red => ((rgb.green - rgb.blue) / diff) % 6.0,
                Channel::Green => ((rgb.blue - rgb.red) / diff + 2.0),
                Channel::Blue => ((rgb.red - rgb.green) / diff + 4.0),
            }
        };

        let saturation = if diff == 0.0 {
            0.0
        } else {
            diff / (1.0 - (2.0 * lightness - 1.0).abs())
        };

        Hsl {
            hue: hue.into(),
            saturation: saturation,
            lightness: lightness,
            alpha: rgb.alpha,
        }
	}
}

impl From<Luma> for Hsl {
	fn from(luma: Luma) -> Hsl {
		Rgb::from(luma).into()
	}
}

impl From<Xyz> for Hsl {
	fn from(xyz: Xyz) -> Hsl {
		Rgb::from(xyz).into()
	}
}

impl From<Lab> for Hsl {
	fn from(lab: Lab) -> Hsl {
		Rgb::from(lab).into()
	}
}

impl From<Lch> for Hsl {
	fn from(lch: Lch) -> Hsl {
		Rgb::from(lch).into()
	}
}

impl From<Hsv> for Hsl {
    fn from(hsv: Hsv) -> Hsl {
    	let x = (2.0 - hsv.saturation) * hsv.value;
    	let saturation = if hsv.value == 0.0 {
    		0.0
    	} else if x < 1.0 {
    		hsv.saturation * hsv.value / x
    	} else {
    		hsv.saturation * hsv.value / (2.0 - x)
    	};

    	Hsl {
    		hue: hsv.hue,
    		saturation: saturation,
    		lightness: x / 2.0,
    		alpha: hsv.alpha,
    	}
    }
}

#[cfg(test)]
mod test {
    use super::Hsl;
    use ::{Rgb, Hsv};

    #[test]
    fn red() {
        let a = Hsl::from(Rgb::rgb(1.0, 0.0, 0.0));
        let b = Hsl::hsl(0.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::hsv(0.0.into(), 1.0, 1.0));

        assert_approx_eq!(a, b, [hue, saturation, lightness]);
        assert_approx_eq!(a, c, [hue, saturation, lightness]);
    }

    #[test]
    fn orange() {
        let a = Hsl::from(Rgb::rgb(1.0, 0.5, 0.0));
        let b = Hsl::hsl(30.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::hsv(30.0.into(), 1.0, 1.0));

        assert_approx_eq!(a, b, [hue, saturation, lightness]);
        assert_approx_eq!(a, c, [hue, saturation, lightness]);
    }

    #[test]
    fn green() {
        let a = Hsl::from(Rgb::rgb(0.0, 1.0, 0.0));
        let b = Hsl::hsl(120.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::hsv(120.0.into(), 1.0, 1.0));

        assert_approx_eq!(a, b, [hue, saturation, lightness]);
        assert_approx_eq!(a, c, [hue, saturation, lightness]);
    }

    #[test]
    fn blue() {
        let a = Hsl::from(Rgb::rgb(0.0, 0.0, 1.0));
        let b = Hsl::hsl(240.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::hsv(240.0.into(), 1.0, 1.0));

        assert_approx_eq!(a, b, [hue, saturation, lightness]);
        assert_approx_eq!(a, c, [hue, saturation, lightness]);
    }

    #[test]
    fn purple() {
        let a = Hsl::from(Rgb::rgb(0.5, 0.0, 1.0));
        let b = Hsl::hsl(270.0.into(), 1.0, 0.5);
        let c = Hsl::from(Hsv::hsv(270.0.into(), 1.0, 1.0));

        assert_approx_eq!(a, b, [hue, saturation, lightness]);
        assert_approx_eq!(a, c, [hue, saturation, lightness]);
    }
}
