use float::Float;

use core::fmt::{self, Display, Formatter};
use {Component, Limited, Hsl, Hsv, Hwb, Lab, Lch, Xyz, Yxy};
use white_point::{D65, WhitePoint};
use rgb::{Rgb, RgbSpace};
use luma::Luma;
use encoding::Linear;

/// FromColor provides conversion from the colors.
///
/// It requires from_xyz, when implemented manually, and derives conversion to other colors as a
/// default from this. These defaults must be overridden when direct conversion exists between
/// colors. For example, Luma has direct conversion to LinRgb. So from_rgb conversion for Luma and
/// from_luma for LinRgb is implemented directly. The from for the same color must override
/// the default. For example, from_rgb for LinRgb will convert via Xyz which needs to be overridden
/// with self to avoid the unnecessary conversion.
///
/// # Deriving
///
/// `FromColor` can be derived in a mostly automatic way. The strength of deriving it is that it
/// will also derive `From` implementations for all of the `palette` color types. The minimum
/// requirement is to implement `From<Xyz>`, but it can also be customized to make use of generics
/// and have other manual implementations.
///
/// ## Item Attributes
///
///  * `#[palette_manual_from(Luma, Rgb = "from_rgb_internal")]`: Specifies the color types that
/// the the custom color type already has `From` implementations for. Adding `= "function_name"`
/// tells it to use that function instead of a `From` implementation. The default, when omitted,
/// is to require `From<Xyz>` to be implemented.
///
///  * `#[palette_white_point = "some::white_point::Type"]`: Sets the white point type that should
/// be used when deriving. The default is `D65`, but it may be any other type, including
/// type parameters.
///
///  * `#[palette_component = "some::component::Type"]`: Sets the color component type that should
/// be used when deriving. The default is `f32`, but it may be any other type, including
/// type parameters.
///
///  * `#[palette_rgb_space = "some::rgb_space::Type"]`: Sets the RGB space type that should
/// be used when deriving. The default is to either use `Srgb` or a best effort to convert between
/// spaces, so sometimes it has to be set to a specific type. This does also accept type parameters.
///
/// ## Field Attributes
///
///  * `#[palette_alpha]`: Specifies that the field is the color's transparency value.
///
/// ## Examples
///
/// Minimum requirements implementation:
///
/// ```rust
/// #[macro_use]
/// extern crate palette;
///
/// use palette::{Srgb, Xyz};
///
/// /// A custom version of Xyz that stores integer values from 0 to 100.
/// #[derive(PartialEq, Debug, FromColor)]
/// struct Xyz100 {
///     x: u8,
///     y: u8,
///     z: u8,
/// }
///
/// // We have to at least implement conversion from Xyz if we don't
/// // specify anything else, using the `palette_manual_from` attribute.
/// impl From<Xyz> for Xyz100 {
///     fn from(color: Xyz) -> Self {
///         let scaled = color * 100.0;
///         Xyz100 {
///             x: scaled.x.max(0.0).min(100.0) as u8,
///             y: scaled.y.max(0.0).min(100.0) as u8,
///             z: scaled.z.max(0.0).min(100.0) as u8,
///         }
///     }
/// }
///
/// fn main() {
///     // Start with an sRGB color and convert it from u8 to f32,
///     // which is the default component type.
///     let rgb = Srgb::new(196u8, 238, 155).into_format();
///
///     // Convert the rgb color to our own format.
///     let xyz = Xyz100::from(rgb);
///
///     assert_eq!(
///         xyz,
///         Xyz100 {
///             x: 59,
///             y: 75,
///             z: 42,
///         }
///     );
/// }
/// ```
///
/// With generic components:
///
/// ```rust
/// #[macro_use]
/// extern crate palette;
/// #[macro_use]
/// extern crate approx;
///
/// use palette::{Component, FromColor, Hsv, Pixel, Srgb};
/// use palette::rgb::{Rgb, RgbSpace};
/// use palette::encoding::Linear;
/// use palette::white_point::D65;
/// use palette::float::Float;
///
/// /// sRGB, but with a reversed memory layout.
/// #[derive(PartialEq, Debug, FromColor, Pixel)]
/// #[palette_manual_from(Rgb = "from_rgb_internal")]
/// #[palette_component = "T"]
/// #[repr(C)] // Makes sure the memory layout is as we want it.
/// struct Bgr<T> {
///     blue: T,
///     green: T,
///     red: T,
/// }
///
/// // Rgb is a bit more complex than other colors, so we are
/// // implementing a private conversion function and letting it
/// // derive `From` automatically. It will take a round trip
/// // through linear format, but that's fine in this case.
/// impl<T: Component + Float> Bgr<T> {
///     // It converts from any linear Rgb type that has the D65
///     // white point, which is the default if we don't specify
///     // anything else with the `palette_white_point` attribute.
///     fn from_rgb_internal<S>(color: Rgb<Linear<S>, T>) -> Self
///     where
///         S: RgbSpace<WhitePoint = D65>,
///     {
///         let srgb = Srgb::from_rgb(color);
///
///         Bgr {
///             blue: srgb.blue,
///             green: srgb.green,
///             red: srgb.red,
///         }
///     }
/// }
///
/// fn main() {
///     let mut buffer = vec![0.0f64, 0.0, 0.0, 0.0, 0.0, 0.0];
///     {
///         let bgr_buffer = Bgr::from_raw_slice_mut(&mut buffer);
///         bgr_buffer[1] = Hsv::new(90.0, 1.0, 0.5).into();
///     }
///
///     assert_relative_eq!(buffer[3], 0.0);
///     assert_relative_eq!(buffer[4], 0.7353569830524495);
///     assert_relative_eq!(buffer[5], 0.5370987304831942);
/// }
/// ```
///
/// With alpha component:
///
/// ```rust
/// #[macro_use]
/// extern crate palette;
///
/// use palette::{FromColor, LinSrgba, Srgb};
/// use palette::rgb::{Rgb, RgbSpace};
/// use palette::encoding::Linear;
/// use palette::white_point::D65;
///
/// /// CSS style sRGB.
/// #[derive(PartialEq, Debug, FromColor)]
/// #[palette_manual_from(Rgb = "from_rgb_internal")]
/// struct CssRgb {
///     red: u8,
///     green: u8,
///     blue: u8,
///     #[palette_alpha]
///     alpha: f32,
/// }
///
/// // We will write a conversion function for opaque RGB and derive
/// // will take care of preserving the transparency for us.
/// impl CssRgb {
///     fn from_rgb_internal<S>(color: Rgb<Linear<S>, f32>) -> Self
///     where
///         S: RgbSpace<WhitePoint = D65>,
///     {
///         // Convert to u8 sRGB
///         let srgb = Srgb::from_rgb(color).into_format();
///
///         CssRgb {
///             red: srgb.red,
///             green: srgb.green,
///             blue: srgb.blue,
///             alpha: 1.0,
///         }
///     }
/// }
///
/// fn main() {
///     let color = LinSrgba::new(0.5, 0.0, 1.0, 0.3);
///     let css_color = CssRgb::from(color);
///
///     assert_eq!(
///         css_color,
///         CssRgb {
///             red: 188,
///             green: 0,
///             blue: 255,
///             alpha: 0.3,
///         }
///     );
/// }
/// ```
pub trait FromColor<Wp = D65, T = f32>: Sized
where
    T: Component + Float,
    Wp: WhitePoint,
{
    ///Convert from XYZ color space
    fn from_xyz(Xyz<Wp, T>) -> Self;

    ///Convert from Yxy color space
    fn from_yxy(inp: Yxy<Wp, T>) -> Self {
        Self::from_xyz(inp.into_xyz())
    }

    ///Convert from L\*a\*b\* color space
    fn from_lab(inp: Lab<Wp, T>) -> Self {
        Self::from_xyz(inp.into_xyz())
    }

    ///Convert from L\*C\*h° color space
    fn from_lch(inp: Lch<Wp, T>) -> Self {
        Self::from_lab(inp.into_lab())
    }

    ///Convert from RGB color space
    fn from_rgb<S: RgbSpace<WhitePoint = Wp>>(inp: Rgb<Linear<S>, T>) -> Self {
        Self::from_xyz(inp.into_xyz())
    }

    ///Convert from HSL color space
    fn from_hsl<S: RgbSpace<WhitePoint = Wp>>(inp: Hsl<S, T>) -> Self {
        Self::from_rgb(Rgb::<Linear<S>, T>::from_hsl(inp))
    }

    ///Convert from HSV color space
    fn from_hsv<S: RgbSpace<WhitePoint = Wp>>(inp: Hsv<S, T>) -> Self {
        Self::from_rgb(Rgb::<Linear<S>, T>::from_hsv(inp))
    }

    ///Convert from HWB color space
    fn from_hwb<S: RgbSpace<WhitePoint = Wp>>(inp: Hwb<S, T>) -> Self {
        Self::from_hsv(Hsv::<S, T>::from_hwb(inp))
    }

    ///Convert from Luma
    fn from_luma(inp: Luma<Linear<Wp>, T>) -> Self {
        Self::from_xyz(inp.into_xyz())
    }
}

/// IntoColor provides conversion to the colors.
///
/// It requires into_xyz, when implemented manually, and derives conversion to other colors as a
/// default from this. These defaults must be overridden when direct conversion exists between
/// colors.
///
/// # Deriving
///
/// `IntoColor` can be derived in a mostly automatic way. The strength of deriving it is that it
/// will also derive `Into` implementations for all of the `palette` color types. The minimum
/// requirement is to implement `Into<Xyz>`, but it can also be customized to make use of generics
/// and have other manual implementations.
///
/// ## Item Attributes
///
///  * `#[palette_manual_into(Luma, Rgb = "into_rgb_internal")]`: Specifies the color types that
/// the the custom color type already has `Into` implementations for. Adding `= "function_name"`
/// tells it to use that function instead of an `Into` implementation. The default, when omitted,
/// is to require `Into<Xyz>` to be implemented.
///
///  * `#[palette_white_point = "some::white_point::Type"]`: Sets the white point type that should
/// be used when deriving. The default is `D65`, but it may be any other type, including
/// type parameters.
///
///  * `#[palette_component = "some::component::Type"]`: Sets the color component type that should
/// be used when deriving. The default is `f32`, but it may be any other type, including
/// type parameters.
///
///  * `#[palette_rgb_space = "some::rgb_space::Type"]`: Sets the RGB space type that should
/// be used when deriving. The default is to either use `Srgb` or a best effort to convert between
/// spaces, so sometimes it has to be set to a specific type. This does also accept type parameters.
///
/// ## Field Attributes
///
///  * `#[palette_alpha]`: Specifies that the field is the color's transparency value.
///
/// ## Examples
///
/// Minimum requirements implementation:
///
/// ```rust
/// #[macro_use]
/// extern crate palette;
///
/// use palette::{Srgb, Xyz};
///
/// /// A custom version of Xyz that stores integer values from 0 to 100.
/// #[derive(PartialEq, Debug, IntoColor)]
/// struct Xyz100 {
///     x: u8,
///     y: u8,
///     z: u8,
/// }
///
/// // We have to at least implement conversion into Xyz if we don't
/// // specify anything else, using the `palette_manual_into` attribute.
/// impl Into<Xyz> for Xyz100 {
///     fn into(self) -> Xyz {
///         Xyz::new(
///             self.x as f32 / 100.0,
///             self.y as f32 / 100.0,
///             self.z as f32 / 100.0,
///         )
///     }
/// }
///
/// fn main() {
///     // Start with an Xyz100 color.
///     let xyz = Xyz100 {
///         x: 59,
///         y: 75,
///         z: 42,
///     };
///
///     // Convert the color to sRGB.
///     let rgb: Srgb = xyz.into();
///
///     assert_eq!(rgb.into_format(), Srgb::new(196u8, 238, 154));
/// }
/// ```
///
/// With generic components:
///
/// ```rust
/// #[macro_use]
/// extern crate palette;
/// #[macro_use]
/// extern crate approx;
///
/// use palette::{Component, Hsv, IntoColor, Pixel, Srgb};
/// use palette::rgb::{Rgb, RgbSpace};
/// use palette::encoding::{Linear, self};
/// use palette::white_point::D65;
/// use palette::float::Float;
///
/// type Hsv64 = Hsv<encoding::Srgb, f64>;
///
/// /// sRGB, but with a reversed memory layout.
/// #[derive(Copy, Clone, IntoColor, Pixel)]
/// #[palette_manual_into(Rgb = "into_rgb_internal")]
/// #[palette_component = "T"]
/// #[repr(C)] // Makes sure the memory layout is as we want it.
/// struct Bgr<T> {
///     blue: T,
///     green: T,
///     red: T,
/// }
///
/// // Rgb is a bit more complex than other colors, so we are
/// // implementing a private conversion function and letting it
/// // derive `Into` automatically.
/// impl<T: Component + Float> Bgr<T> {
///     // It converts from any linear Rgb type that has the D65
///     // white point, which is the default if we don't specify
///     // anything else with the `palette_white_point` attribute.
///     fn into_rgb_internal<S>(self) -> Rgb<Linear<S>, T>
///     where
///         S: RgbSpace<WhitePoint = D65>,
///     {
///         Srgb::new(self.red, self.green, self.blue).into_rgb()
///     }
/// }
///
/// fn main() {
///     let buffer = vec![
///         0.0f64,
///         0.0,
///         0.0,
///         0.0,
///         0.7353569830524495,
///         0.5370987304831942,
///     ];
///     let hsv: Hsv64 = Bgr::from_raw_slice(&buffer)[1].into();
///
///     assert_relative_eq!(hsv, Hsv::new(90.0, 1.0, 0.5));
/// }
/// ```
///
/// With alpha component:
///
/// ```rust
/// #[macro_use]
/// extern crate palette;
/// #[macro_use]
/// extern crate approx;
///
/// use palette::{IntoColor, LinSrgba, Srgb};
/// use palette::rgb::{Rgb, RgbSpace};
/// use palette::encoding::Linear;
/// use palette::white_point::D65;
///
/// /// CSS style sRGB.
/// #[derive(PartialEq, Debug, IntoColor)]
/// #[palette_manual_into(Rgb = "into_rgb_internal")]
/// struct CssRgb {
///     red: u8,
///     green: u8,
///     blue: u8,
///     #[palette_alpha]
///     alpha: f32,
/// }
///
/// // We will write a conversion function for opaque RGB and derive
/// // will take care of preserving the transparency for us.
/// impl CssRgb {
///     fn into_rgb_internal<S>(self) -> Rgb<Linear<S>, f32>
///     where
///         S: RgbSpace<WhitePoint = D65>,
///     {
///         Srgb::new(self.red, self.green, self.blue)
///             .into_format()
///             .into_rgb()
///     }
/// }
///
/// fn main() {
///     let css_color = CssRgb {
///         red: 187,
///         green: 0,
///         blue: 255,
///         alpha: 0.3,
///     };
///     let color: LinSrgba = css_color.into();
///
///     assert_relative_eq!(color, LinSrgba::new(0.496933, 0.0, 1.0, 0.3));
/// }
/// ```
pub trait IntoColor<Wp = D65, T = f32>: Sized
where
    T: Component + Float,
    Wp: WhitePoint,
{
    ///Convert into XYZ space
    fn into_xyz(self) -> Xyz<Wp, T>;

    ///Convert into Yxy color space
    fn into_yxy(self) -> Yxy<Wp, T> {
        Yxy::from_xyz(self.into_xyz())
    }

    ///Convert into L\*a\*b\* color space
    fn into_lab(self) -> Lab<Wp, T> {
        Lab::from_xyz(self.into_xyz())
    }

    ///Convert into L\*C\*h° color space
    fn into_lch(self) -> Lch<Wp, T> {
        Lch::from_lab(self.into_lab())
    }

    ///Convert into RGB color space.
    fn into_rgb<S: RgbSpace<WhitePoint = Wp>>(self) -> Rgb<Linear<S>, T> {
        Rgb::from_xyz(self.into_xyz())
    }

    ///Convert into HSL color space
    fn into_hsl<S: RgbSpace<WhitePoint = Wp>>(self) -> Hsl<S, T> {
        let rgb: Rgb<Linear<S>, T> = self.into_rgb();
        Hsl::from_rgb(rgb)
    }

    ///Convert into HSV color space
    fn into_hsv<S: RgbSpace<WhitePoint = Wp>>(self) -> Hsv<S, T> {
        let rgb: Rgb<Linear<S>, T> = self.into_rgb();
        Hsv::from_rgb(rgb)
    }

    ///Convert into HWB color space
    fn into_hwb<S: RgbSpace<WhitePoint = Wp>>(self) -> Hwb<S, T> {
        let hsv: Hsv<S, T> = self.into_hsv();
        Hwb::from_hsv(hsv)
    }

    ///Convert into Luma
    fn into_luma(self) -> Luma<Linear<Wp>, T> {
        Luma::from_xyz(self.into_xyz())
    }
}

///The error type for a color conversion that converted a color into a color with invalid values.
#[derive(Debug)]
pub struct OutOfBounds<T> {
    color: T,
}

impl<T> OutOfBounds<T> {
    ///Create a new error wrapping a color
    #[inline]
    fn new(color: T) -> Self {
        OutOfBounds { color }
    }

    ///Consume this error and return the wrapped color
    #[inline]
    pub fn color(self) -> T {
        self.color
    }
}

#[cfg(feature = "std")]
impl<T: ::std::fmt::Debug> ::std::error::Error for OutOfBounds<T> {
    fn description(&self) -> &str {
        "Color conversion is out of bounds"
    }
}

impl<T> Display for OutOfBounds<T> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "Color conversion is out of bounds")
    }
}

///A trait for converting a color into another.
pub trait ConvertInto<T>: Into<T> {
    ///Convert into T with values clamped to the color defined bounds
    ///
    ///```
    ///use palette::ConvertInto;
    ///use palette::Limited;
    ///use palette::{Srgb, Lch};
    ///
    ///
    ///let rgb: Srgb = Lch::new(50.0, 100.0, -175.0).convert_into();
    ///assert!(rgb.is_valid());
    ///```
    fn convert_into(self) -> T;

    ///Convert into T. The resulting color might be invalid in its color space
    ///
    ///```
    ///use palette::ConvertInto;
    ///use palette::Limited;
    ///use palette::{Srgb, Lch};
    ///
    ///let rgb: Srgb = Lch::new(50.0, 100.0, -175.0).convert_unclamped_into();
    ///assert!(!rgb.is_valid());
    ///```
    fn convert_unclamped_into(self) -> T;

    ///Convert into T, returning ok if the color is inside of its defined range,
    ///otherwise an `OutOfBounds` error is returned which contains the unclamped color.
    ///
    ///```
    ///use palette::ConvertInto;
    ///use palette::{Srgb, Hsl};
    ///
    ///let rgb: Srgb = match Hsl::new(150.0, 1.0, 1.1).try_convert_into() {
    ///    Ok(color) => color,
    ///    Err(err) => {
    ///        println!("Color is out of bounds");
    ///        err.color()
    ///    },
    ///};
    ///```
    fn try_convert_into(self) -> Result<T, OutOfBounds<T>>;
}

///A trait for converting one color from another.
///
///`convert_unclamped` currently wraps the underlying `From` implementation.
pub trait ConvertFrom<T>: From<T> {
    ///Convert from T with values clamped to the color defined bounds
    ///
    ///```
    ///use palette::ConvertFrom;
    ///use palette::Limited;
    ///use palette::{Srgb, Lch};
    ///
    ///
    ///let rgb = Srgb::convert_from(Lch::new(50.0, 100.0, -175.0));
    ///assert!(rgb.is_valid());
    ///```
    fn convert_from(_: T) -> Self;

    ///Convert from T. The resulting color might be invalid in its color space
    ///
    ///```
    ///use palette::ConvertFrom;
    ///use palette::Limited;
    ///use palette::{Srgb, Lch};
    ///
    ///let rgb = Srgb::convert_unclamped_from(Lch::new(50.0, 100.0, -175.0));
    ///assert!(!rgb.is_valid());
    ///```
    #[inline]
    fn convert_unclamped_from(val: T) -> Self {
        Self::from(val)
    }

    ///Convert from T, returning ok if the color is inside of its defined range,
    ///otherwise an `OutOfBounds` error is returned which contains the unclamped color.
    ///
    ///```
    ///use palette::ConvertFrom;
    ///use palette::{Srgb, Hsl};
    ///
    ///let rgb = match Srgb::try_convert_from(Hsl::new(150.0, 1.0, 1.1)) {
    ///    Ok(color) => color,
    ///    Err(err) => {
    ///        println!("Color is out of bounds");
    ///        err.color()
    ///    },
    ///};
    ///```
    fn try_convert_from(_: T) -> Result<Self, OutOfBounds<Self>>;
}

impl<T, U> ConvertFrom<T> for U where U: From<T> + Limited {
    fn convert_from(t: T) -> U {
        let mut this = U::from(t);
        if !this.is_valid() {
            this.clamp_self();
        }
        this
    }

    fn try_convert_from(t: T) -> Result<U, OutOfBounds<U>> {
        let this = U::from(t);
        if this.is_valid() {
            Ok(this)
        } else {
            Err(OutOfBounds::new(this))
        }
    }
}

// ConvertFrom implies ConvertInto
impl<T, U> ConvertInto<U> for T where U: ConvertFrom<T> {
    #[inline]
    fn convert_into(self) -> U {
        U::convert_from(self)
    }

    #[inline]
    fn convert_unclamped_into(self) -> U {
        U::convert_unclamped_from(self)
    }

    #[inline]
    fn try_convert_into(self) -> Result<U, OutOfBounds<U>> {
        U::try_convert_from(self)
    }
}

macro_rules! impl_into_color {
    ($self_ty: ident, $from_fn: ident) => {
        impl<Wp, T> IntoColor<Wp, T> for $self_ty<Wp, T>
        where
            T: Component + Float,
            Wp: WhitePoint,
        {
            fn into_xyz(self) -> Xyz<Wp, T> {
                Xyz::$from_fn(self)
            }

            fn into_yxy(self) -> Yxy<Wp, T> {
                Yxy::$from_fn(self)
            }

            fn into_lab(self) -> Lab<Wp, T> {
                Lab::$from_fn(self)
            }

            fn into_lch(self) -> Lch<Wp, T> {
                Lch::$from_fn(self)
            }

            fn into_rgb<S: RgbSpace<WhitePoint = Wp>>(self) -> Rgb<Linear<S>, T> {
                Rgb::$from_fn(self)
            }

            fn into_hsl<S: RgbSpace<WhitePoint = Wp>>(self) -> Hsl<S, T> {
                Hsl::$from_fn(self)
            }

            fn into_hsv<S: RgbSpace<WhitePoint = Wp>>(self) -> Hsv<S, T> {
                Hsv::$from_fn(self)
            }

            fn into_luma(self) -> Luma<Linear<Wp>, T> {
                Luma::$from_fn(self)
            }
        }
    };
}

macro_rules! impl_into_color_rgb {
    ($self_ty: ident, $from_fn: ident) => {
        impl<S, Wp, T> IntoColor<Wp, T> for $self_ty<S, T>
        where
            T: Component + Float,
            Wp: WhitePoint,
            S: RgbSpace<WhitePoint = Wp>,
        {
            fn into_xyz(self) -> Xyz<Wp, T> {
                Xyz::$from_fn(self)
            }

            fn into_yxy(self) -> Yxy<Wp, T> {
                Yxy::$from_fn(self)
            }

            fn into_lab(self) -> Lab<Wp, T> {
                Lab::$from_fn(self)
            }

            fn into_lch(self) -> Lch<Wp, T> {
                Lch::$from_fn(self)
            }

            fn into_rgb<Sp: RgbSpace<WhitePoint = Wp>>(self) -> Rgb<Linear<Sp>, T> {
                Rgb::$from_fn(self)
            }

            fn into_hsl<Sp: RgbSpace<WhitePoint = Wp>>(self) -> Hsl<Sp, T> {
                Hsl::$from_fn(self)
            }

            fn into_hsv<Sp: RgbSpace<WhitePoint = Wp>>(self) -> Hsv<Sp, T> {
                Hsv::$from_fn(self)
            }

            fn into_luma(self) -> Luma<Linear<Wp>, T> {
                Luma::$from_fn(self)
            }
        }
    };
}

impl_into_color!(Xyz, from_xyz);
impl_into_color!(Yxy, from_yxy);
impl_into_color!(Lab, from_lab);
impl_into_color!(Lch, from_lch);
impl_into_color_rgb!(Hsl, from_hsl);
impl_into_color_rgb!(Hsv, from_hsv);
impl_into_color_rgb!(Hwb, from_hwb);

#[cfg(test)]
mod tests {
    use core::marker::PhantomData;
    use float::Float;
    use Component;
    use encoding::linear::Linear;
    use rgb::{Rgb, RgbSpace};
    use luma::Luma;
    use {Hsl, Hsv, Hwb, Lab, Lch, Xyz, Yxy};

    #[derive(Copy, Clone, FromColor, IntoColor)]
    #[palette_manual_from(Xyz, Luma = "from_luma_internal")]
    #[palette_manual_into(Xyz, Luma = "into_luma_internal")]
    #[palette_white_point = "S::WhitePoint"]
    #[palette_component = "f64"]
    #[palette_rgb_space = "S"]
    #[palette_internal]
    struct WithXyz<S: RgbSpace>(PhantomData<S>);

    impl<S: RgbSpace> WithXyz<S> {
        fn from_luma_internal(_color: Luma<Linear<S::WhitePoint>, f64>) -> Self {
            WithXyz(PhantomData)
        }

        fn into_luma_internal(self) -> Luma<Linear<S::WhitePoint>, f64> {
            Luma::new(1.0)
        }
    }

    impl<S: RgbSpace> From<Xyz<S::WhitePoint, f64>> for WithXyz<S> {
        fn from(_color: Xyz<S::WhitePoint, f64>) -> Self {
            WithXyz(PhantomData)
        }
    }

    impl<S: RgbSpace> Into<Xyz<S::WhitePoint, f64>> for WithXyz<S> {
        fn into(self) -> Xyz<S::WhitePoint, f64> {
            Xyz::with_wp(0.0, 1.0, 0.0)
        }
    }

    #[derive(Copy, Clone, FromColor, IntoColor)]
    #[palette_manual_from(Lch, Luma = "from_luma_internal")]
    #[palette_manual_into(Lch, Luma = "into_luma_internal")]
    #[palette_white_point = "::white_point::E"]
    #[palette_component = "T"]
    #[palette_rgb_space = "(::encoding::Srgb, ::white_point::E)"]
    #[palette_internal]
    struct WithoutXyz<T: Component + Float>(PhantomData<T>);

    impl<T: Component + Float> WithoutXyz<T> {
        fn from_luma_internal(_color: Luma<Linear<::white_point::E>, T>) -> Self {
            WithoutXyz(PhantomData)
        }

        fn into_luma_internal(self) -> Luma<Linear<::white_point::E>, T> {
            Luma::new(T::one())
        }
    }

    impl<T: Component + Float> From<Lch<::white_point::E, T>> for WithoutXyz<T> {
        fn from(_color: Lch<::white_point::E, T>) -> Self {
            WithoutXyz(PhantomData)
        }
    }

    impl<T: Component + Float> Into<Lch<::white_point::E, T>> for WithoutXyz<T> {
        fn into(self) -> Lch<::white_point::E, T> {
            Lch::with_wp(T::one(), T::zero(), T::zero())
        }
    }

    #[test]
    fn from_with_xyz() {
        let xyz: Xyz<_, f64> = Default::default();
        WithXyz::<::encoding::Srgb>::from(xyz);

        let yxy: Yxy<_, f64> = Default::default();
        WithXyz::<::encoding::Srgb>::from(yxy);

        let lab: Lab<_, f64> = Default::default();
        WithXyz::<::encoding::Srgb>::from(lab);

        let lch: Lch<_, f64> = Default::default();
        WithXyz::<::encoding::Srgb>::from(lch);

        let rgb: Rgb<::encoding::Srgb, f64> = Default::default();
        WithXyz::<::encoding::Srgb>::from(rgb);

        let hsl: Hsl<_, f64> = Default::default();
        WithXyz::<::encoding::Srgb>::from(hsl);

        let hsv: Hsv<_, f64> = Default::default();
        WithXyz::<::encoding::Srgb>::from(hsv);

        let hwb: Hwb<_, f64> = Default::default();
        WithXyz::<::encoding::Srgb>::from(hwb);

        let luma: Luma<::encoding::Srgb, f64> = Default::default();
        WithXyz::<::encoding::Srgb>::from(luma);
    }

    #[test]
    fn into_with_xyz() {
        let color = WithXyz::<::encoding::Srgb>(PhantomData);

        let _xyz: Xyz<_, f64> = color.into();
        let _yxy: Yxy<_, f64> = color.into();
        let _lab: Lab<_, f64> = color.into();
        let _lch: Lch<_, f64> = color.into();
        let _rgb: Rgb<::encoding::Srgb, f64> = color.into();
        let _hsl: Hsl<_, f64> = color.into();
        let _hsv: Hsv<_, f64> = color.into();
        let _hwb: Hwb<_, f64> = color.into();
        let _luma: Luma<::encoding::Srgb, f64> = color.into();
    }

    #[test]
    fn from_without_xyz() {
        let xyz: Xyz<::white_point::E, f64> = Default::default();
        WithoutXyz::<f64>::from(xyz);

        let yxy: Yxy<::white_point::E, f64> = Default::default();
        WithoutXyz::<f64>::from(yxy);

        let lab: Lab<::white_point::E, f64> = Default::default();
        WithoutXyz::<f64>::from(lab);

        let lch: Lch<::white_point::E, f64> = Default::default();
        WithoutXyz::<f64>::from(lch);

        let rgb: Rgb<(_, ::encoding::Srgb), f64> = Default::default();
        WithoutXyz::<f64>::from(rgb);

        let hsl: Hsl<_, f64> = Default::default();
        WithoutXyz::<f64>::from(hsl);

        let hsv: Hsv<_, f64> = Default::default();
        WithoutXyz::<f64>::from(hsv);

        let hwb: Hwb<_, f64> = Default::default();
        WithoutXyz::<f64>::from(hwb);

        let luma: Luma<Linear<::white_point::E>, f64> = Default::default();
        WithoutXyz::<f64>::from(luma);
    }

    #[test]
    fn into_without_xyz() {
        let color = WithoutXyz::<f64>(PhantomData);

        let _xyz: Xyz<::white_point::E, f64> = color.into();
        let _yxy: Yxy<::white_point::E, f64> = color.into();
        let _lab: Lab<::white_point::E, f64> = color.into();
        let _lch: Lch<::white_point::E, f64> = color.into();
        let _rgb: Rgb<(_, ::encoding::Srgb), f64> = color.into();
        let _hsl: Hsl<_, f64> = color.into();
        let _hsv: Hsv<_, f64> = color.into();
        let _hwb: Hwb<_, f64> = color.into();
        let _luma: Luma<Linear<::white_point::E>, f64> = color.into();
    }
}
