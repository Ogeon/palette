use num::Float;

use {Alpha, RgbLinear, Luma, Xyz, Yxy, Lab, Lch, Hsv, Hwb, Hsl, Color};
use white_point::{WhitePoint, D65};
use profile::{Primaries, SrgbProfile};

///FromColor provides conversion between the colors.
///
///It requires from_xyz and derives conversion to other colors as a default from this.
///These defaults must be overridden when direct conversion exists between colors.
///For example, Luma has direct conversion to Rgb. So from_rgb conversion for Luma and
///from_luma for Rgb is implemented directly. The from for the same color must override
///the default. For example, from_rgb for Rgb will convert via Xyz which needs to be overridden
///with self to avoid the unnecessary converison.
pub trait FromColor<Wp = D65, T = f32>: Sized
    where T: Float,
        Wp: WhitePoint<T>
{
    ///Convert from XYZ color space
    fn from_xyz(Xyz<Wp, T>) -> Self;

    ///Convert from Yxy color space
    fn from_yxy(inp: Yxy<Wp, T>) -> Self {
        Self::from_xyz(inp.into_xyz())
    }

    ///Convert from L*a*b* color space
    fn from_lab(inp: Lab<Wp, T>) -> Self {
        Self::from_xyz(inp.into_xyz())
    }

    ///Convert from L*C*h° color space
    fn from_lch(inp: Lch<Wp, T>) -> Self {
        Self::from_lab(inp.into_lab())
    }

    ///Convert from RGB color space
    fn from_rgb<C: Primaries<Wp, T>>(inp: RgbLinear<C, Wp, T>) -> Self {
        Self::from_xyz(inp.into_xyz())
    }

    ///Convert from HSL color space
    fn from_hsl(inp: Hsl<Wp, T>) -> Self {
        Self::from_rgb::<SrgbProfile>(inp.into_rgb())
    }

    ///Convert from HSV color space
    fn from_hsv(inp: Hsv<Wp, T>) -> Self {
        Self::from_rgb::<SrgbProfile>(inp.into_rgb())
    }

    ///Convert from HWB color space
    fn from_hwb(inp: Hwb<Wp, T>) -> Self {
        Self::from_hsv(inp.into_hsv())
    }

    ///Convert from Luma
    fn from_luma(inp: Luma<Wp, T>) -> Self {
        Self::from_xyz(inp.into_xyz())
    }

}


///IntoColor provides conversion between the colors.
///
///It requires into into_xyz and derives conversion to other colors as a default from this.
///These defaults must be overridden when direct conversion exists between colors.
pub trait IntoColor<Wp = D65, T = f32>: Sized
    where T: Float,
     Wp: WhitePoint<T>
{

    ///Convert into XYZ space
    fn into_xyz(self) -> Xyz<Wp, T>;

    ///Convert into Yxy color space
    fn into_yxy(self) -> Yxy<Wp, T> {
        Yxy::from_xyz(self.into_xyz())
    }

    ///Convert into L*a*b* color space
    fn into_lab(self) -> Lab<Wp, T> {
        Lab::from_xyz(self.into_xyz())
    }

    ///Convert into L*C*h° color space
    fn into_lch(self) -> Lch<Wp, T> {
        Lch::from_lab(self.into_lab())
    }

    ///Convert into RGB color space.
    fn into_rgb<C: Primaries<Wp, T>>(self) -> RgbLinear<C, Wp, T> {
        RgbLinear::from_xyz(self.into_xyz())
    }

    ///Convert into HSL color space
    fn into_hsl(self) -> Hsl<Wp, T> {
        Hsl::from_rgb::<SrgbProfile>(self.into_rgb())
    }

    ///Convert into HSV color space
    fn into_hsv(self) -> Hsv<Wp, T> {
        Hsv::from_rgb::<SrgbProfile>(self.into_rgb())
    }

    ///Convert into HWB color space
    fn into_hwb(self) -> Hwb<Wp, T> {
        Hwb::from_hsv(self.into_hsv())
    }

    ///Convert into Luma
    fn into_luma(self) -> Luma<Wp, T> {
        Luma::from_xyz(self.into_xyz())
    }

}

macro_rules! impl_into_color {
    ($self_ty:ident, $from_fn: ident) => {
        impl<Wp, T> IntoColor<Wp, T> for $self_ty<Wp, T>
            where T: Float,
             Wp: WhitePoint<T>
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

            fn into_rgb<C: Primaries<Wp, T>>(self) -> RgbLinear<C, Wp, T> {
                RgbLinear::$from_fn(self)
            }

            fn into_hsl(self) -> Hsl<Wp, T> {
                Hsl::$from_fn(self)
            }

            fn into_hsv(self) -> Hsv<Wp, T> {
                Hsv::$from_fn(self)
            }

            fn into_luma(self) -> Luma<Wp, T> {
                Luma::$from_fn(self)
            }

        }

    }
}
macro_rules! impl_into_color_primaries {
    ($self_ty:ident, $from_fn: ident) => {
        impl<P, Wp, T> IntoColor<Wp, T> for $self_ty<P, Wp, T>
            where T: Float,
             Wp: WhitePoint<T>,
             P: Primaries<Wp, T>,
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

            fn into_rgb<C: Primaries<Wp, T>>(self) -> RgbLinear<C, Wp, T> {
                RgbLinear::$from_fn(self)
            }

            fn into_hsl(self) -> Hsl<Wp, T> {
                Hsl::$from_fn(self)
            }

            fn into_hsv(self) -> Hsv<Wp, T> {
                Hsv::$from_fn(self)
            }

            fn into_luma(self) -> Luma<Wp, T> {
                Luma::$from_fn(self)
            }

        }

    }
}


macro_rules! impl_from_trait {
    (($self_ty: ident, $into_fn: ident) => {$($other: ident),+}) => (
        impl<Wp, T> From<Alpha<$self_ty<Wp, T>, T>> for $self_ty<Wp, T>
            where T: Float,
                Wp: WhitePoint<T>
        {
            fn from(color: Alpha<$self_ty<Wp, T>, T>) -> $self_ty<Wp, T> {
                color.color
            }
        }

        impl<P, Wp, T> From<Color<P, Wp, T>> for $self_ty<Wp, T>
            where T: Float,
                Wp: WhitePoint<T>,
                P: Primaries<Wp, T>
        {
            fn from(color: Color<P, Wp, T>) -> $self_ty<Wp, T> {
                match color {
                    Color::Luma(c) => c.$into_fn(),
                    Color::Rgb(c) => c.$into_fn(),
                    Color::Xyz(c) => c.$into_fn(),
                    Color::Yxy(c) => c.$into_fn(),
                    Color::Lab(c) => c.$into_fn(),
                    Color::Lch(c) => c.$into_fn(),
                    Color::Hsv(c) => c.$into_fn(),
                    Color::Hsl(c) => c.$into_fn(),
                    Color::Hwb(c) => c.$into_fn(),
                }
            }
        }

        impl<P, Wp, T> From<Color<P, Wp, T>> for Alpha<$self_ty<Wp, T>,T>
            where T: Float,
                Wp: WhitePoint<T>,
                P: Primaries<Wp, T>
        {
            fn from(color: Color<P, Wp, T>) -> Alpha<$self_ty<Wp, T>,T> {
                Alpha {
                    color: color.into(),
                    alpha: T::one(),
                }
            }
        }

        impl<P, Wp, T> From<Alpha<Color<P, Wp, T>, T>> for Alpha<$self_ty<Wp, T>,T>
            where T: Float,
                Wp: WhitePoint<T>,
                P: Primaries<Wp, T>
        {
            fn from(color: Alpha<Color<P, Wp, T>, T>) -> Alpha<$self_ty<Wp, T>,T> {
                Alpha {
                    color: color.color.into(),
                    alpha: color.alpha,
                }
            }
        }

        $(
            impl<Wp, T> From<$other<Wp, T>> for $self_ty<Wp, T>
                where T: Float,
                    Wp: WhitePoint<T>
            {
                fn from(other: $other<Wp, T>) -> $self_ty<Wp, T> {
                    other.$into_fn()
                }
            }

            impl<Wp, T> From<Alpha<$other<Wp, T>, T>> for Alpha<$self_ty<Wp, T>, T>
                where T: Float,
                    Wp: WhitePoint<T>
            {
                fn from(other: Alpha<$other<Wp, T>, T>) -> Alpha<$self_ty<Wp, T>, T> {
                    Alpha {
                        color: other.color.$into_fn(),
                        alpha: other.alpha,
                    }
                }
            }

            impl<Wp, T> From<$other<Wp, T>> for Alpha<$self_ty<Wp, T>, T>
                where T: Float,
                    Wp: WhitePoint<T>
            {
                fn from(color: $other<Wp, T>) -> Alpha<$self_ty<Wp, T>, T> {
                    Alpha {
                        color: color.$into_fn(),
                        alpha: T::one(),
                    }
                }
            }

            impl<Wp, T> From<Alpha<$other<Wp, T>, T>> for $self_ty<Wp, T>
                where T: Float,
                    Wp: WhitePoint<T>
            {
                fn from(other: Alpha<$other<Wp, T>, T>) -> $self_ty<Wp, T> {
                    other.color.$into_fn()
                }
            }

        )+
    )
}

macro_rules! impl_from_trait_other_primary {
    (($self_ty: ident, $into_fn: ident) => {$($other: ident),+}) => (

        $(
            impl<P, Wp, T> From<$other<P, Wp, T>> for $self_ty<Wp, T>
                where T: Float,
                    Wp: WhitePoint<T>,
                    P: Primaries<Wp, T>,
            {
                fn from(other: $other<P, Wp, T>) -> $self_ty<Wp, T> {
                    other.$into_fn()
                }
            }

            impl<P, Wp, T> From<Alpha<$other<P, Wp, T>, T>> for Alpha<$self_ty<Wp, T>, T>
                where T: Float,
                    Wp: WhitePoint<T>,
                    P: Primaries<Wp, T>,
            {
                fn from(other: Alpha<$other<P, Wp, T>, T>) -> Alpha<$self_ty<Wp, T>, T> {
                    Alpha {
                        color: other.color.$into_fn(),
                        alpha: other.alpha,
                    }
                }
            }

            impl<P, Wp, T> From<$other<P, Wp, T>> for Alpha<$self_ty<Wp, T>, T>
                where T: Float,
                    Wp: WhitePoint<T>,
                    P: Primaries<Wp, T>,
            {
                fn from(color: $other<P, Wp, T>) -> Alpha<$self_ty<Wp, T>, T> {
                    Alpha {
                        color: color.$into_fn(),
                        alpha: T::one(),
                    }
                }
            }

            impl<P, Wp, T> From<Alpha<$other<P, Wp, T>, T>> for $self_ty<Wp, T>
                where T: Float,
                    Wp: WhitePoint<T>,
                    P: Primaries<Wp, T>,
            {
                fn from(other: Alpha<$other<P, Wp, T>, T>) -> $self_ty<Wp, T> {
                    other.color.$into_fn()
                }
            }

        )+
    )
}

macro_rules! impl_from_trait_self_primary {
    (($self_ty: ident, $into_fn: ident) => {$($other: ident),+}) => (
        impl<C, Wp, T> From<Alpha<$self_ty<C, Wp, T>, T>> for $self_ty<C, Wp, T>
            where T: Float,
                Wp: WhitePoint<T>,
                C: Primaries<Wp, T>,
        {
            fn from(color: Alpha<$self_ty<C, Wp, T>, T>) -> $self_ty<C, Wp, T> {
                color.color
            }
        }

        impl<P, Wp, T> From<Color<P, Wp, T>> for $self_ty<P, Wp, T>
            where T: Float,
                Wp: WhitePoint<T>,
                P: Primaries<Wp, T>,
        {
            fn from(color: Color<P, Wp, T>) -> $self_ty<P, Wp, T> {
                match color {
                    Color::Luma(c) => c.$into_fn(),
                    Color::Rgb(c) => c.$into_fn(),
                    Color::Xyz(c) => c.$into_fn(),
                    Color::Yxy(c) => c.$into_fn(),
                    Color::Lab(c) => c.$into_fn(),
                    Color::Lch(c) => c.$into_fn(),
                    Color::Hsv(c) => c.$into_fn(),
                    Color::Hsl(c) => c.$into_fn(),
                    Color::Hwb(c) => c.$into_fn(),
                }
            }
        }

        impl<P, Wp, T> From<Color<P, Wp, T>> for Alpha<$self_ty<P, Wp, T>,T>
            where T: Float,
                Wp: WhitePoint<T>,
                P: Primaries<Wp, T>,
        {
            fn from(color: Color<P, Wp, T>) -> Alpha<$self_ty<P, Wp, T>,T> {
                Alpha {
                    color: color.into(),
                    alpha: T::one(),
                }
            }
        }

        impl<P, Wp, T> From<Alpha<Color<P, Wp, T>, T>> for Alpha<$self_ty<P, Wp, T>,T>
            where T: Float,
                Wp: WhitePoint<T>,
                P: Primaries<Wp, T>
        {
            fn from(color: Alpha<Color<P, Wp, T>, T>) -> Alpha<$self_ty<P, Wp, T>,T> {
                Alpha {
                    color: color.color.into(),
                    alpha: color.alpha,
                }
            }
        }

        $(
            impl<P, Wp, T> From<$other<Wp, T>> for $self_ty<P, Wp, T>
                where T: Float,
                    Wp: WhitePoint<T>,
                    P: Primaries<Wp, T>
            {
                fn from(other: $other<Wp, T>) -> $self_ty<P, Wp, T> {
                    other.$into_fn()
                }
            }

            impl<P, Wp, T> From<Alpha<$other<Wp, T>, T>> for Alpha<$self_ty<P, Wp, T>, T>
                where T: Float,
                    Wp: WhitePoint<T>,
                    P: Primaries<Wp, T>
            {
                fn from(other: Alpha<$other<Wp, T>, T>) -> Alpha<$self_ty<P, Wp, T>, T> {
                    Alpha {
                        color: other.color.$into_fn(),
                        alpha: other.alpha,
                    }
                }
            }

            impl<P, Wp, T> From<$other<Wp, T>> for Alpha<$self_ty<P, Wp, T>, T>
                where T: Float,
                    Wp: WhitePoint<T>,
                    P: Primaries<Wp, T>
            {
                fn from(color: $other<Wp, T>) -> Alpha<$self_ty<P, Wp, T>, T> {
                    Alpha {
                        color: color.$into_fn(),
                        alpha: T::one(),
                    }
                }
            }

            impl<P, Wp, T> From<Alpha<$other<Wp, T>, T>> for $self_ty<P, Wp, T>
                where T: Float,
                    Wp: WhitePoint<T>,
                    P: Primaries<Wp, T>,
            {
                fn from(other: Alpha<$other<Wp, T>, T>) -> $self_ty<P, Wp, T> {
                    other.color.$into_fn()
                }
            }

        )+
    )
}

impl_into_color!(Xyz, from_xyz);
impl_into_color!(Yxy, from_yxy);
impl_into_color!(Lab, from_lab);
impl_into_color!(Lch, from_lch);
impl_into_color!(Hsl, from_hsl);
impl_into_color!(Hsv, from_hsv);
impl_into_color!(Hwb, from_hwb);
impl_into_color!(Luma ,from_luma);

impl_into_color_primaries!(RgbLinear, from_rgb);


impl_from_trait!((Xyz, into_xyz) => {Yxy, Lab, Lch, Hsl, Hsv, Hwb, Luma});
impl_from_trait_other_primary!((Xyz, into_xyz) => { RgbLinear});

impl_from_trait!((Yxy, into_yxy) => {Xyz, Lab, Lch, Hsl, Hsv, Hwb, Luma});
impl_from_trait_other_primary!((Yxy, into_yxy) => { RgbLinear});

impl_from_trait!((Lab, into_lab) => {Xyz, Yxy, Lch, Hsl, Hsv, Hwb, Luma});
impl_from_trait_other_primary!((Lab, into_lab) => { RgbLinear});

impl_from_trait!((Lch, into_lch) => {Xyz, Yxy, Lab, Hsl, Hsv, Hwb, Luma});
impl_from_trait_other_primary!((Lch, into_lch) => { RgbLinear});

impl_from_trait!((Hsl, into_hsl) => {Xyz, Yxy, Lab, Lch, Hsv, Hwb, Luma});
impl_from_trait_other_primary!((Hsl, into_hsl) => { RgbLinear});

impl_from_trait!((Hsv, into_hsv) => {Xyz, Yxy, Lab, Lch, Hsl, Hwb, Luma});
impl_from_trait_other_primary!((Hsv, into_hsv) => { RgbLinear});

impl_from_trait!((Hwb, into_hwb) => {Xyz, Yxy, Lab, Lch, Hsl, Hsv, Luma});
impl_from_trait_other_primary!((Hwb, into_hwb) => { RgbLinear});

impl_from_trait!((Luma, into_luma) => {Xyz, Yxy, Lab, Lch, Hsl, Hsv, Hwb});
impl_from_trait_other_primary!((Luma, into_luma) => { RgbLinear});

impl_from_trait_self_primary!((RgbLinear, into_rgb) => {Xyz, Yxy, Lab, Lch, Hsl, Hsv, Hwb, Luma});
