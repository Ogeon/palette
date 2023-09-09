use crate::{
    angle::RealAngle,
    color_difference::{DeltaE, ImprovedDeltaE},
    convert::{FromColorUnclamped, IntoColorUnclamped},
    hues::Cam16Hue,
    num::{Arithmetics, Hypot, Ln, One, Real, Trigonometry},
};

use super::{Cam16UcsJab, Colorfulness, Lightness, PartialCam16Jmh};

/// The polar form of CAM16-UCS, or J'M'h'.
#[derive(Clone, Copy, Debug, WithAlpha, ArrayCast, FromColorUnclamped)]
#[palette(
    palette_internal,
    component = "T",
    cam16_chromaticity = "Colorfulness<T>",
    cam16_luminance = "Lightness<T>",
    skip_derives(PartialCam16, Cam16UcsJmh, Cam16UcsJab)
)]
#[repr(C)]
pub struct Cam16UcsJmh<T> {
    /// The [lightness](https://en.wikipedia.org/wiki/Lightness) (J') of the color.
    pub lightness: T,

    /// The [colorfulness](https://en.wikipedia.org/wiki/Colorfulness) (M') of the color.
    pub colorfulness: T,

    /// The [hue](https://en.wikipedia.org/wiki/Hue) (h') of the color.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: Cam16Hue<T>,
}

impl<T> Cam16UcsJmh<T> {
    /// Create a CAM16-UCS J' M' h' color.
    pub fn new<H: Into<Cam16Hue<T>>>(lightness: T, colorfulness: T, hue: H) -> Self {
        Self::new_const(lightness, colorfulness, hue.into())
    }

    /// Create a CAM16-UCS J' M' h' color. This is the same as
    /// `Cam16UcsJmh::new` without the generic hue type. It's temporary until
    /// `const fn` supports traits.
    pub const fn new_const(lightness: T, colorfulness: T, hue: Cam16Hue<T>) -> Self {
        Self {
            lightness,
            colorfulness,
            hue,
        }
    }

    /// Convert to a `(J', M', h')` tuple.
    pub fn into_components(self) -> (T, T, Cam16Hue<T>) {
        (self.lightness, self.colorfulness, self.hue)
    }

    /// Convert from a `(J', M', h')` tuple.
    pub fn from_components<H: Into<Cam16Hue<T>>>(
        (lightness, colorfulness, hue): (T, T, H),
    ) -> Self {
        Self::new(lightness, colorfulness, hue)
    }
}

impl<T> FromColorUnclamped<Cam16UcsJmh<T>> for Cam16UcsJmh<T> {
    fn from_color_unclamped(val: Cam16UcsJmh<T>) -> Self {
        val
    }
}

impl<T> FromColorUnclamped<PartialCam16Jmh<T>> for Cam16UcsJmh<T>
where
    T: Real + One + Ln + Arithmetics,
{
    fn from_color_unclamped(val: PartialCam16Jmh<T>) -> Self {
        let colorfulness =
            (T::one() + T::from_f64(0.0228) * val.chromaticity.0).ln() / T::from_f64(0.0228);
        let lightness =
            T::from_f64(1.7) * &val.luminance.0 / (T::one() + T::from_f64(0.007) * val.luminance.0);

        Cam16UcsJmh {
            lightness,
            colorfulness,
            hue: val.hue,
        }
    }
}

impl<T> FromColorUnclamped<Cam16UcsJab<T>> for Cam16UcsJmh<T>
where
    T: RealAngle + Hypot + Trigonometry + Arithmetics + Clone,
{
    fn from_color_unclamped(val: Cam16UcsJab<T>) -> Self {
        Self {
            lightness: val.lightness,
            colorfulness: val.a.clone().hypot(val.b.clone()),
            hue: Cam16Hue::from_cartesian(val.a, val.b),
        }
    }
}

impl_color_add!(Cam16UcsJmh, [lightness, colorfulness, hue]);
impl_color_sub!(Cam16UcsJmh, [lightness, colorfulness, hue]);

impl<T> DeltaE for Cam16UcsJmh<T>
where
    Cam16UcsJab<T>: DeltaE<Scalar = T> + FromColorUnclamped<Self>,
{
    type Scalar = T;

    #[inline]
    fn delta_e(self, other: Self) -> Self::Scalar {
        // Jab and Jmh have the same delta E.
        Cam16UcsJab::from_color_unclamped(self).delta_e(other.into_color_unclamped())
    }
}

impl<T> ImprovedDeltaE for Cam16UcsJmh<T>
where
    Cam16UcsJab<T>: ImprovedDeltaE<Scalar = T> + FromColorUnclamped<Self>,
{
    #[inline]
    fn improved_delta_e(self, other: Self) -> Self::Scalar {
        // Jab and Jmh have the same delta E.
        Cam16UcsJab::from_color_unclamped(self).improved_delta_e(other.into_color_unclamped())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        cam16::{Cam16UcsJab, Cam16UcsJmh},
        color_difference::{DeltaE, ImprovedDeltaE},
        convert::IntoColorUnclamped,
    };

    #[test]
    fn delta_e_large_hue_diff() {
        let lhs1 = Cam16UcsJmh::<f64>::new(50.0, 64.0, -730.0);
        let rhs1 = Cam16UcsJmh::new(50.0, 64.0, 730.0);

        let lhs2 = Cam16UcsJmh::<f64>::new(50.0, 64.0, -10.0);
        let rhs2 = Cam16UcsJmh::new(50.0, 64.0, 10.0);

        assert_relative_eq!(
            lhs1.delta_e(rhs1),
            lhs2.delta_e(rhs2),
            epsilon = 0.0000000000001
        );
    }

    // Jab and Jmh have the same delta E.
    #[test]
    fn jab_delta_e_equality() {
        let mut jab_colors: Vec<Cam16UcsJab<f64>> = Vec::new();

        for j_step in 0i8..5 {
            for a_step in -2i8..3 {
                for b_step in -2i8..3 {
                    jab_colors.push(Cam16UcsJab::new(
                        j_step as f64 * 25.0,
                        a_step as f64 * 60.0,
                        b_step as f64 * 60.0,
                    ))
                }
            }
        }

        let jmh_colors: Vec<Cam16UcsJmh<_>> = jab_colors.clone().into_color_unclamped();

        for (&lhs_jab, &lhs_jmh) in jab_colors.iter().zip(&jmh_colors) {
            for (&rhs_jab, &rhs_jmh) in jab_colors.iter().zip(&jmh_colors) {
                let delta_e_jab = lhs_jab.delta_e(rhs_jab);
                let delta_e_jmh = lhs_jmh.delta_e(rhs_jmh);
                assert_relative_eq!(delta_e_jab, delta_e_jmh, epsilon = 0.0000000000001);
            }
        }
    }

    // Jab and Jmh have the same delta E, so should also have the same improved
    // delta E.
    #[test]
    fn jab_improved_delta_e_equality() {
        let mut jab_colors: Vec<Cam16UcsJab<f64>> = Vec::new();

        for j_step in 0i8..5 {
            for a_step in -2i8..3 {
                for b_step in -2i8..3 {
                    jab_colors.push(Cam16UcsJab::new(
                        j_step as f64 * 25.0,
                        a_step as f64 * 60.0,
                        b_step as f64 * 60.0,
                    ))
                }
            }
        }

        let jmh_colors: Vec<Cam16UcsJmh<_>> = jab_colors.clone().into_color_unclamped();

        for (&lhs_jab, &lhs_jmh) in jab_colors.iter().zip(&jmh_colors) {
            for (&rhs_jab, &rhs_jmh) in jab_colors.iter().zip(&jmh_colors) {
                let delta_e_jab = lhs_jab.improved_delta_e(rhs_jab);
                let delta_e_jmh = lhs_jmh.improved_delta_e(rhs_jmh);
                assert_relative_eq!(delta_e_jab, delta_e_jmh, epsilon = 0.0000000000001);
            }
        }
    }
}
