// This file is auto-generated and any manual changes to it will be overwritten.
//
// Run `cargo run -p codegen` from the project root to regenerate it.

use super::Rgb;

#[automatically_derived]
impl<S, T>
    crate::convert::FromColorUnclamped<
        crate::hsluv::Hsluv<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    > for Rgb<S, T>
where
    S: crate::rgb::RgbStandard,
    crate::xyz::Xyz<<<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<
            crate::hsluv::Hsluv<
                <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
                T,
            >,
        >,
    crate::xyz::Xyz<<<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(
        color: crate::hsluv::Hsluv<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    ) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >::from_color_unclamped(color)
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T> crate::convert::FromColorUnclamped<crate::hwb::Hwb<S, T>> for Rgb<S, T>
where
    crate::hsv::Hsv<S, T>: crate::convert::FromColorUnclamped<crate::hwb::Hwb<S, T>>,
    crate::hsv::Hsv<S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hwb::Hwb<S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::hsv::Hsv::<S, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T>
    crate::convert::FromColorUnclamped<
        crate::lab::Lab<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    > for Rgb<S, T>
where
    S: crate::rgb::RgbStandard,
    crate::xyz::Xyz<<<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<
            crate::lab::Lab<
                <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
                T,
            >,
        >,
    crate::xyz::Xyz<<<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(
        color: crate::lab::Lab<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    ) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >::from_color_unclamped(color)
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T>
    crate::convert::FromColorUnclamped<
        crate::lch::Lch<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    > for Rgb<S, T>
where
    S: crate::rgb::RgbStandard,
    crate::xyz::Xyz<<<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<
            crate::lch::Lch<
                <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
                T,
            >,
        >,
    crate::xyz::Xyz<<<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(
        color: crate::lch::Lch<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    ) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >::from_color_unclamped(color)
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T>
    crate::convert::FromColorUnclamped<
        crate::lchuv::Lchuv<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    > for Rgb<S, T>
where
    S: crate::rgb::RgbStandard,
    crate::xyz::Xyz<<<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<
            crate::lchuv::Lchuv<
                <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
                T,
            >,
        >,
    crate::xyz::Xyz<<<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(
        color: crate::lchuv::Lchuv<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    ) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >::from_color_unclamped(color)
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T, _LmsM> crate::convert::FromColorUnclamped<crate::lms::Lms<_LmsM, T>> for Rgb<S, T>
where
    _LmsM: crate::xyz::meta::HasXyzMeta<
        XyzMeta = <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
    >,
    S: crate::rgb::RgbStandard,
    crate::xyz::Xyz<<<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<crate::lms::Lms<_LmsM, T>>,
    crate::xyz::Xyz<<<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::lms::Lms<_LmsM, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >::from_color_unclamped(color)
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T>
    crate::convert::FromColorUnclamped<
        crate::luv::Luv<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    > for Rgb<S, T>
where
    S: crate::rgb::RgbStandard,
    crate::xyz::Xyz<<<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<
            crate::luv::Luv<
                <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
                T,
            >,
        >,
    crate::xyz::Xyz<<<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(
        color: crate::luv::Luv<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    ) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >::from_color_unclamped(color)
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T> crate::convert::FromColorUnclamped<crate::oklch::Oklch<T>> for Rgb<S, T>
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
impl<S, T> crate::convert::FromColorUnclamped<crate::okhsl::Okhsl<T>> for Rgb<S, T>
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
impl<S, T> crate::convert::FromColorUnclamped<crate::okhsv::Okhsv<T>> for Rgb<S, T>
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
impl<S, T> crate::convert::FromColorUnclamped<crate::okhwb::Okhwb<T>> for Rgb<S, T>
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
impl<S, T>
    crate::convert::FromColorUnclamped<
        crate::yxy::Yxy<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    > for Rgb<S, T>
where
    S: crate::rgb::RgbStandard,
    crate::xyz::Xyz<<<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<
            crate::yxy::Yxy<
                <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
                T,
            >,
        >,
    crate::xyz::Xyz<<<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(
        color: crate::yxy::Yxy<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    ) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >::from_color_unclamped(color)
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T, _C, _A> crate::convert::FromColorUnclamped<crate::Alpha<_C, _A>> for Rgb<S, T>
where
    _C: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::Alpha<_C, _A>) -> Self {
        color.color.into_color_unclamped()
    }
}

