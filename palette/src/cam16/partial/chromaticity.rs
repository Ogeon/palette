use crate::{
    bool_mask::{LazySelect, Select},
    cam16::{
        math::{self, DependentParameters},
        BakedParameters,
    },
    num::{Arithmetics, Clamp, ClampAssign, PartialCmp, Real, Sqrt, Zero},
};

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
    pub(crate) fn into_cam16<Wp>(
        self,
        lightness: T,
        parameters: BakedParameters<Wp, T>,
    ) -> (T, T, T)
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
