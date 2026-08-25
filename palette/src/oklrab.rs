//! Types for the Oklrab color space.

use core::ops::Mul;

pub use alpha::Oklraba;

use crate::{
    angle::RealAngle,
    bool_mask::HasBoolMask,
    convert::FromColorUnclamped,
    num::{Arithmetics, Hypot, MinMax, One, Powi, Real, Sqrt, Trigonometry, Zero},
    ok_utils::toe,
    white_point::D65,
    Oklab, Oklrch,
};

pub use self::properties::Iter;

#[cfg(feature = "random")]
pub use self::random::UniformOklrab;

mod alpha;
mod properties;
#[cfg(feature = "random")]
mod random;
#[cfg(test)]
#[cfg(feature = "approx")]
mod visual_eq;

/// Oklab with a reference lightness, using the [`Lr` lightness
/// estimate](https://bottosson.github.io/posts/colorpicker/#intermission---a-new-lightness-estimate-for-oklab).
///
/// `Oklrab` has the same `a` and `b` components as [`Oklab`], but replaces
/// `Oklab`'s lightness `l` with the reference lightness `Lr`. `Lr` is meant to
/// better match the lightness behavior of [CIE L\*a\*b\*](crate::Lab), so that
/// a middle grey lands closer to the middle of the lightness range. It's the
/// same lightness that [`Okhsl`](crate::Okhsl) and [`Okhsv`](crate::Okhsv) are
/// built on.
///
/// The two lightness scales agree at `0` and `1`, so pure black and the full
/// white point are unchanged, but they differ in between. Converting to and
/// from `Oklab` only rescales the lightness and leaves `a` and `b` untouched.
///
/// It assumes a D65 whitepoint and normal well-lit viewing conditions, like
/// `Oklab`.
#[derive(Debug, Copy, Clone, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "D65",
    component = "T",
    skip_derives(Oklab, Oklrab, Oklrch)
)]
#[repr(C)]
pub struct Oklrab<T = f32> {
    /// `l` is the reference lightness of the color. `0` gives absolute black
    /// and `1` gives the full white point luminance of the display medium.
    ///
    /// Unlike [`Oklab`]'s `l`, this is the `Lr` estimate, which is spaced to be
    /// closer to a perceptually uniform lightness scale.
    pub l: T,

    /// `a` changes the hue from reddish to greenish, when moving from positive
    /// to negative values and becomes more intense with larger absolute values.
    ///
    /// It's the same as [`Oklab`]'s `a`.
    pub a: T,

    /// `b` changes the hue from yellowish to blueish, when moving from positive
    /// to negative values and becomes more intense with larger absolute values.
    ///
    /// It's the same as [`Oklab`]'s `b`.
    pub b: T,
}

impl<T> Oklrab<T> {
    /// Create an Oklrab color.
    pub const fn new(l: T, a: T, b: T) -> Self {
        Self { l, a, b }
    }

    /// Convert to a `(Lr, a, b)` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.l, self.a, self.b)
    }

    /// Convert from a `(Lr, a, b)` tuple.
    pub fn from_components((l, a, b): (T, T, T)) -> Self {
        Self::new(l, a, b)
    }
}

impl<T> Oklrab<T>
where
    T: Zero + One,
{
    /// Return the `l` value minimum.
    pub fn min_l() -> T {
        T::zero()
    }

    /// Return the `l` value maximum.
    pub fn max_l() -> T {
        T::one()
    }
}

impl_reference_component_methods!(Oklrab, [l, a, b]);
impl_struct_of_arrays_methods!(Oklrab, [l, a, b]);

impl<T> Oklrab<T>
where
    T: Hypot + Clone,
{
    /// Returns the chroma.
    pub(crate) fn get_chroma(&self) -> T {
        T::hypot(self.a.clone(), self.b.clone())
    }
}

impl<T> FromColorUnclamped<Oklrab<T>> for Oklrab<T> {
    fn from_color_unclamped(color: Self) -> Self {
        color
    }
}

impl<T> FromColorUnclamped<Oklab<T>> for Oklrab<T>
where
    T: Real + Powi + Sqrt + Arithmetics + One + Clone,
{
    fn from_color_unclamped(color: Oklab<T>) -> Self {
        Self::new(toe(color.l), color.a, color.b)
    }
}

impl<T> FromColorUnclamped<Oklrch<T>> for Oklrab<T>
where
    T: RealAngle + Zero + MinMax + Trigonometry + Mul<Output = T> + Clone,
{
    fn from_color_unclamped(color: Oklrch<T>) -> Self {
        let (a, b) = color.hue.into_cartesian();
        let chroma = color.chroma.max(T::zero());

        Oklrab {
            l: color.l,
            a: a * chroma.clone(),
            b: b * chroma,
        }
    }
}

impl_tuple_conversion!(Oklrab as (T, T, T));

impl<T> HasBoolMask for Oklrab<T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<T> Default for Oklrab<T>
where
    T: Zero,
{
    fn default() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Zeroable for Oklrab<T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Pod for Oklrab<T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use crate::Oklrab;

    test_convert_into_from_xyz!(Oklrab);

    #[cfg(feature = "approx")]
    mod conversion {
        use crate::{convert::FromColorUnclamped, visual::VisuallyEqual, LinSrgb, Oklab, Oklrab};

        #[test]
        fn roundtrip_oklrab_oklab_is_original() {
            let colors = [
                ("red", LinSrgb::new(1.0, 0.0, 0.0)),
                ("green", LinSrgb::new(0.0, 1.0, 0.0)),
                ("blue", LinSrgb::new(0.0, 0.0, 1.0)),
                ("white", LinSrgb::new(1.0, 1.0, 1.0)),
                ("black", LinSrgb::new(0.0, 0.0, 0.0)),
                ("grey", LinSrgb::new(0.5, 0.5, 0.5)),
            ];

            const EPSILON: f64 = 1e-14;

            for (name, rgb) in colors {
                let oklab = Oklab::from_color_unclamped(rgb);
                let oklrab = Oklrab::from_color_unclamped(oklab);
                let roundtrip = Oklab::from_color_unclamped(oklrab);
                assert!(
                    Oklab::visually_eq(roundtrip, oklab, EPSILON),
                    "'{name}' failed.\n{roundtrip:?}\n!=\n{oklab:?}"
                );
            }
        }

        #[test]
        fn black_and_white_are_unchanged() {
            // Lr and Oklab's l agree at 0 and 1.
            let black = Oklrab::from_color_unclamped(Oklab::new(0.0, 0.0, 0.0));
            let white = Oklrab::from_color_unclamped(Oklab::new(1.0, 0.0, 0.0));
            assert_relative_eq!(black.l, 0.0, epsilon = 1e-9);
            assert_relative_eq!(white.l, 1.0, epsilon = 1e-9);
        }

        #[test]
        fn middle_lightness_differs_from_oklab() {
            // Lr rescales the lightness, so a mid Oklab lightness lands lower on
            // the reference scale. The expected value comes from the toe function.
            let oklrab = Oklrab::from_color_unclamped(Oklab::new(0.5, 0.0, 0.0));
            assert!(oklrab.l < 0.5, "Lr was {}", oklrab.l);
            assert_relative_eq!(oklrab.l, 0.421140_f64, epsilon = 1e-6);
        }
    }

    #[test]
    fn ranges() {
        assert_ranges! {
            Oklrab<f64>;
            clamped {
                l: 0.0 => 1.0
                // a and b are unbounded --> not part of test
            }
            clamped_min {}
            unclamped {}
        };
    }

    #[test]
    fn check_min_max_components() {
        assert_eq!(Oklrab::<f32>::min_l(), 0.0);
        assert_eq!(Oklrab::<f32>::max_l(), 1.0);
    }

    struct_of_arrays_tests!(
        Oklrab[l, a, b],
        super::Oklraba::new(0.1f32, 0.2, 0.3, 0.4),
        super::Oklraba::new(0.2, 0.3, 0.4, 0.5),
        super::Oklraba::new(0.3, 0.4, 0.5, 0.6)
    );

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Oklrab::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"l":0.3,"a":0.8,"b":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Oklrab = ::serde_json::from_str(r#"{"l":0.3,"a":0.8,"b":0.1}"#).unwrap();

        assert_eq!(deserialized, Oklrab::new(0.3, 0.8, 0.1));
    }

    test_uniform_distribution! {
        Oklrab {
            l: (0.0, 1.0),
            a: (-1.0, 1.0),
            b: (-1.0, 1.0)
        },
        min: Oklrab::new(0.0, -1.0, -1.0),
        max: Oklrab::new(1.0, 1.0, 1.0)
    }
}
