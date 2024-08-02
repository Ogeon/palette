use core::marker::PhantomData;

use crate::{
    bool_mask::HasBoolMask,
    convert::{ConvertOnce, FromColorUnclamped, Matrix3},
    num::{Arithmetics, Zero},
    stimulus::{FromStimulus, Stimulus, StimulusColor},
    xyz::meta::HasXyzMeta,
    Alpha, Xyz,
};

use super::matrix::{HasLmsMatrix, XyzToLms};

/// Generic LMS with an alpha component. See [`Lmsa` implementation in
/// `Alpha`][crate::Alpha#Lmsa].
pub type Lmsa<M, T> = Alpha<Lms<M, T>, T>;

/// Generic LMS.
///
/// LMS represents the response of the eye's cone cells. L, M and S are for
/// "long", "medium" and "short" wavelengths, roughly corresponding to red,
/// green and blue. Many newer mentions of an LMS representation use the letters
/// R, G and B instead (or sometimes ρ, γ, β), but this library sticks to LMS to
/// differentiate it from [`Rgb`][crate::rgb::Rgb].
///
/// The LMS color space is a model of the physiological response to color
/// stimuli. It has some mathematical shortcomings that [`Xyz`] improves on,
/// such as severe spectral sensitivity overlap between L, M and S. Despite
/// this, LMS has a lot of uses, include chromatic adaptation and emulating
/// different types of color vision deficiency, and it's sometimes part of the
/// conversion process between other color spaces.
///
/// # Creating a Value
///
/// An LMS value is often derived from another color space, through a conversion
/// matrix. Two such matrices are [`VonKries`][super::matrix::VonKries] and
/// [`Bradford`][super::matrix::Bradford], and Palette offers type aliases in the
/// [`lms`][crate::lms] module to make using them a bit more convenient. It's of
/// course also possible to simply use [`Lms::new`], but it may not be as
/// intuitive.
///
/// ```
/// use palette::{
///     lms::{Lms, VonKriesLms, matrix::VonKries},
///     white_point::D65,
///     Srgb, FromColor
/// };
///
/// let von_kries_lms = Lms::<VonKries, f32>::new(0.1, 0.2, 0.3);
/// let von_kries_d65_lms = VonKriesLms::<D65, f32>::new(0.1, 0.2, 0.3);
///
/// // `new` is also `const`:
/// const VON_KRIES_LMS: Lms<VonKries, f32> = Lms::new(0.1, 0.2, 0.3);
///
/// // Von Kries LMS from sRGB:
/// let lms_from_srgb = VonKriesLms::<D65, f32>::from_color(Srgb::new(0.3f32, 0.8, 0.1));
///
/// // It's also possible to convert from (and to) arrays and tuples:
/// let lms_from_array = VonKriesLms::<D65, f32>::from([0.1, 0.2, 0.3]);
/// let lms_from_tuple = VonKriesLms::<D65, f32>::from((0.1, 0.2, 0.3));
/// ```
#[derive(Debug, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(palette_internal, component = "T", skip_derives(Lms, Xyz))]
#[repr(C)]
pub struct Lms<M, T> {
    /// Stimulus from long wavelengths, or red, or ρ. The typical range is
    /// between 0.0 and 1.0, but it doesn't have an actual upper bound.
    pub long: T,

    /// Stimulus from medium wavelengths, or green, or γ. The typical range is
    /// between 0.0 and 1.0, but it doesn't have an actual upper bound.
    pub medium: T,

    /// Stimulus from short wavelengths, or blue, or β. The typical range is
    /// between 0.0 and 1.0, but it doesn't have an actual upper bound.
    pub short: T,

    /// Type level meta information, such as reference white, or which matrix
    /// was used when converting from XYZ.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub meta: PhantomData<M>,
}

impl<M, T> Lms<M, T> {
    /// Create a new LMS color.
    pub const fn new(long: T, medium: T, short: T) -> Self {
        Self {
            long,
            medium,
            short,
            meta: PhantomData,
        }
    }

    /// Convert the LMS components into another number type.
    ///
    /// ```
    /// use palette::{
    ///     lms::VonKriesLms,
    ///     white_point::D65,
    /// };
    ///
    /// let lms_f64: VonKriesLms<D65, f64> = VonKriesLms::new(0.3f32, 0.7, 0.2).into_format();
    /// ```
    pub fn into_format<U>(self) -> Lms<M, U>
    where
        U: FromStimulus<T>,
    {
        Lms {
            long: U::from_stimulus(self.long),
            medium: U::from_stimulus(self.medium),
            short: U::from_stimulus(self.short),
            meta: PhantomData,
        }
    }

    /// Convert the LMS components from another number type.
    ///
    /// ```
    /// use palette::{
    ///     lms::VonKriesLms,
    ///     white_point::D65,
    /// };
    ///
    /// let lms_f64 = VonKriesLms::<D65, f64>::from_format(VonKriesLms::new(0.3f32, 0.7, 0.2));
    /// ```
    pub fn from_format<U>(color: Lms<M, U>) -> Self
    where
        T: FromStimulus<U>,
    {
        color.into_format()
    }

    /// Convert to a `(long, medium, short)` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.long, self.medium, self.short)
    }

    /// Convert from a `(long, medium, short)` tuple.
    pub fn from_components((long, medium, short): (T, T, T)) -> Self {
        Self::new(long, medium, short)
    }

    /// Changes the meta type without changing the color value.
    ///
    /// This function doesn't change the numerical values, and thus the stimuli
    /// it represents in an absolute sense. However, the appearance of the color
    /// may not be the same. The effect may be similar to taking a photo with an
    /// incorrect white balance.
    pub fn with_meta<NewM>(self) -> Lms<NewM, T> {
        Lms {
            long: self.long,
            medium: self.medium,
            short: self.short,
            meta: PhantomData,
        }
    }
}

impl<M, T> Lms<M, T>
where
    T: Zero,
{
    /// Return the `short` value minimum.
    pub fn min_short() -> T {
        T::zero()
    }

    /// Return the `medium` value minimum.
    pub fn min_medium() -> T {
        T::zero()
    }

    /// Return the `long` value minimum.
    pub fn min_long() -> T {
        T::zero()
    }
}

impl<M, T> Lms<M, T> {
    /// Produce a conversion matrix from [`Xyz`] to [`Lms`].
    #[inline]
    pub fn matrix_from_xyz() -> Matrix3<Xyz<M::XyzMeta, T>, Self>
    where
        M: HasXyzMeta + HasLmsMatrix,
        M::LmsMatrix: XyzToLms<T>,
    {
        Matrix3::from_array(M::LmsMatrix::xyz_to_lms_matrix())
    }
}

/// <span id="Lmsa"></span>[`Lmsa`][Lmsa] implementations.
impl<S, T, A> Alpha<Lms<S, T>, A> {
    /// Create an LMSA color.
    pub const fn new(red: T, green: T, blue: T, alpha: A) -> Self {
        Alpha {
            color: Lms::new(red, green, blue),
            alpha,
        }
    }

    /// Convert the LMSA components into other number types.
    ///
    /// ```
    /// use palette::{
    ///     lms::VonKriesLmsa,
    ///     white_point::D65,
    /// };
    ///
    /// let lmsa_f64: VonKriesLmsa<D65, f64> = VonKriesLmsa::new(0.3f32, 0.7, 0.2, 0.5).into_format();
    /// ```
    pub fn into_format<U, B>(self) -> Alpha<Lms<S, U>, B>
    where
        U: FromStimulus<T>,
        B: FromStimulus<A>,
    {
        Alpha {
            color: self.color.into_format(),
            alpha: B::from_stimulus(self.alpha),
        }
    }

    /// Convert the LMSA components from other number types.
    ///
    /// ```
    /// use palette::{
    ///     lms::VonKriesLmsa,
    ///     white_point::D65,
    /// };
    ///
    /// let lmsa_f64 = VonKriesLmsa::<D65, f64>::from_format(VonKriesLmsa::new(0.3f32, 0.7, 0.2, 0.5));
    /// ```
    pub fn from_format<U, B>(color: Alpha<Lms<S, U>, B>) -> Self
    where
        T: FromStimulus<U>,
        A: FromStimulus<B>,
    {
        color.into_format()
    }

    /// Convert to a `(long, medium, short, alpha)` tuple.
    pub fn into_components(self) -> (T, T, T, A) {
        (
            self.color.long,
            self.color.medium,
            self.color.short,
            self.alpha,
        )
    }

    /// Convert from a `(long, medium, short, alpha)` tuple.
    pub fn from_components((long, medium, short, alpha): (T, T, T, A)) -> Self {
        Self::new(long, medium, short, alpha)
    }

    /// Changes the meta type without changing the color value.
    ///
    /// This function doesn't change the numerical values, and thus the stimuli
    /// it represents in an absolute sense. However, the appearance of the color
    /// may not be the same. The effect may be similar to taking a photo with an
    /// incorrect white balance.
    pub fn with_meta<NewM>(self) -> Alpha<Lms<NewM, T>, A> {
        Alpha {
            color: self.color.with_meta(),
            alpha: self.alpha,
        }
    }
}

impl<M, T> FromColorUnclamped<Lms<M, T>> for Lms<M, T> {
    #[inline]
    fn from_color_unclamped(val: Lms<M, T>) -> Self {
        val
    }
}

impl<M, T> FromColorUnclamped<Xyz<M::XyzMeta, T>> for Lms<M, T>
where
    M: HasLmsMatrix + HasXyzMeta,
    M::LmsMatrix: XyzToLms<T>,
    T: Arithmetics,
{
    #[inline]
    fn from_color_unclamped(val: Xyz<M::XyzMeta, T>) -> Self {
        Self::matrix_from_xyz().convert_once(val)
    }
}

impl<M, T> StimulusColor for Lms<M, T> where T: Stimulus {}

impl<M, T> HasBoolMask for Lms<M, T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<M, T> Default for Lms<M, T>
where
    T: Default,
{
    fn default() -> Lms<M, T> {
        Lms::new(T::default(), T::default(), T::default())
    }
}

impl<M> From<Lms<M, f32>> for Lms<M, f64> {
    #[inline]
    fn from(color: Lms<M, f32>) -> Self {
        color.into_format()
    }
}

impl<M> From<Lmsa<M, f32>> for Lmsa<M, f64> {
    #[inline]
    fn from(color: Lmsa<M, f32>) -> Self {
        color.into_format()
    }
}

impl<M> From<Lms<M, f64>> for Lms<M, f32> {
    #[inline]
    fn from(color: Lms<M, f64>) -> Self {
        color.into_format()
    }
}

impl<M> From<Lmsa<M, f64>> for Lmsa<M, f32> {
    #[inline]
    fn from(color: Lmsa<M, f64>) -> Self {
        color.into_format()
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<M, T> bytemuck::Zeroable for Lms<M, T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<M: 'static, T> bytemuck::Pod for Lms<M, T> where T: bytemuck::Pod {}

impl_reference_component_methods!(Lms<M>, [long, medium, short], meta);
impl_struct_of_arrays_methods!(Lms<M>, [long, medium, short], meta);

impl_is_within_bounds! {
    Lms<M> {
        long => [Self::min_long(), None],
        medium => [Self::min_medium(), None],
        short => [Self::min_short(), None]
    }
    where T: Stimulus
}
impl_clamp! {
    Lms<M> {
        long => [Self::min_long()],
        medium => [Self::min_medium()],
        short => [Self::min_short()]
    }
    other {meta}
    where T: Stimulus
}

impl_mix!(Lms<M>);
impl_premultiply!(Lms<M> {long, medium, short} phantom: meta);
impl_euclidean_distance!(Lms<M> {long, medium, short});

impl_color_add!(Lms<M>, [long, medium, short], meta);
impl_color_sub!(Lms<M>, [long, medium, short], meta);
impl_color_mul!(Lms<M>, [long, medium, short], meta);
impl_color_div!(Lms<M>, [long, medium, short], meta);

impl_tuple_conversion!(Lms<M> as (T, T, T));
impl_array_casts!(Lms<M, T>, [T; 3]);
impl_simd_array_conversion!(Lms<M>, [long, medium, short], meta);
impl_struct_of_array_traits!(Lms<M>, [long, medium, short], meta);

impl_eq!(Lms<M>, [long, medium, short]);
impl_copy_clone!(Lms<M>, [long, medium, short], meta);

impl_rand_traits_cartesian!(UniformLms, Lms<M> {long, medium, short} phantom: meta: PhantomData<M>);

#[cfg(test)]
mod test {
    use crate::{lms::VonKriesLms, white_point::D65};

    #[cfg(feature = "alloc")]
    use super::Lmsa;

    #[cfg(feature = "random")]
    use super::Lms;

    #[cfg(feature = "approx")]
    use crate::{convert::FromColorUnclamped, lms::BradfordLms, Xyz};

    test_convert_into_from_xyz!(VonKriesLms<D65, f32>);
    raw_pixel_conversion_tests!(VonKriesLms<D65>: long, medium, short);
    raw_pixel_conversion_fail_tests!(VonKriesLms<D65>: long, medium, short);

    #[cfg(feature = "approx")]
    #[test]
    fn von_kries_xyz_roundtrip() {
        let xyz = Xyz::new(0.2f32, 0.4, 0.8);
        let lms = VonKriesLms::<D65, _>::from_color_unclamped(xyz);
        assert_relative_eq!(Xyz::from_color_unclamped(lms), xyz);
    }

    #[cfg(feature = "approx")]
    #[test]
    fn bradford_xyz_roundtrip() {
        let xyz = Xyz::new(0.2f32, 0.4, 0.8);
        let lms = BradfordLms::<D65, _>::from_color_unclamped(xyz);
        assert_relative_eq!(Xyz::from_color_unclamped(lms), xyz);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized =
            ::serde_json::to_string(&VonKriesLms::<D65, f32>::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"long":0.3,"medium":0.8,"short":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: VonKriesLms<D65, f32> =
            ::serde_json::from_str(r#"{"long":0.3,"medium":0.8,"short":0.1}"#).unwrap();

        assert_eq!(deserialized, VonKriesLms::<D65, f32>::new(0.3, 0.8, 0.1));
    }

    struct_of_arrays_tests!(
        VonKriesLms<D65>[long, medium, short] phantom: meta,
        Lmsa::new(0.1f32, 0.2, 0.3, 0.4),
        Lmsa::new(0.2, 0.3, 0.4, 0.5),
        Lmsa::new(0.3, 0.4, 0.5, 0.6)
    );

    test_uniform_distribution! {
        VonKriesLms<D65, f32> {
            long: (0.0, 1.0),
            medium: (0.0, 1.0),
            short: (0.0, 1.0)
        },
        min: Lms::new(0.0f32, 0.0, 0.0),
        max: Lms::new(1.0, 1.0, 1.0)
    }
}
