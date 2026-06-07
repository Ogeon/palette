// This file is auto-generated and any manual changes to it will be overwritten.
//
// Run `cargo run -p codegen` from the project root to regenerate it.

use super::Lms;

#[automatically_derived]
impl<M, T, _S, _Wp> crate::convert::FromColorUnclamped<crate::rgb::Rgb<_S, T>> for Lms<M, T>
where
    _S: crate::rgb::RgbStandard,
    _S::Space: crate::rgb::RgbSpace<WhitePoint = _Wp>,
    crate::xyz::Xyz<_Wp, T>: crate::convert::FromColorUnclamped<crate::rgb::Rgb<_S, T>>,
    crate::xyz::Xyz<_Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::rgb::Rgb<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<_Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<M, T, _S, _Wp> crate::convert::FromColorUnclamped<crate::luma::Luma<_S, T>> for Lms<M, T>
where
    _S: crate::luma::LumaStandard<WhitePoint = _Wp>,
    crate::xyz::Xyz<_Wp, T>: crate::convert::FromColorUnclamped<crate::luma::Luma<_S, T>>,
    crate::xyz::Xyz<_Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::luma::Luma<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<_Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<M, T, _S, _Wp> crate::convert::FromColorUnclamped<crate::hsl::Hsl<_S, T>> for Lms<M, T>
where
    _S: crate::rgb::RgbStandard,
    _S::Space: crate::rgb::RgbSpace<WhitePoint = _Wp>,
    crate::xyz::Xyz<_Wp, T>: crate::convert::FromColorUnclamped<crate::hsl::Hsl<_S, T>>,
    crate::xyz::Xyz<_Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hsl::Hsl<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<_Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<M, T, _Wp> crate::convert::FromColorUnclamped<crate::hsluv::Hsluv<_Wp, T>> for Lms<M, T>
where
    crate::xyz::Xyz<_Wp, T>: crate::convert::FromColorUnclamped<crate::hsluv::Hsluv<_Wp, T>>,
    crate::xyz::Xyz<_Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hsluv::Hsluv<_Wp, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<_Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<M, T, _S, _Wp> crate::convert::FromColorUnclamped<crate::hsv::Hsv<_S, T>> for Lms<M, T>
where
    _S: crate::rgb::RgbStandard,
    _S::Space: crate::rgb::RgbSpace<WhitePoint = _Wp>,
    crate::xyz::Xyz<_Wp, T>: crate::convert::FromColorUnclamped<crate::hsv::Hsv<_S, T>>,
    crate::xyz::Xyz<_Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hsv::Hsv<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<_Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<M, T, _S, _Wp> crate::convert::FromColorUnclamped<crate::hwb::Hwb<_S, T>> for Lms<M, T>
where
    _S: crate::rgb::RgbStandard,
    _S::Space: crate::rgb::RgbSpace<WhitePoint = _Wp>,
    crate::xyz::Xyz<_Wp, T>: crate::convert::FromColorUnclamped<crate::hwb::Hwb<_S, T>>,
    crate::xyz::Xyz<_Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hwb::Hwb<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<_Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<M, T, _Wp> crate::convert::FromColorUnclamped<crate::lab::Lab<_Wp, T>> for Lms<M, T>
where
    crate::xyz::Xyz<_Wp, T>: crate::convert::FromColorUnclamped<crate::lab::Lab<_Wp, T>>,
    crate::xyz::Xyz<_Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::lab::Lab<_Wp, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<_Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<M, T, _Wp> crate::convert::FromColorUnclamped<crate::lch::Lch<_Wp, T>> for Lms<M, T>
where
    crate::xyz::Xyz<_Wp, T>: crate::convert::FromColorUnclamped<crate::lch::Lch<_Wp, T>>,
    crate::xyz::Xyz<_Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::lch::Lch<_Wp, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<_Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<M, T, _Wp> crate::convert::FromColorUnclamped<crate::lchuv::Lchuv<_Wp, T>> for Lms<M, T>
where
    crate::xyz::Xyz<_Wp, T>: crate::convert::FromColorUnclamped<crate::lchuv::Lchuv<_Wp, T>>,
    crate::xyz::Xyz<_Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::lchuv::Lchuv<_Wp, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<_Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<M, T, _Wp> crate::convert::FromColorUnclamped<crate::luv::Luv<_Wp, T>> for Lms<M, T>
where
    crate::xyz::Xyz<_Wp, T>: crate::convert::FromColorUnclamped<crate::luv::Luv<_Wp, T>>,
    crate::xyz::Xyz<_Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::luv::Luv<_Wp, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<_Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<M, T> crate::convert::FromColorUnclamped<crate::oklab::Oklab<T>> for Lms<M, T>
where
    crate::xyz::Xyz<crate::white_point::D65, T>:
        crate::convert::FromColorUnclamped<crate::oklab::Oklab<T>>,
    crate::xyz::Xyz<crate::white_point::D65, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::oklab::Oklab<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<crate::white_point::D65, T>::from_color_unclamped(color)
            .into_color_unclamped()
    }
}
#[automatically_derived]
impl<M, T> crate::convert::FromColorUnclamped<crate::oklch::Oklch<T>> for Lms<M, T>
where
    crate::xyz::Xyz<crate::white_point::D65, T>:
        crate::convert::FromColorUnclamped<crate::oklch::Oklch<T>>,
    crate::xyz::Xyz<crate::white_point::D65, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::oklch::Oklch<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<crate::white_point::D65, T>::from_color_unclamped(color)
            .into_color_unclamped()
    }
}
#[automatically_derived]
impl<M, T> crate::convert::FromColorUnclamped<crate::okhsl::Okhsl<T>> for Lms<M, T>
where
    crate::xyz::Xyz<crate::white_point::D65, T>:
        crate::convert::FromColorUnclamped<crate::okhsl::Okhsl<T>>,
    crate::xyz::Xyz<crate::white_point::D65, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::okhsl::Okhsl<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<crate::white_point::D65, T>::from_color_unclamped(color)
            .into_color_unclamped()
    }
}
#[automatically_derived]
impl<M, T> crate::convert::FromColorUnclamped<crate::okhsv::Okhsv<T>> for Lms<M, T>
where
    crate::xyz::Xyz<crate::white_point::D65, T>:
        crate::convert::FromColorUnclamped<crate::okhsv::Okhsv<T>>,
    crate::xyz::Xyz<crate::white_point::D65, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::okhsv::Okhsv<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<crate::white_point::D65, T>::from_color_unclamped(color)
            .into_color_unclamped()
    }
}
#[automatically_derived]
impl<M, T> crate::convert::FromColorUnclamped<crate::okhwb::Okhwb<T>> for Lms<M, T>
where
    crate::xyz::Xyz<crate::white_point::D65, T>:
        crate::convert::FromColorUnclamped<crate::okhwb::Okhwb<T>>,
    crate::xyz::Xyz<crate::white_point::D65, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::okhwb::Okhwb<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<crate::white_point::D65, T>::from_color_unclamped(color)
            .into_color_unclamped()
    }
}
#[automatically_derived]
impl<M, T, _Wp> crate::convert::FromColorUnclamped<crate::yxy::Yxy<_Wp, T>> for Lms<M, T>
where
    crate::xyz::Xyz<_Wp, T>: crate::convert::FromColorUnclamped<crate::yxy::Yxy<_Wp, T>>,
    crate::xyz::Xyz<_Wp, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::yxy::Yxy<_Wp, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<_Wp, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<M, T, _C, _A> crate::convert::FromColorUnclamped<crate::Alpha<_C, _A>> for Lms<M, T>
where
    _C: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::Alpha<_C, _A>) -> Self {
        color.color.into_color_unclamped()
    }
}

