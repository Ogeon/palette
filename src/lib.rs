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
//!use palette::{Rgb, RgbaLinear};
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

#[macro_use]
extern crate approx;

extern crate num;

#[cfg(feature = "phf")]
extern crate phf;

use num::{Float, ToPrimitive, NumCast};

use approx::ApproxEq;

use blend::PreAlpha;

pub use gradient::Gradient;
pub use alpha::Alpha;
pub use blend::Blend;

pub use rgb_linear::{RgbLinear, RgbaLinear};
pub use rgb_gamma::{Rgb, Rgba};
pub use luma::{Luma, Lumaa};
pub use xyz::{Xyz, Xyza};
pub use lab::{Lab, Laba};
pub use lch::{Lch, Lcha};
pub use hsv::{Hsv, Hsva};
pub use hsl::{Hsl, Hsla};
pub use yxy::{Yxy, Yxya};
pub use hwb::{Hwb, Hwba};

pub use hues::{LabHue, RgbHue};
pub use convert::{FromColor, IntoColor};
pub use matrix::Mat3;

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
            use std::marker::PhantomData;
            use Limited;
            use white_point::D65;

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
                    let c = $ty::<D65, f64> {
                        $($limited: $limited.into(),)+
                        $($limited_min: $limited_min.into(),)*
                        $($unlimited: $unlimited.into(),)*
                        white_point: PhantomData,
                    };
                    let clamped = c.clamp();
                    let expected = $ty::<D65, f64> {
                        $($limited: $limited_from.into(),)+
                        $($limited_min: $limited_min_from.into(),)*
                        $($unlimited: $unlimited.into(),)*
                        white_point: PhantomData,
                    };

                    assert!(!c.is_valid());
                    assert_relative_eq!(clamped, expected);
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
                    let c = $ty::<D65, f64> {
                        $($limited: $limited.into(),)+
                        $($limited_min: $limited_min.into(),)*
                        $($unlimited: $unlimited.into(),)*
                        white_point: PhantomData,
                    };
                    let clamped = c.clamp();

                    assert!(c.is_valid());
                    assert_relative_eq!(clamped, c);
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
                    let c = $ty::<D65, f64> {
                        $($limited: $limited.into(),)+
                        $($limited_min: $limited_min.into(),)*
                        $($unlimited: $unlimited.into(),)*
                        white_point: PhantomData,
                    };
                    let clamped = c.clamp();
                    let expected = $ty::<D65, _> {
                        $($limited: $limited_to.into(),)+
                        $($limited_min: $limited_min.into(),)*
                        $($unlimited: $unlimited.into(),)*
                        white_point: PhantomData,
                    };

                    assert!(!c.is_valid());
                    assert_relative_eq!(clamped, expected);
                }

                println!("ok")
            }
        }
    );
}

pub mod gradient;
pub mod pixel;
pub mod blend;

#[cfg(feature = "named")]
pub mod named;

mod alpha;
mod rgb_linear;
mod rgb_gamma;
mod luma;
mod yxy;
mod xyz;
mod lab;
mod lch;
mod hsv;
mod hsl;
mod hwb;

mod hues;

mod convert;
mod equality;
pub mod chromatic_adaptation;
pub mod white_point;
mod matrix;
mod profile;

use white_point::{WhitePoint, D65};
use profile::{Primaries, SrgbProfile};

macro_rules! make_color {
    ($(
        #[$variant_comment:meta]
        ($var_label: ident, $variant: ident, $var_ty: ty) $(and $($representations: ty),+ )* {$(
            #[$ctor_comment:meta]
            $ctor_name:ident $( <$( $ty_params:ident: $ty_param_traits:ident $( <$( $ty_inner_traits:ident ),*> )*),*> )* ($($ctor_field:ident : $ctor_ty:ty),*) [alpha: $alpha_ty:ty] => $ctor_original:ident;
        )+}
    )+) => (

        ///Generic color with an alpha component. See the [`Colora` implementation in `Alpha`](struct.Alpha.html#Colora).
        pub type Colora<P = SrgbProfile, Wp = D65, T = f32> = Alpha<Color<P, Wp, T>, T>;

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
        #[derive(Debug)]
        pub enum Color<P = SrgbProfile, Wp = D65, T = f32>
            where T: Float,
                Wp: WhitePoint<T>,
                P: Primaries<Wp, T>,
        {
            $(#[$variant_comment] $var_label($var_ty)),+
        }

        impl<P, Wp, T> Copy for Color<P, Wp, T>
            where T: Float,
                Wp: WhitePoint<T>,
                P: Primaries<Wp, T>,
        {}

        impl<P, Wp, T> Clone for Color<P, Wp, T>
            where T: Float,
                Wp: WhitePoint<T>,
                P: Primaries<Wp, T>,
        {
            fn clone(&self) -> Color<P, Wp, T> { *self }
        }

        impl<T: Float> Color<SrgbProfile, D65, T> {
            $(
                $(
                    #[$ctor_comment]
                    pub fn $ctor_name$(<$($ty_params : $ty_param_traits$( <$( $ty_inner_traits ),*> )*),*>)*($($ctor_field: $ctor_ty),*) -> Color<SrgbProfile, D65, T> {
                        Color::$var_label($variant::$ctor_original($($ctor_field),*))
                    }
                )+
            )+
        }

        ///<span id="Colora"></span>[`Colora`](type.Colora.html) implementations.
        impl<T: Float> Alpha<Color<SrgbProfile, D65, T>, T> {
            $(
                $(
                    #[$ctor_comment]
                    pub fn $ctor_name$(<$($ty_params : $ty_param_traits$( <$( $ty_inner_traits ),*> )*),*>)*($($ctor_field: $ctor_ty,)* alpha: $alpha_ty) -> Colora<SrgbProfile, D65, T> {
                        Alpha::<$var_ty, T>::$ctor_original($($ctor_field,)* alpha).into()
                    }
                )+
            )+
        }

        impl<P, Wp, T> Mix for Color<P, Wp, T>
            where T: Float,
                Wp: WhitePoint<T>,
                P: Primaries<Wp, T>,
        {
            type Scalar = T;

            fn mix(&self, other: &Color<P, Wp, T>, factor: T) -> Color<P, Wp, T> {
                RgbLinear::from(*self).mix(&RgbLinear::from(*other), factor).into()
            }
        }

        impl<P, Wp, T> Shade for Color<P, Wp, T>
            where T: Float,
                Wp: WhitePoint<T>,
                P: Primaries<Wp, T>,
        {
            type Scalar = T;

            fn lighten(&self, amount: T) -> Color<P, Wp, T> {
                Lab::from(*self).lighten(amount).into()
            }
        }

        impl<P, Wp, T> GetHue for Color<P, Wp, T>
            where T: Float,
                Wp: WhitePoint<T>,
                P: Primaries<Wp, T>,
        {
            type Hue = LabHue<T>;

            fn get_hue(&self) -> Option<LabHue<T>> {
                Lch::from(*self).get_hue()
            }
        }

        impl<P, Wp, T> Hue for Color<P, Wp, T>
            where T: Float,
                Wp: WhitePoint<T>,
                P: Primaries<Wp, T>,
        {
            fn with_hue(&self, hue: LabHue<T>) -> Color<P, Wp, T> {
                Lch::from(*self).with_hue(hue).into()
            }

            fn shift_hue(&self, amount: LabHue<T>) -> Color<P, Wp, T> {
                Lch::from(*self).shift_hue(amount).into()
            }
        }

        impl<P, Wp, T> Saturate for Color<P, Wp, T>
            where T: Float,
                Wp: WhitePoint<T>,
                P: Primaries<Wp, T>,
        {
            type Scalar = T;

            fn saturate(&self, factor: T) -> Color<P, Wp, T> {
                Lch::from(*self).saturate(factor).into()
            }
        }

        impl<P, Wp, T> Blend for Color<P, Wp, T>
            where T: Float,
                Wp: WhitePoint<T>,
                P: Primaries<Wp, T>,
        {
            type Color = RgbLinear<P, Wp, T>;

            fn into_premultiplied(self) -> PreAlpha<RgbLinear<P, Wp, T>, T> {
                RgbaLinear::from(self).into()
            }

            fn from_premultiplied(color: PreAlpha<RgbLinear<P, Wp, T>, T>) -> Self {
                RgbaLinear::from(color).into()
            }
        }

        impl<P, Wp, T> ApproxEq for Color<P, Wp, T>
            where T: Float + ApproxEq,
                T::Epsilon: Copy + Float,
                Wp: WhitePoint<T>,
                P: Primaries<Wp, T>,
        {
            type Epsilon = T::Epsilon;

            fn default_epsilon() -> Self::Epsilon {
                T::default_epsilon()
            }

            fn default_max_relative() -> Self::Epsilon {
                T::default_max_relative()
            }

            fn default_max_ulps() -> u32 {
                T::default_max_ulps()
            }

            fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
                match (*self, *other) {
                    $((Color::$var_label(ref s), Color::$var_label(ref o)) => s.relative_eq(o, epsilon, max_relative),)+
                    _ => false
                }
            }

            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool{
                match (*self, *other) {
                    $((Color::$var_label(ref s), Color::$var_label(ref o)) => s.ulps_eq(o, epsilon, max_ulps),)+
                    _ => false
                }
            }
        }

        $(
            impl<P, Wp, T> From<$var_ty> for Color<P, Wp, T>
                where T: Float,
                    Wp: WhitePoint<T>,
                    P: Primaries<Wp, T>,
            {
                fn from(color: $var_ty) -> Color<P, Wp, T> {
                    Color::$var_label(color)
                }
            }

            impl<P, Wp, T> From<Alpha<$var_ty, T>> for Color<P, Wp, T>
                where T: Float,
                    Wp: WhitePoint<T>,
                    P: Primaries<Wp, T>,
            {
                fn from(color: Alpha<$var_ty,T>) -> Color<P, Wp, T> {
                    Color::$var_label(color.color)
                }
            }

            impl<P, Wp, T> From<Alpha<$var_ty, T>> for Alpha<Color<P, Wp, T>, T>
                where T: Float,
                    Wp: WhitePoint<T>,
                    P: Primaries<Wp, T>,
            {
                fn from(color: Alpha<$var_ty,T>) -> Alpha<Color<P, Wp, T>, T> {
                    Alpha {
                        color: Color::$var_label(color.color),
                        alpha: color.alpha,
                    }
                }
            }

            $($(
                impl<P, Wp, T> From<$representations> for Color<P, Wp, T>
                    where T: Float,
                        Wp: WhitePoint<T>,
                        P: Primaries<Wp, T>,
                {
                    fn from(color: $representations) -> Color<P, Wp, T> {
                        Color::$var_label(color.into())
                    }
                }
            )+)*
        )+
    )
}





fn clamp<T: Float>(v: T, min: T, max: T) -> T {
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
    (Luma, Luma, Luma<Wp, T>) {
        ///Linear luminance.
        y(luma: T)[alpha: T] => new;

        ///Linear luminance from an 8 bit value.
        y_u8(luma: u8)[alpha: u8] => new_u8;
    }

    ///Linear RGB.
    (Rgb, RgbLinear, RgbLinear<P, Wp, T> ) {
        ///Linear RGB.
        rgb(red: T, green: T, blue: T)[alpha: T] => new;

        ///Linear RGB from 8 bit values.
        rgb_u8(red: u8, green: u8, blue: u8)[alpha: u8] => new_u8;
    }

    ///CIE 1931 XYZ.
    (Xyz, Xyz, Xyz<Wp, T>) {
        ///CIE XYZ.
        xyz(x: T, y: T, z: T)[alpha: T] => new;
    }

    ///CIE 1931 Yxy.
    (Yxy, Yxy, Yxy<Wp, T>) {
        ///CIE Yxy.
        yxy(x: T, y: T, luma: T)[alpha: T] => new;
    }

    ///CIE L*a*b* (CIELAB).
    (Lab, Lab, Lab<Wp, T>) {
        ///CIE L*a*b*.
        lab(l: T, a: T, b: T)[alpha: T] => new;
    }

    ///CIE L*C*h°, a polar version of CIE L*a*b*.
    (Lch, Lch, Lch<Wp, T>) {
        ///CIE L*C*h°.
        lch(l: T, chroma: T, hue: LabHue<T>)[alpha: T] => new;
    }

    ///Linear HSV, a cylindrical version of RGB.
    (Hsv, Hsv, Hsv<Wp, T>) {
        ///Linear HSV.
        hsv(hue: RgbHue<T>, saturation: T, value: T)[alpha: T] => new;
    }

    ///Linear HSL, a cylindrical version of RGB.
    (Hsl, Hsl, Hsl<Wp, T>) {
        ///Linear HSL.
        hsl(hue: RgbHue<T>, saturation: T, lightness: T)[alpha: T] => new;
    }

    ///Linear HWB, an intuitive cylindrical version of RGB.
    (Hwb, Hwb, Hwb<Wp, T>) {
        ///Linear HWB.
        hwb(hue: RgbHue<T>, whiteness: T, balckness: T)[alpha: T] => new;
    }
}

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

///Perform a unary or binary operation on each component of a color.
pub trait ComponentWise {
    ///The scalar type for color components.
    type Scalar: Float;

    ///Perform a binary operation on this and an other color.
    fn component_wise<F: FnMut(Self::Scalar, Self::Scalar) -> Self::Scalar>(&self, other: &Self, f: F) -> Self;

    ///Perform a unary operation on this color.
    fn component_wise_self<F: FnMut(Self::Scalar) -> Self::Scalar>(&self, f: F) -> Self;
}

///A convenience function to convert a constant number to Float Type
fn flt<T: num::Float, P: ToPrimitive>(prim: P) -> T {
    NumCast::from(prim).unwrap()
}
