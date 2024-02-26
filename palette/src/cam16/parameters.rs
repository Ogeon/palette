use core::marker::PhantomData;

use crate::{
    bool_mask::LazySelect,
    num::{Abs, Arithmetics, Clamp, Exp, One, PartialCmp, Powf, Real, Signum, Sqrt, Zero},
    white_point::{self, WhitePoint, D65},
    Xyz,
};

/// An alias for [`Parameters`] with a static white point.
///
/// This alias helps the compiler infer the type parameters, which it may
/// struggle with if `Parameters::default` is used.
/// [`Parameters::default_static_wp`] can also help when specifying the white
/// point.
pub type ParametersStaticWp<Wp, T> = Parameters<StaticWp<Wp>, T>;

/// An alias for [`Parameters`] with a dynamic white point.
///
/// This alias helps the compiler infer the type parameters, which it may
/// struggle with if `Parameters::default` is used.
/// [`Parameters::default_dynamic_wp`] can also help when specifying the white
/// point.
pub type ParametersDynamicWp<T> = Parameters<Xyz<white_point::Any, T>, T>;

/// Parameters for CAM16 that describe the viewing conditions.
///
/// These parameters describe the viewing conditions for a more accurate color
/// appearance metric. The CAM16 attributes and derived values are only really
/// comparable if they were calculated with the same parameters. The parameters
/// are, however, too dynamic to all be part of the type parameters of
/// [`Cam16`][super::Cam16].
///
/// The default values are mostly a "blank slate", with a couple of educated
/// guesses. Be sure to at least customize the luminances according to the
/// expected environment:
///
/// ```
/// use palette::{Srgb, Xyz, IntoColor, cam16::{Parameters, Cam16}};
///
/// let mut example_parameters = Parameters::default_static_wp();
/// example_parameters.adapting_luminance = 40.0;
/// example_parameters.background_luminance = 20.0;
///
/// let example_color_xyz = Srgb::from(0x5588cc).into_linear().into_color();
/// let cam16 = Cam16::from_xyz(example_color_xyz, example_parameters);
/// ```
///
/// See also Moroney (2000) [Usage Guidelines for CIECAM97s][moroney_2000] for
/// more information and advice on how to customize these parameters.
///
/// [moroney_2000]:
///     https://www.imaging.org/common/uploaded%20files/pdfs/Papers/2000/PICS-0-81/1611.pdf
#[derive(Clone, Copy)]
#[non_exhaustive]
pub struct Parameters<WpParam, T> {
    /// The reference white point. Defaults to `Wp` when it implements
    /// [`WhitePoint`], or [`D65`] when `Wp` is [`white_point::Any`]. It can
    /// also be set to a custom value if `Wp` results in the wrong white point.
    pub white_point: WpParam,

    /// The average luminance of the environment (*L<sub>A</sub>*) in
    /// *cd/m<sup>2</sup>* (nits). Under a “gray world” assumption this is 20%
    /// of the luminance of a white reference. Defaults to `T::default()` (0.0
    /// for `f32` and `f64`).
    pub adapting_luminance: T,

    /// The relative luminance of the nearby background (*Y<sub>b</sub>*), out
    /// to 10°, on a scale of 0 to 100. Defaults to `T::default()` (0.0 for
    /// `f32` and `f64`).
    pub background_luminance: T,

    /// A description of the peripheral area, with a value from `0` to `2`. Any
    /// value outside that range will be clamped to `0` or `2`. It has presets
    /// for "dark", "dim" and "average". Defaults to "average" (`2`).
    pub surround: Surround<T>,

    /// Set to `true` to assume that the observer's eyes have fully adapted to
    /// the illuminant. The degree of discounting will be set based on the other
    /// parameters. Defaults to `false`.
    pub discounting: bool,
}

impl<WpParam, T> Parameters<WpParam, T>
where
    WpParam: WhitePointParameter<T>,
{
    fn into_any_white_point(self) -> Parameters<Xyz<white_point::Any, T>, T> {
        Parameters {
            white_point: self.white_point.into_xyz(),
            adapting_luminance: self.adapting_luminance,
            background_luminance: self.background_luminance,
            surround: self.surround,
            discounting: self.discounting,
        }
    }
}

impl<Wp, T> Parameters<StaticWp<Wp>, T> {
    /// Creates a new set of parameters with a static white point and their
    /// default values set.
    ///
    /// These parameters need to be further customized according to the viewing
    /// conditions to be useful.
    ///
    /// This function helps the compiler infer the type parameters, which it may
    /// struggle with if `Parameters::default` is used. [`ParametersStaticWp`]
    /// can also help when specifying the white point.
    #[inline]
    pub fn default_static_wp() -> Self
    where
        Self: Default,
    {
        Self::default()
    }
}

impl<T> Parameters<Xyz<white_point::Any, T>, T> {
    /// Creates a new set of parameters with a dynamic white point and their default
    /// values set.
    ///
    /// These parameters need to be further customized according to the viewing
    /// conditions to be useful.
    ///
    /// This function helps the compiler infer the type parameters, which it may
    /// struggle with if `Parameters::default` is used.
    /// [`ParametersDynamicWp`] can also help when specifying the white point.
    #[inline]
    pub fn default_dynamic_wp() -> Self
    where
        Self: Default,
    {
        Self::default()
    }
}

#[cfg(test)]
impl<Wp> Parameters<StaticWp<Wp>, f64> {
    /// Only used in unit tests and corresponds to the defaults from https://observablehq.com/@jrus/cam16.
    pub(crate) const TEST_DEFAULTS: Self = Self {
        white_point: StaticWp(PhantomData),
        adapting_luminance: 40.0f64,
        background_luminance: 20.0f64,
        surround: Surround::Average,
        discounting: false,
    };
}

impl<Wp, T> Default for Parameters<StaticWp<Wp>, T>
where
    T: Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            white_point: StaticWp(PhantomData),
            adapting_luminance: T::default(),
            background_luminance: T::default(),
            surround: Surround::Average,
            discounting: false,
        }
    }
}

impl<T> Default for Parameters<Xyz<white_point::Any, T>, T>
where
    T: Real + Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            white_point: D65::get_xyz(),
            adapting_luminance: T::default(),
            background_luminance: T::default(),
            surround: Surround::Average,
            discounting: false,
        }
    }
}

/// Pre-calculated variables for CAM16, that only depend on the viewing
/// conditions.
///
/// Derived from [`Parameters`], the `BakedParameters` can help reducing the
/// amount of repeated work required for converting multiple colors.
pub struct BakedParameters<WpParam, T> {
    pub(super) inner: super::math::DependentParameters<T>,
    white_point: PhantomData<WpParam>,
}

impl<WpParam, T> Clone for BakedParameters<WpParam, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            white_point: PhantomData,
        }
    }
}

impl<WpParam, T> Copy for BakedParameters<WpParam, T> where T: Copy {}

impl<WpParam, T> From<Parameters<WpParam, T>> for BakedParameters<WpParam, T>
where
    WpParam: WhitePointParameter<T>,
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
        + Clone,
    T::Mask: LazySelect<T>,
{
    fn from(value: Parameters<WpParam, T>) -> Self {
        Self {
            inner: super::math::prepare_parameters(value.into_any_white_point()),
            white_point: PhantomData,
        }
    }
}

/// A description of the peripheral area.
#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum Surround<T> {
    /// Represents a dark room, such as a movie theatre. Corresponds to a
    /// surround value of `0`.
    Dark,

    /// Represents a dimly lit room with a bright TV or monitor. Corresponds to
    /// a surround value of `1`.
    Dim,

    /// Represents a surface color. Corresponds to a surround value of `2`.
    Average,

    /// Any custom value from `0` to `2`. Any value outside that range will be
    /// clamped to either `0` or `2`.
    Custom(T),
}

impl<T> Surround<T> {
    pub(crate) fn into_value(self) -> T
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

/// A trait for types that can be used as white point parameters in
/// [`Parameters`].
pub trait WhitePointParameter<T> {
    /// The static representation of this white point, or [`white_point::Any`]
    /// if it's dynamic.
    type StaticWp;

    /// Returns the XYZ value for this white point.
    fn into_xyz(self) -> Xyz<white_point::Any, T>;
}

impl<T> WhitePointParameter<T> for Xyz<white_point::Any, T> {
    type StaticWp = white_point::Any;

    fn into_xyz(self) -> Xyz<white_point::Any, T> {
        self
    }
}

/// Represents a static white point in [`Parameters`], as opposed to a dynamic
/// [`Xyz`] value.
pub struct StaticWp<Wp>(PhantomData<Wp>);

impl<T, Wp> WhitePointParameter<T> for StaticWp<Wp>
where
    Wp: WhitePoint<T>,
{
    type StaticWp = Wp;

    fn into_xyz(self) -> Xyz<white_point::Any, T> {
        Wp::get_xyz()
    }
}

impl<Wp> Clone for StaticWp<Wp> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Wp> Copy for StaticWp<Wp> {}
