//! Types for the Oklrch color space.

pub use alpha::Oklrcha;

use crate::{
    bool_mask::HasBoolMask,
    convert::FromColorUnclamped,
    num::{Hypot, One, Zero},
    white_point::D65,
    GetHue, OklabHue, Oklrab,
};

pub use self::properties::Iter;

#[cfg(feature = "random")]
pub use self::random::UniformOklrch;

mod alpha;
mod properties;
#[cfg(feature = "random")]
mod random;

/// Oklrch, a polar version of [Oklrab].
///
/// It's the same as [`Oklch`](crate::Oklch), but built on top of [`Oklrab`]'s
/// reference lightness `Lr` instead of `Oklab`'s lightness. That makes its `l`
/// component behave more like the lightness of [CIE L\*C\*h°](crate::Lch).
///
/// It's a cylindrical color space, like [HSL](crate::Hsl) and
/// [HSV](crate::Hsv). This gives it the same ability to directly change the hue
/// and colorfulness of a color, while preserving other visual aspects.
///
/// It assumes a D65 whitepoint and normal well-lit viewing conditions, like
/// Oklrab.
///
/// # Examples
///
/// Create an `Oklrch` color from its channels, with the hue in degrees:
///
/// ```
/// use palette::Oklrch;
///
/// let color = Oklrch::new(0.5f32, 0.1, 30.0);
/// ```
///
/// It can also be converted from another color space:
///
/// ```
/// use palette::{FromColor, Oklrch, Srgb};
///
/// let color = Oklrch::from_color(Srgb::new(0.8f32, 0.3, 0.1));
/// ```
#[derive(Debug, Copy, Clone, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "D65",
    component = "T",
    skip_derives(Oklrab, Oklrch)
)]
#[repr(C)]
pub struct Oklrch<T = f32> {
    /// Lr is the reference lightness of the color. 0 gives absolute black and 1
    /// gives the brightest white.
    pub l: T,

    /// `chroma` is the colorfulness of the color.
    /// A color with `chroma == 0` is a shade of grey.
    /// In a transformation from `Oklrab` it is computed as `chroma = √(a²+b²)`.
    /// `chroma` is unbounded
    pub chroma: T,

    /// h is the hue of the color, in degrees. Decides if it's red, blue, purple,
    /// etc.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: OklabHue<T>,
}

impl<T> Oklrch<T> {
    /// Create an `Oklrch` color.
    pub fn new<H: Into<OklabHue<T>>>(l: T, chroma: T, hue: H) -> Self {
        Oklrch {
            l,
            chroma,
            hue: hue.into(),
        }
    }

    /// Create an `Oklrch` color. This is the same as `Oklrch::new` without the
    /// generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(l: T, chroma: T, hue: OklabHue<T>) -> Self {
        Oklrch { l, chroma, hue }
    }

    /// Convert to a `(Lr, C, h)` tuple.
    pub fn into_components(self) -> (T, T, OklabHue<T>) {
        (self.l, self.chroma, self.hue)
    }

    /// Convert from a `(Lr, C, h)` tuple.
    pub fn from_components<H: Into<OklabHue<T>>>((l, chroma, hue): (T, T, H)) -> Self {
        Self::new(l, chroma, hue)
    }
}

impl<T> Oklrch<T>
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

    /// Return the `chroma` value minimum.
    pub fn min_chroma() -> T {
        T::zero()
    }
}

impl_reference_component_methods_hue!(Oklrch, [l, chroma]);
impl_struct_of_arrays_methods_hue!(Oklrch, [l, chroma]);

impl<T> FromColorUnclamped<Oklrch<T>> for Oklrch<T> {
    fn from_color_unclamped(color: Oklrch<T>) -> Self {
        color
    }
}

impl<T> FromColorUnclamped<Oklrab<T>> for Oklrch<T>
where
    T: Hypot + Clone,
    Oklrab<T>: GetHue<Hue = OklabHue<T>>,
{
    fn from_color_unclamped(color: Oklrab<T>) -> Self {
        let hue = color.get_hue();
        let chroma = color.get_chroma();
        Oklrch::new(color.l, chroma, hue)
    }
}

impl_tuple_conversion_hue!(Oklrch as (T, T, H), OklabHue);

impl<T> HasBoolMask for Oklrch<T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<T> Default for Oklrch<T>
where
    T: Zero + One,
    OklabHue<T>: Default,
{
    fn default() -> Oklrch<T> {
        Oklrch::new(Self::min_l(), Self::min_chroma(), OklabHue::default())
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Zeroable for Oklrch<T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Pod for Oklrch<T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use crate::Oklrch;

    test_convert_into_from_xyz!(Oklrch);

    #[cfg(feature = "approx")]
    mod conversion {
        use crate::{convert::FromColorUnclamped, visual::VisuallyEqual, LinSrgb, Oklrab, Oklrch};

        #[test]
        fn roundtrip_oklrch_oklrab_is_original() {
            let colors = [
                ("red", LinSrgb::new(1.0, 0.0, 0.0)),
                ("green", LinSrgb::new(0.0, 1.0, 0.0)),
                ("blue", LinSrgb::new(0.0, 0.0, 1.0)),
                ("white", LinSrgb::new(1.0, 1.0, 1.0)),
                ("grey", LinSrgb::new(0.5, 0.5, 0.5)),
            ];

            const EPSILON: f64 = 1e-14;

            for (name, rgb) in colors {
                let oklrab = Oklrab::from_color_unclamped(rgb);
                let oklrch = Oklrch::from_color_unclamped(oklrab);
                let roundtrip = Oklrab::from_color_unclamped(oklrch);
                assert!(
                    Oklrab::visually_eq(roundtrip, oklrab, EPSILON),
                    "'{name}' failed.\n{roundtrip:?}\n!=\n{oklrab:?}"
                );
            }
        }
    }

    #[test]
    fn ranges() {
        // chroma: 0.0 => infinity
        assert_ranges! {
            Oklrch< f64>;
            clamped {
                l: 0.0 => 1.0
            }
            clamped_min {}
            unclamped {
                hue: 0.0 => 360.0
            }
        }
    }

    #[test]
    fn check_min_max_components() {
        assert_eq!(Oklrch::<f32>::min_l(), 0.0);
        assert_eq!(Oklrch::<f32>::max_l(), 1.0);
        assert_eq!(Oklrch::<f32>::min_chroma(), 0.0);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Oklrch::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"l":0.3,"chroma":0.8,"hue":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Oklrch =
            ::serde_json::from_str(r#"{"l":0.3,"chroma":0.8,"hue":0.1}"#).unwrap();

        assert_eq!(deserialized, Oklrch::new(0.3, 0.8, 0.1));
    }

    struct_of_arrays_tests!(
        Oklrch[l, chroma, hue],
        super::Oklrcha::new(0.1f32, 0.2, 0.3, 0.4),
        super::Oklrcha::new(0.2, 0.3, 0.4, 0.5),
        super::Oklrcha::new(0.3, 0.4, 0.5, 0.6)
    );

    test_uniform_distribution! {
        Oklrch<f32> as crate::Oklrab {
            l: (0.0, 1.0),
            a: (-0.7, 0.7),
            b: (-0.7, 0.7),
        },
        min: Oklrch::new(0.0f32, 0.0, 0.0),
        max: Oklrch::new(1.0, 1.0, 360.0)
    }
}
