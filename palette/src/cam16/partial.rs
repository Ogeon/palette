use crate::{
    bool_mask::{LazySelect, Select},
    convert::{FromColorUnclamped, IntoColorUnclamped},
    hues::Cam16Hue,
    num::{Arithmetics, Clamp, ClampAssign, Exp, One, PartialCmp, Real, Sqrt, Zero},
    Xyz,
};

use super::{
    math::{self, DependentParameters},
    BakedParameters, Cam16, Cam16UcsJmh, FromCam16, IntoCam16, WhitePointParameter,
};

/// An alias for [`PartialCam16`], where the chromaticity and luminance
/// attributes are decided during runtime.
pub type DynPartialCam16<T> = PartialCam16<T, LuminanceType<T>, ChromaticityType<T>>;

/// An alias for [`PartialCam16`], with lightness and chroma.
pub type PartialCam16Jch<T> = PartialCam16<T, Lightness<T>, Chroma<T>>;

/// An alias for [`PartialCam16`], with lightness and colorfulness.
pub type PartialCam16Jmh<T> = PartialCam16<T, Lightness<T>, Colorfulness<T>>;

/// An alias for [`PartialCam16`], with lightness and saturation.
pub type PartialCam16Jsh<T> = PartialCam16<T, Lightness<T>, Saturation<T>>;

/// An alias for [`PartialCam16`], with brightness and chroma.
pub type PartialCam16Qch<T> = PartialCam16<T, Brightness<T>, Chroma<T>>;

/// An alias for [`PartialCam16`], with brightness and colorfulness.
pub type PartialCam16Qmh<T> = PartialCam16<T, Brightness<T>, Colorfulness<T>>;

/// An alias for [`PartialCam16`], with brightness and saturation.
pub type PartialCam16Qsh<T> = PartialCam16<T, Brightness<T>, Saturation<T>>;

/// A partial version of [`Cam16`] with only one of each kind of attribute.
///
/// It's likely preferred to use one of its aliases:
///
/// * [`DynPartialCam16`]: dynamic attributes.
/// * [`PartialCam16Jch`]: lightness and chroma.
/// * [`PartialCam16Jmh`]: lightness and colorfulness.
/// * [`PartialCam16Jsh`]: lightness and saturation.
/// * [`PartialCam16Qch`]: brightness and chroma.
/// * [`PartialCam16Qmh`]: brightness and colorfulness.
/// * [`PartialCam16Qsh`]: brightness and saturation.
///
/// This is enough information for converting CAM16 to other color spaces.
#[derive(Clone, Copy, Debug, WithAlpha, FromColorUnclamped)]
#[palette(
    palette_internal,
    component = "T",
    skip_derives(Xyz, Cam16, PartialCam16, Cam16UcsJmh)
)]
pub struct PartialCam16<T, L, C> {
    /// The [lightness](https://en.wikipedia.org/wiki/Lightness) (J) or
    /// [brightness](https://en.wikipedia.org/wiki/Brightness) (Q) of the color.
    pub luminance: L,

    /// The [chroma](https://en.wikipedia.org/wiki/Colorfulness#Chroma) (C),
    /// [colorfulness](https://en.wikipedia.org/wiki/Colorfulness) (M), or
    /// [saturation](https://en.wikipedia.org/wiki/Colorfulness#Saturation) (s)
    /// of the color.
    pub chromaticity: C,

    /// The [hue](https://en.wikipedia.org/wiki/Hue) (h) of the color.
    pub hue: Cam16Hue<T>,
}

impl<T, L, C> PartialCam16<T, L, C>
where
    L: Cam16Luminance<T>,
    C: Cam16Chromaticity<T>,
{
    /// Turn the chromaticity and luminance into dynamically decided attributes.
    pub fn into_dynamic(self) -> DynPartialCam16<T> {
        let PartialCam16 {
            hue,
            chromaticity,
            luminance,
        } = self;

        PartialCam16 {
            hue,
            chromaticity: chromaticity.into_dynamic(),
            luminance: luminance.into_dynamic(),
        }
    }
}

impl<T, L, C> FromColorUnclamped<PartialCam16<T, L, C>> for PartialCam16<T, L, C> {
    fn from_color_unclamped(val: PartialCam16<T, L, C>) -> Self {
        val
    }
}

impl<T, L, C> FromColorUnclamped<Cam16<T>> for PartialCam16<T, L, C>
where
    L: Cam16Luminance<T>,
    C: Cam16Chromaticity<T>,
{
    fn from_color_unclamped(val: Cam16<T>) -> Self {
        Self::from(val)
    }
}

impl<Wp, T, L, C> FromColorUnclamped<Xyz<Wp, T>> for PartialCam16<T, L, C>
where
    Self: FromColorUnclamped<Cam16<T>>,
    Cam16<T>: FromColorUnclamped<Xyz<Wp, T>>,
{
    fn from_color_unclamped(val: Xyz<Wp, T>) -> Self {
        Cam16::from_color_unclamped(val).into_color_unclamped()
    }
}

impl<T> FromColorUnclamped<Cam16UcsJmh<T>> for PartialCam16Jmh<T>
where
    T: Real + One + Exp + Arithmetics + Clone,
{
    fn from_color_unclamped(val: Cam16UcsJmh<T>) -> Self {
        let colorfulness =
            ((val.colorfulness * T::from_f64(0.0228)).exp() - T::one()) / T::from_f64(0.0228);
        let lightness =
            val.lightness.clone() / (T::from_f64(1.7) - T::from_f64(0.007) * val.lightness);

        Self {
            hue: val.hue,
            chromaticity: Colorfulness(colorfulness),
            luminance: Lightness(lightness),
        }
    }
}

impl<T> FromColorUnclamped<Cam16UcsJmh<T>> for DynPartialCam16<T>
where
    PartialCam16Jmh<T>: FromColorUnclamped<Cam16UcsJmh<T>>,
{
    fn from_color_unclamped(val: Cam16UcsJmh<T>) -> Self {
        PartialCam16Jmh::from_color_unclamped(val).into_dynamic()
    }
}

impl<WpParam, T, Ls, Cs> FromCam16<WpParam, T> for PartialCam16<T, Ls, Cs>
where
    WpParam: WhitePointParameter<T>,
    T: Real + Zero + Arithmetics + Sqrt + PartialCmp + Clone,
    T::Mask: LazySelect<T> + Clone,
    Ls: Cam16Luminance<T>,
    Cs: Cam16Chromaticity<T>,
{
    fn from_cam16(cam16: Cam16<T>, _parameters: BakedParameters<WpParam, T>) -> Self {
        cam16.into()
    }

    fn from_partial_cam16<L, C>(
        cam16: PartialCam16<T, L, C>,
        parameters: BakedParameters<WpParam, T>,
    ) -> Self
    where
        L: Cam16Luminance<T>,
        C: Cam16Chromaticity<T>,
    {
        cam16.into_cam16(parameters).into()
    }
}

impl<WpParam, T, L, C> IntoCam16<WpParam, T> for PartialCam16<T, L, C>
where
    WpParam: WhitePointParameter<T>,
    T: Real + Zero + Arithmetics + Sqrt + PartialCmp + Clone,
    T::Mask: LazySelect<T> + Clone,
    L: Cam16Luminance<T>,
    C: Cam16Chromaticity<T>,
{
    fn into_cam16(self, parameters: BakedParameters<WpParam, T>) -> Cam16<T> {
        let PartialCam16 {
            hue,
            chromaticity,
            luminance,
        } = self.into_dynamic();

        let (lightness, brightness) = luminance.into_cam16(parameters.clone());
        let (chroma, colorfulness, saturation) =
            chromaticity.into_cam16(lightness.clone(), parameters);

        Cam16 {
            lightness,
            chroma,
            hue,
            brightness,
            colorfulness,
            saturation,
        }
    }
}

impl<T, L, C> crate::Clamp for PartialCam16<T, L, C>
where
    C: crate::Clamp,
    L: crate::Clamp,
{
    fn clamp(self) -> Self {
        Self {
            hue: self.hue,
            chromaticity: self.chromaticity.clamp(),
            luminance: self.luminance.clamp(),
        }
    }
}

impl<T, L, C> crate::ClampAssign for PartialCam16<T, L, C>
where
    T: ClampAssign + Zero,
    C: crate::ClampAssign,
    L: crate::ClampAssign,
{
    fn clamp_assign(&mut self) {
        self.chromaticity.clamp_assign();
        self.luminance.clamp_assign();
    }
}

impl<T, L, C> From<Cam16<T>> for PartialCam16<T, L, C>
where
    L: Cam16Luminance<T>,
    C: Cam16Chromaticity<T>,
{
    fn from(value: Cam16<T>) -> Self {
        let Cam16 {
            lightness,
            chroma,
            hue,
            brightness,
            colorfulness,
            saturation,
        } = value;

        PartialCam16 {
            hue,
            chromaticity: C::from_cam16(chroma, colorfulness, saturation),
            luminance: L::from_cam16(lightness, brightness),
        }
    }
}

/// Common methods for types representing apparent chromatic intensity metrics
/// of CAM16.
pub trait Cam16Chromaticity<T> {
    /// Create `Self` from a CAM16 chromaticity attribute.
    fn from_cam16(chroma: T, colorfulness: T, saturation: T) -> Self;

    /// Convert `self` into a dynamically decided chromaticity attribute.
    fn into_dynamic(self) -> ChromaticityType<T>;
}

/// One of the apparent chromatic intensity metrics of CAM16, to be used in
/// [`PartialCam16`].
///
/// Combined with the hue and one of [`LuminanceType`], it can describe a
/// complete color as [`PartialCam16`].
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum ChromaticityType<T> {
    /// The [chroma](https://en.wikipedia.org/wiki/Colorfulness#Chroma) (C) of a
    /// color.
    #[doc(alias = "C")]
    Chroma(T),

    /// The [colorfulness](https://en.wikipedia.org/wiki/Colorfulness) (M) of a
    /// color.
    #[doc(alias = "M")]
    Colorfulness(T),

    /// The [saturation](https://en.wikipedia.org/wiki/Colorfulness#Saturation)
    /// (s) of a color.
    #[doc(alias = "s")]
    Saturation(T),
}

impl<T> ChromaticityType<T> {
    fn into_cam16<Wp>(self, lightness: T, parameters: BakedParameters<Wp, T>) -> (T, T, T)
    where
        T: Real + Zero + Arithmetics + Sqrt + PartialCmp + Clone,
        T::Mask: LazySelect<T> + Clone,
    {
        let DependentParameters { c, a_w, f_l_4, .. } = parameters.inner;
        let is_black = lightness.eq(&T::zero());

        match self {
            ChromaticityType::Chroma(chroma) => {
                let colorfulness = lazy_select! {
                    if is_black.clone() => T::zero(),
                    else => math::chroma_to_colorfulness(chroma.clone(), f_l_4)
                };
                let saturation = lazy_select! {
                        if is_black.clone() => T::zero(),
                        else => math::chroma_to_saturation(
                        chroma.clone(),
                        lightness,
                        c,
                        a_w,
                    )
                };
                let chroma = is_black.select(T::zero(), chroma);

                (chroma, colorfulness, saturation)
            }
            ChromaticityType::Colorfulness(colorfulness) => {
                let chroma = lazy_select! {
                    if is_black.clone() => T::zero(),
                    else => math::colorfulness_to_chroma(colorfulness.clone(), f_l_4)
                };
                let saturation = lazy_select! {
                        if is_black.clone() => T::zero(),
                        else => math::chroma_to_saturation(
                        chroma.clone(),
                        lightness,
                        c,
                        a_w,
                    )
                };
                let colorfulness = is_black.select(T::zero(), colorfulness);

                (chroma, colorfulness, saturation)
            }
            ChromaticityType::Saturation(saturation) => {
                let chroma = lazy_select! {
                        if is_black.clone() => T::zero(),
                        else => math::saturation_to_chroma(
                        saturation.clone(),
                        lightness,
                        c,
                        a_w,
                    )
                };
                let colorfulness = lazy_select! {
                    if is_black.clone() => T::zero(),
                    else => math::chroma_to_colorfulness(chroma.clone(), f_l_4)
                };
                let saturation = is_black.select(T::zero(), saturation);

                (chroma, colorfulness, saturation)
            }
        }
    }
}

impl<T> Cam16Chromaticity<T> for ChromaticityType<T> {
    fn from_cam16(chroma: T, _colorfulness: T, _saturation: T) -> Self {
        ChromaticityType::Chroma(chroma)
    }

    fn into_dynamic(self) -> ChromaticityType<T> {
        self
    }
}

impl<T> crate::Clamp for ChromaticityType<T>
where
    T: Zero + Clamp,
{
    fn clamp(self) -> Self {
        match self {
            ChromaticityType::Chroma(chroma) => {
                ChromaticityType::Chroma(chroma.clamp_min(T::zero()))
            }
            ChromaticityType::Colorfulness(colorfulness) => {
                ChromaticityType::Colorfulness(colorfulness.clamp_min(T::zero()))
            }
            ChromaticityType::Saturation(saturation) => {
                ChromaticityType::Saturation(saturation.clamp_min(T::zero()))
            }
        }
    }
}

impl<T> crate::ClampAssign for ChromaticityType<T>
where
    T: Zero + ClampAssign,
{
    fn clamp_assign(&mut self) {
        match self {
            ChromaticityType::Chroma(chroma) => chroma.clamp_min_assign(T::zero()),
            ChromaticityType::Colorfulness(colorfulness) => {
                colorfulness.clamp_min_assign(T::zero())
            }
            ChromaticityType::Saturation(saturation) => saturation.clamp_min_assign(T::zero()),
        }
    }
}

/// The [chroma](https://en.wikipedia.org/wiki/Colorfulness#Chroma) (C) of a
/// color, to be used in [`PartialCam16`].
#[derive(Clone, Copy, Debug)]
pub struct Chroma<T>(pub T);

impl<T> Cam16Chromaticity<T> for Chroma<T> {
    fn from_cam16(chroma: T, _colorfulness: T, _saturation: T) -> Self {
        Self(chroma)
    }

    fn into_dynamic(self) -> ChromaticityType<T> {
        ChromaticityType::Chroma(self.0)
    }
}

impl<T> crate::Clamp for Chroma<T>
where
    T: Zero + Clamp,
{
    fn clamp(self) -> Self {
        Self(self.0.clamp_min(T::zero()))
    }
}

impl<T> crate::ClampAssign for Chroma<T>
where
    T: Zero + ClampAssign,
{
    fn clamp_assign(&mut self) {
        self.0.clamp_min_assign(T::zero());
    }
}

/// The [colorfulness](https://en.wikipedia.org/wiki/Colorfulness) (M) of a
/// color, to be used in [`PartialCam16`].
#[derive(Clone, Copy, Debug)]
pub struct Colorfulness<T>(pub T);

impl<T> Cam16Chromaticity<T> for Colorfulness<T> {
    fn from_cam16(_chroma: T, colorfulness: T, _saturation: T) -> Self {
        Self(colorfulness)
    }

    fn into_dynamic(self) -> ChromaticityType<T> {
        ChromaticityType::Colorfulness(self.0)
    }
}

impl<T> crate::Clamp for Colorfulness<T>
where
    T: Zero + Clamp,
{
    fn clamp(self) -> Self {
        Self(self.0.clamp_min(T::zero()))
    }
}

impl<T> crate::ClampAssign for Colorfulness<T>
where
    T: Zero + ClampAssign,
{
    fn clamp_assign(&mut self) {
        self.0.clamp_min_assign(T::zero());
    }
}

/// The [saturation](https://en.wikipedia.org/wiki/Colorfulness#Saturation) (s)
/// of a color, to be used in [`PartialCam16`].
#[derive(Clone, Copy, Debug)]
pub struct Saturation<T>(pub T);

impl<T> Cam16Chromaticity<T> for Saturation<T> {
    fn from_cam16(_chroma: T, _colorfulness: T, saturation: T) -> Self {
        Self(saturation)
    }

    fn into_dynamic(self) -> ChromaticityType<T> {
        ChromaticityType::Saturation(self.0)
    }
}

impl<T> crate::Clamp for Saturation<T>
where
    T: Zero + Clamp,
{
    fn clamp(self) -> Self {
        Self(self.0.clamp_min(T::zero()))
    }
}

impl<T> crate::ClampAssign for Saturation<T>
where
    T: Zero + ClampAssign,
{
    fn clamp_assign(&mut self) {
        self.0.clamp_min_assign(T::zero());
    }
}

/// Common methods for types representing apparent luminance metrics of CAM16.
pub trait Cam16Luminance<T> {
    /// Create `Self` from a CAM16 luminance attribute.
    fn from_cam16(lightness: T, brightness: T) -> Self;

    /// Convert `self` into a dynamically decided luminance attribute.
    fn into_dynamic(self) -> LuminanceType<T>;
}

/// One of the apparent luminance metrics of CAM16.
///
/// Combined with the hue and one of [`ChromaticityType`], it can describe a
/// complete color as [`PartialCam16`].
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum LuminanceType<T> {
    /// The [lightness](https://en.wikipedia.org/wiki/Lightness) (J) of a color.
    #[doc(alias = "J")]
    Lightness(T),

    /// The [brightness](https://en.wikipedia.org/wiki/Brightness) (Q) of a
    /// color.
    #[doc(alias = "Q")]
    Brightness(T),
}

impl<T> LuminanceType<T> {
    fn into_cam16<Wp>(self, parameters: BakedParameters<Wp, T>) -> (T, T)
    where
        T: Real + Zero + Arithmetics + Sqrt + PartialCmp + Clone,
        T::Mask: LazySelect<T> + Clone,
    {
        let parameters = parameters.inner;

        match self {
            LuminanceType::Lightness(lightness) => {
                let is_black = lightness.eq(&T::zero());
                let brightness = lazy_select! {
                        if is_black => T::zero(),
                        else => math::lightness_to_brightness(
                        lightness.clone(),
                        parameters.c,
                        parameters.a_w,
                        parameters.f_l_4,
                    )
                };

                (lightness, brightness)
            }
            LuminanceType::Brightness(brightness) => {
                let is_black = brightness.eq(&T::zero());
                let lightness = lazy_select! {
                        if is_black => T::zero(),
                        else => math::brightness_to_lightness(
                        brightness.clone(),
                        parameters.c,
                        parameters.a_w,
                        parameters.f_l_4,
                    )
                };

                (lightness, brightness)
            }
        }
    }
}

impl<T> Cam16Luminance<T> for LuminanceType<T> {
    fn from_cam16(lightness: T, _brightness: T) -> Self {
        LuminanceType::Lightness(lightness)
    }

    fn into_dynamic(self) -> LuminanceType<T> {
        self
    }
}

impl<T> crate::Clamp for LuminanceType<T>
where
    T: Zero + Clamp,
{
    fn clamp(self) -> Self {
        match self {
            LuminanceType::Lightness(lightness) => {
                LuminanceType::Lightness(lightness.clamp_min(T::zero()))
            }
            LuminanceType::Brightness(brightness) => {
                LuminanceType::Brightness(brightness.clamp_min(T::zero()))
            }
        }
    }
}

impl<T> crate::ClampAssign for LuminanceType<T>
where
    T: Zero + ClampAssign,
{
    fn clamp_assign(&mut self) {
        match self {
            LuminanceType::Lightness(lightness) => lightness.clamp_min_assign(T::zero()),
            LuminanceType::Brightness(brightness) => brightness.clamp_min_assign(T::zero()),
        }
    }
}

/// The [lightness](https://en.wikipedia.org/wiki/Lightness) (J) of a color, to
/// be used in [`PartialCam16`].
#[derive(Clone, Copy, Debug)]
pub struct Lightness<T>(pub T);

impl<T> Cam16Luminance<T> for Lightness<T> {
    fn from_cam16(lightness: T, _brightness: T) -> Self {
        Self(lightness)
    }

    fn into_dynamic(self) -> LuminanceType<T> {
        LuminanceType::Lightness(self.0)
    }
}

impl<T> crate::Clamp for Lightness<T>
where
    T: Zero + Clamp,
{
    fn clamp(self) -> Self {
        Self(self.0.clamp_min(T::zero()))
    }
}

impl<T> crate::ClampAssign for Lightness<T>
where
    T: Zero + ClampAssign,
{
    fn clamp_assign(&mut self) {
        self.0.clamp_min_assign(T::zero());
    }
}

/// The [brightness](https://en.wikipedia.org/wiki/Brightness) (Q) of a color,
/// to be used in [`PartialCam16`].
#[derive(Clone, Copy, Debug)]
pub struct Brightness<T>(pub T);

impl<T> Cam16Luminance<T> for Brightness<T> {
    fn from_cam16(_lightness: T, brightness: T) -> Self {
        Self(brightness)
    }

    fn into_dynamic(self) -> LuminanceType<T> {
        LuminanceType::Brightness(self.0)
    }
}

impl<T> crate::Clamp for Brightness<T>
where
    T: Zero + Clamp,
{
    fn clamp(self) -> Self {
        Self(self.0.clamp_min(T::zero()))
    }
}

impl<T> crate::ClampAssign for Brightness<T>
where
    T: Zero + ClampAssign,
{
    fn clamp_assign(&mut self) {
        self.0.clamp_min_assign(T::zero());
    }
}

#[cfg(test)]
mod test {
    use super::{
        PartialCam16Jch, PartialCam16Jmh, PartialCam16Jsh, PartialCam16Qch, PartialCam16Qmh,
        PartialCam16Qsh,
    };
    use crate::{
        cam16::{BakedParameters, Cam16, IntoCam16, StaticWp},
        convert::IntoColorUnclamped,
        white_point::D65,
        Srgb,
    };

    macro_rules! assert_partial_to_full {
        ($cam16: expr) => {assert_partial_to_full!($cam16,)};
        ($cam16: expr, $($params:tt)*) => {
            assert_relative_eq!(
                PartialCam16Jch::from($cam16).into_cam16(BakedParameters::<StaticWp<D65>, _>::default()),
                $cam16,
                $($params)*
            );
            assert_relative_eq!(
                PartialCam16Jmh::from($cam16).into_cam16(BakedParameters::<StaticWp<D65>, _>::default()),
                $cam16,
                $($params)*
            );
            assert_relative_eq!(
                PartialCam16Jsh::from($cam16).into_cam16(BakedParameters::<StaticWp<D65>, _>::default()),
                $cam16,
                $($params)*
            );

            assert_relative_eq!(
                PartialCam16Qch::from($cam16).into_cam16(BakedParameters::<StaticWp<D65>, _>::default()),
                $cam16,
                $($params)*
            );
            assert_relative_eq!(
                PartialCam16Qmh::from($cam16).into_cam16(BakedParameters::<StaticWp<D65>, _>::default()),
                $cam16,
                $($params)*
            );
            assert_relative_eq!(
                PartialCam16Qsh::from($cam16).into_cam16(BakedParameters::<StaticWp<D65>, _>::default()),
                $cam16,
                $($params)*
            );
        };
    }

    #[test]
    fn example_blue() {
        // Uses the example color from https://observablehq.com/@jrus/cam16
        let cam16: Cam16<f64> = Srgb::from(0x5588cc).into_linear().into_color_unclamped();
        assert_partial_to_full!(cam16);
    }

    #[test]
    fn black() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let cam16: Cam16<f64> = Srgb::from(0x000000).into_linear().into_color_unclamped();
        assert_partial_to_full!(cam16);
    }

    #[test]
    fn white() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let cam16: Cam16<f64> = Srgb::from(0xffffff).into_linear().into_color_unclamped();
        assert_partial_to_full!(cam16, epsilon = 0.000000000000001);
    }

    #[test]
    fn red() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let cam16: Cam16<f64> = Srgb::from(0xff0000).into_linear().into_color_unclamped();
        assert_partial_to_full!(cam16, epsilon = 0.0000000000001);
    }

    #[test]
    fn green() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let cam16: Cam16<f64> = Srgb::from(0x00ff00).into_linear().into_color_unclamped();
        assert_partial_to_full!(cam16, epsilon = 0.0000000000001);
    }

    #[test]
    fn blue() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let cam16: Cam16<f64> = Srgb::from(0x0000ff).into_linear().into_color_unclamped();
        assert_partial_to_full!(cam16);
    }
}
