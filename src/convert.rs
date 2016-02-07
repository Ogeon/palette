use num::Float;

use {Alpha, Rgb, Luma, Xyz, Yxy, Lab, Lch, Hsv, Hsl, Color};

///FromColor provides conversion between the colors.
///
///It requires from_xyz and derives conversion to other colors as a default from this.
///These defaults must be overridden when direct conversion exists between colors.
///For example, Luma has direct conversion to Rgb. So from_rgb conversion for Luma and
///from_luma for Rgb is implemented directly. The from for the same color must override
///the default. For example, from_rgb for Rgb will convert via Xyz which needs to be overridden
///with self to avoid the unnecessary converison.
pub trait FromColor<T>: Sized
    where T: Float,
{
    ///Convert from XYZ color space
    fn from_xyz(Xyz<T>) -> Self;

    ///Convert from Yxy color space
    fn from_yxy(inp: Yxy<T>) -> Self {
        Self::from_xyz(inp.into_xyz())
    }

    ///Convert from L*a*b* color space
    fn from_lab(inp: Lab<T>) -> Self {
        Self::from_xyz(inp.into_xyz())
    }

    ///Convert from L*C*h° color space
    fn from_lch(inp: Lch<T>) -> Self {
        Self::from_lab(inp.into_lab())
    }

    ///Convert from RGB color space
    fn from_rgb(inp: Rgb<T>) -> Self {
        Self::from_xyz(inp.into_xyz())
    }

    ///Convert from HSL color space
    fn from_hsl(inp: Hsl<T>) -> Self {
        Self::from_rgb(inp.into_rgb())
    }

    ///Convert from HSV color space
    fn from_hsv(inp: Hsv<T>) -> Self {
        Self::from_rgb(inp.into_rgb())
    }

    ///Convert from Luma
    fn from_luma(inp: Luma<T>) -> Self {
        Self::from_xyz(inp.into_xyz())
    }

}


///IntoColor provides conversion between the colors.
///
///It requires into into_xyz and derives conversion to other colors as a default from this.
///These defaults must be overridden when direct conversion exists between colors.
pub trait IntoColor<T>: Sized
    where T: Float,
{

    ///Convert into XYZ space
    fn into_xyz(self) -> Xyz<T>;

    ///Convert into Yxy color space
    fn into_yxy(self) -> Yxy<T> {
        Yxy::from_xyz(self.into_xyz())
    }

    ///Convert into L*a*b* color space
    fn into_lab(self) -> Lab<T> {
        Lab::from_xyz(self.into_xyz())
    }

    ///Convert into L*C*h° color space
    fn into_lch(self) -> Lch<T> {
        Lch::from_lab(self.into_lab())
    }

    ///Convert into RGB color space.
    fn into_rgb(self) -> Rgb<T> {
        Rgb::from_xyz(self.into_xyz())
    }

    ///Convert into HSL color space
    fn into_hsl(self) -> Hsl<T> {
        Hsl::from_rgb(self.into_rgb())
    }

    ///Convert into HSV color space
    fn into_hsv(self) -> Hsv<T> {
        Hsv::from_rgb(self.into_rgb())
    }

    ///Convert into Luma
    fn into_luma(self) -> Luma<T> {
        Luma::from_xyz(self.into_xyz())
    }

}

macro_rules! impl_into_color {
    ($self_ty:ident, $from_fn: ident) => {
        impl< T: Float > IntoColor<T> for $self_ty<T> {

            fn into_xyz(self) -> Xyz<T> {
                Xyz::$from_fn(self)
            }

            fn into_yxy(self) -> Yxy<T> {
                Yxy::$from_fn(self)
            }

            fn into_lab(self) -> Lab<T> {
                Lab::$from_fn(self)
            }

            fn into_lch(self) -> Lch<T> {
                Lch::$from_fn(self)
            }

            fn into_rgb(self) -> Rgb<T> {
                Rgb::$from_fn(self)
            }

            fn into_hsl(self) -> Hsl<T> {
                Hsl::$from_fn(self)
            }

            fn into_hsv(self) -> Hsv<T> {
                Hsv::$from_fn(self)
            }

            fn into_luma(self) -> Luma<T> {
                Luma::$from_fn(self)
            }

        }

    }
}


macro_rules! impl_from_trait {
    (($self_ty:ident, $into_fn: ident) => {$($other:ident),+}) => (
        impl<T: Float> From<Alpha<$self_ty<T>, T>> for $self_ty<T> {
            fn from(color: Alpha<$self_ty<T>, T>) -> $self_ty<T> {
                color.color
            }
        }

        impl<T: Float> From<Color<T>> for $self_ty<T> {
            fn from(color: Color<T>) -> $self_ty<T> {
                match color {
                    Color::Luma(c) => c.$into_fn(),
                    Color::Rgb(c) => c.$into_fn(),
                    Color::Xyz(c) => c.$into_fn(),
                    Color::Yxy(c) => c.$into_fn(),
                    Color::Lab(c) => c.$into_fn(),
                    Color::Lch(c) => c.$into_fn(),
                    Color::Hsv(c) => c.$into_fn(),
                    Color::Hsl(c) => c.$into_fn(),
                }
            }
        }

        impl<T: Float> From<Color<T>> for Alpha<$self_ty<T>,T> {
            fn from(color: Color<T>) -> Alpha<$self_ty<T>,T> {
                Alpha {
                    color: color.into(),
                    alpha: T::one(),
                }
            }
        }

        $(
            impl<T: Float> From<$other<T>> for $self_ty<T> {
                fn from(other: $other<T>) -> $self_ty<T> {
                    other.$into_fn()
                }
            }

            impl<T: Float> From<Alpha<$other<T>, T>> for Alpha<$self_ty<T>, T> {
                fn from(other: Alpha<$other<T>, T>) -> Alpha<$self_ty<T>, T> {
                    Alpha {
                        color: other.color.$into_fn(),
                        alpha: other.alpha,
                    }
                }
            }

            impl<T: Float> From<$other<T>> for Alpha<$self_ty<T>, T> {
                fn from(color: $other<T>) -> Alpha<$self_ty<T>, T> {
                    Alpha {
                        color: color.$into_fn(),
                        alpha: T::one(),
                    }
                }
            }

            impl<T: Float> From<Alpha<$other<T>, T>> for $self_ty<T> {
                fn from(other: Alpha<$other<T>, T>) -> $self_ty<T> {
                    other.color.$into_fn()
                }
            }

        )+
    )
}

impl_into_color!(Xyz,from_xyz);
impl_into_color!(Yxy,from_yxy);
impl_into_color!(Lab,from_lab);
impl_into_color!(Lch,from_lch);
impl_into_color!(Rgb,from_rgb);
impl_into_color!(Hsl,from_hsl);
impl_into_color!(Hsv,from_hsv);
impl_into_color!(Luma,from_luma);


impl_from_trait!((Xyz,into_xyz) => {Yxy, Lab, Lch, Rgb, Hsl, Hsv, Luma});
impl_from_trait!((Yxy,into_yxy) => {Xyz, Lab, Lch, Rgb, Hsl, Hsv, Luma});
impl_from_trait!((Lab,into_lab) => {Xyz, Yxy, Lch, Rgb, Hsl, Hsv, Luma});
impl_from_trait!((Lch,into_lch) => {Xyz, Yxy, Lab, Rgb, Hsl, Hsv, Luma});
impl_from_trait!((Rgb,into_rgb) => {Xyz, Yxy, Lab, Lch, Hsl, Hsv, Luma});
impl_from_trait!((Hsl,into_hsl) => {Xyz, Yxy, Lab, Lch, Rgb, Hsv, Luma});
impl_from_trait!((Hsv,into_hsv) => {Xyz, Yxy, Lab, Lch, Rgb, Hsl, Luma});
impl_from_trait!((Luma,into_luma) => {Xyz, Yxy, Lab, Lch, Rgb, Hsl, Hsv});
