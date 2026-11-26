// This file is auto-generated and any manual changes to it will be overwritten.
//
// Run `cargo run -p codegen` from the project root to regenerate it.

use super::Lch;

#[automatically_derived]
impl<Wp, T> crate::convert::FromColorUnclamped<crate::xyz::Xyz<Wp, T>> for Lch<Wp, T>
where
    Wp: crate::white_point::WhitePoint<T>,
    crate::lab::Lab<Wp, T>: crate::convert::FromColorUnclamped<crate::xyz::Xyz<Wp, T>>,
    crate::lab::Lab<Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::xyz::Xyz<Wp, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::lab::Lab::<Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T, _S> crate::convert::FromColorUnclamped<crate::rgb::Rgb<_S, T>> for Lch<Wp, T>
where
    _S: crate::rgb::RgbStandard,
    _S::Space: crate::rgb::RgbSpace<WhitePoint = Wp>,
    Wp: crate::white_point::WhitePoint<T>,
    crate::lab::Lab<Wp, T>: crate::convert::FromColorUnclamped<crate::rgb::Rgb<_S, T>>,
    crate::lab::Lab<Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::rgb::Rgb<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::lab::Lab::<Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T, _S> crate::convert::FromColorUnclamped<crate::luma::Luma<_S, T>> for Lch<Wp, T>
where
    _S: crate::luma::LumaStandard<WhitePoint = Wp>,
    Wp: crate::white_point::WhitePoint<T>,
    crate::lab::Lab<Wp, T>: crate::convert::FromColorUnclamped<crate::luma::Luma<_S, T>>,
    crate::lab::Lab<Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::luma::Luma<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::lab::Lab::<Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T, _S> crate::convert::FromColorUnclamped<crate::hsl::Hsl<_S, T>> for Lch<Wp, T>
where
    _S: crate::rgb::RgbStandard,
    _S::Space: crate::rgb::RgbSpace<WhitePoint = Wp>,
    Wp: crate::white_point::WhitePoint<T>,
    crate::lab::Lab<Wp, T>: crate::convert::FromColorUnclamped<crate::hsl::Hsl<_S, T>>,
    crate::lab::Lab<Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hsl::Hsl<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::lab::Lab::<Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T> crate::convert::FromColorUnclamped<crate::hsluv::Hsluv<Wp, T>> for Lch<Wp, T>
where
    Wp: crate::white_point::WhitePoint<T>,
    crate::lab::Lab<Wp, T>: crate::convert::FromColorUnclamped<crate::hsluv::Hsluv<Wp, T>>,
    crate::lab::Lab<Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hsluv::Hsluv<Wp, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::lab::Lab::<Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T, _S> crate::convert::FromColorUnclamped<crate::hsv::Hsv<_S, T>> for Lch<Wp, T>
where
    _S: crate::rgb::RgbStandard,
    _S::Space: crate::rgb::RgbSpace<WhitePoint = Wp>,
    Wp: crate::white_point::WhitePoint<T>,
    crate::lab::Lab<Wp, T>: crate::convert::FromColorUnclamped<crate::hsv::Hsv<_S, T>>,
    crate::lab::Lab<Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hsv::Hsv<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::lab::Lab::<Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T, _S> crate::convert::FromColorUnclamped<crate::hwb::Hwb<_S, T>> for Lch<Wp, T>
where
    _S: crate::rgb::RgbStandard,
    _S::Space: crate::rgb::RgbSpace<WhitePoint = Wp>,
    Wp: crate::white_point::WhitePoint<T>,
    crate::lab::Lab<Wp, T>: crate::convert::FromColorUnclamped<crate::hwb::Hwb<_S, T>>,
    crate::lab::Lab<Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hwb::Hwb<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::lab::Lab::<Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T> crate::convert::FromColorUnclamped<crate::lchuv::Lchuv<Wp, T>> for Lch<Wp, T>
where
    Wp: crate::white_point::WhitePoint<T>,
    crate::lab::Lab<Wp, T>: crate::convert::FromColorUnclamped<crate::lchuv::Lchuv<Wp, T>>,
    crate::lab::Lab<Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::lchuv::Lchuv<Wp, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::lab::Lab::<Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T, _LmsM> crate::convert::FromColorUnclamped<crate::lms::Lms<_LmsM, T>> for Lch<Wp, T>
where
    _LmsM: crate::xyz::meta::HasXyzMeta<XyzMeta = Wp>,
    Wp: crate::white_point::WhitePoint<T>,
    crate::lab::Lab<Wp, T>: crate::convert::FromColorUnclamped<crate::lms::Lms<_LmsM, T>>,
    crate::lab::Lab<Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::lms::Lms<_LmsM, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::lab::Lab::<Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T> crate::convert::FromColorUnclamped<crate::luv::Luv<Wp, T>> for Lch<Wp, T>
where
    Wp: crate::white_point::WhitePoint<T>,
    crate::lab::Lab<Wp, T>: crate::convert::FromColorUnclamped<crate::luv::Luv<Wp, T>>,
    crate::lab::Lab<Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::luv::Luv<Wp, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::lab::Lab::<Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T> crate::convert::FromColorUnclamped<crate::oklab::Oklab<T>> for Lch<Wp, T>
where
    Wp: crate::white_point::WhitePoint<T>,
    crate::lab::Lab<Wp, T>: crate::convert::FromColorUnclamped<crate::oklab::Oklab<T>>,
    crate::lab::Lab<Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::oklab::Oklab<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::lab::Lab::<Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T> crate::convert::FromColorUnclamped<crate::oklch::Oklch<T>> for Lch<Wp, T>
where
    Wp: crate::white_point::WhitePoint<T>,
    crate::lab::Lab<Wp, T>: crate::convert::FromColorUnclamped<crate::oklch::Oklch<T>>,
    crate::lab::Lab<Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::oklch::Oklch<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::lab::Lab::<Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T> crate::convert::FromColorUnclamped<crate::okhsl::Okhsl<T>> for Lch<Wp, T>
where
    Wp: crate::white_point::WhitePoint<T>,
    crate::lab::Lab<Wp, T>: crate::convert::FromColorUnclamped<crate::okhsl::Okhsl<T>>,
    crate::lab::Lab<Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::okhsl::Okhsl<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::lab::Lab::<Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T> crate::convert::FromColorUnclamped<crate::okhsv::Okhsv<T>> for Lch<Wp, T>
where
    Wp: crate::white_point::WhitePoint<T>,
    crate::lab::Lab<Wp, T>: crate::convert::FromColorUnclamped<crate::okhsv::Okhsv<T>>,
    crate::lab::Lab<Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::okhsv::Okhsv<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::lab::Lab::<Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T> crate::convert::FromColorUnclamped<crate::okhwb::Okhwb<T>> for Lch<Wp, T>
where
    Wp: crate::white_point::WhitePoint<T>,
    crate::lab::Lab<Wp, T>: crate::convert::FromColorUnclamped<crate::okhwb::Okhwb<T>>,
    crate::lab::Lab<Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::okhwb::Okhwb<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::lab::Lab::<Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T> crate::convert::FromColorUnclamped<crate::yxy::Yxy<Wp, T>> for Lch<Wp, T>
where
    Wp: crate::white_point::WhitePoint<T>,
    crate::lab::Lab<Wp, T>: crate::convert::FromColorUnclamped<crate::yxy::Yxy<Wp, T>>,
    crate::lab::Lab<Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::yxy::Yxy<Wp, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::lab::Lab::<Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<Wp, T, _C, _A> crate::convert::FromColorUnclamped<crate::Alpha<_C, _A>> for Lch<Wp, T>
where
    _C: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::Alpha<_C, _A>) -> Self {
        color.color.into_color_unclamped()
    }
}

