pub use alpha::Oklcha;

use crate::num::UpperBounded;
use crate::{
    bool_mask::HasBoolMask,
    convert::FromColorUnclamped,
    num::{Hypot, One, Zero},
    white_point::D65,
    GetHue, Oklab, OklabHue, Xyz,
};

mod alpha;
mod properties;
#[cfg(feature = "random")]
mod random;

/// Oklch, a polar version of [Oklab](crate::Oklab).
///
/// It is Oklab’s equivalent of [CIE L\*C\*h°](crate::Lch).
///
/// It's a cylindrical color space, like [HSL](crate::Hsl) and
/// [HSV](crate::Hsv). This gives it the same ability to directly change
/// the hue and colorfulness of a color, while preserving other visual aspects.
///
/// It assumes a D65 whitepoint and normal well-lit viewing conditions,
/// like Oklab.
#[derive(Debug, Copy, Clone, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "D65",
    component = "T",
    skip_derives(Oklab, Oklch, Xyz)
)]
#[repr(C)]
pub struct Oklch<T = f32> {
    /// L is the lightness of the color. 0 gives absolute black and 1 gives the brightest white.
    pub l: T,

    /// `chroma` is the colorfulness of the color.
    /// A color with `chroma == 0` is a shade of grey.
    /// In a transformation from `Oklab` it is computed as `chroma = √(a²+b²)`.
    /// `chroma` is unbounded
    pub chroma: T,

    /// h is the hue of the color, in degrees. Decides if it's red, blue, purple,
    /// etc.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: OklabHue<T>,
}

impl<T> Oklch<T> {
    /// Create an `Oklch` color.
    pub fn new<H: Into<OklabHue<T>>>(l: T, chroma: T, hue: H) -> Self {
        Oklch {
            l,
            chroma,
            hue: hue.into(),
        }
    }

    /// Create an `Oklch` color. This is the same as `Oklch::new` without the
    /// generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(l: T, chroma: T, hue: OklabHue<T>) -> Self {
        Oklch { l, chroma, hue }
    }

    /// Convert to a `(L, C, h)` tuple.
    pub fn into_components(self) -> (T, T, OklabHue<T>) {
        (self.l, self.chroma, self.hue)
    }

    /// Convert from a `(L, C, h)` tuple.
    pub fn from_components<H: Into<OklabHue<T>>>((l, chroma, hue): (T, T, H)) -> Self {
        Self::new(l, chroma, hue)
    }
}

impl<T> Oklch<T>
where
    T: Zero + One + UpperBounded,
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

    /// Return the `chroma` value maximum.
    pub fn max_chroma() -> T {
        T::max_value()
    }
}

impl<T> FromColorUnclamped<Oklch<T>> for Oklch<T> {
    fn from_color_unclamped(color: Oklch<T>) -> Self {
        color
    }
}

impl<T> FromColorUnclamped<Xyz<D65, T>> for Oklch<T>
where
    Oklab<T>: FromColorUnclamped<Xyz<D65, T>>,
    Self: FromColorUnclamped<Oklab<T>>,
{
    fn from_color_unclamped(color: Xyz<D65, T>) -> Self {
        let lab = Oklab::<T>::from_color_unclamped(color);
        Self::from_color_unclamped(lab)
    }
}

impl<T> FromColorUnclamped<Oklab<T>> for Oklch<T>
where
    T: Hypot + Clone,
    Oklab<T>: GetHue<Hue = OklabHue<T>>,
{
    fn from_color_unclamped(color: Oklab<T>) -> Self {
        let hue = color.get_hue();
        let chroma = color.chroma();
        Oklch::new(color.l, chroma, hue)
    }
}

impl<T, H: Into<OklabHue<T>>> From<(T, T, H)> for Oklch<T> {
    fn from(components: (T, T, H)) -> Self {
        Self::from_components(components)
    }
}

impl<T> From<Oklch<T>> for (T, T, OklabHue<T>) {
    fn from(color: Oklch<T>) -> (T, T, OklabHue<T>) {
        color.into_components()
    }
}

impl<T> HasBoolMask for Oklch<T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<T> Default for Oklch<T>
where
    T: Zero + One + UpperBounded,
    OklabHue<T>: Default,
{
    fn default() -> Oklch<T> {
        Oklch::new(Self::min_l(), Self::min_chroma(), OklabHue::default())
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Zeroable for Oklch<T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Pod for Oklch<T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use crate::Oklch;

    #[test]
    fn ranges() {
        assert_ranges! {
            Oklch< f64>;
            clamped {
                l: 0.0 => 1.0,
                chroma: 0.0 => f64::MAX
            }
            clamped_min {}
            unclamped {
                hue: 0.0 => 360.0
            }
        }
    }

    #[test]
    fn check_min_max_components() {
        assert_relative_eq!(Oklch::<f32>::min_l(), 0.0);
        assert_relative_eq!(Oklch::<f32>::max_l(), 1.0);
        assert_relative_eq!(Oklch::<f32>::min_chroma(), 0.0);
        assert_relative_eq!(Oklch::<f32>::max_chroma(), f32::MAX);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Oklch::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"l":0.3,"chroma":0.8,"hue":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Oklch =
            ::serde_json::from_str(r#"{"l":0.3,"chroma":0.8,"hue":0.1}"#).unwrap();

        assert_eq!(deserialized, Oklch::new(0.3, 0.8, 0.1));
    }

    #[cfg(feature = "random")]
    test_uniform_distribution! {
        Oklch<f32> as crate::Oklab {
            l: (0.0, 1.0),
            a: (-0.7, 0.7),
            b: (-0.7, 0.7),
        },
        min: Oklch::new(0.0f32, 0.0, 0.0),
        max: Oklch::new(1.0, 1.0, 360.0)
    }
}
