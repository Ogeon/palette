use num_traits::Float;

use {Component, Hsl, Hsv, Hwb, Lab, Lch, Xyz, Yxy};
use white_point::{D65, WhitePoint};
use rgb::{Rgb, RgbSpace};
use luma::Luma;
use encoding::Linear;

/// FromColor provides conversion between the colors.
///
/// It requires from_xyz, when implemented manually, and derives conversion to other colors as a
/// default from this. These defaults must be overridden when direct conversion exists between
/// colors. For example, Luma has direct conversion to LinRgb. So from_rgb conversion for Luma and
/// from_luma for LinRgb is implemented directly. The from for the same color must override
/// the default. For example, from_rgb for LinRgb will convert via Xyz which needs to be overridden
/// with self to avoid the unnecessary converison.
///
/// # Deriving
///
/// `FromColor` can be derived in a mostly automatic way. The strength of deriving it is that it
/// will also derive `From` implementations from all of the `palette` color types. The minimum
/// requirement is to implement `From<Xyz>`, but it can also be customized to make use of generics
/// and have other manual implementations.
///
/// ## Attributes
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
/// be used when deriving. The default is a best effort to convert between, so sometimes it has to
/// be set to a specific type. This does also accept type parameters.
///
///
/// ## Examples
///
/// Minimum requirements implementation:
///
/// ```rust
/// #[macro_use]
/// extern crate palette_derive;
/// extern crate palette;
///
/// use palette::{Xyz, Srgb};
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
///
/// fn main() {
///     // Start with an sRGB color and convert it from u8 to f32,
///     // which is the default component type.
///     let rgb = Srgb::new(100u8, 23, 59).into_format();
///
///     // Convert the rgb color to our own format.
///     let xyz = Xyz100::from(rgb);
///
///     assert_eq!(xyz, Xyz100 {x: 6, y: 3, z: 4});
/// }
/// ```
///
/// With generic components:
///
/// ```rust
/// #[macro_use]
/// extern crate palette_derive;
/// extern crate palette;
/// extern crate num_traits;
/// #[macro_use]
/// extern crate approx;
///
/// use palette::{Srgb, Hsv, Pixel, Component, FromColor};
/// use palette::rgb::{Rgb, RgbSpace};
/// use palette::encoding::Linear;
/// use palette::white_point::D65;
/// use num_traits::Float;
///
/// /// sRGB, but with a reversed memory layout.
/// #[derive(PartialEq, Debug, FromColor)]
/// #[palette_manual_from(Rgb = "from_rgb_internal")]
/// #[palette_component = "T"]
/// #[repr(C)] // Makes sure the memory layout is as we want it.
/// struct Bgr<T> {
///     blue: T,
///     green: T,
///     red: T,
/// }
///
/// // Careful with this one! It requires `#[repr(C)]`.
/// unsafe impl<T> Pixel<T> for Bgr<T> {
///     const CHANNELS: usize = 3;
/// }
///
/// // Rgb is a bit more complex than other colors, so we are
/// // implementing a private conversion function and letting it
/// // derive `From` automatically. It will take a round trip
/// // through linear format, but that's fine in this case.
/// impl<T: Component + Float> Bgr<T> {
///
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
///             red: srgb.red
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

///IntoColor provides conversion between the colors.
///
///It requires into into_xyz and derives conversion to other colors as a default from this.
///These defaults must be overridden when direct conversion exists between colors.
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

macro_rules! impl_into_color {
    ($self_ty:ident, $from_fn: ident) => {
        impl<Wp, T> IntoColor<Wp, T> for $self_ty<Wp, T>
            where T: Component + Float,
             Wp: WhitePoint
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

            fn into_rgb<S: RgbSpace<WhitePoint=Wp>>(self) -> Rgb<Linear<S>, T> {
                Rgb::$from_fn(self)
            }

            fn into_hsl<S: RgbSpace<WhitePoint=Wp>>(self) -> Hsl<S, T> {
                Hsl::$from_fn(self)
            }

            fn into_hsv<S: RgbSpace<WhitePoint=Wp>>(self) -> Hsv<S, T> {
                Hsv::$from_fn(self)
            }

            fn into_luma(self) -> Luma<Linear<Wp>, T> {
                Luma::$from_fn(self)
            }
        }
    }
}

macro_rules! impl_into_color_rgb {
    ($self_ty:ident, $from_fn: ident) => {
        impl<S, Wp, T> IntoColor<Wp, T> for $self_ty<S, T> where
            T: Component + Float,
            Wp: WhitePoint,
            S: RgbSpace<WhitePoint=Wp>,
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

            fn into_rgb<Sp: RgbSpace<WhitePoint=Wp>>(self) -> Rgb<Linear<Sp>, T> {
                Rgb::$from_fn(self)
            }

            fn into_hsl<Sp: RgbSpace<WhitePoint=Wp>>(self) -> Hsl<Sp, T> {
                Hsl::$from_fn(self)
            }

            fn into_hsv<Sp: RgbSpace<WhitePoint=Wp>>(self) -> Hsv<Sp, T> {
                Hsv::$from_fn(self)
            }

            fn into_luma(self) -> Luma<Linear<Wp>, T> {
                Luma::$from_fn(self)
            }

        }

    }
}

impl_into_color!(Xyz, from_xyz);
impl_into_color!(Yxy, from_yxy);
impl_into_color!(Lab, from_lab);
impl_into_color!(Lch, from_lch);
impl_into_color_rgb!(Hsl, from_hsl);
impl_into_color_rgb!(Hsv, from_hsv);
impl_into_color_rgb!(Hwb, from_hwb);
