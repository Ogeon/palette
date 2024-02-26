use crate::{
    bool_mask::LazySelect,
    cam16::BakedParameters,
    num::{Arithmetics, Clamp, ClampAssign, PartialCmp, Real, Sqrt, Zero},
};

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
    pub(crate) fn into_cam16<Wp>(self, parameters: BakedParameters<Wp, T>) -> (T, T)
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
                        else => crate::cam16::math::lightness_to_brightness(
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
                        else => crate::cam16::math::brightness_to_lightness(
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
