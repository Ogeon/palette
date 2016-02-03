//!A library that makes linear color calculations and conversion easy and
//!accessible for anyone. It provides both precision tools that lets you work
//!in exactly the color space you want to, as well as a general color type
//!that abstracts away some of the technical details.
//!
//!# Linear?
//!
//!Colors in, for example, images are often "gamma corrected" or stored in
//!sRGB format as a compression method and to prevent banding. This is also a
//!bit of a legacy from the ages of the CRT monitors, where the output from
//!the electron guns was nonlinear. The problem is that these formats doesn't
//!represent the actual intensities, and the compression has to be reverted to
//!make sure that any operations on the colors are accurate. This library uses
//!a completely linear work flow, and comes with the tools for transitioning
//!between linear and non-linear RGB.
//!
//!# Transparency
//!
//!There are many cases where pixel transparency is important, but there are
//!also many cases where it becomes a dead weight, if it's always stored
//!together with the color, but not used. Palette has therefore adopted a
//!structure where the transparency component (alpha) is attachable using the
//![`Alpha`](struct.Alpha.html) type, instead of having copies of each color
//!space.
//!
//!This approach comes with the extra benefit of allowing operations to
//!selectively affect the alpha component:
//!
//!```
//!use palette::{Rgb, Rgba};
//!
//!let mut c1 = Rgba::new(1.0, 0.5, 0.5, 0.8);
//!let c2 = Rgb::new(0.5, 1.0, 1.0);
//!
//!c1.color = c1.color * c2; //Leave the alpha as it is
//!c1.blue += 0.2; //The color components can easily be accessed
//!c1 = c1 * 0.5; //Scale both the color and the alpha
//!```


#![doc(html_root_url = "http://ogeon.github.io/docs/palette/master/")]

#![cfg_attr(feature = "strict", deny(missing_docs))]
#![cfg_attr(feature = "strict", deny(warnings))]

#[cfg(test)]
#[macro_use]
extern crate approx;

extern crate num;

#[cfg(feature = "phf")]
extern crate phf;

use num::{Float, ToPrimitive, NumCast};

// use pixel::{Srgb, GammaRgb};

pub use gradient::Gradient;
pub use alpha::Alpha;
pub use rgb::{Rgb, Rgba};
pub use luma::{Luma, Lumaa};
pub use xyz::{Xyz, Xyza};
pub use lab::{Lab, Laba};
pub use lch::{Lch, Lcha};
pub use hsv::{Hsv, Hsva};
pub use hsl::{Hsl, Hsla};
pub use yxy::{Yxy, Yxya};
pub use white_point::{WhitePoint, D50, D65};

pub use hues::{LabHue, RgbHue};

// macro_rules! from_color {
//     (to $to:ident from $($from:ident),+) => (
//         impl<T:Float> From<Color<T>> for $to<T> {
//             fn from(color: Color<T>) -> $to<T> {
//                 match color {
//                     Color::$to(c) => c,
//                     $(Color::$from(c) => c.into(),)+
//                 }
//             }
//         }
//     )
// }
//
// macro_rules! alpha_from {
//     ($self_ty:ident {$($other:ident),+}) => (
//         impl<T: Float> From<Alpha<$self_ty<T>, T>> for $self_ty<T> {
//             fn from(color: Alpha<$self_ty<T>, T>) -> $self_ty<T> {
//                 color.color
//             }
//         }
//
//         $(
//             impl<T: Float> From<Alpha<$other<T>, T>> for Alpha<$self_ty<T>, T> {
//                 fn from(other: Alpha<$other<T>, T>) -> Alpha<$self_ty<T>, T> {
//                     Alpha {
//                         color: other.color.into(),
//                         alpha: other.alpha,
//                     }
//                 }
//             }
//
//             impl<T: Float> From<$other<T>> for Alpha<$self_ty<T>, T> {
//                 fn from(color: $other<T>) -> Alpha<$self_ty<T>, T> {
//                     Alpha {
//                         color: color.into(),
//                         alpha: T::one(),
//                     }
//                 }
//             }
//
//             impl<T: Float> From<Alpha<$other<T>, T>> for $self_ty<T> {
//                 fn from(other: Alpha<$other<T>, T>) -> $self_ty<T> {
//                     other.color.into()
//                 }
//             }
//         )+
//     )
// }

//Helper macro for approximate component wise comparison. Most color spaces
//are in roughly the same ranges, so this epsilon should be alright.
#[cfg(test)]
macro_rules! assert_approx_eq {
    ($a:ident, $b:ident, [$($components:ident),+]) => ({
        $(
            let a: f32 = $a.$components.into();
            let b: f32 = $b.$components.into();
            assert_relative_eq!(a, b, epsilon = 0.0001);
        )+
    })
}

//Helper macro for checking ranges and clamping.
#[cfg(test)]
macro_rules! assert_ranges {
    (@make_tuple $first:pat, $next:ident,) => (($first, $next));

    (@make_tuple $first:pat, $next:ident, $($rest:ident,)*) => (
        assert_ranges!(@make_tuple ($first, $next), $($rest,)*)
    );

    (
        $ty:ident;
        limited {$($limited:ident: $limited_from:expr => $limited_to:expr),+}
        limited_min {$($limited_min:ident: $limited_min_from:expr => $limited_min_to:expr),*}
        unlimited {$($unlimited:ident: $unlimited_from:expr => $unlimited_to:expr),*}
    ) => (
        {
            use std::iter::repeat;
            use Limited;

            {
                print!("checking below limits ... ");
                $(
                    let from = $limited_from;
                    let to = $limited_to;
                    let diff = to - from;
                    let $limited = (1..11).map(|i| from - (i as f64 / 10.0) * diff);
                )+

                $(
                    let from = $limited_min_from;
                    let to = $limited_min_to;
                    let diff = to - from;
                    let $limited_min = (1..11).map(|i| from - (i as f64 / 10.0) * diff);
                )*

                $(
                    let from = $unlimited_from;
                    let to = $unlimited_to;
                    let diff = to - from;
                    let $unlimited = (1..11).map(|i| from - (i as f64 / 10.0) * diff);
                )*

                for assert_ranges!(@make_tuple (), $($limited,)+ $($limited_min,)* $($unlimited,)* ) in repeat(()) $(.zip($limited))+ $(.zip($limited_min))* $(.zip($unlimited))* {
                    let c = $ty::<f64> {
                        $($limited: $limited.into(),)+
                        $($limited_min: $limited_min.into(),)*
                        $($unlimited: $unlimited.into(),)*
                    };
                    let clamped = c.clamp();
                    let expected = $ty {
                        $($limited: $limited_from.into(),)+
                        $($limited_min: $limited_min_from.into(),)*
                        $($unlimited: $unlimited.into(),)*
                    };

                    assert!(!c.is_valid());
                    assert_eq!(clamped, expected);
                }

                println!("ok")
            }

            {
                print!("checking within limits ... ");
                $(
                    let from = $limited_from;
                    let to = $limited_to;
                    let diff = to - from;
                    let $limited = (0..11).map(|i| from + (i as f64 / 10.0) * diff);
                )+

                $(
                    let from = $limited_min_from;
                    let to = $limited_min_to;
                    let diff = to - from;
                    let $limited_min = (0..11).map(|i| from + (i as f64 / 10.0) * diff);
                )*

                $(
                    let from = $unlimited_from;
                    let to = $unlimited_to;
                    let diff = to - from;
                    let $unlimited = (0..11).map(|i| from + (i as f64 / 10.0) * diff);
                )*

                for assert_ranges!(@make_tuple (), $($limited,)+ $($limited_min,)* $($unlimited,)* ) in repeat(()) $(.zip($limited))+ $(.zip($limited_min))* $(.zip($unlimited))* {
                    let c = $ty::<f64> {
                        $($limited: $limited.into(),)+
                        $($limited_min: $limited_min.into(),)*
                        $($unlimited: $unlimited.into(),)*
                    };
                    let clamped = c.clamp();

                    assert!(c.is_valid());
                    assert_eq!(clamped, c);
                }

                println!("ok")
            }

            {
                print!("checking above limits ... ");
                $(
                    let from = $limited_from;
                    let to = $limited_to;
                    let diff = to - from;
                    let $limited = (1..11).map(|i| to + (i as f64 / 10.0) * diff);
                )+

                $(
                    let from = $limited_min_from;
                    let to = $limited_min_to;
                    let diff = to - from;
                    let $limited_min = (1..11).map(|i| to + (i as f64 / 10.0) * diff);
                )*

                $(
                    let from = $unlimited_from;
                    let to = $unlimited_to;
                    let diff = to - from;
                    let $unlimited = (1..11).map(|i| to + (i as f64 / 10.0) * diff);
                )*

                for assert_ranges!(@make_tuple (), $($limited,)+ $($limited_min,)* $($unlimited,)* ) in repeat(()) $(.zip($limited))+ $(.zip($limited_min))* $(.zip($unlimited))* {
                    let c = $ty::<f64> {
                        $($limited: $limited.into(),)+
                        $($limited_min: $limited_min.into(),)*
                        $($unlimited: $unlimited.into(),)*
                    };
                    let clamped = c.clamp();
                    let expected = $ty {
                        $($limited: $limited_to.into(),)+
                        $($limited_min: $limited_min.into(),)*
                        $($unlimited: $unlimited.into(),)*
                    };

                    assert!(!c.is_valid());
                    assert_eq!(clamped, expected);
                }

                println!("ok")
            }
        }
    );
}

pub mod gradient;
pub mod pixel;

#[cfg(feature = "named")]
pub mod named;

mod alpha;
mod rgb;
mod luma;
mod yxy;
mod xyz;
mod lab;
mod lch;
mod hsv;
mod hsl;

mod hues;

mod white_point;
mod rgb_variant;
mod tristimulus;

// macro_rules! make_color {
//     ($(
//         #[$variant_comment:meta]
//         $variant:ident $(and $($representations:ident),+ )* {$(
//             #[$ctor_comment:meta]
//             $ctor_name:ident $( <$( $ty_params:ident: $ty_param_traits:ident $( <$( $ty_inner_traits:ident ),*> )*),*> )* ($($ctor_field:ident : $ctor_ty:ty),*) [alpha: $alpha_ty:ty] => $ctor_original:ident;
//         )+}
//     )+) => (
//
//         ///Generic color with an alpha component. See the [`Colora` implementation in `Alpha`](struct.Alpha.html#Colora).
//         pub type Colora<T = f32> = Alpha<Color<T>, T>;
//
//         ///A generic color type.
//         ///
//         ///The `Color` may belong to any color space and it may change
//         ///depending on which operation is performed. That makes it immune to
//         ///the "without conversion" rule of the operations it supports. The
//         ///color spaces are selected as follows:
//         ///
//         /// * `Mix`: RGB for no particular reason, except that it's intuitive.
//         /// * `Shade`: CIE L*a*b* for its luminance component.
//         /// * `Hue` and `GetHue`: CIE L*C*h° for its hue component and how it preserves the apparent lightness.
//         /// * `Saturate`: CIE L*C*h° for its chromaticity component and how it preserves the apparent lightness.
//         ///
//         ///It's not recommended to use `Color` when full control is necessary,
//         ///but it can easily be converted to a fixed color space in those
//         ///cases.
//         // #[derive(Clone, Copy, Debug)]
//         pub enum Color<T:Float = f32> {
//             $(#[$variant_comment] $variant($variant<T>)),+
//         }
//
//         impl<T:Float> Color<T> {
//             $(
//                 $(
//                     #[$ctor_comment]
//                     pub fn $ctor_name$(<$($ty_params : $ty_param_traits$( <$( $ty_inner_traits ),*> )*),*>)*($($ctor_field: $ctor_ty),*) -> Color<T> {
//                         Color::$variant($variant::$ctor_original($($ctor_field),*))
//                     }
//                 )+
//             )+
//         }
//
//         ///<span id="Colora"></span>[`Colora`](type.Colora.html) implementations.
//         impl<T:Float> Alpha<Color<T>, T> {
//             $(
//                 $(
//                     #[$ctor_comment]
//                     pub fn $ctor_name$(<$($ty_params : $ty_param_traits$( <$( $ty_inner_traits ),*> )*),*>)*($($ctor_field: $ctor_ty,)* alpha: $alpha_ty) -> Colora<T> {
//                         Alpha::<$variant<T>, T>::$ctor_original($($ctor_field,)* alpha).into()
//                     }
//                 )+
//             )+
//         }
//
//         impl<T:Float> Mix for Color<T> {
//             type Scalar = T;
//
//             fn mix(&self, other: &Color<T>, factor: T) -> Color<T> {
//                 Rgb::from(*self).mix(&Rgb::from(*other), factor).into()
//             }
//         }
//
//         impl<T:Float> Shade for Color<T> {
//             type Scalar = T;
//
//             fn lighten(&self, amount: T) -> Color<T> {
//                 Lab::from(*self).lighten(amount).into()
//             }
//         }
//
//         impl<T:Float> GetHue for Color<T> {
//             type Hue = LabHue<T>;
//
//             fn get_hue(&self) -> Option<LabHue<T>> {
//                 Lch::from(*self).get_hue()
//             }
//         }
//
//         impl<T:Float> Hue for Color<T> {
//             fn with_hue(&self, hue: LabHue<T>) -> Color<T> {
//                 Lch::from(*self).with_hue(hue).into()
//             }
//
//             fn shift_hue(&self, amount: LabHue<T>) -> Color<T> {
//                 Lch::from(*self).shift_hue(amount).into()
//             }
//         }
//
//         impl<T:Float> Saturate for Color<T> {
//             type Scalar = T;
//
//             fn saturate(&self, factor: T) -> Color<T> {
//                 Lch::from(*self).saturate(factor).into()
//             }
//         }
//
//         $(
//             impl<T:Float> From<$variant<T>> for Color<T> {
//                 fn from(color: $variant<T>) -> Color<T> {
//                     Color::$variant(color)
//                 }
//             }
//
//             $($(
//                 impl<T:Float> From<$representations<T>> for Color<T> {
//                     fn from(color: $representations<T>) -> Color<T> {
//                         Color::$variant(color.into())
//                     }
//                 }
//             )+)*
//         )+
//
//         // alpha_from!(Color {$($variant),+});
//
//     )
// }

fn clamp<T:Float>(v: T, min: T, max: T) -> T {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

// make_color! {
//     ///Linear luminance.
//     Luma {
//         ///Linear luminance.
//         y(luma: T)[alpha: T] => new;
//
//         ///Linear luminance from an 8 bit value.
//         y_u8(luma: u8)[alpha: u8] => new_u8;
//     }
//
//     ///Linear RGB.
//     Rgb and Srgb, GammaRgb {
//         ///Linear RGB.
//         rgb(red: T, green: T, blue: T)[alpha: T] => new;
//
//         ///Linear RGB from 8 bit values.
//         rgb_u8(red: u8, green: u8, blue: u8)[alpha: u8] => new_u8;
//     }
//
//     ///CIE 1931 XYZ.
//     Xyz {
//         ///CIE XYZ.
//         xyz(x: T, y: T, z: T)[alpha: T] => new;
//     }
//
//     ///CIE 1931 Yxy.
//     Yxy {
//         ///CIE Yxy.
//         yxy(x: T, y: T, luma: T)[alpha: T] => new;
//     }
//
//     ///CIE L*a*b* (CIELAB).
//     Lab {
//         ///CIE L*a*b*.
//         lab(l: T, a: T, b: T)[alpha: T] => new;
//     }
//
//     ///CIE L*C*h°, a polar version of CIE L*a*b*.
//     Lch {
//         ///CIE L*C*h°.
//         lch(l: T, chroma: T, hue: LabHue<T>)[alpha: T] => new;
//     }
//
//     ///Linear HSV, a cylindrical version of RGB.
//     Hsv {
//         ///Linear HSV.
//         hsv(hue: RgbHue<T>, saturation: T, value: T)[alpha: T] => new;
//     }
//
//     ///Linear HSL, a cylindrical version of RGB.
//     Hsl {
//         ///Linear HSL.
//         hsl(hue: RgbHue<T>, saturation: T, lightness: T)[alpha: T] => new;
//     }
// }

///A trait for clamping and checking if colors are within their ranges.
pub trait Limited {
    ///Check if the color's components are within the expected ranges.
    fn is_valid(&self) -> bool;

    ///Return a new color where the components has been clamped to the nearest
    ///valid values.
    fn clamp(&self) -> Self;

    ///Clamp the color's components to the nearest valid values.
    fn clamp_self(&mut self);
}

///A trait for linear color interpolation.
///
///```
///use palette::{Rgb, Mix};
///
///let a = Rgb::new(0.0, 0.5, 1.0);
///let b = Rgb::new(1.0, 0.5, 0.0);
///
///assert_eq!(a.mix(&b, 0.0), a);
///assert_eq!(a.mix(&b, 0.5), Rgb::new(0.5, 0.5, 0.5));
///assert_eq!(a.mix(&b, 1.0), b);
///```
pub trait Mix {
    ///The type of the mixing factor.
    type Scalar: Float;

    ///Mix the color with an other color, by `factor`.
    ///
    ///`factor` sould be between `0.0` and `1.0`, where `0.0` will result in
    ///the same color as `self` and `1.0` will result in the same color as
    ///`other`.
    fn mix(&self, other: &Self, factor: Self::Scalar) -> Self;
}

///The `Shade` trait allows a color to be lightened or darkened.
///
///```
///use palette::{Rgb, Shade};
///
///let a = Rgb::new(0.4, 0.4, 0.4);
///let b = Rgb::new(0.6, 0.6, 0.6);
///
///assert_eq!(a.lighten(0.1), b.darken(0.1));
///```
pub trait Shade: Sized {
    ///The type of the lighten/darken amount.
    type Scalar: Float;

    ///Lighten the color by `amount`.
    fn lighten(&self, amount: Self::Scalar) -> Self;

    ///Darken the color by `amount`.
    fn darken(&self, amount: Self::Scalar) -> Self {
        self.lighten(-amount)
    }
}

///A trait for colors where a hue may be calculated.
///
///```
///use palette::{Rgb, GetHue};
///
///let red = Rgb::new(1.0f32, 0.0, 0.0);
///let green = Rgb::new(0.0f32, 1.0, 0.0);
///let blue = Rgb::new(0.0f32, 0.0, 1.0);
///let gray = Rgb::new(0.5f32, 0.5, 0.5);
///
///assert_eq!(red.get_hue(), Some(0.0.into()));
///assert_eq!(green.get_hue(), Some(120.0.into()));
///assert_eq!(blue.get_hue(), Some(240.0.into()));
///assert_eq!(gray.get_hue(), None);
///```
pub trait GetHue {
    ///The kind of hue unit this color space uses.
    ///
    ///The hue is most commonly calculated as an angle around a color circle
    ///and may not always be uniform between color spaces. It's therefore not
    ///recommended to take one type of hue and apply it to a color space that
    ///expects an other.
    type Hue;

    ///Calculate a hue if possible.
    ///
    ///Colors in the gray scale has no well defined hue and should preferably
    ///return `None`.
    fn get_hue(&self) -> Option<Self::Hue>;
}

///A trait for colors where the hue can be manipulated without conversion.
pub trait Hue: GetHue {
    ///Return a new copy of `self`, but with a specific hue.
    fn with_hue(&self, hue: Self::Hue) -> Self;

    ///Return a new copy of `self`, but with the hue shifted by `amount`.
    fn shift_hue(&self, amount: Self::Hue) -> Self;
}

///A trait for colors where the saturation (or chroma) can be manipulated
///without conversion.
///
///```
///use palette::{Hsv, Saturate};
///
///let a = Hsv::new(0.0.into(), 0.25, 1.0);
///let b = Hsv::new(0.0.into(), 1.0, 1.0);
///
///assert_eq!(a.saturate(1.0), b.desaturate(0.5));
///```
pub trait Saturate: Sized {
    ///The type of the (de)saturation factor.
    type Scalar: Float;

    ///Increase the saturation by `factor`.
    fn saturate(&self, factor: Self::Scalar) -> Self;

    ///Decrease the saturation by `factor`.
    fn desaturate(&self, factor: Self::Scalar) -> Self {
        self.saturate(-factor)
    }
}

///A convenience function to convert a constant number to Float Type
fn flt<T: num::Float, P: ToPrimitive>(prim: P) -> T {
    NumCast::from(prim).unwrap()
}
