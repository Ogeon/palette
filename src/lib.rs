pub use range::Range;
pub use rgb::Rgb;
pub use luma::Luma;
pub use xyz::Xyz;
pub use lab::Lab;
pub use lch::Lch;

pub use hues::{LabHue, RgbHue};

macro_rules! from_color {
    (to $to:ident from $($from:ident),+) => (
        impl From<Color> for $to {
            fn from(color: Color) -> $to {
                match color {
                    Color::$to(c) => c,
                    $(Color::$from(c) => c.into(),)+
                }
            }
        }
    )
}

pub mod range;

mod rgb;
mod luma;
mod xyz;
mod lab;
mod lch;

mod hues;

mod tristimulus;

macro_rules! make_color {
    ($(
        #[$variant_comment:meta]
        $variant:ident {$(
            #[$ctor_comment:meta]
            $ctor_name:ident ($($ctor_field:ident : $ctor_ty:ty),*) -> $ctor_proxy:ident;
        )+}
    )+) => (

        ///A generic color type.
        ///
        ///The `Color` may belong to any color space and it may change
        ///depending on which operation is performed.
        #[derive(Clone, Debug)]
        pub enum Color {
            $(#[$variant_comment] $variant($variant)),+
        }

        impl Color {
            $(
                $(
                    #[$ctor_comment]
                    pub fn $ctor_name($($ctor_field: $ctor_ty),*) -> Color {
                        Color::$variant($variant::$ctor_proxy($($ctor_field),*))
                    }
                )+
            )+
        }

        impl Mix for Color {
            fn mix(&self, other: &Color, factor: f32) -> Color {
                match (self, other) {
                    $((&Color::$variant(ref a), &Color::$variant(ref b)) => a.mix(b, factor).into(),)+
                    (a, b) => Rgb::from(a.clone()).mix(&Rgb::from(b.clone()), factor).into()
                }
            }
        }

        impl Shade for Color {
            fn lighten(&self, amount: f32) -> Color {
                Lab::from(self.clone()).lighten(amount).into()
            }
        }

        impl GetHue for Color {
            type Hue = LabHue;

            fn get_hue(&self) -> Option<LabHue> {
                Lch::from(self.clone()).get_hue()
            }
        }

        impl Hue for Color {
            fn with_hue(&self, hue: LabHue) -> Color {
                Lch::from(self.clone()).with_hue(hue).into()
            }

            fn shift_hue(&self, amount: LabHue) -> Color {
                Lch::from(self.clone()).shift_hue(amount).into()
            }
        }

        $(
            impl From<$variant> for Color {
                fn from(color: $variant) -> Color {
                    Color::$variant(color)
                }
            }
        )+
    )
}

fn clamp(v: f32, min: f32, max: f32) -> f32 {
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
        y(luma: f32) -> y;

        ///Linear luminance with transparency.
        ya(luma: f32, alpha: f32) -> ya;

        ///Linear luminance from an 8 bit value.
        y8(luma: u8) -> y8;

        ///Linear luminance and transparency from 8 bit values.
        ya8(luma: u8, alpha: u8) -> ya8;
    }

    ///Linear RGB.
    Rgb {
        ///Linear RGB.
        rgb(red: f32, green: f32, blue: f32) -> rgb;

        ///Linear RGB and transparency.
        rgba(red: f32, green: f32, blue: f32, alpha: f32) -> rgba;

        ///Linear RGB from 8 bit values.
        rgb8(red: u8, green: u8, blue: u8) -> rgb8;

        ///Linear RGB and transparency from 8 bit values.
        rgba8(red: u8, green: u8, blue: u8, alpha: u8) -> rgba8;

        ///Linear RGB from sRGB.
        srgb(red: f32, green: f32, blue: f32) -> srgb;

        ///Linear RGB from sRGB with transparency.
        srgba(red: f32, green: f32, blue: f32, alpha: f32) -> srgba;

        ///Linear RGB from 8 bit sRGB.
        srgb8(red: u8, green: u8, blue: u8) -> srgb8;

        ///Linear RGB from 8 bit sRGB with transparency.
        srgba8(red: u8, green: u8, blue: u8, alpha: u8) -> srgba8;
    }

    ///CIE 1931 XYZ.
    Xyz {
        ///CIE XYZ.
        xyz(x: f32, y: f32, z: f32) -> xyz;

        ///CIE XYZ and transparency.
        xyza(x: f32, y: f32, z: f32, alpha: f32) -> xyza;
    }

    ///CIE L*a*b* (CIELAB).
    Lab {
        ///CIE L*a*b*.
        lab(l: f32, a: f32, b: f32) -> lab;

        ///CIE L*a*b* and transparency.
        laba(l: f32, a: f32, b: f32, alpha: f32) -> laba;
    }

    ///CIE L*C*h°, a polar version of CIE L*a*b*.
    Lch {
        ///CIE L*C*h°.
        lch(l: f32, chroma: f32, hue: LabHue) -> lch;

        ///CIE L*C*h° and transparency.
        lcha(l: f32, chroma: f32, hue: LabHue, alpha: f32) -> lcha;
    }
}

///A trait for linear color interpolation.
///
///```
///use palette::{Rgb, Mix};
///
///let a = Rgb::rgb(0.0, 0.5, 1.0);
///let b = Rgb::rgb(1.0, 0.5, 0.0);
///
///assert_eq!(a.mix(&b, 0.0), a);
///assert_eq!(a.mix(&b, 0.5), Rgb::rgb(0.5, 0.5, 0.5));
///assert_eq!(a.mix(&b, 1.0), b);
///```
pub trait Mix {
    ///Mix the color with an other color, by `factor`.
    ///
    ///`factor` sould be between `0.0` and `1.0`, where `0.0` will result in
    ///the same color as `self` and `1.0` will result in the same color as
    ///`other`.
    fn mix(&self, other: &Self, factor: f32) -> Self;
}

///The `Shade` trait allows a color to be lightened or darkened.
///
///```
///use palette::{Rgb, Shade};
///
///let a = Rgb::rgb(0.4, 0.4, 0.4);
///let b = Rgb::rgb(0.6, 0.6, 0.6);
///
///assert_eq!(a.lighten(0.1), b.darken(0.1));
///```
pub trait Shade: Sized {
    ///Lighten the color by `amount`.
    fn lighten(&self, amount: f32) -> Self;

    ///Darken the color by `amount`.
    fn darken(&self, amount: f32) -> Self {
        self.lighten(-amount)
    }
}

///A trait for colors where a hue may be calculated.
///
///```
///use palette::{Rgb, GetHue};
///
///let red = Rgb::rgb(1.0, 0.0, 0.0);
///let green = Rgb::rgb(0.0, 1.0, 0.0);
///let blue = Rgb::rgb(0.0, 0.0, 1.0);
///let gray = Rgb::rgb(0.5, 0.5, 0.5);
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
