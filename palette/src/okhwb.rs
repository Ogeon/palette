use core::fmt::Debug;

pub use alpha::Okhwba;

use crate::angle::{FromAngle, RealAngle};
use crate::num::{FromScalar, Hypot, Recip, Sqrt};
use crate::stimulus::{FromStimulus, Stimulus};
use crate::white_point::D65;
use crate::HasBoolMask;
use crate::{
    convert::FromColorUnclamped,
    num::{Arithmetics, Cbrt, IsValidDivisor, One, Real, Trigonometry, Zero},
    Okhsv, OklabHue,
};

mod alpha;
mod properties;
#[cfg(feature = "random")]
mod random;

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
    /// Create an `Okhwb` color.
    pub fn new<H: Into<OklabHue<T>>>(hue: H, whiteness: T, blackness: T) -> Self {
        let hue = hue.into();
        Self {
            hue,
            whiteness,
            blackness,
        }
    }

    /// Create an `Okhwb` color. This is the same as `Okhwb::new` without the
    /// generic hue type. It's temporary until `const fn` supports traits.
    pub fn new_const(hue: OklabHue<T>, whiteness: T, blackness: T) -> Self {
        Self {
            hue,
            whiteness,
            blackness,
        }
    }
    /// Convert into another component type.
    pub fn into_format<U>(self) -> Okhwb<U>
    where
        U: FromStimulus<T> + FromAngle<T>,
    {
        Okhwb {
            hue: self.hue.into_format(),
            whiteness: U::from_stimulus(self.whiteness),
            blackness: U::from_stimulus(self.blackness),
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

impl<T> Okhwb<T>
where
    T: Real + Stimulus,
{
    /// Return the `whiteness` value minimum.
    pub fn min_whiteness() -> T {
        T::zero()
    }

    /// Return the `whiteness` value maximum.
    pub fn max_whiteness() -> T {
        T::max_intensity()
    }

    /// Return the `blackness` value minimum.
    pub fn min_blackness() -> T {
        T::zero()
    }

    /// Return the `blackness` value maximum.
    pub fn max_blackness() -> T {
        T::max_intensity()
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
        + RealAngle,
    T::Scalar: Real
        + Zero
        + One
        + Recip
        + Hypot
        + IsValidDivisor<Mask = bool>
        + Arithmetics
        + Clone
        + FromScalar<Scalar = T::Scalar>,
{
    /// Converts `lab` to `Okhwb` in the bounds of sRGB.
    ///
    /// # See
    /// https://bottosson.github.io/posts/colorpicker/#okhwb
    /// See [`srgb_to_okhwb`](https://bottosson.github.io/posts/colorpicker/#okhwb-2).
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

impl<T> HasBoolMask for Okhwb<T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<T> Default for Okhwb<T>
where
    T: Real + Stimulus,
    OklabHue<T>: Default,
{
    fn default() -> Okhwb<T> {
        Okhwb::new(
            OklabHue::default(),
            Self::min_whiteness(),
            Self::max_blackness(),
        )
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Zeroable for Okhwb<T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Pod for Okhwb<T> where T: bytemuck::Pod {}

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
            let epsilon = f32::EPSILON;
            assert!(
                Okhwb::visually_eq(okhwb, okhwb_from_okhsv, f32::EPSILON),
                "Okhwb \n{:?} is not visually equal to Okhwb from Okhsv \n{:?}\nwithin epsilon {}",
                okhwb,
                okhwb_from_okhsv,
                epsilon
            );
            let okhsv_from_okhwb = Okhsv::from_color_unclamped(okhwb);
            let epsilon = f32::EPSILON;
            assert!(
                Okhsv::visually_eq(okhsv, okhsv_from_okhwb, f32::EPSILON),
                "Okhsv \n{:?} is not visually equal to Okhsv from Okhsv from Okhwb \n{:?}\nwithin epsilon {}",
                okhsv,
                okhsv_from_okhwb,  epsilon
            );

            let roundtrip_color = Oklab::from_color_unclamped(okhwb);
            let oklab_from_okhsv = Oklab::from_color_unclamped(okhsv);
            let epsilon = f32::EPSILON;
            assert!(
                Oklab::visually_eq(roundtrip_color, oklab_from_okhsv, epsilon),
                "roundtrip color \n{:?} does not match \n{:?}\nwithin epsilon {}",
                roundtrip_color,
                oklab_from_okhsv,
                epsilon
            );
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
