#[cfg(feature = "approx")]
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use core::fmt::Debug;
#[cfg(feature = "approx")]
use core::ops::Neg;

use crate::angle::AngleEq;
use crate::num::{FromScalar, Hypot, Powi, Recip, Sqrt};
use crate::ok_utils::{toe, ChromaValues};
use crate::white_point::D65;
use crate::{
    angle::RealAngle,
    convert::FromColorUnclamped,
    num::{Arithmetics, Cbrt, IsValidDivisor, MinMax, One, Real, Trigonometry, Zero},
    Alpha, HasBoolMask, Oklab, OklabHue,
};

/// Okhsl with an alpha component.
pub type Okhsla<T = f32> = Alpha<Okhsl<T>, T>;

///<span id="Okhsla"></span>[`Okhsla`](crate::Okhsla) implementations.
impl<T, A> Alpha<Okhsl<T>, A> {
    /// Create an Oklab color with transparency.
    pub fn new<H: Into<OklabHue<T>>>(hue: H, saturation: T, lightness: T, alpha: A) -> Self {
        Alpha {
            color: Okhsl::new(hue, saturation, lightness),
            alpha,
        }
    }

    /// Convert to a `(hue, saturation, lightness, alpha)` tuple.
    pub fn into_components(self) -> (OklabHue<T>, T, T, A) {
        (
            self.color.hue,
            self.color.saturation,
            self.color.lightness,
            self.alpha,
        )
    }

    /// Convert from a `(hue, saturation, lightness, alpha)` tuple.
    pub fn from_components((hue, saturation, lightness, alpha): (OklabHue<T>, T, T, A)) -> Self {
        Self::new(hue, saturation, lightness, alpha)
    }
}

/// A Hue/Saturation/Lightness representation of [`Oklab`] in the `sRGB` color space.
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
    /// * `1.0` corresponds to white
    ///
    /// `Okhsv`'s `value` component goes from black to non-black
    /// -- a maximally bright color.
    ///
    /// `Okhsl`'s `lightness` component goes from black to white.
    pub lightness: T,
}

impl<T> Copy for Okhsl<T> where T: Copy {}

impl<T> Clone for Okhsl<T>
where
    T: Clone,
{
    fn clone(&self) -> Okhsl<T> {
        Okhsl {
            hue: self.hue.clone(),
            saturation: self.saturation.clone(),
            lightness: self.lightness.clone(),
        }
    }
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
#[cfg(feature = "approx")]
impl<T> Okhsl<T>
where
    T: PartialOrd + HasBoolMask<Mask = bool> + One + Zero + AbsDiffEq<Epsilon = T>,
    T::Epsilon: Clone + Neg<Output = T::Epsilon>,
    OklabHue<T>: AbsDiffEq<Epsilon = T::Epsilon>,
{
    /// Returns true, if `saturation == 0`
    #[allow(dead_code)]
    pub(crate) fn is_grey(&self, epsilon: T::Epsilon) -> bool {
        debug_assert!(self.saturation >= -epsilon.clone());
        self.saturation.abs_diff_eq(&T::zero(), epsilon)
    }

    /// Returns true, if `self.lightness >= 1`,
    /// i.e. colors outside the sRGB gamut are also considered white
    pub(crate) fn is_white(&self, epsilon: T::Epsilon) -> bool {
        self.lightness > T::one() || self.lightness.abs_diff_eq(&T::one(), epsilon)
    }

    /// Returns true, if `self.lightness == 0`.
    pub(crate) fn is_black(&self, epsilon: T::Epsilon) -> bool {
        debug_assert!(self.lightness >= -epsilon.clone());
        self.lightness <= epsilon
    }

    /// Returns true, if `self` and `other` are either both white or both black
    fn both_black_or_both_white(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        self.is_white(epsilon.clone()) && other.is_white(epsilon.clone())
            || self.is_black(epsilon.clone()) && other.is_black(epsilon)
    }
}

impl<T> PartialEq for Okhsl<T>
where
    T: PartialEq,
    OklabHue<T>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.hue == other.hue
            && self.saturation == other.saturation
            && self.lightness == other.lightness
    }
}
impl<T> Eq for Okhsl<T>
where
    T: Eq + AngleEq,
    OklabHue<T>: Eq,
{
}
#[cfg(feature = "approx")]
impl<T> AbsDiffEq for Okhsl<T>
where
    T: PartialOrd + HasBoolMask<Mask = bool> + One + Zero + AbsDiffEq<Epsilon = T>,
    T::Epsilon: Clone + Neg<Output = T::Epsilon>,
    OklabHue<T>: AbsDiffEq<Epsilon = T::Epsilon>,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    /// Returns true, if `self ` and `other` are visually indiscernible, even
    /// if they hold are both black or both white and their `hue` and
    /// `saturation` values differ.
    ///
    /// `epsilon` must be large enough to detect white (see [Oklab::is_white])
    fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        self.both_black_or_both_white(other, epsilon.clone())
            || self.hue.abs_diff_eq(&other.hue, epsilon.clone())
                && self
                    .saturation
                    .abs_diff_eq(&other.saturation, epsilon.clone())
                && self.lightness.abs_diff_eq(&other.lightness, epsilon)
    }
    fn abs_diff_ne(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        !self.both_black_or_both_white(other, epsilon.clone())
            && (self.hue.abs_diff_ne(&other.hue, epsilon.clone())
                || self
                    .saturation
                    .abs_diff_ne(&other.saturation, epsilon.clone())
                || self.lightness.abs_diff_ne(&other.lightness, epsilon))
    }
}
#[cfg(feature = "approx")]
impl<T> RelativeEq for Okhsl<T>
where
    T: PartialOrd + HasBoolMask<Mask = bool> + One + Zero + RelativeEq + AbsDiffEq<Epsilon = T>,
    T::Epsilon: Clone + Neg<Output = T::Epsilon>,
    OklabHue<T>: RelativeEq + AbsDiffEq<Epsilon = T::Epsilon>,
{
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
        self.both_black_or_both_white(other, epsilon.clone())
            || self
                .hue
                .relative_eq(&other.hue, epsilon.clone(), max_relative.clone())
                && self.saturation.relative_eq(
                    &other.saturation,
                    epsilon.clone(),
                    max_relative.clone(),
                )
                && self
                    .lightness
                    .relative_eq(&other.lightness, epsilon, max_relative)
    }
    fn relative_ne(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
        !self.both_black_or_both_white(other, epsilon.clone())
            && (self
                .hue
                .relative_ne(&other.hue, epsilon.clone(), max_relative.clone())
                || self.saturation.relative_ne(
                    &other.saturation,
                    epsilon.clone(),
                    max_relative.clone(),
                )
                || self
                    .lightness
                    .relative_ne(&other.lightness, epsilon, max_relative))
    }
}
#[cfg(feature = "approx")]
impl<T> UlpsEq for Okhsl<T>
where
    T: PartialOrd + HasBoolMask<Mask = bool> + One + Zero + UlpsEq + AbsDiffEq<Epsilon = T>,
    T::Epsilon: Clone + Neg<Output = T::Epsilon>,
    OklabHue<T>: UlpsEq + AbsDiffEq<Epsilon = T::Epsilon>,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        self.both_black_or_both_white(other, epsilon.clone())
            || self.hue.ulps_eq(&other.hue, epsilon.clone(), max_ulps)
                && self
                    .saturation
                    .ulps_eq(&other.saturation, epsilon.clone(), max_ulps)
                && self.lightness.ulps_eq(&other.lightness, epsilon, max_ulps)
    }
    fn ulps_ne(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        !self.both_black_or_both_white(other, epsilon.clone())
            && (self.hue.ulps_ne(&other.hue, epsilon.clone(), max_ulps)
                || self
                    .saturation
                    .ulps_ne(&other.saturation, epsilon.clone(), max_ulps)
                || self.lightness.ulps_ne(&other.lightness, epsilon, max_ulps))
    }
}
/// # See
/// See [`srgb_to_okhsl`](https://bottosson.github.io/posts/colorpicker/#hsl-2)
impl<T> FromColorUnclamped<Oklab<T>> for Okhsl<T>
where
    T: Real
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
    use core::str::FromStr;

    use crate::convert::FromColorUnclamped;
    use crate::rgb::Rgb;
    use crate::{encoding, LinSrgb, Okhsl, Oklab, Srgb};

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
            (
                "white",
                Oklab::from_color_unclamped(LinSrgb::new(1.0, 1.0, 1.0)),
            ),
        ];

        // unlike in okhwb we are using f64 here, which actually works.
        // So we can afford a small tolerance.
        // For some reason the roundtrip of Okhsl seems to produce a greater
        // divergence than the round trip of Okhsv (1e-8 vs 1e-10)
        const EPSILON: f64 = 1e-8;

        for (name, color) in colors {
            let rgb: Rgb<encoding::Srgb, u8> =
                crate::Srgb::<f64>::from_color_unclamped(color).into_format();
            println!(
                "\n\
            roundtrip of {} (#{:x} / {:?})\n\
            =================================================",
                name, rgb, color
            );

            println!("Color is white: {}", color.is_white(EPSILON));

            let okhsl = Okhsl::from_color_unclamped(color);
            println!("Okhsl: {:?}", okhsl);
            let roundtrip_color = Oklab::from_color_unclamped(okhsl);
            assert!(
                relative_eq!(roundtrip_color, color, epsilon = EPSILON),
                "'{}' failed.\n{:?}\n!=\n{:?}",
                name,
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
            "{}\n!=\n{}",
            okhsl.hue.into_raw_degrees(),
            360.0 * 0.7334778365225699
        );
        assert!(
            abs_diff_eq!(okhsl.saturation, 0.9999999897262261, epsilon = 1e-8),
            "{}\n!=\n{}",
            okhsl.saturation,
            0.9999999897262261
        );
        assert!(
            abs_diff_eq!(okhsl.lightness, 0.366565335813274, epsilon = 1e-10),
            "{}\n!=\n{}",
            okhsl.lightness,
            0.366565335813274
        );
    }

    #[test]
    fn test_srgb_to_okhsl() {
        let red_hex = "#834941";
        let rgb: Srgb<f64> = Srgb::from_str(red_hex).unwrap().into_format();
        let lin_rgb = LinSrgb::<f64>::from_color_unclamped(rgb);
        let oklab = Oklab::from_color_unclamped(lin_rgb);
        println!(
            "RGB: {:?}\n\
            LinRgb: {:?}\n\
            Oklab: {:?}",
            rgb, lin_rgb, oklab
        );
        let okhsl = Okhsl::from_color_unclamped(oklab);

        // test data from Ok Color picker
        assert_relative_eq!(
            okhsl.hue.into_raw_degrees(),
            360.0 * 0.07992730371382328,
            epsilon = 1e-10,
            max_relative = 1e-13
        );
        assert_relative_eq!(okhsl.saturation, 0.4629217183454986, epsilon = 1e-10);
        assert_relative_eq!(okhsl.lightness, 0.3900998146147427, epsilon = 1e-10);
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
