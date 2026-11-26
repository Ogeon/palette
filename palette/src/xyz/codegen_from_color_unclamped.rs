// This file is auto-generated and any manual changes to it will be overwritten.
//
// Run `cargo run -p codegen` from the project root to regenerate it.

use super::Xyz;

#[automatically_derived]
impl<Wp, T, _S> crate::convert::FromColorUnclamped<crate::hsl::Hsl<_S, T>> for Xyz<Wp, T>
where
    _S: crate::rgb::RgbStandard,
    _S::Space: crate::rgb::RgbSpace<WhitePoint = Wp>,
    Wp: crate::white_point::WhitePoint<T>,
    crate::rgb::Rgb<_S, T>: crate::convert::FromColorUnclamped<crate::hsl::Hsl<_S, T>>,
    crate::rgb::Rgb<_S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hsl::Hsl<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::rgb::Rgb::<_S, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T> crate::convert::FromColorUnclamped<crate::hsluv::Hsluv<Wp, T>> for Xyz<Wp, T>
where
    Wp: crate::white_point::WhitePoint<T>,
    crate::luv::Luv<Wp, T>: crate::convert::FromColorUnclamped<crate::hsluv::Hsluv<Wp, T>>,
    crate::luv::Luv<Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hsluv::Hsluv<Wp, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::luv::Luv::<Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T, _S> crate::convert::FromColorUnclamped<crate::hsv::Hsv<_S, T>> for Xyz<Wp, T>
where
    _S: crate::rgb::RgbStandard,
    _S::Space: crate::rgb::RgbSpace<WhitePoint = Wp>,
    Wp: crate::white_point::WhitePoint<T>,
    crate::rgb::Rgb<_S, T>: crate::convert::FromColorUnclamped<crate::hsv::Hsv<_S, T>>,
    crate::rgb::Rgb<_S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hsv::Hsv<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::rgb::Rgb::<_S, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T, _S> crate::convert::FromColorUnclamped<crate::hwb::Hwb<_S, T>> for Xyz<Wp, T>
where
    _S: crate::rgb::RgbStandard,
    _S::Space: crate::rgb::RgbSpace<WhitePoint = Wp>,
    Wp: crate::white_point::WhitePoint<T>,
    crate::rgb::Rgb<_S, T>: crate::convert::FromColorUnclamped<crate::hwb::Hwb<_S, T>>,
    crate::rgb::Rgb<_S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hwb::Hwb<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::rgb::Rgb::<_S, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T> crate::convert::FromColorUnclamped<crate::lch::Lch<Wp, T>> for Xyz<Wp, T>
where
    Wp: crate::white_point::WhitePoint<T>,
    crate::lab::Lab<Wp, T>: crate::convert::FromColorUnclamped<crate::lch::Lch<Wp, T>>,
    crate::lab::Lab<Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::lch::Lch<Wp, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::lab::Lab::<Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T> crate::convert::FromColorUnclamped<crate::lchuv::Lchuv<Wp, T>> for Xyz<Wp, T>
where
    Wp: crate::white_point::WhitePoint<T>,
    crate::luv::Luv<Wp, T>: crate::convert::FromColorUnclamped<crate::lchuv::Lchuv<Wp, T>>,
    crate::luv::Luv<Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::lchuv::Lchuv<Wp, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::luv::Luv::<Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T> crate::convert::FromColorUnclamped<crate::oklch::Oklch<T>> for Xyz<Wp, T>
where
    crate::oklab::Oklab<T>: crate::convert::FromColorUnclamped<crate::oklch::Oklch<T>>,
    crate::oklab::Oklab<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::oklch::Oklch<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::oklab::Oklab::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T> crate::convert::FromColorUnclamped<crate::okhsl::Okhsl<T>> for Xyz<Wp, T>
where
    crate::oklab::Oklab<T>: crate::convert::FromColorUnclamped<crate::okhsl::Okhsl<T>>,
    crate::oklab::Oklab<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::okhsl::Okhsl<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::oklab::Oklab::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T> crate::convert::FromColorUnclamped<crate::okhsv::Okhsv<T>> for Xyz<Wp, T>
where
    crate::oklab::Oklab<T>: crate::convert::FromColorUnclamped<crate::okhsv::Okhsv<T>>,
    crate::oklab::Oklab<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::okhsv::Okhsv<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::oklab::Oklab::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T> crate::convert::FromColorUnclamped<crate::okhwb::Okhwb<T>> for Xyz<Wp, T>
where
    crate::oklab::Oklab<T>: crate::convert::FromColorUnclamped<crate::okhwb::Okhwb<T>>,
    crate::oklab::Oklab<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::okhwb::Okhwb<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::oklab::Oklab::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T, _C, _A> crate::convert::FromColorUnclamped<crate::Alpha<_C, _A>> for Xyz<Wp, T>
where
    _C: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::Alpha<_C, _A>) -> Self {
        color.color.into_color_unclamped()
    }
}

