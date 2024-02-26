use crate::{
    angle::{RealAngle, SignedAngle},
    bool_mask::LazySelect,
    convert::FromColorUnclamped,
    hues::Cam16Hue,
    num::{
        Abs, Arithmetics, ClampAssign, Exp, One, PartialCmp, Powf, Real, Signum, Sqrt,
        Trigonometry, Zero,
    },
    white_point, Xyz,
};

use super::{BakedParameters, Cam16, Cam16UcsJmh, WhitePointParameter};

pub use chromaticity::*;
pub use luminance::*;

mod chromaticity;
mod luminance;

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
    color_group = "cam16",
    skip_derives(Cam16, PartialCam16, Cam16UcsJmh)
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
    /// Derive partial CIE CAM16 attributes for the provided color, under the provided
    /// viewing conditions.
    #[inline]
    pub fn from_xyz<WpParam>(
        color: Xyz<WpParam::StaticWp, T>,
        parameters: impl Into<BakedParameters<WpParam, T>>,
    ) -> Self
    where
        WpParam: WhitePointParameter<T>,
        T: Real + Arithmetics + Powf + Sqrt + Abs + Signum + Trigonometry + RealAngle + Clone,
    {
        super::math::xyz_to_cam16(color.with_white_point(), parameters.into().inner).into()
    }

    /// Construct an XYZ color from these CIE CAM16 attributes, under the
    /// provided viewing conditions.
    #[inline]
    pub fn into_xyz<WpParam>(
        self,
        parameters: impl Into<BakedParameters<WpParam, T>>,
    ) -> Xyz<WpParam::StaticWp, T>
    where
        WpParam: WhitePointParameter<T>,
        T: Real
            + One
            + Zero
            + Sqrt
            + Powf
            + Abs
            + Signum
            + Arithmetics
            + Trigonometry
            + RealAngle
            + SignedAngle
            + PartialCmp
            + Clone,
        T::Mask: LazySelect<Xyz<white_point::Any, T>>,
    {
        super::math::cam16_to_xyz(self.into_dynamic(), parameters.into().inner).with_white_point()
    }

    /// Create a partial set of CIE CAM16 attributes.
    ///
    /// It's also possible to use `PartialCam16::from` or `Cam16::into`.
    #[inline]
    pub fn from_full(full: Cam16<T>) -> Self {
        let Cam16 {
            lightness,
            chroma,
            hue,
            brightness,
            colorfulness,
            saturation,
        } = full;

        PartialCam16 {
            hue,
            chromaticity: C::from_cam16(chroma, colorfulness, saturation),
            luminance: L::from_cam16(lightness, brightness),
        }
    }

    /// Reconstruct a full set of CIE CAM16 attributes, using the original viewing conditions.
    #[inline]
    pub fn into_full<WpParam>(self, parameters: impl Into<BakedParameters<WpParam, T>>) -> Cam16<T>
    where
        WpParam: WhitePointParameter<T>,
        T: Real + Zero + Arithmetics + Sqrt + PartialCmp + Clone,
        T::Mask: LazySelect<T> + Clone,
    {
        let parameters = parameters.into();
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
        Self::from_full(val)
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
        Self::from_full(value)
    }
}

#[cfg(test)]
mod test {
    use super::{
        PartialCam16Jch, PartialCam16Jmh, PartialCam16Jsh, PartialCam16Qch, PartialCam16Qmh,
        PartialCam16Qsh,
    };
    use crate::{
        cam16::{Cam16, Parameters, ParametersStaticWp},
        convert::IntoColorUnclamped,
        white_point::D65,
        Srgb,
    };

    macro_rules! assert_partial_to_full {
        ($cam16: expr) => {assert_partial_to_full!($cam16,)};
        ($cam16: expr, $($params:tt)*) => {
            assert_relative_eq!(
                PartialCam16Jch::from($cam16).into_full(ParametersStaticWp::<D65, _>::TEST_DEFAULTS),
                $cam16,
                $($params)*
            );
            assert_relative_eq!(
                PartialCam16Jmh::from($cam16).into_full(ParametersStaticWp::<D65, _>::TEST_DEFAULTS),
                $cam16,
                $($params)*
            );
            assert_relative_eq!(
                PartialCam16Jsh::from($cam16).into_full(ParametersStaticWp::<D65, _>::TEST_DEFAULTS),
                $cam16,
                $($params)*
            );

            assert_relative_eq!(
                PartialCam16Qch::from($cam16).into_full(ParametersStaticWp::<D65, _>::TEST_DEFAULTS),
                $cam16,
                $($params)*
            );
            assert_relative_eq!(
                PartialCam16Qmh::from($cam16).into_full(ParametersStaticWp::<D65, _>::TEST_DEFAULTS),
                $cam16,
                $($params)*
            );
            assert_relative_eq!(
                PartialCam16Qsh::from($cam16).into_full(ParametersStaticWp::<D65, _>::TEST_DEFAULTS),
                $cam16,
                $($params)*
            );
        };
    }

    #[test]
    fn example_blue() {
        // Uses the example color from https://observablehq.com/@jrus/cam16
        let xyz = Srgb::from(0x5588cc).into_linear().into_color_unclamped();
        let cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
        assert_partial_to_full!(cam16);
    }

    #[test]
    fn black() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let xyz = Srgb::from(0x000000).into_linear().into_color_unclamped();
        let cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
        assert_partial_to_full!(cam16);
    }

    #[test]
    fn white() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let xyz = Srgb::from(0xffffff).into_linear().into_color_unclamped();
        let cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
        assert_partial_to_full!(cam16, epsilon = 0.000000000000001);
    }

    #[test]
    fn red() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let xyz = Srgb::from(0xff0000).into_linear().into_color_unclamped();
        let cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
        assert_partial_to_full!(cam16, epsilon = 0.0000000000001);
    }

    #[test]
    fn green() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let xyz = Srgb::from(0x00ff00).into_linear().into_color_unclamped();
        let cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
        assert_partial_to_full!(cam16, epsilon = 0.0000000000001);
    }

    #[test]
    fn blue() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let xyz = Srgb::from(0x0000ff).into_linear().into_color_unclamped();
        let cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
        assert_partial_to_full!(cam16);
    }
}
