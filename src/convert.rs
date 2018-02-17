use num_traits::Float;

use {Alpha, Color, Hsl, Hsv, Hwb, Lab, Lch, Luma, Xyz, Yxy};
use white_point::{D65, WhitePoint};
use rgb::{Linear, Rgb, RgbSpace, RgbStandard};

///FromColor provides conversion between the colors.
///
///It requires from_xyz and derives conversion to other colors as a default from this.
///These defaults must be overridden when direct conversion exists between colors.
///For example, Luma has direct conversion to LinRgb. So from_rgb conversion for Luma and
///from_luma for LinRgb is implemented directly. The from for the same color must override
///the default. For example, from_rgb for LinRgb will convert via Xyz which needs to be overridden
///with self to avoid the unnecessary converison.
pub trait FromColor<Wp = D65, T = f32>: Sized
where
    T: Float,
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
    fn from_luma(inp: Luma<Wp, T>) -> Self {
        Self::from_xyz(inp.into_xyz())
    }
}

///IntoColor provides conversion between the colors.
///
///It requires into into_xyz and derives conversion to other colors as a default from this.
///These defaults must be overridden when direct conversion exists between colors.
pub trait IntoColor<Wp = D65, T = f32>: Sized
where
    T: Float,
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
    fn into_luma(self) -> Luma<Wp, T> {
        Luma::from_xyz(self.into_xyz())
    }
}

macro_rules! impl_into_color {
    ($self_ty:ident, $from_fn: ident) => {
        impl<Wp, T> IntoColor<Wp, T> for $self_ty<Wp, T>
            where T: Float,
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

            fn into_luma(self) -> Luma<Wp, T> {
                Luma::$from_fn(self)
            }

        }

    }
}

macro_rules! impl_into_color_rgb {
    ($self_ty:ident, $from_fn: ident) => {
        impl<S, Wp, T> IntoColor<Wp, T> for $self_ty<S, T> where
            T: Float,
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

            fn into_luma(self) -> Luma<Wp, T> {
                Luma::$from_fn(self)
            }

        }

    }
}

macro_rules! impl_from_trait {
    (<$s:ident, $t:ident>($self_ty: ty, $into_fn: ident) => {$($other: ty),+}) => (
        impl<$s, $t> From<Color<$s, $t>> for $self_ty
            where $t: Float,
                $s: RgbSpace
        {
            fn from(color: Color<$s, $t>) -> $self_ty {
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

        impl<$s, $t> From<Color<$s, $t>> for Alpha<$self_ty,$t>
            where $t: Float,
                $s: RgbSpace
        {
            fn from(color: Color<$s, $t>) -> Alpha<$self_ty,$t> {
                Alpha {
                    color: color.into(),
                    alpha: $t::one(),
                }
            }
        }

        impl<$s, $t> From<Alpha<Color<$s, $t>, $t>> for $self_ty
            where $t: Float,
                $s: RgbSpace
        {
            fn from(color: Alpha<Color<$s, $t>, $t>) -> $self_ty {
                color.color.into()
            }
        }

        impl<$s, $t> From<Alpha<Color<$s, $t>, $t>> for Alpha<$self_ty,$t>
            where $t: Float,
                $s: RgbSpace
        {
            fn from(color: Alpha<Color<$s, $t>, $t>) -> Alpha<$self_ty,$t> {
                Alpha {
                    color: color.color.into(),
                    alpha: color.alpha,
                }
            }
        }

        $(
            impl<$s, $t> From<$other> for $self_ty
                where $t: Float,
                    $s: RgbSpace
            {
                fn from(other: $other) -> $self_ty {
                    other.$into_fn()
                }
            }

            impl<$s, $t> From<Alpha<$other, $t>> for Alpha<$self_ty, $t>
                where $t: Float,
                    $s: RgbSpace
            {
                fn from(other: Alpha<$other, $t>) -> Alpha<$self_ty, $t> {
                    Alpha {
                        color: other.color.$into_fn(),
                        alpha: other.alpha,
                    }
                }
            }

            impl<$s, $t> From<$other> for Alpha<$self_ty, $t>
                where $t: Float,
                    $s: RgbSpace
            {
                fn from(color: $other) -> Alpha<$self_ty, $t> {
                    Alpha {
                        color: color.$into_fn(),
                        alpha: $t::one(),
                    }
                }
            }

            impl<$s, $t> From<Alpha<$other, $t>> for $self_ty
                where $t: Float,
                    $s: RgbSpace
            {
                fn from(other: Alpha<$other, $t>) -> $self_ty {
                    other.color.$into_fn()
                }
            }

        )+
    )
}

macro_rules! impl_from_trait_other {
    (<$s:ident : $s_ty:ident, $t:ident>($self_ty: ty, |$into_ident:ident| $into_expr:expr) => {$($other: ty),+}) => (
        $(
            impl<$s, $t> From<$other> for $self_ty
                where $t: Float,
                    $s: $s_ty
            {
                fn from(other: $other) -> $self_ty {
                    let $into_ident = other;
                    $into_expr
                }
            }

            impl<$s, $t> From<Alpha<$other, $t>> for Alpha<$self_ty, $t>
                where $t: Float,
                    $s: $s_ty
            {
                fn from(other: Alpha<$other, $t>) -> Alpha<$self_ty, $t> {
                    let $into_ident = other.color;
                    Alpha {
                        color: $into_expr,
                        alpha: other.alpha,
                    }
                }
            }

            impl<$s, $t> From<$other> for Alpha<$self_ty, $t>
                where $t: Float,
                    $s: $s_ty
            {
                fn from(color: $other) -> Alpha<$self_ty, $t> {
                    let $into_ident = color;
                    Alpha {
                        color: $into_expr,
                        alpha: $t::one(),
                    }
                }
            }

            impl<$s, $t> From<Alpha<$other, $t>> for $self_ty
                where $t: Float,
                    $s: $s_ty
            {
                fn from(other: Alpha<$other, $t>) -> $self_ty {
                    let $into_ident = other.color;
                    $into_expr
                }
            }

        )+
    );


    (<$s:ident : $s_ty:ident, $t:ident>($self_ty: ty, $into_fn: ident) => {$($other: ty),+}) => (
        impl_from_trait_other!(<$s: $s_ty, $t>($self_ty, |a| a.$into_fn()) => {$($other),+});
    );
}

impl_into_color!(Xyz, from_xyz);
impl_into_color!(Yxy, from_yxy);
impl_into_color!(Lab, from_lab);
impl_into_color!(Lch, from_lch);
impl_into_color!(Luma, from_luma);
impl_into_color_rgb!(Hsl, from_hsl);
impl_into_color_rgb!(Hsv, from_hsv);
impl_into_color_rgb!(Hwb, from_hwb);

impl_from_trait!(<S, T> (Xyz<S::WhitePoint, T>, into_xyz) => {Hsl<S, T>, Hsv<S, T>, Hwb<S, T>});
impl_from_trait!(<S, T> (Yxy<S::WhitePoint, T>, into_yxy) => {Hsl<S, T>, Hsv<S, T>, Hwb<S, T>});
impl_from_trait!(<S, T> (Lab<S::WhitePoint, T>, into_lab) => {Hsl<S, T>, Hsv<S, T>, Hwb<S, T>});
impl_from_trait!(<S, T> (Lch<S::WhitePoint, T>, into_lch) => {Hsl<S, T>, Hsv<S, T>, Hwb<S, T>});
impl_from_trait!(<S, T> (Luma<S::WhitePoint, T>, into_luma) => {Hsl<S, T>, Hsv<S, T>, Hwb<S, T>});

impl_from_trait!(<S, T> (Rgb<Linear<S>, T>, into_rgb) => {Xyz<S::WhitePoint, T>, Yxy<S::WhitePoint, T>, Lab<S::WhitePoint, T>, Lch<S::WhitePoint, T>, Hsl<S, T>, Hsv<S, T>, Hwb<S, T>, Luma<S::WhitePoint, T>});
impl_from_trait!(<S, T> (Hsl<S, T>, into_hsl) => {Xyz<S::WhitePoint, T>, Yxy<S::WhitePoint, T>, Lab<S::WhitePoint, T>, Lch<S::WhitePoint, T>, Hsv<S, T>, Hwb<S, T>, Luma<S::WhitePoint, T>});
impl_from_trait!(<S, T> (Hsv<S, T>, into_hsv) => {Xyz<S::WhitePoint, T>, Yxy<S::WhitePoint, T>, Lab<S::WhitePoint, T>, Lch<S::WhitePoint, T>, Hsl<S, T>, Hwb<S, T>, Luma<S::WhitePoint, T>});
impl_from_trait!(<S, T> (Hwb<S, T>, into_hwb) => {Xyz<S::WhitePoint, T>, Yxy<S::WhitePoint, T>, Lab<S::WhitePoint, T>, Lch<S::WhitePoint, T>, Hsl<S, T>, Hsv<S, T>, Luma<S::WhitePoint, T>});

impl_from_trait_other!(<Wp: WhitePoint, T> (Xyz<Wp, T>, into_xyz) => {Yxy<Wp, T>, Lab<Wp, T>, Lch<Wp, T>, Luma<Wp, T>});
impl_from_trait_other!(<Wp: WhitePoint, T> (Yxy<Wp, T>, into_yxy) => {Xyz<Wp, T>, Lab<Wp, T>, Lch<Wp, T>, Luma<Wp, T>});
impl_from_trait_other!(<Wp: WhitePoint, T> (Lab<Wp, T>, into_lab) => {Xyz<Wp, T>, Yxy<Wp, T>, Lch<Wp, T>, Luma<Wp, T>});
impl_from_trait_other!(<Wp: WhitePoint, T> (Lch<Wp, T>, into_lch) => {Xyz<Wp, T>, Yxy<Wp, T>, Lab<Wp, T>, Luma<Wp, T>});
impl_from_trait_other!(<Wp: WhitePoint, T> (Luma<Wp, T>, into_luma) => {Xyz<Wp, T>, Yxy<Wp, T>, Lab<Wp, T>, Lch<Wp, T>});

impl_from_trait_other!(<S: RgbStandard, T> (Xyz<<S::Space as RgbSpace>::WhitePoint, T>, into_xyz) => {Rgb<S, T>});
impl_from_trait_other!(<S: RgbStandard, T> (Yxy<<S::Space as RgbSpace>::WhitePoint, T>, into_yxy) => {Rgb<S, T>});
impl_from_trait_other!(<S: RgbStandard, T> (Lab<<S::Space as RgbSpace>::WhitePoint, T>, into_lab) => {Rgb<S, T>});
impl_from_trait_other!(<S: RgbStandard, T> (Lch<<S::Space as RgbSpace>::WhitePoint, T>, into_lch) => {Rgb<S, T>});
impl_from_trait_other!(<S: RgbStandard, T> (Hsl<S::Space, T>, into_hsl) => {Rgb<S, T>});
impl_from_trait_other!(<S: RgbStandard, T> (Hsv<S::Space, T>, into_hsv) => {Rgb<S, T>});
impl_from_trait_other!(<S: RgbStandard, T> (Hwb<S::Space, T>, into_hwb) => {Rgb<S, T>});
impl_from_trait_other!(<S: RgbStandard, T> (Luma<<S::Space as RgbSpace>::WhitePoint, T>, into_luma) => {Rgb<S, T>});
