//! Traits for converting between color spaces.
//!
//! Each color space type, such as [`Rgb`](crate::rgb::Rgb) and
//! [`Hsl`](crate::Hsl), implement a number of conversion traits:
//!
//! * [`FromColor`] - Similar to [`From`], converts from another color space.
//! * [`IntoColor`] - Similar to [`Into`], converts into another color space.
//! * [`FromColorUnclamped`] - The same as [`FromColor`], but the resulting
//!   values may be outside the typical bounds.
//! * [`IntoColorUnclamped`] - The same as [`IntoColor`], but the resulting
//!   values may be outside the typical bounds.
//!
//! ```
//! use palette::{FromColor, IntoColor, Srgb, Hsl};
//!
//! let rgb = Srgb::new(0.3f32, 0.8, 0.1);
//!
//! let hsl1: Hsl = rgb.into_color();
//! let hsl2 = Hsl::from_color(rgb);
//! ```
//!
//! Most of the color space types can be converted directly to each other, with
//! these traits. If you look at the implemented traits for any color type, you
//! will see a substantial list of `FromColorUnclamped` implementations. There
//! are, however, exceptions and restrictions in some cases:
//!
//! * **It's not always possible to change the component type while
//!   converting.** This can only be enabled in specific cases, to allow type
//!   inference to work. The input and output component types need to be the
//!   same in the general case.
//! * **It's not always possible to change meta types while converting.** Meta
//!   types are the additional input types on colors, such as white point or RGB
//!   standard. Similar to component types, these are generally restricted to
//!   help type inference.
//! * **Some color spaces want specific component types.** For example,
//!   [`Xyz`](crate::Xyz) and many other color spaces require real-ish numbers
//!   (`f32`, `f64`, etc.).
//! * **Some color spaces want specific meta types.** For example,
//!   [`Oklab`](crate::Oklab) requires the white point to be
//!   [`D65`](crate::white_point::D65).
//!
//! These limitations are usually the reason for why the compiler gives an error
//! when calling `into_color`, `from_color`, or the corresponding unclamped
//! methods. They are possible to work around by splitting the conversion into
//! multiple steps.
//!
//! # In-place Conversion
//!
//! It's possible for some color spaces to be converted in-place, meaning the
//! destination color will use the memory space of the source color. The
//! requirement for this is that the source and destination color types have the
//! same memory layout. That is, the same component types and the same number of
//! components. This is verified by the [`ArrayCast`](crate::cast::ArrayCast)
//! trait.
//!
//! In-place conversion is done with the [`FromColorMut`] and [`IntoColorMut`]
//! traits, as well as their unclamped counterparts, [`FromColorUnclampedMut`]
//! and [`IntoColorUnclampedMut`]. They work for both single colors and slices
//! of colors.
//!
//! ```
//! use palette::{convert::FromColorMut, Srgb, Hsl, Hwb};
//!
//! let mut rgb_colors: Vec<Srgb<f32>> = vec![/* ... */];
//!
//! {
//!     // Creates a scope guard that prevents `rgb_colors` from being modified as RGB.
//!     let hsl_colors = <[Hsl]>::from_color_mut(&mut rgb_colors);
//!
//!     // The converted colors can be converted again, without keeping the previous guard around.
//!     let hwb_colors = hsl_colors.then_into_color_mut::<[Hwb]>();
//!
//!     // The colors are automatically converted back to RGB at the end of the scope.
//!     // The use of `then_into_color_mut` above makes this conversion a single HWB -> RGB step,
//!     // instead of HWB -> HSL -> RGB, since it consumed the HSL guard.
//! }
//! ```
//!
//! # Reusing Intermediate Data
//!
//! Some conversions produce intermediate data that is the same for all values.
//! For example, conversion between [`Rgb`][crate::rgb::Rgb] and
//! [`Xyz`][crate::xyz::Xyz] uses a [`Matrix3`], while
//! [`Cam16`][crate::cam16::Cam16] uses a set of viewing condition as
//! [`BakedParameters`][crate::cam16::BakedParameters]. These values can be used
//! as "converters" via the [`Convert`] and [`ConvertOnce`] traits.
//!
//! **Remember:** As with anything related to optimization, the performance
//! difference may vary, depending on the compiler's ability to detect and
//! optimize the repeated work on its own.
//!
//! It's sometimes possible to save some computation time by explicitly caching
//! and reusing these intermediate values:
//!
//! ```
//! use palette::{
//!     Srgb, Xyz, FromColor,
//!     cam16::{Cam16, Parameters},
//!     convert::Convert,
//! };
//! # let image_data: [u8;0] = [];
//!
//! // Parameters only need to "bake" once per set of viewing conditions:
//! let parameters = Parameters::default_static_wp(40.0).bake();
//!
//! let pixels: &[Srgb<u8>] = palette::cast::from_component_slice(&image_data);
//! for &color in pixels {
//!     let input = Xyz::from_color(color.into_linear());
//!     let cam16: Cam16<f32> = parameters.convert(input);
//!
//!     // ...
//! }
//! ```
//!
//! It may also be possible to combine multiple matrix steps into a single
//! matrix, instead of applying them one-by-one:
//!
//! ```
//! use palette::{
//!     Srgb, Xyz, lms::BradfordLms,
//!     convert::Convert,
//!     white_point::D65,
//! };
//! # let image_data: [u8;0] = [];
//!
//! // While each matrix may be a compile time constant, the compiler may not
//! // combine them into a single matrix by itself:
//! let matrix = Xyz::matrix_from_rgb()
//!     .then(BradfordLms::<D65, f32>::matrix_from_xyz());
//!
//! let pixels: &[Srgb<u8>] = palette::cast::from_component_slice(&image_data);
//! for &color in pixels {
//!     let lms = matrix.convert(color.into_linear());
//!
//!     // ...
//! }
//! ```
//!
//! # Deriving `FromColorUnclamped`
//!
//! `FromColorUnclamped` can be derived in a mostly automatic way. The other
//! traits are blanket implemented based on it. The default minimum requirement
//! is to implement `FromColorUnclamped<Xyz>`, but it can also be customized to
//! make use of generics and have other manual implementations.
//!
//! It is also recommended to derive or implement
//! [`WithAlpha`](crate::WithAlpha), to be able to convert between all `Alpha`
//! wrapped color types.
//!
//! ## Configuration Attributes
//!
//! The derives can be configured using one or more `#[palette(...)]`
//! attributes. They can be attached to either the item itself, or to the
//! fields.
//!
//! ```
//! # use palette::rgb::{RgbStandard, RgbSpace};
//! # use palette::convert::FromColorUnclamped;
//! # use palette::{Xyz, stimulus::Stimulus};
//! #
//! #[derive(FromColorUnclamped)]
//! #[palette(
//!     component = "T",
//!     rgb_standard = "S",
//! )]
//! #[repr(C)]
//! struct ExampleType<S, T> {
//!     // ...
//!     #[palette(alpha)]
//!     alpha: T,
//!     standard: std::marker::PhantomData<S>,
//! }
//!
//! # impl<S, T> FromColorUnclamped<Xyz<<S::Space as RgbSpace>::WhitePoint, T>> for ExampleType<S, T>
//! # where
//! #   S: RgbStandard,
//! #   T: Stimulus,
//! # {
//! #   fn from_color_unclamped(color: Xyz<<S::Space as RgbSpace>::WhitePoint, T>) -> Self {
//! #       ExampleType {alpha: T::max_intensity(), standard: std::marker::PhantomData}
//! #   }
//! # }
//! #
//! # impl<S, T> FromColorUnclamped<ExampleType<S, T>> for Xyz<<S::Space as RgbSpace>::WhitePoint, T>
//! # where
//! #   S: RgbStandard,
//! #   Self: Default,
//! # {
//! #   fn from_color_unclamped(color: ExampleType<S, T>) -> Self {
//! #       Xyz::default()
//! #   }
//! # }
//! ```
//!
//! ### Item Attributes
//!
//! * `skip_derives(Luma, Rgb)`: No conversion derives will be implemented for
//!   these colors. They are instead to be implemented manually, and serve as
//!   the basis for the automatic implementations.
//!
//! * `white_point = "some::white_point::Type"`: Sets the white point type that
//!   should be used when deriving. The default is `D65`, but it may be any
//!   other type, including type parameters.
//!
//! * `component = "some::component::Type"`: Sets the color component type that
//!   should be used when deriving. The default is `f32`, but it may be any
//!   other type, including type parameters.
//!
//! * `rgb_standard = "some::rgb_standard::Type"`: Sets the RGB standard type
//!   that should be used when deriving. The default is to either use `Srgb` or
//!   a best effort to convert between standards, but sometimes it has to be set
//!   to a specific type. This also accepts type parameters.
//!
//! * `luma_standard = "some::rgb_standard::Type"`: Sets the Luma standard type
//!   that should be used when deriving, similar to `rgb_standard`.
//!
//! ### Field Attributes
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
//! // Start with an Xyz100 color.
//! let xyz = Xyz100 {
//!     x: 59,
//!     y: 75,
//!     z: 42,
//! };
//!
//! // Convert the color to sRGB.
//! let rgb: Srgb = xyz.into_color();
//!
//! assert_eq!(rgb.into_format(), Srgb::new(196u8, 238, 154));
//! ```
//!
//! With generic components:
//!
//! ```rust
//! #[macro_use]
//! extern crate approx;
//!
//! use palette::cast::{ComponentsAs, ArrayCast};
//! use palette::rgb::{Rgb, RgbSpace, RgbStandard};
//! use palette::encoding::Linear;
//! use palette::white_point::D65;
//! use palette::convert::{FromColorUnclamped, IntoColorUnclamped};
//! use palette::{Hsv, Srgb, IntoColor};
//!
//! /// sRGB, but with a reversed memory layout.
//! #[derive(Copy, Clone, ArrayCast, FromColorUnclamped)]
//! #[palette(
//!     skip_derives(Rgb),
//!     component = "T",
//!     rgb_standard = "palette::encoding::Srgb"
//! )]
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
//!     S: RgbStandard,
//!     S::Space: RgbSpace<WhitePoint = D65>,
//!     Srgb<T>: IntoColorUnclamped<Rgb<S, T>>,
//! {
//!     fn from_color_unclamped(color: Bgr<T>) -> Rgb<S, T> {
//!         Srgb::new(color.red, color.green, color.blue)
//!             .into_color_unclamped()
//!     }
//! }
//!
//! impl<S, T> FromColorUnclamped<Rgb<S, T>> for Bgr<T>
//! where
//!     S: RgbStandard,
//!     S::Space: RgbSpace<WhitePoint = D65>,
//!     Srgb<T>: FromColorUnclamped<Rgb<S, T>>,
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
//!     let buffer: &[Bgr<_>] = buffer.components_as();
//!     let hsv: Hsv<_, f64> = buffer[1].into_color();
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
//! use palette::rgb::Rgb;
//! use palette::convert::{FromColorUnclamped, IntoColorUnclamped};
//!
//! /// CSS style sRGB.
//! #[derive(PartialEq, Debug, FromColorUnclamped, WithAlpha)]
//! #[palette(
//!     skip_derives(Rgb),
//!     rgb_standard = "palette::encoding::Srgb"
//! )]
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
//!     Srgb<f32>: FromColorUnclamped<Rgb<S, f32>>
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
//!     Srgb<f32>: IntoColorUnclamped<Rgb<S, f32>>
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
//!     assert_relative_eq!(color, LinSrgba::new(0.496933, 0.0, 1.0, 0.3), epsilon = 0.000001);
//! }
//! ```

pub use self::{
    from_into_color::*, from_into_color_mut::*, from_into_color_unclamped::*,
    from_into_color_unclamped_mut::*, matrix3::*, try_from_into_color::*,
};

mod from_into_color;
mod from_into_color_mut;
mod from_into_color_unclamped;
mod from_into_color_unclamped_mut;
mod matrix3;
mod try_from_into_color;

/// Represents types that can convert a value from one type to another.
pub trait Convert<I, O>: ConvertOnce<I, O> {
    /// Convert an input value of type `I` to a value of type `O`.
    ///
    /// The conversion may produce values outside the typical range of `O`.
    ///
    /// ```
    /// use palette::{
    ///     Xyz, Srgb,
    ///     convert::Convert,
    /// };
    ///
    /// let matrix = Xyz::matrix_from_rgb();
    /// let rgb = Srgb::new(0.8f32, 0.3, 0.3).into_linear();
    /// let xyz = matrix.convert(rgb);
    /// ```
    #[must_use]
    fn convert(&self, input: I) -> O;
}

impl<T, I, O> Convert<I, O> for &T
where
    T: Convert<I, O> + ?Sized,
{
    #[inline]
    fn convert(&self, input: I) -> O {
        T::convert(self, input)
    }
}

#[cfg(feature = "alloc")]
impl<T, I, O> Convert<I, O> for alloc::boxed::Box<T>
where
    T: Convert<I, O>,
{
    #[inline]
    fn convert(&self, input: I) -> O {
        T::convert(self, input)
    }
}

/// Represents types that can convert a value from one type to another at least once.
pub trait ConvertOnce<I, O> {
    /// Convert an input value of type `I` to a value of type `O`, while consuming itself.
    ///
    /// The conversion may produce values outside the typical range of `O`.
    ///
    /// ```
    /// use palette::{
    ///     Xyz, Srgb,
    ///     convert::ConvertOnce,
    /// };
    ///
    /// let matrix = Xyz::matrix_from_rgb();
    /// let rgb = Srgb::new(0.8f32, 0.3, 0.3).into_linear();
    /// let xyz = matrix.convert_once(rgb);
    /// ```
    #[must_use]
    fn convert_once(self, input: I) -> O;
}

impl<T, I, O> ConvertOnce<I, O> for &T
where
    T: Convert<I, O> + ?Sized,
{
    #[inline]
    fn convert_once(self, input: I) -> O {
        T::convert(self, input)
    }
}

#[cfg(feature = "alloc")]
impl<T, I, O> ConvertOnce<I, O> for alloc::boxed::Box<T>
where
    T: ConvertOnce<I, O>,
{
    #[inline]
    fn convert_once(self, input: I) -> O {
        T::convert_once(*self, input)
    }
}
