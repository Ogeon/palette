use core::fmt::Debug;
use std::ops::{Mul, Neg, Sub};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use crate::angle::{AngleEq, HalfRotation, SignedAngle};
use crate::convert::IntoColorUnclamped;
use crate::num::{FromScalar, Hypot, Powi, Recip, Sqrt};
use crate::ok_utils::{LC, ST};
use crate::white_point::D65;
use crate::{
    angle::RealAngle,
    convert::FromColorUnclamped,
    num::{Arithmetics, Cbrt, IsValidDivisor, MinMax, One, Real, Trigonometry, Zero},
    ok_utils, Alpha, HasBoolMask, LinSrgb, Okhwb, Oklab, OklabHue,
};

/// Okhsv with an alpha component. See the [`Okhsva` implementation in
/// `Alpha`](crate::Alpha#Okhsva).
pub type Okhsva<T = f32> = Alpha<Okhsv<T>, T>;

/// A Hue/Saturation/Value representation of [`Oklab`] in the `sRGB` color space.
///
/// Allows
/// * changing lightness/chroma/saturation while keeping perceived Hue constant
/// (like HSV promises but delivers only partially)  
/// * finding the strongest color (maximum chroma) at s == 1 (like HSV)  
#[derive(Debug, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "D65",
    component = "T",
    skip_derives(Oklab, Okhwb)
)]
#[repr(C)]
pub struct Okhsv<T = f32> {
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

    /// The saturation (freedom of whitishness) of the color.
    ///
    /// * `0.0` corresponds to pure mixture of black and white without any color.
    /// The black to white relation depends on v.
    /// * `1.0` to a fully saturated color without any white.
    ///
    /// For v == 0 the saturation is irrelevant.
    pub saturation: T,

    /// The monochromatic brightness of the color.
    /// * `0.0` corresponds to pure black
    /// * `1.0` corresponds to a maximally bright colour -- be it very colorful or very  white
    ///
    /// `Okhsl`'s `lightness` component goes from black to white.
    /// `Okhsv`'s `value` component goes from black to non-black -- a maximally bright color..
    pub value: T,
}
//impl_eq!(Okhsv, [hue, saturation, value]);
impl<T> Okhsv<T>
where
    T: PartialOrd
        + HasBoolMask<Mask = bool>
        + AbsDiffEq<Epsilon = T>
        + One
        + Zero
        + Neg<Output = T>,
    T::Epsilon: Clone,
    OklabHue<T>: AbsDiffEq<Epsilon = T::Epsilon>,
{
    /// Returns true, if `saturation == 0`
    pub fn is_grey(&self, epsilon: T::Epsilon) -> bool {
        self.saturation.abs_diff_eq(&T::zero(), epsilon)
    }

    /// Returns true, if `Self::is_grey` && `value >= 1`,
    /// i.e. the color's hue is irrelevant **and** it is at or beyond the
    /// `sRGB` maximum brightness. A color at or beyond maximum brightness isn't
    /// necessarily white. It can also be a bright shining hue.
    pub fn is_white(&self, epsilon: T::Epsilon) -> bool {
        self.is_grey(epsilon.clone()) && self.value > T::one()
            || self.value.abs_diff_eq(&T::one(), epsilon)
    }

    /// Returns true if `value == 0`
    pub fn is_black(&self, epsilon: T::Epsilon) -> bool {
        debug_assert!(self.value >= -epsilon.clone());
        self.value.abs_diff_eq(&T::zero(), epsilon)
    }

    /// Returns true, if `self` and `other` are either both white or both black
    fn both_black_or_both_white(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        self.is_white(epsilon.clone()) && other.is_white(epsilon.clone())
            || self.is_black(epsilon.clone()) && other.is_black(epsilon)
    }

    /// Returns true, if `self` and `other` are both fully desaturated
    fn both_greyscale(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        self.is_grey(epsilon.clone()) && other.is_grey(epsilon)
    }
}

impl<T> PartialEq for Okhsv<T>
where
    T: PartialEq,
    OklabHue<T>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.hue == other.hue && self.saturation == other.saturation && self.value == other.value
    }
}
impl<T> Eq for Okhsv<T>
where
    T: Eq,
    OklabHue<T>: Eq,
{
}
impl<T> AbsDiffEq for Okhsv<T>
where
    T: PartialOrd
        + HasBoolMask<Mask = bool>
        + RealAngle
        + SignedAngle
        + Zero
        + One
        + AngleEq<Mask = bool>
        + Sub<Output = T>
        + AbsDiffEq<Epsilon = T>
        + Neg<Output = T>
        + Clone,
    T::Epsilon: Clone + HalfRotation + Mul<Output = T::Epsilon>,
    OklabHue<T>: AbsDiffEq<Epsilon = T::Epsilon>,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        self.both_black_or_both_white(other, epsilon.clone())
            || self.both_greyscale(other, epsilon.clone())
                && self.value.abs_diff_eq(&other.value, epsilon.clone())
            || self.hue.abs_diff_eq(&other.hue, epsilon.clone())
                && self
                    .saturation
                    .abs_diff_eq(&other.saturation, epsilon.clone())
                && self.value.abs_diff_eq(&other.value, epsilon)
    }
    fn abs_diff_ne(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        !(self.both_black_or_both_white(other, epsilon.clone())
            || self.both_greyscale(other, epsilon.clone())
                && self.value.abs_diff_eq(&other.value, epsilon.clone()))
            && self.hue.abs_diff_ne(&other.hue, epsilon.clone())
            || self
                .saturation
                .abs_diff_ne(&other.saturation, epsilon.clone())
            || self.value.abs_diff_ne(&other.value, epsilon)
    }
}
impl<T> RelativeEq for Okhsv<T>
where
    T: PartialOrd
        + HasBoolMask<Mask = bool>
        + RealAngle
        + SignedAngle
        + Zero
        + One
        + AngleEq<Mask = bool>
        + Sub<Output = T>
        + RelativeEq<Epsilon = T>
        + Neg<Output = T>
        + Clone,
    T::Epsilon: Clone + HalfRotation + Mul<Output = T::Epsilon>,
    OklabHue<T>: RelativeEq + AbsDiffEq<Epsilon = T::Epsilon>,
{
    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
        self.both_black_or_both_white(other, epsilon.clone())
            || self.both_greyscale(other, epsilon.clone())
                && self.value.abs_diff_eq(&other.value, epsilon.clone())
            || self
                .hue
                .relative_eq(&other.hue, epsilon.clone(), max_relative.clone())
                && self.saturation.relative_eq(
                    &other.saturation,
                    epsilon.clone(),
                    max_relative.clone(),
                )
                && self.value.relative_eq(&other.value, epsilon, max_relative)
    }
    fn relative_ne(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
        !(self.both_black_or_both_white(other, epsilon.clone())
            || self.both_greyscale(other, epsilon.clone())
                && self.value.abs_diff_eq(&other.value, epsilon.clone()))
            && self
                .hue
                .relative_ne(&other.hue, epsilon.clone(), max_relative.clone())
            || self
                .saturation
                .relative_ne(&other.saturation, epsilon.clone(), max_relative.clone())
            || self.value.relative_ne(&other.value, epsilon, max_relative)
    }
}
impl<T> UlpsEq for Okhsv<T>
where
    T: PartialOrd
        + HasBoolMask<Mask = bool>
        + RealAngle
        + SignedAngle
        + Zero
        + One
        + AngleEq<Mask = bool>
        + Sub<Output = T>
        + UlpsEq<Epsilon = T>
        + Neg<Output = T>
        + Clone,
    T::Epsilon: Clone + HalfRotation + Mul<Output = T::Epsilon>,
    OklabHue<T>: UlpsEq + AbsDiffEq<Epsilon = T::Epsilon>,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        self.both_black_or_both_white(other, epsilon.clone())
            || self.both_greyscale(other, epsilon.clone())
                && self.value.abs_diff_eq(&other.value, epsilon.clone())
            || self.hue.ulps_eq(&other.hue, epsilon.clone(), max_ulps)
                && self
                    .saturation
                    .ulps_eq(&other.saturation, epsilon.clone(), max_ulps)
                && self.value.ulps_eq(&other.value, epsilon, max_ulps)
    }
    fn ulps_ne(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        !(self.both_black_or_both_white(other, epsilon.clone())
            || self.both_greyscale(other, epsilon.clone())
                && self.value.abs_diff_eq(&other.value, epsilon.clone()))
            && self.hue.ulps_ne(&other.hue, epsilon.clone(), max_ulps)
            || self
                .saturation
                .ulps_ne(&other.saturation, epsilon.clone(), max_ulps)
            || self.value.ulps_ne(&other.value, epsilon, max_ulps)
    }
}

impl<T> Copy for Okhsv<T> where T: Copy {}

impl<T> Clone for Okhsv<T>
where
    T: Clone,
{
    fn clone(&self) -> Okhsv<T> {
        Okhsv {
            hue: self.hue.clone(),
            saturation: self.saturation.clone(),
            value: self.value.clone(),
        }
    }
}

impl<T> Okhsv<T> {
    /// Create an Okhsv color.
    pub fn new<H: Into<OklabHue<T>>>(hue: H, saturation: T, value: T) -> Self {
        Self {
            hue: hue.into(),
            saturation,
            value,
        }
    }

    /// Convert to a `(h, s, v)` tuple.
    pub fn into_components(self) -> (OklabHue<T>, T, T) {
        (self.hue, self.saturation, self.value)
    }

    /// Convert from a `(h, s, v)` tuple.
    pub fn from_components<H: Into<OklabHue<T>>>((hue, saturation, value): (H, T, T)) -> Self {
        Self::new(hue, saturation, value)
    }
}

/// Converts `lab` to `Okhsv` in the bounds of sRGB.
///
/// # See
/// See [`srgb_to_okhsv`](https://bottosson.github.io/posts/colorpicker/#hsv-2).
/// This implementation differs from srgb_to_okhsv in that it starts with the `lab`
/// value and produces hues in degrees, whereas `srgb_to_okhsv` produces degree/360.
impl<T> FromColorUnclamped<Oklab<T>> for Okhsv<T>
where
    T: Real
        + AbsDiffEq
        + PartialOrd
        + HasBoolMask<Mask = bool>
        + MinMax
        + Copy
        + Powi
        + Sqrt
        + Cbrt
        + Arithmetics
        + Trigonometry
        + Zero
        + Hypot
        + One
        + FromScalar
        + Debug
        + RealAngle,
    T::Scalar: Real
        + Zero
        + One
        + Recip
        + Hypot
        + IsValidDivisor<Mask = bool>
        + Arithmetics
        + Clone
        + FromScalar<Scalar = T::Scalar>
        + Debug,
{
    fn from_color_unclamped(lab: Oklab<T>) -> Self {
        //println!("Lab {:?}, {:?}, {:?}", lab.l, lab.a, lab.b);
        if lab.l == T::zero() {
            // the color is pure black
            return Self::new(T::zero(), T::zero(), T::zero());
        }
        if lab.a == T::zero() && lab.b == T::zero() {
            // `a` describes how green/red the color is, `b` how blue/yellow the color is
            // both are zero -> the color is totally desaturated.
            let v = ok_utils::toe(lab.l);

            return Self::new(T::zero(), T::zero(), v);
        }

        // compute hue and chroma as for OkLCh
        // we will use hue as is.
        let chroma = T::hypot(lab.a, lab.b);
        let a_ = lab.a / chroma;
        let b_ = lab.b / chroma;

        // use negative a and be and rotate, to ensure hue is normalized
        let hue = T::from_f64(180.0) + T::atan2(-lab.b, -lab.a).radians_to_degrees();

        // For each hue the sRGB gamut can be drawn on a 2-dimensional space.
        // Let L_r, the lightness in relation to the possible luminance of sRGB, be spread
        // along the y-axis (bottom is black, top is bright) and Chroma along the x-axis
        // (left is desaturated, right is colorful). The gamut then takes a triangular shape,
        // with a concave top side and a cusp to the right.
        // To use saturation and brightness values, the gamut must be mapped to a square.
        // The lower point of the triangle is expanded to the lower side of the square.
        // The left side remains unchanged and the cusp of the triangle moves to the upper right.
        let cusp = LC::find_cusp(a_, b_);
        //println!("CSUP: L: {:?}, C: {:?}", cusp.lightness, cusp.chroma);
        let st_max: ST<T> = cusp.into();
        let s_0 = T::from_f64(0.5);
        let k = T::one() - s_0 / st_max.s;

        // first we find L_v, C_v, L_vt and C_vt
        let t = st_max.t / (chroma + lab.l * st_max.t);
        let l_v = t * lab.l;
        let c_v = t * chroma;

        let l_vt = ok_utils::toe_inv(l_v);
        let c_vt = c_v * l_vt / l_v;

        // we can then use these to invert the step that compensates for the toe and the curved top part of the triangle:
        let rgb_scale: LinSrgb<T> = Oklab::new(l_vt, a_ * c_vt, b_ * c_vt).into_color_unclamped();
        let lightness_scale_factor = T::cbrt(
            T::one()
                / T::max(
                    T::max(rgb_scale.red, rgb_scale.green),
                    T::max(rgb_scale.blue, T::zero()),
                ),
        );

        //chroma = chroma / lightness_scale_factor;

        // use L_r instead of L and also scale C by L_r/L
        let l_r = ok_utils::toe(lab.l / lightness_scale_factor);
        //chroma = chroma * l_r / (lab.l / lightness_scale_factor);

        // we can now compute v and s:
        let v = l_r / l_v;
        let s = (s_0 + st_max.t) * c_v / ((st_max.t * s_0) + st_max.t * k * c_v);

        Self::new(hue, s, v)
    }
}
impl<T> FromColorUnclamped<Okhwb<T>> for Okhsv<T>
where
    T: Real
        + AbsDiffEq
        + PartialOrd
        + MinMax
        + Copy
        + Powi
        + Sqrt
        + Cbrt
        + Arithmetics
        + Trigonometry
        + Zero
        + Hypot
        + One
        + FromScalar
        + Debug
        + RealAngle,
    T::Scalar: Real
        + Zero
        + One
        + Recip
        + Hypot
        + IsValidDivisor<Mask = bool>
        + Arithmetics
        + Clone
        + FromScalar<Scalar = T::Scalar>
        + Debug,
{
    fn from_color_unclamped(hwb: Okhwb<T>) -> Self {
        if hwb.blackness == T::one() {
            return Self::new(hwb.hue, T::zero(), T::zero());
        }
        Self::new(
            hwb.hue,
            T::one() - hwb.whiteness / (T::one() - hwb.blackness),
            T::one() - hwb.blackness,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::convert::FromColorUnclamped;
    use crate::rgb::Rgb;
    use crate::{encoding, LinSrgb, Okhsv, Oklab, OklabHue, Srgb};

    #[test]
    fn test_roundtrip_okhsv_oklab_is_original() {
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
                crate::Srgb::<f64>::from_color_unclamped(color).into_format();
            println!(
                "\n\
            roundtrip of {} (#{:x} / {:?})\n\
            =================================================",
                name, rgb, color
            );

            let okhsv = Okhsv::from_color_unclamped(color);
            println!("Okhsv: {:?}", okhsv);
            let roundtrip_color = Oklab::from_color_unclamped(okhsv);
            assert!(
                relative_eq!(roundtrip_color, color, epsilon = 1e-10),
                "'{}' failed.\n{:?}\n!=\n{:?}",
                name,
                roundtrip_color,
                color
            );
        }
    }

    #[test]
    fn blue() {
        let color = Oklab::<f64>::from_color_unclamped(LinSrgb::new(0.0, 0.0, 1.0));
        let okhsv = Okhsv::from_color_unclamped(color);
        println!("Okhsv f64: {:?}\n", okhsv);
        assert_abs_diff_eq!(okhsv.value, 1.0, epsilon = 1e-5);
        assert_abs_diff_eq!(okhsv.saturation, 1.0, epsilon = 1e-5);
        assert_abs_diff_eq!(okhsv.hue, OklabHue::new(264.05), epsilon = 1.0);

        let color = Oklab::<f32>::from_color_unclamped(LinSrgb::new(0.0, 0.0, 1.0));
        let okhsv = Okhsv::from_color_unclamped(color);
        println!("Okhsv f32: {:?}", okhsv);
        assert_abs_diff_eq!(okhsv.value, 1.0, epsilon = 1e-1);
        assert_abs_diff_eq!(okhsv.saturation, 1.0, epsilon = 1e-1);
        assert_abs_diff_eq!(okhsv.hue, OklabHue::new(264.05), epsilon = 1.0);
    }

    #[test]
    fn test_srgb_to_okhsv() {
        let red_hex = "#ff0004";
        let rgb: Srgb = Rgb::<encoding::Srgb, _>::from_str(red_hex)
            .unwrap()
            .into_format();
        let oklab = Oklab::from_color_unclamped(rgb);
        let okhsv = Okhsv::from_color_unclamped(oklab);
        assert_relative_eq!(okhsv.saturation, 1.0, epsilon = 1e-3);
        assert_relative_eq!(okhsv.value, 1.0, epsilon = 1e-3);
        assert_relative_eq!(
            okhsv.hue.into_raw_degrees(),
            29.0,
            epsilon = 1e-3,
            max_relative = 1e-3
        );
    }

    #[test]
    fn test_okhsv_to_srgb() {
        let okhsv = Okhsv::new(0.0_f32, 0.5, 0.5);
        let oklab = Oklab::from_color_unclamped(okhsv);
        let rgb = Srgb::from_color_unclamped(oklab);
        let rgb8: Rgb<encoding::Srgb, u8> = rgb.into_format();
        let hex_str = format!("{:x}", rgb8);
        assert_eq!(hex_str, "7a4355");
    }

    #[test]
    fn black_eq_different_black() {
        assert_abs_diff_eq!(
            Okhsv::from_color_unclamped(Oklab::new(0.0, 1.0, 0.0)),
            Okhsv::from_color_unclamped(Oklab::new(0.0, 0.0, 1.0)),
            epsilon = 1e-12
        );
    }

    #[test]
    fn white_eq_different_white() {
        assert_abs_diff_eq!(
            Okhsv::new(240.0, 0.0, 1.0),
            Okhsv::new(24.0, 0.0, 1.0),
            epsilon = 1e-12
        );
    }

    #[test]
    fn white_ne_grey_or_black() {
        assert_abs_diff_ne!(
            Okhsv::new(0.0, 0.0, 0.0),
            Okhsv::new(0.0, 0.0, 1.0),
            epsilon = 1e-12
        );
        assert_abs_diff_ne!(
            Okhsv::new(0.0, 0.0, 0.3),
            Okhsv::new(0.0, 0.0, 1.0),
            epsilon = 1e-12
        );
    }

    #[test]
    fn color_neq_different_color() {
        assert_abs_diff_ne!(
            Okhsv::new(10.0, 0.01, 0.5),
            Okhsv::new(11.0, 0.01, 0.5),
            epsilon = 1e-12
        );
        assert_abs_diff_ne!(
            Okhsv::new(10.0, 0.01, 0.5),
            Okhsv::new(10.0, 0.02, 0.5),
            epsilon = 1e-12
        );
        assert_abs_diff_ne!(
            Okhsv::new(10.0, 0.01, 0.5),
            Okhsv::new(10.0, 0.01, 0.6),
            epsilon = 1e-12
        );
    }

    #[test]
    fn grey_vs_grey() {
        // greys of different lightness are not equal
        assert_abs_diff_ne!(
            Okhsv::new(0.0, 0.0, 0.3),
            Okhsv::new(0.0, 0.0, 0.4),
            epsilon = 1e-12
        );
        // greys of same lightness but different hue are equal
        assert_abs_diff_eq!(
            Okhsv::new(0.0, 0.0, 0.3),
            Okhsv::new(12.0, 0.0, 0.3),
            epsilon = 1e-12
        );
    }
}
