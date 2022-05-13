use crate::white_point::D65;
use approx::AbsDiffEq;
use core::fmt::Debug;

use crate::num::{FromScalar, Hypot, Powi, Recip, Sqrt};
use crate::ok_utils::{toe, ChromaValues};
use crate::{
    angle::RealAngle,
    convert::FromColorUnclamped,
    num::{Arithmetics, Cbrt, IsValidDivisor, MinMax, One, Real, Trigonometry, Zero},
    Alpha, HasBoolMask, Oklab, OklabHue,
};

/// Okhsl with an alpha component.
pub type Okhsla<T = f32> = Alpha<Okhsl<T>, T>;

/// A Hue/Saturation/Lightness representation of [`Oklab`].
///
/// Allows
/// * changing hue/chroma/saturation, while keeping perceived lightness constant (like HSLuv)
/// * changing lightness/chroma/saturation, while keeping perceived hue constant
/// * changing the perceived saturation (more or less) proportionally with the numerical
/// amount of change (unlike HSLuv)
#[derive(Debug, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "D65",
    component = "T",
    skip_derives(Oklab)
)]
#[repr(C)]
pub struct Okhsl<T = f32> {
    /// The hue of the color, in degrees of a circle, where for all `h`: `h+n*360 ==  h`.
    ///
    /// For fully saturated, bright colors
    /// * 0° corresponds to a kind of magenta-pink (RBG #ff0188),
    /// * 90° to a kind of yellow (RBG RGB #ffcb00)
    /// * 180° to a kind of cyan (RBG #00ffe1) and
    /// * 240° to a kind of blue (RBG #00aefe).
    ///
    /// For s == 0 or v == 0, the hue is irrelevant.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: OklabHue<T>,

    /// The saturation (freedom of black or white) of the color.
    ///
    /// * `0.0` corresponds to pure mixture of black and white without any color.
    /// The black to white relation depends on v.
    /// * `1.0` to a fully saturated color without any white.
    ///
    /// For v == 0 the saturation is irrelevant.
    pub saturation: T,

    /// The amount of black and white "paint in the mixture".
    /// While changes do not affect the saturation, they do affect
    /// * `0.0` corresponds to pure black
    /// * `1.0` corresponds to a maximally bright colour
    pub lightness: T,
}

impl<T> Okhsl<T> {
    /// Create an Okhsl color.
    pub fn new<H: Into<OklabHue<T>>>(hue: H, saturation: T, lightness: T) -> Self {
        Self {
            hue: hue.into(),
            saturation,
            lightness,
        }
    }

    /// Convert to a `(h, s, l)` tuple.
    pub fn into_components(self) -> (OklabHue<T>, T, T) {
        (self.hue, self.saturation, self.lightness)
    }

    /// Convert from a `(h, s, l)` tuple.
    pub fn from_components<H: Into<OklabHue<T>>>((hue, saturation, lightness): (H, T, T)) -> Self {
        Self::new(hue, saturation, lightness)
    }
}

/// # See
/// See [`srgb_to_okhsl`](https://bottosson.github.io/posts/colorpicker/#hsl-2)
impl<T> FromColorUnclamped<Oklab<T>> for Okhsl<T>
where
    T: Real
        + Debug
        + AbsDiffEq
        + One
        + Zero
        + Arithmetics
        + Sqrt
        + MinMax
        + Copy
        + PartialOrd
        + HasBoolMask<Mask = bool>
        + Powi
        + Cbrt
        + Hypot
        + Trigonometry
        + RealAngle
        + FromScalar,
    T::Scalar: Real
        + Zero
        + One
        + Recip
        + IsValidDivisor<Mask = bool>
        + Arithmetics
        + Clone
        + FromScalar<Scalar = T::Scalar>,
{
    fn from_color_unclamped(lab: Oklab<T>) -> Self {
        let l = toe(lab.l);

        if lab.a == T::zero() && lab.b == T::zero() {
            // `a` describes how green/red the color is, `b` how blue/yellow the color is
            // both are zero -> the color is totally desaturated.
            return Self::new(T::zero(), T::zero(), l);
        }

        let chroma = T::hypot(lab.a, lab.b);
        let a_ = lab.a / chroma;
        let b_ = lab.b / chroma;

        // use negative a and be and rotate, to ensure hue is normalized
        let h = T::from_f64(180.0) + T::atan2(-lab.b, -lab.a).radians_to_degrees();

        let cs = ChromaValues::from_normalized(lab.l, a_, b_);

        // Inverse of the interpolation in okhsl_to_srgb:

        let mid = T::from_f64(0.8);
        let mid_inv = T::from_f64(1.25);

        let s = if chroma < cs.mid {
            let k_1 = mid * cs.zero;
            let k_2 = T::one() - k_1 / cs.mid;

            let t = chroma / (k_1 + k_2 * chroma);
            t * mid
        } else {
            let k_0 = cs.mid;
            let k_1 = (T::one() - mid) * (cs.mid * mid_inv).powi(2) / cs.zero;
            let k_2 = T::one() - (k_1) / (cs.max - cs.mid);

            let t = (chroma - k_0) / (k_1 + k_2 * (chroma - k_0));
            mid + (T::one() - mid) * t
        };

        Self::new(h, s, l)
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::FromColorUnclamped;
    use crate::rgb::Rgb;
    use crate::{encoding, LinSrgb, Okhsl, Oklab, Srgb};
    use std::str::FromStr;

    #[test]
    fn test_roundtrip_okhsl_oklab_is_original() {
        let colors = [
            (
                "red",
                Oklab::from_color_unclamped(LinSrgb::new(1.0, 0.0, 0.0)),
            ),
            (
                "green",
                Oklab::from_color_unclamped(LinSrgb::new(0.0, 1.0, 0.0)),
            ),
            (
                "cyan",
                Oklab::from_color_unclamped(LinSrgb::new(0.0, 1.0, 1.0)),
            ),
            (
                "magenta",
                Oklab::from_color_unclamped(LinSrgb::new(1.0, 0.0, 1.0)),
            ),
            (
                "white",
                Oklab::from_color_unclamped(LinSrgb::new(1.0, 1.0, 1.0)),
            ),
            (
                "black",
                Oklab::from_color_unclamped(LinSrgb::new(0.0, 0.0, 0.0)),
            ),
            (
                "grey",
                Oklab::from_color_unclamped(LinSrgb::new(0.5, 0.5, 0.5)),
            ),
            (
                "yellow",
                Oklab::from_color_unclamped(LinSrgb::new(1.0, 1.0, 0.0)),
            ),
            (
                "blue",
                Oklab::from_color_unclamped(LinSrgb::new(0.0, 0.0, 1.0)),
            ),
        ];
        for (name, color) in colors {
            let rgb: Rgb<encoding::Srgb, u8> =
                crate::Srgb::<f32>::from_color_unclamped(color).into_format();
            println!(
                "\n\
            roundtrip of {name} (#{:x} / {:?})\n\
            =================================================",
                rgb, color
            );

            let okhsl = Okhsl::from_color_unclamped(color);
            println!("Okhsl: {:?}", okhsl);
            let roundtrip_color = Oklab::from_color_unclamped(okhsl);
            assert!(
                relative_eq!(roundtrip_color, color, epsilon = 1e-4),
                "'{name}' failed. {:?} != {:?}",
                roundtrip_color,
                color
            );
        }
    }

    #[test]
    fn test_blue() {
        let lab = Oklab::new(
            0.45201371519623734_f64,
            -0.03245697990291002,
            -0.3115281336419824,
        );
        let okhsl = Okhsl::<f64>::from_color_unclamped(lab);
        assert!(
            abs_diff_eq!(
                okhsl.hue.into_raw_degrees(),
                360.0 * 0.7334778365225699,
                epsilon = 1e-10
            ),
            "{} != {}",
            okhsl.hue.into_raw_degrees(),
            360.0 * 0.7334778365225699
        );
        assert!(
            abs_diff_eq!(okhsl.saturation, 0.9999999897262261, epsilon = 1e-10),
            "{} != {}",
            okhsl.saturation,
            0.9999999897262261
        );
        assert!(
            abs_diff_eq!(okhsl.lightness, 0.366565335813274, epsilon = 1e-10),
            "{} != {}",
            okhsl.lightness,
            0.366565335813274
        );
    }

    #[test]
    fn test_srgb_to_okhsl() {
        let red_hex = "#834941";
        let rgb: Srgb<f64> = Srgb::from_str(red_hex).unwrap().into_format();
        let lin_rgb = LinSrgb::<f64>::from_color_unclamped(rgb);
        // FIXME: test data from Ok Color picker, that's hex to lin-rgb slightly differs.
        let lin_rgb = LinSrgb::new(
            0.22696587351009836,
            0.06662593864377289,
            0.052860647023180246,
        );
        let oklab = Oklab::from_color_unclamped(lin_rgb);
        println!(
            "RGB: {rgb:?}\n\
        LinRgb: {lin_rgb:?}\n\
        Oklab: {oklab:?}"
        );
        let okhsl = Okhsl::from_color_unclamped(oklab);

        // test data from Ok Color picker
        // FIXME: results strangely are not very similar. Is there a way to reduce the epsilons?
        assert_relative_eq!(
            okhsl.hue.into_raw_degrees(),
            360.0 * 0.07992730371382328,
            epsilon = 1e-3,
            max_relative = 1e-3
        );
        assert_relative_eq!(okhsl.saturation, 0.4629217183454986, epsilon = 1e-4);
        assert_relative_eq!(okhsl.lightness, 0.3900998146147427, epsilon = 1e-4);
    }

    #[test]
    fn test_okhsl_to_srgb() {
        let okhsl = Okhsl::new(0.0_f32, 0.5, 0.5);
        let oklab = Oklab::from_color_unclamped(okhsl);
        let rgb = Srgb::from_color_unclamped(oklab);
        let rgb8: Rgb<encoding::Srgb, u8> = rgb.into_format();
        let hex_str = format!("{:x}", rgb8);
        assert_eq!(hex_str, "aa5a74");
    }
}
