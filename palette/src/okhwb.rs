#[cfg(feature = "approx")]
use crate::angle::{AngleEq, HalfRotation, SignedAngle};
use crate::num::{FromScalar, Hypot, Recip, Sqrt};
#[cfg(feature = "approx")]
use crate::visual::{VisualColor, VisuallyEqual};
use crate::white_point::D65;
#[cfg(feature = "approx")]
use crate::HasBoolMask;
use crate::{
    angle::RealAngle,
    convert::FromColorUnclamped,
    num::{Arithmetics, Cbrt, IsValidDivisor, One, Real, Trigonometry, Zero},
    Alpha, Okhsv, OklabHue,
};
#[cfg(feature = "approx")]
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
#[cfg(feature = "approx")]
use core::borrow::Borrow;
use core::fmt::Debug;

/// Okhwb with an alpha component. See the [`Okhwba` implementation in
/// `Alpha`](crate::Alpha#Okhwba).
pub type Okhwba<T = f32> = Alpha<Okhwb<T>, T>;

/// A Hue/Whiteness/Blackness representation of [`Oklab`] in the `sRGB` color space.
/// # See
/// https://bottosson.github.io/posts/colorpicker/#okhwb
#[derive(Debug, Copy, Clone, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "D65",
    component = "T",
    skip_derives(Okhwb, Okhsv)
)]
#[repr(C)]
pub struct Okhwb<T = f32> {
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

    /// The amount of white, mixed in the pure hue, ranging from `0.0` to `1.0`.
    /// `0.0` produces pure, possibly black color. `1.0` a white or grey.
    pub whiteness: T,

    /// The amount of black, mixed in the pure hue, ranging from `0.0` to `1.0`.
    /// `0.0` produces a pure bright or whitened color. `1.0` a black or grey.
    pub blackness: T,
}

impl<T> Okhwb<T> {
    /// Create an Okhwb color.
    pub fn new<H: Into<OklabHue<T>>>(hue: H, whiteness: T, blackness: T) -> Self {
        Self {
            hue: hue.into(),
            whiteness,
            blackness,
        }
    }

    /// Convert to a `(h, w, b)` tuple.
    pub fn into_components(self) -> (OklabHue<T>, T, T) {
        (self.hue, self.whiteness, self.blackness)
    }

    /// Convert from a `(h, w, b)` tuple.
    pub fn from_components<H: Into<OklabHue<T>>>((hue, whiteness, blackness): (H, T, T)) -> Self {
        Self::new(hue, whiteness, blackness)
    }
}

impl<T> FromColorUnclamped<Okhsv<T>> for Okhwb<T>
where
    T: Real
        + Copy
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
    /// Converts `lab` to `Okhwb` in the bounds of sRGB.
    ///
    /// # See
    /// https://bottosson.github.io/posts/colorpicker/#okhwb
    /// See [`srgb_to_okhwb`](https://bottosson.github.io/posts/colorpicker/#hwb-2).
    /// This implementation differs from srgb_to_okhwb in that it starts with the `lab`
    /// value and produces hues in degrees, whereas `srgb_to_okhwb` produces degree/360.
    fn from_color_unclamped(hsv: Okhsv<T>) -> Self {
        Self::new(
            hsv.hue,
            (T::one() - hsv.saturation) * hsv.value,
            T::one() - hsv.value,
        )
    }
}

impl_eq_hue!(Okhwb, OklabHue, [hue, whiteness, blackness]);

#[cfg(feature = "approx")]
impl<T> VisualColor<T> for Okhwb<T>
where
    T: PartialOrd
        + Copy
        + HasBoolMask<Mask = bool>
        + AbsDiffEq<Epsilon = T>
        + One
        + Zero
        + Arithmetics,
    T::Epsilon: Clone,
    OklabHue<T>: AbsDiffEq<Epsilon = T::Epsilon>,
{
    /// Returns `true`, if `self.blackness + self.whiteness >= 1`,
    /// assuming (but not asserting) that neither
    /// `blackness` nor `whiteness` can be negative.
    fn is_grey(&self, epsilon: T::Epsilon) -> bool {
        let wb_sum = self.blackness + self.whiteness;
        wb_sum > T::one() || wb_sum.abs_diff_eq(&T::one(), epsilon)
    }

    /// Returns `true`, if `Self::is_grey && blackness == 0`,
    /// i.e. the color's hue is irrelevant **and** the color contains
    /// no black component it must be white.
    fn is_white(&self, epsilon: T::Epsilon) -> bool {
        self.is_grey(epsilon.clone()) && self.blackness < epsilon
    }

    /// Returns `true` if `Self::is_grey && whiteness == 0`
    fn is_black(&self, epsilon: T::Epsilon) -> bool {
        self.is_grey(epsilon.clone()) && self.whiteness < epsilon
    }
}

#[cfg(feature = "approx")]
impl<S, O, T> VisuallyEqual<O, S, T> for Okhwb<T>
where
    T: PartialOrd
        + Copy
        + HasBoolMask<Mask = bool>
        + RealAngle
        + SignedAngle
        + Zero
        + One
        + AngleEq<Mask = bool>
        + Arithmetics
        + AbsDiffEq<Epsilon = T>
        + Clone,
    T::Epsilon: Clone + HalfRotation,
    S: Borrow<Self> + Copy,
    O: Borrow<Self> + Copy,
{
    fn visually_eq(s: S, o: O, epsilon: T::Epsilon) -> bool {
        VisuallyEqual::both_black_or_both_white(s, o, epsilon.clone())
            || VisuallyEqual::both_greyscale(s, o, epsilon.clone())
                && s.borrow()
                    .whiteness
                    .abs_diff_eq(&o.borrow().whiteness, epsilon.clone())
                && s.borrow()
                    .blackness
                    .abs_diff_eq(&o.borrow().blackness, epsilon.clone())
            || s.borrow().hue.abs_diff_eq(&o.borrow().hue, epsilon.clone())
                && s.borrow()
                    .blackness
                    .abs_diff_eq(&o.borrow().blackness, epsilon.clone())
                && s.borrow()
                    .whiteness
                    .abs_diff_eq(&o.borrow().whiteness, epsilon)
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::FromColorUnclamped;
    use crate::rgb::Rgb;
    use crate::visual::VisuallyEqual;
    use crate::{encoding, LinSrgb, Okhsv, Okhwb, Oklab};
    #[test]
    fn test_roundtrip_okhwb_oklab_is_original() {
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

        // unlike in okhsv and okhsl (but like in olhsf::tests::blue)
        // we are using f32 here, which for some AMD CPUs is
        // broken already in the reference implementation.
        //
        // We need a huge tolerance. Even this tolerance works only because
        // `crate::ok_utils::max_saturation::MAX_ITER == 1` accidentally hides the
        // real error. If raised the error becomes much larger.
        //FIXME: Fix the error and use a small tolerance
        const EPSILON: f32 = 1e-1;

        for (name, color) in colors {
            let rgb: Rgb<encoding::Srgb, u8> =
                crate::Srgb::<f32>::from_color_unclamped(color).into_format();
            println!(
                "\n\
            roundtrip of {} (#{:x} / {:?})\n\
            =================================================",
                name, rgb, color
            );

            let okhsv = Okhsv::from_color_unclamped(color);
            println!("Okhsv: {:?}", okhsv);
            let okhwb_from_okhsv = Okhwb::from_color_unclamped(okhsv);
            let okhwb = Okhwb::from_color_unclamped(color);
            println!("Okhwb: {:?}", okhwb);
            assert!(Okhwb::visually_eq(okhwb, okhwb_from_okhsv, f32::EPSILON));
            let okhsv_from_okhwb = Okhsv::from_color_unclamped(okhwb);
            assert!(Okhsv::visually_eq(okhsv, okhsv_from_okhwb, f32::EPSILON));

            let roundtrip_color = Oklab::from_color_unclamped(okhwb);
            let oklab_from_okhsv = Oklab::from_color_unclamped(okhsv);
            assert!(Oklab::visually_eq(
                roundtrip_color,
                oklab_from_okhsv,
                f32::EPSILON
            ));
            assert!(
                Oklab::visually_eq(roundtrip_color, color, EPSILON),
                "'{}' failed. {:?} != {:?}",
                name,
                roundtrip_color,
                color
            );
        }
    }
}
