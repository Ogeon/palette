use crate::white_point::D65;
use core::fmt::Debug;

use crate::num::{FromScalar, Hypot, Recip, Sqrt};
use crate::{
    angle::RealAngle,
    convert::FromColorUnclamped,
    num::{Arithmetics, Cbrt, IsValidDivisor, One, Real, Trigonometry, Zero},
    Alpha, Okhsv, OklabHue,
};

/// Okhwb with an alpha component. See the [`Okhwba` implementation in
/// `Alpha`](crate::Alpha#Okhwba).
pub type Okhwba<T = f32> = Alpha<Okhwb<T>, T>;

/// A Hue/Whiteness/Blackness representation of [`Oklab`].
/// # See
/// https://bottosson.github.io/posts/colorpicker/#okhwb
#[derive(Debug, ArrayCast, FromColorUnclamped, WithAlpha)]
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

impl<T> Copy for Okhwb<T> where T: Copy {}

impl<T> Clone for Okhwb<T>
where
    T: Clone,
{
    fn clone(&self) -> Okhwb<T> {
        Okhwb {
            hue: self.hue.clone(),
            whiteness: self.whiteness.clone(),
            blackness: self.blackness.clone(),
        }
    }
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

/// Converts `lab` to `Okhwb` in the bounds of sRGB.
///
/// # See
/// See [`srgb_to_okhwb`](https://bottosson.github.io/posts/colorpicker/#hwb-2).
/// This implementation differs from srgb_to_okhwb in that it starts with the `lab`
/// value and produces hues in degrees, whereas `srgb_to_okhwb` produces degree/360.
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
    fn from_color_unclamped(hsv: Okhsv<T>) -> Self {
        Self::new(
            hsv.hue,
            (T::one() - hsv.saturation) * hsv.value,
            T::one() - hsv.value,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::FromColorUnclamped;
    use crate::rgb::Rgb;
    use crate::{encoding, LinSrgb, Okhwb, Oklab};

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
        for (name, color) in colors {
            let rgb: Rgb<encoding::Srgb, u8> =
                crate::Srgb::<f32>::from_color_unclamped(color).into_format();
            println!(
                "\n\
            roundtrip of {name} (#{:x} / {:?})\n\
            =================================================",
                rgb, color
            );

            let okhwb = Okhwb::from_color_unclamped(color);
            println!("Okhwb: {:?}", okhwb);
            let roundtrip_color = Oklab::from_color_unclamped(okhwb);
            assert!(
                relative_eq!(roundtrip_color, color, epsilon = 1e-3),
                "'{name}' failed. {:?} != {:?}",
                roundtrip_color,
                color
            );
        }
    }
}
