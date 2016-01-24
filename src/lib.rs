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


#![doc(html_root_url = "http://ogeon.github.io/docs/palette/master/")]

#![cfg_attr(feature = "strict", deny(missing_docs))]
#![cfg_attr(feature = "strict", deny(warnings))]

#[cfg(test)]
#[macro_use]
extern crate approx;
extern crate num;

use num::traits::Float;

use pixel::{RgbPixel, Srgb};

pub use gradient::Gradient;
pub use rgb::Rgb;
pub use luma::Luma;
pub use xyz::Xyz;
pub use lab::Lab;
pub use lch::Lch;
pub use hsv::Hsv;
pub use hsl::Hsl;

pub use hues::{LabHue, RgbHue};

macro_rules! from_color {
    (to $to:ident from $($from:ident),+) => (
        impl<T:Float> From<Color<T>> for $to<T> {
            fn from(color: Color<T>) -> $to<T> {
                match color {
                    Color::$to(c) => c,
                    $(Color::$from(c) => c.into(),)+
                }
            }
        }
    )
}

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
        assert_relative_eq!($a.alpha, $b.alpha, epsilon = 0.0001);
    })
}

pub mod gradient;
pub mod pixel;

mod rgb;
mod luma;
mod xyz;
mod lab;
mod lch;
mod hsv;
mod hsl;

mod hues;

mod tristimulus;

macro_rules! make_color {
    ($(
        #[$variant_comment:meta]
        $variant:ident $(and $($representations:ident),+ )* {$(
            #[$ctor_comment:meta]
            $ctor_name:ident $( <$( $ty_params:ident: $ty_param_traits:ident $( <$( $ty_inner_traits:ident ),*> )*),*> )* ($($ctor_field:ident : $ctor_ty:ty),*);
        )+}
    )+) => (

        ///A generic color type.
        ///
        ///The `Color` may belong to any color space and it may change
        ///depending on which operation is performed. That makes it immune to
        ///the "without conversion" rule of the operations it supports. The
        ///color spaces are selected as follows:
        ///
        /// * `Mix`: RGB for no particular reason, except that it's intuitive.
        /// * `Shade`: CIE L*a*b* for its luminance component.
        /// * `Hue` and `GetHue`: CIE L*C*h° for its hue component and how it preserves the apparent lightness.
        /// * `Saturate`: CIE L*C*h° for its chromaticity component and how it preserves the apparent lightness.
        ///
        ///It's not recommended to use `Color` when full control is necessary,
        ///but it can easily be converted to a fixed color space in those
        ///cases.
        #[derive(Clone, Copy, Debug)]
        pub enum Color<T:Float = f32> {
            $(#[$variant_comment] $variant($variant<T>)),+
        }

        $(
            impl<T:Float> Color<T> {
                $(
                    #[$ctor_comment]
                    pub fn $ctor_name$(<$($ty_params : $ty_param_traits$( <$( $ty_inner_traits ),*> )*),*>)*($($ctor_field: $ctor_ty),*) -> Color<T> {
                        Color::$variant($variant::$ctor_name($($ctor_field),*))
                    }
                )+
            }
        )+

        impl<T:Float> Mix for Color<T> {
            type Scalar = T;

            fn mix(&self, other: &Color<T>, factor: T) -> Color<T> {
                Rgb::from(*self).mix(&Rgb::from(*other), factor).into()
            }
        }

        impl<T:Float> Shade for Color<T> {
            type Scalar = T;

            fn lighten(&self, amount: T) -> Color<T> {
                Lab::from(*self).lighten(amount).into()
            }
        }

        impl<T:Float> GetHue for Color<T> {
            type Hue = LabHue<T>;

            fn get_hue(&self) -> Option<LabHue<T>> {
                Lch::from(*self).get_hue()
            }
        }

        impl<T:Float> Hue for Color<T> {
            fn with_hue(&self, hue: LabHue<T>) -> Color<T> {
                Lch::from(*self).with_hue(hue).into()
            }

            fn shift_hue(&self, amount: LabHue<T>) -> Color<T> {
                Lch::from(*self).shift_hue(amount).into()
            }
        }

        impl<T:Float> Saturate for Color<T> {
            type Scalar = T;

            fn saturate(&self, factor: T) -> Color<T> {
                Lch::from(*self).saturate(factor).into()
            }
        }

        $(
            impl<T:Float> From<$variant<T>> for Color<T> {
                fn from(color: $variant<T>) -> Color<T> {
                    Color::$variant(color)
                }
            }

            $($(
                impl<T:Float> From<$representations<T>> for Color<T> {
                    fn from(color: $representations<T>) -> Color<T> {
                        Color::$variant(color.into())
                    }
                }
            )+)*
        )+
    )
}

fn clamp<T:Float>(v: T, min: T, max: T) -> T {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

make_color! {
    ///Linear luminance.
    Luma {
        ///Linear luminance.
        y(luma: T);

        ///Linear luminance with transparency.
        ya(luma: T, alpha: T);

        ///Linear luminance from an 8 bit value.
        y8(luma: u8);

        ///Linear luminance and transparency from 8 bit values.
        ya8(luma: u8, alpha: u8);
    }

    ///Linear RGB.
    Rgb and Srgb {
        ///Linear RGB.
        linear_rgb(red: T, green: T, blue: T);

        ///Linear RGB and transparency.
        linear_rgba(red: T, green: T, blue: T, alpha: T);

        ///Linear RGB from 8 bit values.
        linear_rgb8(red: u8, green: u8, blue: u8);

        ///Linear RGB and transparency from 8 bit values.
        linear_rgba8(red: u8, green: u8, blue: u8, alpha: u8);

        ///Linear RGB from a linear pixel value.
        linear_pixel<P: RgbPixel<T> >(pixel: &P);

        ///Linear RGB from gamma corrected RGB.
        gamma_rgb(red: T, green: T, blue: T, gamma: T);

        ///Linear RGB from gamma corrected RGB with transparency.
        gamma_rgba(red: T, green: T, blue: T, alpha: T, gamma: T);

        ///Linear RGB from 8 bit gamma corrected RGB.
        gamma_rgb8(red: u8, green: u8, blue: u8, gamma: T);

        ///Linear RGB from 8 bit gamma corrected RGB with transparency.
        gamma_rgba8(red: u8, green: u8, blue: u8, alpha: u8, gamma: T);

        ///Linear RGB from a gamma corrected pixel value.
        gamma_pixel<P: RgbPixel<T> >(pixel: &P, gamma: T);
    }

    ///CIE 1931 XYZ.
    Xyz {
        ///CIE XYZ.
        xyz(x: T, y: T, z: T);

        ///CIE XYZ and transparency.
        xyza(x: T, y: T, z: T, alpha: T);
    }

    ///CIE L*a*b* (CIELAB).
    Lab {
        ///CIE L*a*b*.
        lab(l: T, a: T, b: T);

        ///CIE L*a*b* and transparency.
        laba(l: T, a: T, b: T, alpha: T);
    }

    ///CIE L*C*h°, a polar version of CIE L*a*b*.
    Lch {
        ///CIE L*C*h°.
        lch(l: T, chroma: T, hue: LabHue<T>);

        ///CIE L*C*h° and transparency.
        lcha(l: T, chroma: T, hue: LabHue<T>, alpha: T);
    }

    ///Linear HSV, a cylindrical version of RGB.
    Hsv {
        ///Linear HSV.
        hsv(hue: RgbHue<T>, saturation: T, value: T);

        ///Linear HSV and transparency.
        hsva(hue: RgbHue<T>, saturation: T, value: T, alpha: T);
    }

    ///Linear HSL, a cylindrical version of RGB.
    Hsl {
        ///Linear HSL.
        hsl(hue: RgbHue<T>, saturation: T, lightness: T);

        ///Linear HSL and transparency.
        hsla(hue: RgbHue<T>, saturation: T, lightness: T, alpha: T);
    }
}

///Common functionality for color spaces.
pub trait ColorSpace {
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
///let a = Rgb::linear_rgb(0.0, 0.5, 1.0);
///let b = Rgb::linear_rgb(1.0, 0.5, 0.0);
///
///assert_eq!(a.mix(&b, 0.0), a);
///assert_eq!(a.mix(&b, 0.5), Rgb::linear_rgb(0.5, 0.5, 0.5));
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
///let a = Rgb::linear_rgb(0.4, 0.4, 0.4);
///let b = Rgb::linear_rgb(0.6, 0.6, 0.6);
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
///let red = Rgb::linear_rgb(1.0_f32, 0.0, 0.0);
///let green = Rgb::linear_rgb(0.0_f32, 1.0, 0.0);
///let blue = Rgb::linear_rgb(0.0_f32, 0.0, 1.0);
///let gray = Rgb::linear_rgb(0.5_f32, 0.5, 0.5);
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
///let a = Hsv::hsv(0.0.into(), 0.25, 1.0);
///let b = Hsv::hsv(0.0.into(), 1.0, 1.0);
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
