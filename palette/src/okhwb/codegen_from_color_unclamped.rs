// This file is auto-generated and any manual changes to it will be overwritten.
//
// Run `cargo run -p codegen` from the project root to regenerate it.

use super::Okhwb;

#[automatically_derived]
impl<T> crate::convert::FromColorUnclamped<crate::xyz::Xyz<crate::white_point::D65, T>> for Okhwb<T>
where
    crate::white_point::D65: crate::white_point::WhitePoint<T>,
    crate::okhsv::Okhsv<T>:
        crate::convert::FromColorUnclamped<crate::xyz::Xyz<crate::white_point::D65, T>>,
    crate::okhsv::Okhsv<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::xyz::Xyz<crate::white_point::D65, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::okhsv::Okhsv::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T, _S> crate::convert::FromColorUnclamped<crate::rgb::Rgb<_S, T>> for Okhwb<T>
where
    _S: crate::rgb::RgbStandard,
    _S::Space: crate::rgb::RgbSpace<WhitePoint = crate::white_point::D65>,
    crate::white_point::D65: crate::white_point::WhitePoint<T>,
    crate::okhsv::Okhsv<T>: crate::convert::FromColorUnclamped<crate::rgb::Rgb<_S, T>>,
    crate::okhsv::Okhsv<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::rgb::Rgb<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::okhsv::Okhsv::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T, _S> crate::convert::FromColorUnclamped<crate::luma::Luma<_S, T>> for Okhwb<T>
where
    _S: crate::luma::LumaStandard<WhitePoint = crate::white_point::D65>,
    crate::white_point::D65: crate::white_point::WhitePoint<T>,
    crate::okhsv::Okhsv<T>: crate::convert::FromColorUnclamped<crate::luma::Luma<_S, T>>,
    crate::okhsv::Okhsv<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::luma::Luma<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::okhsv::Okhsv::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T, _S> crate::convert::FromColorUnclamped<crate::hsl::Hsl<_S, T>> for Okhwb<T>
where
    _S: crate::rgb::RgbStandard,
    _S::Space: crate::rgb::RgbSpace<WhitePoint = crate::white_point::D65>,
    crate::white_point::D65: crate::white_point::WhitePoint<T>,
    crate::okhsv::Okhsv<T>: crate::convert::FromColorUnclamped<crate::hsl::Hsl<_S, T>>,
    crate::okhsv::Okhsv<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hsl::Hsl<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::okhsv::Okhsv::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T> crate::convert::FromColorUnclamped<crate::hsluv::Hsluv<crate::white_point::D65, T>>
    for Okhwb<T>
where
    crate::white_point::D65: crate::white_point::WhitePoint<T>,
    crate::okhsv::Okhsv<T>:
        crate::convert::FromColorUnclamped<crate::hsluv::Hsluv<crate::white_point::D65, T>>,
    crate::okhsv::Okhsv<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hsluv::Hsluv<crate::white_point::D65, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::okhsv::Okhsv::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T, _S> crate::convert::FromColorUnclamped<crate::hsv::Hsv<_S, T>> for Okhwb<T>
where
    _S: crate::rgb::RgbStandard,
    _S::Space: crate::rgb::RgbSpace<WhitePoint = crate::white_point::D65>,
    crate::white_point::D65: crate::white_point::WhitePoint<T>,
    crate::okhsv::Okhsv<T>: crate::convert::FromColorUnclamped<crate::hsv::Hsv<_S, T>>,
    crate::okhsv::Okhsv<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hsv::Hsv<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::okhsv::Okhsv::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T, _S> crate::convert::FromColorUnclamped<crate::hwb::Hwb<_S, T>> for Okhwb<T>
where
    _S: crate::rgb::RgbStandard,
    _S::Space: crate::rgb::RgbSpace<WhitePoint = crate::white_point::D65>,
    crate::white_point::D65: crate::white_point::WhitePoint<T>,
    crate::okhsv::Okhsv<T>: crate::convert::FromColorUnclamped<crate::hwb::Hwb<_S, T>>,
    crate::okhsv::Okhsv<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hwb::Hwb<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::okhsv::Okhsv::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T> crate::convert::FromColorUnclamped<crate::lab::Lab<crate::white_point::D65, T>> for Okhwb<T>
where
    crate::white_point::D65: crate::white_point::WhitePoint<T>,
    crate::okhsv::Okhsv<T>:
        crate::convert::FromColorUnclamped<crate::lab::Lab<crate::white_point::D65, T>>,
    crate::okhsv::Okhsv<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::lab::Lab<crate::white_point::D65, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::okhsv::Okhsv::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T> crate::convert::FromColorUnclamped<crate::lch::Lch<crate::white_point::D65, T>> for Okhwb<T>
where
    crate::white_point::D65: crate::white_point::WhitePoint<T>,
    crate::okhsv::Okhsv<T>:
        crate::convert::FromColorUnclamped<crate::lch::Lch<crate::white_point::D65, T>>,
    crate::okhsv::Okhsv<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::lch::Lch<crate::white_point::D65, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::okhsv::Okhsv::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T> crate::convert::FromColorUnclamped<crate::lchuv::Lchuv<crate::white_point::D65, T>>
    for Okhwb<T>
where
    crate::white_point::D65: crate::white_point::WhitePoint<T>,
    crate::okhsv::Okhsv<T>:
        crate::convert::FromColorUnclamped<crate::lchuv::Lchuv<crate::white_point::D65, T>>,
    crate::okhsv::Okhsv<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::lchuv::Lchuv<crate::white_point::D65, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::okhsv::Okhsv::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T, _LmsM> crate::convert::FromColorUnclamped<crate::lms::Lms<_LmsM, T>> for Okhwb<T>
where
    _LmsM: crate::xyz::meta::HasXyzMeta<XyzMeta = crate::white_point::D65>,
    crate::white_point::D65: crate::white_point::WhitePoint<T>,
    crate::okhsv::Okhsv<T>: crate::convert::FromColorUnclamped<crate::lms::Lms<_LmsM, T>>,
    crate::okhsv::Okhsv<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::lms::Lms<_LmsM, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::okhsv::Okhsv::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T> crate::convert::FromColorUnclamped<crate::luv::Luv<crate::white_point::D65, T>> for Okhwb<T>
where
    crate::white_point::D65: crate::white_point::WhitePoint<T>,
    crate::okhsv::Okhsv<T>:
        crate::convert::FromColorUnclamped<crate::luv::Luv<crate::white_point::D65, T>>,
    crate::okhsv::Okhsv<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::luv::Luv<crate::white_point::D65, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::okhsv::Okhsv::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T> crate::convert::FromColorUnclamped<crate::oklab::Oklab<T>> for Okhwb<T>
where
    crate::okhsv::Okhsv<T>: crate::convert::FromColorUnclamped<crate::oklab::Oklab<T>>,
    crate::okhsv::Okhsv<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::oklab::Oklab<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::okhsv::Okhsv::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T> crate::convert::FromColorUnclamped<crate::oklch::Oklch<T>> for Okhwb<T>
where
    crate::okhsv::Okhsv<T>: crate::convert::FromColorUnclamped<crate::oklch::Oklch<T>>,
    crate::okhsv::Okhsv<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::oklch::Oklch<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::okhsv::Okhsv::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T> crate::convert::FromColorUnclamped<crate::okhsl::Okhsl<T>> for Okhwb<T>
where
    crate::okhsv::Okhsv<T>: crate::convert::FromColorUnclamped<crate::okhsl::Okhsl<T>>,
    crate::okhsv::Okhsv<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::okhsl::Okhsl<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::okhsv::Okhsv::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T> crate::convert::FromColorUnclamped<crate::yxy::Yxy<crate::white_point::D65, T>> for Okhwb<T>
where
    crate::white_point::D65: crate::white_point::WhitePoint<T>,
    crate::okhsv::Okhsv<T>:
        crate::convert::FromColorUnclamped<crate::yxy::Yxy<crate::white_point::D65, T>>,
    crate::okhsv::Okhsv<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::yxy::Yxy<crate::white_point::D65, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::okhsv::Okhsv::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T, _C, _A> crate::convert::FromColorUnclamped<crate::Alpha<_C, _A>> for Okhwb<T>
where
    _C: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::Alpha<_C, _A>) -> Self {
        color.color.into_color_unclamped()
    }
}

