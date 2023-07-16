use core::{fmt::Debug, marker::PhantomData};

use crate::{
    angle::RealAngle,
    bool_mask::{HasBoolMask, LazySelect},
    convert::{FromColorUnclamped, IntoColorUnclamped},
    hues::Cam16Hue,
    num::{
        Abs, Arithmetics, Clamp, Exp, One, PartialCmp, Powf, Real, Signum, Sqrt, Trigonometry, Zero,
    },
    white_point::{self, WhitePoint, D65},
    FromColor, Xyz,
};

#[cfg(feature = "approx")]
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

mod math;

#[derive(Debug)]
pub struct Cam16<Wp, T> {
    #[doc(alias = "J")]
    pub luminance: T,
    #[doc(alias = "C")]
    pub chroma: T,
    #[doc(alias = "h")]
    pub hue: Cam16Hue<T>,
    #[doc(alias = "Q")]
    pub brightness: T,
    #[doc(alias = "M")]
    pub colorfulness: T,
    #[doc(alias = "s")]
    pub saturation: T,

    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Cam16<Wp, T> {
    fn with_white_point<Wp2>(self) -> Cam16<Wp2, T> {
        let Cam16 {
            luminance,
            chroma,
            hue,
            brightness,
            colorfulness,
            saturation,
            white_point: _,
        } = self;

        Cam16 {
            luminance,
            chroma,
            hue,
            brightness,
            colorfulness,
            saturation,
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Clone for Cam16<Wp, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            luminance: self.luminance.clone(),
            chroma: self.chroma.clone(),
            hue: self.hue.clone(),
            brightness: self.brightness.clone(),
            colorfulness: self.colorfulness.clone(),
            saturation: self.saturation.clone(),
            white_point: PhantomData,
        }
    }
}

impl<Wp, T> Copy for Cam16<Wp, T> where T: Copy {}

impl_eq_hue!(
    Cam16<Wp>,
    Cam16Hue,
    [luminance, chroma, brightness, colorfulness, saturation]
);

pub trait IntoCam16<Wp, T> {
    fn into_cam16(self, parameters: Parameters<Wp, T>) -> Cam16<Wp, T>;
}

impl<C, Wp, T> IntoCam16<Wp, T> for C
where
    C: IntoColorUnclamped<Xyz<Wp, T>>,
    T: Real
        + One
        + Zero
        + Clamp
        + PartialCmp
        + Arithmetics
        + Powf
        + Sqrt
        + Exp
        + Abs
        + Signum
        + Trigonometry
        + RealAngle
        + HasBoolMask
        + Clone,
    T::Mask: LazySelect<T>,
    Wp: WhitePoint<T>,
{
    fn into_cam16(self, parameters: Parameters<Wp, T>) -> Cam16<Wp, T> {
        math::xyz_to_cam16(
            self.into_color_unclamped().with_white_point(),
            parameters.into_any_white_point(),
        )
        .with_white_point()
    }
}

impl<C, Wp, T> FromColorUnclamped<C> for Cam16<Wp, T>
where
    C: IntoCam16<Wp, T>,
    T: Real,
{
    fn from_color_unclamped(val: C) -> Self {
        val.into_cam16(Parameters::default())
    }
}

impl<C, Wp, T> FromColor<C> for Cam16<Wp, T>
where
    C: IntoCam16<Wp, T>,
    T: Real,
{
    fn from_color(val: C) -> Self {
        Self::from_color_unclamped(val)
    }
}

#[non_exhaustive]
pub struct Parameters<Wp, T> {
    pub white_point: WhitePointParameter<Wp, T>,
    pub adapting_luminance: T,
    pub background_luminance: T,
    pub surround: Surround<T>,
    pub discounting: bool,
}

impl<Wp, T> Parameters<Wp, T>
where
    Wp: WhitePoint<T>,
{
    fn into_any_white_point(self) -> Parameters<white_point::Any, T> {
        Parameters {
            white_point: WhitePointParameter::Custom(self.white_point.into_xyz()),
            adapting_luminance: self.adapting_luminance,
            background_luminance: self.background_luminance,
            surround: self.surround,
            discounting: self.discounting,
        }
    }
}

impl<Wp, T> Parameters<Wp, T>
where
    T: Real,
{
    /// Create a new set of default parameters.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<Wp, T> Default for Parameters<Wp, T>
where
    T: Real,
{
    fn default() -> Self {
        Self {
            white_point: WhitePointParameter::Default,
            adapting_luminance: T::from_f64(40.0),
            background_luminance: T::from_f64(20.0),
            surround: Surround::Average,
            discounting: Default::default(),
        }
    }
}

#[non_exhaustive]
pub enum Surround<T> {
    Dark,
    Dim,
    Average,
    Custom(T),
}

impl<T> Surround<T> {
    fn into_value(self) -> T
    where
        T: Real + Clamp,
    {
        match self {
            Surround::Dark => T::from_f64(0.0),
            Surround::Dim => T::from_f64(1.0),
            Surround::Average => T::from_f64(2.0),
            Surround::Custom(value) => value.clamp(T::from_f64(0.0), T::from_f64(2.0)),
        }
    }
}

#[non_exhaustive]
pub enum WhitePointParameter<Wp, T> {
    Default,
    Custom(Xyz<Wp, T>),
}

impl<Wp, T> WhitePointParameter<Wp, T>
where
    Wp: WhitePoint<T>,
{
    fn into_xyz(self) -> Xyz<white_point::Any, T> {
        match self {
            WhitePointParameter::Default => Wp::get_xyz(),
            WhitePointParameter::Custom(xyz) => xyz.with_white_point(),
        }
    }
}

impl<T> WhitePointParameter<white_point::Any, T>
where
    T: Real,
{
    fn any_into_xyz(self) -> Xyz<white_point::Any, T> {
        match self {
            WhitePointParameter::Default => D65::get_xyz(),
            WhitePointParameter::Custom(xyz) => xyz,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{convert::IntoColor, Srgb};

    use super::Cam16;

    #[test]
    fn example_blue() {
        // Uses the example color from https://observablehq.com/@jrus/cam16
        let mut cam16: Cam16<_, f64> = Srgb::from(0x5588cc).into_linear().into_color();
        cam16.hue = cam16.hue.into_positive_degrees().into();

        assert_relative_eq!(
            cam16,
            Cam16 {
                luminance: 45.544264720360346,
                chroma: 45.07001048293764,
                hue: 259.225345298129.into(),
                brightness: 132.96974182692045,
                colorfulness: 39.4130607870103,
                saturation: 54.4432031413259,
                white_point: core::marker::PhantomData
            },
            epsilon = 0.01
        );
    }
}
