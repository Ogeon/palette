//! Traits for converting between color spaces.
//!
//! # Deriving
//!
//! `FromColorUnclamped` can be derived in a mostly automatic way.
//! The default minimum requirement is to implement `FromColorUnclamped<Xyz>`, but it can
//! also be customized to make use of generics and have other manual implementations.
//!
//! It is also recommended to derive or implement [`WithAlpha`](crate::WithAlpha),
//! to be able to convert between all `Alpha` wrapped color types.
//!
//! ## Configuration Attributes
//!
//! The derives can be configured using one or more `#[palette(...)]` attributes.
//! They can be attached to either the item itself, or to the fields.
//!
//! ```
//! # use palette::rgb::{RgbStandard, RgbSpace};
//! # use palette::convert::FromColorUnclamped;
//! # use palette::{Xyz, Component, FloatComponent};
//! #
//! #[palette(
//!     component = "T",
//!     rgb_standard = "S",
//!     white_point = "<S::Space as RgbSpace>::WhitePoint",
//! )]
//! #[derive(FromColorUnclamped)]
//! #[repr(C)]
//! struct ExampleType<S: RgbStandard, T: Component> {
//!     // ...
//!     #[palette(alpha)]
//!     alpha: T,
//!     standard: std::marker::PhantomData<S>,
//! }
//!
//! # impl<S, T> FromColorUnclamped<Xyz<<S::Space as RgbSpace>::WhitePoint, T>> for ExampleType<S, T>
//! # where
//! #   S: RgbStandard,
//! #   T: FloatComponent
//! # {
//! #   fn from_color_unclamped(color: Xyz<<S::Space as RgbSpace>::WhitePoint, T>) -> Self {
//! #       ExampleType {alpha: T::max_intensity(), standard: std::marker::PhantomData}
//! #   }
//! # }
//! #
//! # impl<S, T> FromColorUnclamped<ExampleType<S, T>> for Xyz<<S::Space as RgbSpace>::WhitePoint, T>
//! # where
//! #   S: RgbStandard,
//! #   T: FloatComponent
//! # {
//! #   fn from_color_unclamped(color: ExampleType<S, T>) -> Self {
//! #       Xyz::default()
//! #   }
//! # }
//! ```
//!
//! ### Item Attributes
//!
//! * `skip_derives(Luma, Rgb)`: No conversion derives will be implemented for these colors.
//! They are instead to be implemented manually, and serve as the basis for the automatic
//! implementations.
//!
//! * `white_point = "some::white_point::Type"`: Sets the white
//! point type that should be used when deriving. The default is `D65`, but it
//! may be any other type, including type parameters.
//!
//! * `component = "some::component::Type"`: Sets the color
//! component type that should be used when deriving. The default is `f32`, but
//! it may be any other type, including type parameters.
//!
//! * `rgb_standard = "some::rgb_standard::Type"`: Sets the RGB standard
//! type that should be used when deriving. The default is to either use `Srgb`
//! or a best effort to convert between standards, but sometimes it has to be set
//! to a specific type. This also accepts type parameters.
//!
//! ## Field Attributes
//!
//! * `alpha`: Specifies field as the color's transparency value.
//!
//! ## Examples
//!
//! Minimum requirements implementation:
//!
//! ```rust
//! use palette::convert::FromColorUnclamped;
//! use palette::{Srgb, Xyz, IntoColor};
//!
//! /// A custom version of Xyz that stores integer values from 0 to 100.
//! #[derive(PartialEq, Debug, FromColorUnclamped)]
//! struct Xyz100 {
//!     x: u8,
//!     y: u8,
//!     z: u8,
//! }
//!
//! // We have to implement at least one "manual" conversion. The default
//! // is to and from `Xyz`, but it can be customized with `skip_derives(...)`.
//! impl FromColorUnclamped<Xyz> for Xyz100 {
//!     fn from_color_unclamped(color: Xyz) -> Xyz100 {
//!         Xyz100 {
//!             x: (color.x * 100.0) as u8,
//!             y: (color.y * 100.0) as u8,
//!             z: (color.z * 100.0) as u8,
//!         }
//!     }
//! }
//!
//! impl FromColorUnclamped<Xyz100> for Xyz {
//!     fn from_color_unclamped(color: Xyz100) -> Xyz {
//!         Xyz::new(
//!             color.x as f32 / 100.0,
//!             color.y as f32 / 100.0,
//!             color.z as f32 / 100.0,
//!         )
//!     }
//! }
//!
//! fn main() {
//!     // Start with an Xyz100 color.
//!     let xyz = Xyz100 {
//!         x: 59,
//!         y: 75,
//!         z: 42,
//!     };
//!
//!     // Convert the color to sRGB.
//!     let rgb: Srgb = xyz.into_color();
//!
//!     assert_eq!(rgb.into_format(), Srgb::new(196u8, 238, 154));
//! }
//! ```
//!
//! With generic components:
//!
//! ```rust
//! #[macro_use]
//! extern crate approx;
//!
//! use palette::rgb::{Rgb, RgbSpace, RgbStandard};
//! use palette::encoding::Linear;
//! use palette::white_point::D65;
//! use palette::convert::{FromColorUnclamped, IntoColorUnclamped};
//! use palette::{FloatComponent, Hsv, Pixel, Srgb, IntoColor};
//!
//! /// sRGB, but with a reversed memory layout.
//! #[palette(
//!     skip_derives(Rgb),
//!     component = "T",
//!     rgb_standard = "palette::encoding::Srgb"
//! )]
//! #[derive(Copy, Clone, Pixel, FromColorUnclamped)]
//! #[repr(C)] // Makes sure the memory layout is as we want it.
//! struct Bgr<T> {
//!     blue: T,
//!     green: T,
//!     red: T,
//! }
//!
//! // It converts from and into any linear Rgb type that has the
//! // D65 white point, which is the default if we don't specify
//! // anything else with the `white_point` attribute argument.
//! impl<S, T> FromColorUnclamped<Bgr<T>> for Rgb<S, T>
//! where
//!     T: FloatComponent,
//!     S: RgbStandard,
//!     S::Space: RgbSpace<WhitePoint = D65>
//! {
//!     fn from_color_unclamped(color: Bgr<T>) -> Rgb<S, T> {
//!         Srgb::new(color.red, color.green, color.blue)
//!             .into_color_unclamped()
//!     }
//! }
//!
//! impl<S, T> FromColorUnclamped<Rgb<S, T>> for Bgr<T>
//! where
//!     T: FloatComponent,
//!     S: RgbStandard,
//!     S::Space: RgbSpace<WhitePoint = D65>
//! {
//!     fn from_color_unclamped(color: Rgb<S, T>) -> Bgr<T> {
//!         let color = Srgb::from_color_unclamped(color);
//!         Bgr {
//!             blue: color.blue,
//!             green: color.green,
//!             red: color.red,
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let buffer = vec![
//!         0.0f64,
//!         0.0,
//!         0.0,
//!         0.0,
//!         0.5,
//!         0.25,
//!     ];
//!     let hsv: Hsv<_, f64> = Bgr::from_raw_slice(&buffer)[1].into_color();
//!
//!     assert_relative_eq!(hsv, Hsv::new(90.0, 1.0, 0.5));
//! }
//! ```
//!
//! With alpha component:
//!
//! ```rust
//! #[macro_use]
//! extern crate approx;
//!
//! use palette::{LinSrgba, Srgb, IntoColor, WithAlpha};
//! use palette::rgb::{Rgb, RgbSpace, RgbStandard};
//! use palette::encoding::Linear;
//! use palette::white_point::D65;
//! use palette::convert::{FromColorUnclamped, IntoColorUnclamped};
//!
//! /// CSS style sRGB.
//! #[palette(
//!     skip_derives(Rgb),
//!     rgb_standard = "palette::encoding::Srgb"
//! )]
//! #[derive(PartialEq, Debug, FromColorUnclamped, WithAlpha)]
//! struct CssRgb {
//!     red: u8,
//!     green: u8,
//!     blue: u8,
//!     #[palette(alpha)]
//!     alpha: f32,
//! }
//!
//! // We will write a conversion function for opaque RGB and
//! // `impl_default_conversions` will take care of preserving
//! // the transparency for us.
//! impl<S> FromColorUnclamped<Rgb<S, f32>> for CssRgb
//! where
//!     S: RgbStandard,
//!     S::Space: RgbSpace<WhitePoint = D65>,
//! {
//!     fn from_color_unclamped(color: Rgb<S, f32>) -> CssRgb{
//!         let srgb = Srgb::from_color_unclamped(color)
//!             .into_format();
//!
//!         CssRgb {
//!             red: srgb.red,
//!             green: srgb.green,
//!             blue: srgb.blue,
//!             alpha: 1.0
//!         }
//!     }
//! }
//!
//! impl<S> FromColorUnclamped<CssRgb> for Rgb<S, f32>
//! where
//!     S: RgbStandard,
//!     S::Space: RgbSpace<WhitePoint = D65>,
//! {
//!     fn from_color_unclamped(color: CssRgb) -> Rgb<S, f32>{
//!         Srgb::new(color.red, color.green, color.blue)
//!             .into_format()
//!             .into_color_unclamped()
//!     }
//! }
//!
//! fn main() {
//!     let css_color = CssRgb {
//!         red: 187,
//!         green: 0,
//!         blue: 255,
//!         alpha: 0.3,
//!     };
//!     let color: LinSrgba = css_color.into_color();
//!
//!     assert_relative_eq!(color, LinSrgba::new(0.496933, 0.0, 1.0, 0.3));
//! }
//! ```

use core::fmt::{self, Display, Formatter};

#[doc(hidden)]
pub use palette_derive::FromColorUnclamped;

use crate::Clamp;

/// The error type for a color conversion that converted a color into a color
/// with invalid values.
#[derive(Debug)]
pub struct OutOfBounds<T> {
    color: T,
}

impl<T> OutOfBounds<T> {
    /// Create a new error wrapping a color
    #[inline]
    fn new(color: T) -> Self {
        OutOfBounds { color }
    }

    /// Consume this error and return the wrapped color
    #[inline]
    pub fn color(self) -> T {
        self.color
    }
}

#[cfg(feature = "std")]
impl<T: ::std::fmt::Debug> ::std::error::Error for OutOfBounds<T> {
    fn description(&self) -> &str {
        "color conversion is out of bounds"
    }
}

impl<T> Display for OutOfBounds<T> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "color conversion is out of bounds")
    }
}

/// A trait for converting a color into another, in a possibly lossy way.
///
/// `U: IntoColor<T>` is implemented for every type `T: FromColor<U>`.
///
/// See [`FromColor`](crate::convert::FromColor) for more details.
pub trait IntoColor<T>: Sized {
    /// Convert into T with values clamped to the color defined bounds
    ///
    /// ```
    /// use palette::{Clamp, IntoColor, Lch, Srgb};
    ///
    /// let rgb: Srgb = Lch::new(50.0, 100.0, -175.0).into_color();
    /// assert!(rgb.is_within_bounds());
    /// ```
    fn into_color(self) -> T;
}

/// A trait for unchecked conversion of a color into another.
///
/// `U: IntoColorUnclamped<T>` is implemented for every type `T: FromColorUnclamped<U>`.
///
/// See [`FromColorUnclamped`](crate::convert::FromColorUnclamped) for more details.
pub trait IntoColorUnclamped<T>: Sized {
    /// Convert into T. The resulting color might be invalid in its color space
    ///
    /// ```
    /// use palette::convert::IntoColorUnclamped;
    /// use palette::{Clamp, Lch, Srgb};
    ///
    ///let rgb: Srgb = Lch::new(50.0, 100.0, -175.0).into_color_unclamped();
    ///assert!(!rgb.is_within_bounds());
    ///```
    fn into_color_unclamped(self) -> T;
}

/// A trait for fallible conversion of a color into another.
///
/// `U: TryIntoColor<T>` is implemented for every type `T: TryFromColor<U>`.
///
/// See [`TryFromColor`](crate::convert::TryFromColor) for more details.
pub trait TryIntoColor<T>: Sized {
    /// Convert into T, returning ok if the color is inside of its defined
    /// range, otherwise an `OutOfBounds` error is returned which contains
    /// the unclamped color.
    ///
    ///```
    /// use palette::convert::TryIntoColor;
    /// use palette::{Hsl, Srgb};
    ///
    /// let rgb: Srgb = match Hsl::new(150.0, 1.0, 1.1).try_into_color() {
    ///     Ok(color) => color,
    ///     Err(err) => {
    ///         println!("Color is out of bounds");
    ///         err.color()
    ///     }
    /// };
    /// ```
    fn try_into_color(self) -> Result<T, OutOfBounds<T>>;
}

///A trait for converting one color from another, in a possibly lossy way.
///
/// `U: FromColor<T>` is implemented for every type `U: FromColorUnclamped<T> + Clamp`.
///
/// See [`FromColorUnclamped`](crate::convert::FromColorUnclamped) for a lossless version of this trait.
/// See [`TryFromColor`](crate::convert::TryFromColor) for a trait that gives an error when the result
/// is out of bounds.
///
/// # The Difference Between FromColor and From
///
/// The conversion traits, including `FromColor`, were added to gain even more flexibility
/// than what `From` and the other standard library traits can give. There are a few subtle,
/// but important, differences in their semantics:
///
/// * `FromColor` and `IntoColor` are allowed to be lossy, meaning converting `A -> B -> A`
/// may result in a different value than the original. This applies to `A -> A` as well.
/// * `From<Self>` and `Into<Self>` are blanket implemented, while `FromColor<Self>` and
/// `IntoColor<Self>` have to be manually implemented. This allows additional flexibility,
/// such as allowing implementing `FromColor<Rgb<S2, T>> for Rgb<S1, T>`.
/// * Implementing `FromColorUnclamped` and `Clamp` is enough to get all the other conversion
/// traits, while `From` and `Into` would not be possible to blanket implement in the same way.
/// This also reduces the work that needs to be done by macros.
///
/// See the [`convert`](crate::convert) module for how to implement `FromColorUnclamped` for
/// custom colors.
pub trait FromColor<T>: Sized {
    /// Convert from T with values clamped to the color defined bounds.
    ///
    /// ```
    /// use palette::{Clamp, FromColor, Lch, Srgb};
    ///
    /// let rgb = Srgb::from_color(Lch::new(50.0, 100.0, -175.0));
    /// assert!(rgb.is_within_bounds());
    /// ```
    fn from_color(t: T) -> Self;
}

/// A trait for unchecked conversion of one color from another.
///
/// See [`FromColor`](crate::convert::FromColor) for a lossy version of this trait.
/// See [`TryFromColor`](crate::convert::TryFromColor) for a trait that gives an error when the result
/// is out of bounds.
///
/// See the [`convert`](crate::convert) module for how to implement `FromColorUnclamped` for
/// custom colors.
pub trait FromColorUnclamped<T>: Sized {
    /// Convert from T. The resulting color might be invalid in its color space.
    ///
    /// ```
    /// use palette::convert::FromColorUnclamped;
    /// use palette::{Clamp, Lch, Srgb};
    ///
    /// let rgb = Srgb::from_color_unclamped(Lch::new(50.0, 100.0, -175.0));
    /// assert!(!rgb.is_within_bounds());
    /// ```
    fn from_color_unclamped(val: T) -> Self;
}

/// A trait for fallible conversion of one color from another.
///
/// `U: TryFromColor<T>` is implemented for every type `U: FromColorUnclamped<T> + Clamp`.
///
/// See [`FromColor`](crate::convert::FromColor) for a lossy version of this trait.
/// See [`FromColorUnclamped`](crate::convert::FromColorUnclamped) for a lossless version.
///
/// See the [`convert`](crate::convert) module for how to implement `FromColorUnclamped` for
/// custom colors.
pub trait TryFromColor<T>: Sized {
    /// Convert from T, returning ok if the color is inside of its defined
    /// range, otherwise an `OutOfBounds` error is returned which contains
    /// the unclamped color.
    ///
    ///```
    /// use palette::convert::TryFromColor;
    /// use palette::{Hsl, Srgb};
    ///
    /// let rgb = match Srgb::try_from_color(Hsl::new(150.0, 1.0, 1.1)) {
    ///     Ok(color) => color,
    ///     Err(err) => {
    ///         println!("Color is out of bounds");
    ///         err.color()
    ///     }
    /// };
    /// ```
    fn try_from_color(t: T) -> Result<Self, OutOfBounds<Self>>;
}

impl<T, U> FromColor<T> for U
where
    U: FromColorUnclamped<T> + Clamp,
{
    #[inline]
    fn from_color(t: T) -> Self {
        let mut this = Self::from_color_unclamped(t);
        if !this.is_within_bounds() {
            this.clamp_self();
        }
        this
    }
}

impl<T, U> TryFromColor<T> for U
where
    U: FromColorUnclamped<T> + Clamp,
{
    #[inline]
    fn try_from_color(t: T) -> Result<Self, OutOfBounds<Self>> {
        let this = Self::from_color_unclamped(t);
        if this.is_within_bounds() {
            Ok(this)
        } else {
            Err(OutOfBounds::new(this))
        }
    }
}

impl<T, U> IntoColor<U> for T
where
    U: FromColor<T>,
{
    #[inline]
    fn into_color(self) -> U {
        U::from_color(self)
    }
}

impl<T, U> IntoColorUnclamped<U> for T
where
    U: FromColorUnclamped<T>,
{
    #[inline]
    fn into_color_unclamped(self) -> U {
        U::from_color_unclamped(self)
    }
}

impl<T, U> TryIntoColor<U> for T
where
    U: TryFromColor<T>,
{
    #[inline]
    fn try_into_color(self) -> Result<U, OutOfBounds<U>> {
        U::try_from_color(self)
    }
}

#[cfg(test)]
mod tests {
    use core::marker::PhantomData;

    use super::{FromColor, FromColorUnclamped, IntoColor};
    use crate::encoding::linear::Linear;
    use crate::luma::{Luma, LumaStandard};
    use crate::rgb::{Rgb, RgbSpace};
    use crate::{Alpha, Hsl, Hsluv, Hsv, Hwb, Lab, Lch, Luv, Xyz, Yxy};
    use crate::{Clamp, FloatComponent};

    #[derive(FromColorUnclamped, WithAlpha)]
    #[palette(
        skip_derives(Xyz, Luma),
        white_point = "S::WhitePoint",
        component = "f64",
        rgb_standard = "Linear<S>",
        palette_internal,
        palette_internal_not_base_type
    )]
    struct WithXyz<S: RgbSpace>(PhantomData<S>);

    impl<S: RgbSpace> Clone for WithXyz<S> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<S: RgbSpace> Copy for WithXyz<S> {}

    impl<S: RgbSpace> Clamp for WithXyz<S> {
        fn is_within_bounds(&self) -> bool {
            true
        }

        fn clamp(&self) -> Self {
            *self
        }

        fn clamp_self(&mut self) {}
    }

    impl<S1, S2> FromColorUnclamped<WithXyz<S2>> for WithXyz<S1>
    where
        S1: RgbSpace,
        S2: RgbSpace<WhitePoint = S1::WhitePoint>,
    {
        fn from_color_unclamped(_color: WithXyz<S2>) -> Self {
            WithXyz(PhantomData)
        }
    }

    impl<S: RgbSpace> FromColorUnclamped<Xyz<S::WhitePoint, f64>> for WithXyz<S> {
        fn from_color_unclamped(_color: Xyz<S::WhitePoint, f64>) -> Self {
            WithXyz(PhantomData)
        }
    }

    impl<S: RgbSpace> FromColorUnclamped<WithXyz<S>> for Xyz<S::WhitePoint, f64> {
        fn from_color_unclamped(_color: WithXyz<S>) -> Xyz<S::WhitePoint, f64> {
            Xyz::with_wp(0.0, 1.0, 0.0)
        }
    }

    impl<Rs: RgbSpace, Ls: LumaStandard<WhitePoint = Rs::WhitePoint>>
        FromColorUnclamped<Luma<Ls, f64>> for WithXyz<Rs>
    {
        fn from_color_unclamped(_color: Luma<Ls, f64>) -> Self {
            WithXyz(PhantomData)
        }
    }

    impl<Rs: RgbSpace, Ls: LumaStandard<WhitePoint = Rs::WhitePoint>>
        FromColorUnclamped<WithXyz<Rs>> for Luma<Ls, f64>
    {
        fn from_color_unclamped(_color: WithXyz<Rs>) -> Self {
            Luma::new(1.0)
        }
    }

    #[derive(Copy, Clone, FromColorUnclamped, WithAlpha)]
    #[palette(
        skip_derives(Lch, Luma),
        white_point = "crate::white_point::E",
        component = "T",
        rgb_standard = "Linear<(crate::encoding::Srgb, crate::white_point::E)>",
        palette_internal,
        palette_internal_not_base_type
    )]
    struct WithoutXyz<T: FloatComponent>(PhantomData<T>);

    impl<T: FloatComponent> Clamp for WithoutXyz<T> {
        fn is_within_bounds(&self) -> bool {
            true
        }

        fn clamp(&self) -> Self {
            *self
        }

        fn clamp_self(&mut self) {}
    }

    impl<T: FloatComponent> FromColorUnclamped<WithoutXyz<T>> for WithoutXyz<T> {
        fn from_color_unclamped(color: WithoutXyz<T>) -> Self {
            color
        }
    }

    impl<T: FloatComponent> FromColorUnclamped<Lch<crate::white_point::E, T>> for WithoutXyz<T> {
        fn from_color_unclamped(_color: Lch<crate::white_point::E, T>) -> Self {
            WithoutXyz(PhantomData)
        }
    }

    impl<T: FloatComponent> FromColorUnclamped<WithoutXyz<T>> for Lch<crate::white_point::E, T> {
        fn from_color_unclamped(_color: WithoutXyz<T>) -> Lch<crate::white_point::E, T> {
            Lch::with_wp(T::one(), T::zero(), T::zero())
        }
    }

    impl<T: FloatComponent> FromColorUnclamped<Luma<Linear<crate::white_point::E>, T>>
        for WithoutXyz<T>
    {
        fn from_color_unclamped(_color: Luma<Linear<crate::white_point::E>, T>) -> Self {
            WithoutXyz(PhantomData)
        }
    }

    impl<T: FloatComponent> FromColorUnclamped<WithoutXyz<T>>
        for Luma<Linear<crate::white_point::E>, T>
    {
        fn from_color_unclamped(_color: WithoutXyz<T>) -> Luma<Linear<crate::white_point::E>, T> {
            Luma::new(T::one())
        }
    }

    #[test]
    fn from_with_xyz() {
        let color: WithXyz<crate::encoding::Srgb> = WithXyz(Default::default());
        WithXyz::<crate::encoding::Srgb>::from_color(color);

        let xyz: Xyz<_, f64> = Default::default();
        WithXyz::<crate::encoding::Srgb>::from_color(xyz);

        let yxy: Yxy<_, f64> = Default::default();
        WithXyz::<crate::encoding::Srgb>::from_color(yxy);

        let lab: Lab<_, f64> = Default::default();
        WithXyz::<crate::encoding::Srgb>::from_color(lab);

        let lch: Lch<_, f64> = Default::default();
        WithXyz::<crate::encoding::Srgb>::from_color(lch);

        let luv: Hsl<_, f64> = Default::default();
        WithXyz::<crate::encoding::Srgb>::from_color(luv);

        let rgb: Rgb<_, f64> = Default::default();
        WithXyz::<crate::encoding::Srgb>::from_color(rgb);

        let hsl: Hsl<_, f64> = Default::default();
        WithXyz::<crate::encoding::Srgb>::from_color(hsl);

        let hsluv: Hsluv<_, f64> = Default::default();
        WithXyz::<crate::encoding::Srgb>::from_color(hsluv);

        let hsv: Hsv<_, f64> = Default::default();
        WithXyz::<crate::encoding::Srgb>::from_color(hsv);

        let hwb: Hwb<_, f64> = Default::default();
        WithXyz::<crate::encoding::Srgb>::from_color(hwb);

        let luma: Luma<crate::encoding::Srgb, f64> = Default::default();
        WithXyz::<crate::encoding::Srgb>::from_color(luma);
    }

    #[test]
    fn from_with_xyz_alpha() {
        let color: Alpha<WithXyz<crate::encoding::Srgb>, u8> =
            Alpha::from(WithXyz(Default::default()));
        WithXyz::<crate::encoding::Srgb>::from_color(color);

        let xyz: Alpha<Xyz<_, f64>, u8> = Alpha::from(Xyz::default());
        WithXyz::<crate::encoding::Srgb>::from_color(xyz);

        let yxy: Alpha<Yxy<_, f64>, u8> = Alpha::from(Yxy::default());
        WithXyz::<crate::encoding::Srgb>::from_color(yxy);

        let lab: Alpha<Lab<_, f64>, u8> = Alpha::from(Lab::default());
        WithXyz::<crate::encoding::Srgb>::from_color(lab);

        let lch: Alpha<Lch<_, f64>, u8> = Alpha::from(Lch::default());
        WithXyz::<crate::encoding::Srgb>::from_color(lch);

        let luv: Alpha<Luv<_, f64>, u8> = Alpha::from(Luv::default());
        WithXyz::<crate::encoding::Srgb>::from_color(luv);

        let rgb: Alpha<Rgb<_, f64>, u8> = Alpha::from(Rgb::default());
        WithXyz::<crate::encoding::Srgb>::from_color(rgb);

        let hsl: Alpha<Hsl<_, f64>, u8> = Alpha::from(Hsl::default());
        WithXyz::<crate::encoding::Srgb>::from_color(hsl);

        let hsluv: Alpha<Hsluv<_, f64>, u8> = Alpha::from(Hsluv::default());
        WithXyz::<crate::encoding::Srgb>::from_color(hsluv);

        let hsv: Alpha<Hsv<_, f64>, u8> = Alpha::from(Hsv::default());
        WithXyz::<crate::encoding::Srgb>::from_color(hsv);

        let hwb: Alpha<Hwb<_, f64>, u8> = Alpha::from(Hwb::default());
        WithXyz::<crate::encoding::Srgb>::from_color(hwb);

        let luma: Alpha<Luma<crate::encoding::Srgb, f64>, u8> =
            Alpha::from(Luma::<crate::encoding::Srgb, f64>::default());
        WithXyz::<crate::encoding::Srgb>::from_color(luma);
    }

    #[test]
    fn from_with_xyz_into_alpha() {
        let color: WithXyz<crate::encoding::Srgb> = WithXyz(Default::default());
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(color);

        let xyz: Xyz<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(xyz);

        let yxy: Yxy<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(yxy);

        let lab: Lab<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(lab);

        let lch: Lch<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(lch);

        let luv: Hsl<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(luv);

        let rgb: Rgb<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(rgb);

        let hsl: Hsl<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(hsl);

        let hsluv: Hsluv<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(hsluv);

        let hsv: Hsv<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(hsv);

        let hwb: Hwb<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(hwb);

        let luma: Luma<crate::encoding::Srgb, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(luma);
    }

    #[test]
    fn from_with_xyz_alpha_into_alpha() {
        let color: Alpha<WithXyz<crate::encoding::Srgb>, u8> =
            Alpha::from(WithXyz(Default::default()));
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(color);

        let xyz: Xyz<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(xyz);

        let yxy: Yxy<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(yxy);

        let lab: Lab<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(lab);

        let lch: Lch<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(lch);

        let luv: Luv<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(luv);

        let rgb: Rgb<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(rgb);

        let hsl: Hsl<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(hsl);

        let hsluv: Hsluv<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(hsluv);

        let hsv: Hsv<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(hsv);

        let hwb: Hwb<_, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(hwb);

        let luma: Luma<crate::encoding::Srgb, f64> = Default::default();
        Alpha::<WithXyz<crate::encoding::Srgb>, u8>::from_color(luma);
    }

    #[test]
    fn into_from_with_xyz() {
        let color = WithXyz::<crate::encoding::Srgb>(PhantomData);

        let _self: WithXyz<crate::encoding::Srgb> = color.into_color();
        let _xyz: Xyz<_, f64> = color.into_color();
        let _yxy: Yxy<_, f64> = color.into_color();
        let _lab: Lab<_, f64> = color.into_color();
        let _lch: Lch<_, f64> = color.into_color();
        let _luv: Luv<_, f64> = color.into_color();
        let _rgb: Rgb<_, f64> = color.into_color();
        let _hsl: Hsl<_, f64> = color.into_color();
        let _hsluv: Hsluv<_, f64> = color.into_color();
        let _hsv: Hsv<_, f64> = color.into_color();
        let _hwb: Hwb<_, f64> = color.into_color();
        let _luma: Luma<crate::encoding::Srgb, f64> = color.into_color();
    }

    #[test]
    fn into_from_with_xyz_alpha() {
        let color: Alpha<WithXyz<crate::encoding::Srgb>, u8> =
            Alpha::from(WithXyz::<crate::encoding::Srgb>(PhantomData));

        let _self: WithXyz<crate::encoding::Srgb> = color.into_color();
        let _xyz: Xyz<_, f64> = color.into_color();
        let _yxy: Yxy<_, f64> = color.into_color();
        let _lab: Lab<_, f64> = color.into_color();
        let _lch: Lch<_, f64> = color.into_color();
        let _luv: Luv<_, f64> = color.into_color();
        let _rgb: Rgb<_, f64> = color.into_color();
        let _hsl: Hsl<_, f64> = color.into_color();
        let _hsluv: Hsluv<_, f64> = color.into_color();
        let _hsv: Hsv<_, f64> = color.into_color();
        let _hwb: Hwb<_, f64> = color.into_color();
        let _luma: Luma<crate::encoding::Srgb, f64> = color.into_color();
    }

    #[test]
    fn into_alpha_from_with_xyz() {
        let color = WithXyz::<crate::encoding::Srgb>(PhantomData);

        let _self: Alpha<WithXyz<crate::encoding::Srgb>, u8> = color.into_color();
        let _xyz: Alpha<Xyz<_, f64>, u8> = color.into_color();
        let _yxy: Alpha<Yxy<_, f64>, u8> = color.into_color();
        let _lab: Alpha<Lab<_, f64>, u8> = color.into_color();
        let _lch: Alpha<Lch<_, f64>, u8> = color.into_color();
        let _luv: Alpha<Luv<_, f64>, u8> = color.into_color();
        let _rgb: Alpha<Rgb<_, f64>, u8> = color.into_color();
        let _hsl: Alpha<Hsl<_, f64>, u8> = color.into_color();
        let _hsluv: Alpha<Hsluv<_, f64>, u8> = color.into_color();
        let _hsv: Alpha<Hsv<_, f64>, u8> = color.into_color();
        let _hwb: Alpha<Hwb<_, f64>, u8> = color.into_color();
        let _luma: Alpha<Luma<crate::encoding::Srgb, f64>, u8> = color.into_color();
    }

    #[test]
    fn into_alpha_from_with_xyz_alpha() {
        let color: Alpha<WithXyz<crate::encoding::Srgb>, u8> =
            Alpha::from(WithXyz::<crate::encoding::Srgb>(PhantomData));

        let _self: Alpha<WithXyz<crate::encoding::Srgb>, u8> = color.into_color();
        let _xyz: Alpha<Xyz<_, f64>, u8> = color.into_color();
        let _yxy: Alpha<Yxy<_, f64>, u8> = color.into_color();
        let _lab: Alpha<Lab<_, f64>, u8> = color.into_color();
        let _lch: Alpha<Lch<_, f64>, u8> = color.into_color();
        let _luv: Alpha<Luv<_, f64>, u8> = color.into_color();
        let _rgb: Alpha<Rgb<_, f64>, u8> = color.into_color();
        let _hsl: Alpha<Hsl<_, f64>, u8> = color.into_color();
        let _hsluv: Alpha<Hsluv<_, f64>, u8> = color.into_color();
        let _hsv: Alpha<Hsv<_, f64>, u8> = color.into_color();
        let _hwb: Alpha<Hwb<_, f64>, u8> = color.into_color();
        let _luma: Alpha<Luma<crate::encoding::Srgb, f64>, u8> = color.into_color();
    }

    #[test]
    fn from_without_xyz() {
        let color: WithoutXyz<f64> = WithoutXyz(Default::default());
        WithoutXyz::<f64>::from_color(color);

        let xyz: Xyz<crate::white_point::E, f64> = Default::default();
        WithoutXyz::<f64>::from_color(xyz);

        let yxy: Yxy<crate::white_point::E, f64> = Default::default();
        WithoutXyz::<f64>::from_color(yxy);

        let lab: Lab<crate::white_point::E, f64> = Default::default();
        WithoutXyz::<f64>::from_color(lab);

        let lch: Lch<crate::white_point::E, f64> = Default::default();
        WithoutXyz::<f64>::from_color(lch);

        let luv: Luv<crate::white_point::E, f64> = Default::default();
        WithoutXyz::<f64>::from_color(luv);

        let rgb: Rgb<_, f64> = Default::default();
        WithoutXyz::<f64>::from_color(rgb);

        let hsl: Hsl<_, f64> = Default::default();
        WithoutXyz::<f64>::from_color(hsl);

        let hsluv: Hsluv<_, f64> = Default::default();
        WithoutXyz::<f64>::from_color(hsluv);

        let hsv: Hsv<_, f64> = Default::default();
        WithoutXyz::<f64>::from_color(hsv);

        let hwb: Hwb<_, f64> = Default::default();
        WithoutXyz::<f64>::from_color(hwb);

        let luma: Luma<Linear<crate::white_point::E>, f64> = Default::default();
        WithoutXyz::<f64>::from_color(luma);
    }

    #[test]
    fn into_without_xyz() {
        let color = WithoutXyz::<f64>(PhantomData);

        let _self: WithoutXyz<f64> = color.into_color();
        let _xyz: Xyz<crate::white_point::E, f64> = color.into_color();
        let _yxy: Yxy<crate::white_point::E, f64> = color.into_color();
        let _lab: Lab<crate::white_point::E, f64> = color.into_color();
        let _lch: Lch<crate::white_point::E, f64> = color.into_color();
        let _luv: Luv<crate::white_point::E, f64> = color.into_color();
        let _rgb: Rgb<_, f64> = color.into_color();
        let _hsl: Hsl<_, f64> = color.into_color();
        let _hsluv: Hsluv<_, f64> = color.into_color();
        let _hsv: Hsv<_, f64> = color.into_color();
        let _hwb: Hwb<_, f64> = color.into_color();
        let _luma: Luma<Linear<crate::white_point::E>, f64> = color.into_color();
    }
}
