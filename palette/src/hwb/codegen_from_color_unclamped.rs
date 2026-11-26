// This file is auto-generated and any manual changes to it will be overwritten.
//
// Run `cargo run -p codegen` from the project root to regenerate it.

use super::Hwb;

#[automatically_derived]
impl<S, T>
    crate::convert::FromColorUnclamped<
        crate::xyz::Xyz<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    > for Hwb<S, T>
where
    S: crate::rgb::RgbStandard,
    crate::hsv::Hsv<S, T>: crate::convert::FromColorUnclamped<
        crate::xyz::Xyz<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    >,
    crate::hsv::Hsv<S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(
        color: crate::xyz::Xyz<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    ) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::hsv::Hsv::<S, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T> crate::convert::FromColorUnclamped<crate::rgb::Rgb<S, T>> for Hwb<S, T>
where
    crate::hsv::Hsv<S, T>: crate::convert::FromColorUnclamped<crate::rgb::Rgb<S, T>>,
    crate::hsv::Hsv<S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::rgb::Rgb<S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::hsv::Hsv::<S, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T, _S> crate::convert::FromColorUnclamped<crate::luma::Luma<_S, T>> for Hwb<S, T>
where
    _S: crate::luma::LumaStandard<
        WhitePoint = <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
    >,
    S: crate::rgb::RgbStandard,
    crate::hsv::Hsv<S, T>: crate::convert::FromColorUnclamped<crate::luma::Luma<_S, T>>,
    crate::hsv::Hsv<S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::luma::Luma<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::hsv::Hsv::<S, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T> crate::convert::FromColorUnclamped<crate::hsl::Hsl<S, T>> for Hwb<S, T>
where
    crate::hsv::Hsv<S, T>: crate::convert::FromColorUnclamped<crate::hsl::Hsl<S, T>>,
    crate::hsv::Hsv<S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hsl::Hsl<S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::hsv::Hsv::<S, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T>
    crate::convert::FromColorUnclamped<
        crate::hsluv::Hsluv<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    > for Hwb<S, T>
where
    S: crate::rgb::RgbStandard,
    crate::hsv::Hsv<S, T>: crate::convert::FromColorUnclamped<
        crate::hsluv::Hsluv<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    >,
    crate::hsv::Hsv<S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(
        color: crate::hsluv::Hsluv<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    ) -> Self {
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
    > for Hwb<S, T>
where
    S: crate::rgb::RgbStandard,
    crate::hsv::Hsv<S, T>: crate::convert::FromColorUnclamped<
        crate::lab::Lab<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    >,
    crate::hsv::Hsv<S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(
        color: crate::lab::Lab<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    ) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::hsv::Hsv::<S, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T>
    crate::convert::FromColorUnclamped<
        crate::lch::Lch<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    > for Hwb<S, T>
where
    S: crate::rgb::RgbStandard,
    crate::hsv::Hsv<S, T>: crate::convert::FromColorUnclamped<
        crate::lch::Lch<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    >,
    crate::hsv::Hsv<S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(
        color: crate::lch::Lch<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    ) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::hsv::Hsv::<S, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T>
    crate::convert::FromColorUnclamped<
        crate::lchuv::Lchuv<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    > for Hwb<S, T>
where
    S: crate::rgb::RgbStandard,
    crate::hsv::Hsv<S, T>: crate::convert::FromColorUnclamped<
        crate::lchuv::Lchuv<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    >,
    crate::hsv::Hsv<S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(
        color: crate::lchuv::Lchuv<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    ) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::hsv::Hsv::<S, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T, _LmsM> crate::convert::FromColorUnclamped<crate::lms::Lms<_LmsM, T>> for Hwb<S, T>
where
    _LmsM: crate::xyz::meta::HasXyzMeta<
        XyzMeta = <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
    >,
    S: crate::rgb::RgbStandard,
    crate::hsv::Hsv<S, T>: crate::convert::FromColorUnclamped<crate::lms::Lms<_LmsM, T>>,
    crate::hsv::Hsv<S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::lms::Lms<_LmsM, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::hsv::Hsv::<S, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T>
    crate::convert::FromColorUnclamped<
        crate::luv::Luv<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    > for Hwb<S, T>
where
    S: crate::rgb::RgbStandard,
    crate::hsv::Hsv<S, T>: crate::convert::FromColorUnclamped<
        crate::luv::Luv<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    >,
    crate::hsv::Hsv<S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(
        color: crate::luv::Luv<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    ) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::hsv::Hsv::<S, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T> crate::convert::FromColorUnclamped<crate::oklab::Oklab<T>> for Hwb<S, T>
where
    crate::hsv::Hsv<S, T>: crate::convert::FromColorUnclamped<crate::oklab::Oklab<T>>,
    crate::hsv::Hsv<S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::oklab::Oklab<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::hsv::Hsv::<S, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T> crate::convert::FromColorUnclamped<crate::oklch::Oklch<T>> for Hwb<S, T>
where
    crate::hsv::Hsv<S, T>: crate::convert::FromColorUnclamped<crate::oklch::Oklch<T>>,
    crate::hsv::Hsv<S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::oklch::Oklch<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::hsv::Hsv::<S, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T> crate::convert::FromColorUnclamped<crate::okhsl::Okhsl<T>> for Hwb<S, T>
where
    crate::hsv::Hsv<S, T>: crate::convert::FromColorUnclamped<crate::okhsl::Okhsl<T>>,
    crate::hsv::Hsv<S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::okhsl::Okhsl<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::hsv::Hsv::<S, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T> crate::convert::FromColorUnclamped<crate::okhsv::Okhsv<T>> for Hwb<S, T>
where
    crate::hsv::Hsv<S, T>: crate::convert::FromColorUnclamped<crate::okhsv::Okhsv<T>>,
    crate::hsv::Hsv<S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::okhsv::Okhsv<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::hsv::Hsv::<S, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T> crate::convert::FromColorUnclamped<crate::okhwb::Okhwb<T>> for Hwb<S, T>
where
    crate::hsv::Hsv<S, T>: crate::convert::FromColorUnclamped<crate::okhwb::Okhwb<T>>,
    crate::hsv::Hsv<S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::okhwb::Okhwb<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::hsv::Hsv::<S, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T>
    crate::convert::FromColorUnclamped<
        crate::yxy::Yxy<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    > for Hwb<S, T>
where
    S: crate::rgb::RgbStandard,
    crate::hsv::Hsv<S, T>: crate::convert::FromColorUnclamped<
        crate::yxy::Yxy<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    >,
    crate::hsv::Hsv<S, T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(
        color: crate::yxy::Yxy<
            <<S as crate::rgb::RgbStandard>::Space as crate::rgb::RgbSpace>::WhitePoint,
            T,
        >,
    ) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::hsv::Hsv::<S, T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T, _C, _A> crate::convert::FromColorUnclamped<crate::Alpha<_C, _A>> for Hwb<S, T>
where
    _C: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::Alpha<_C, _A>) -> Self {
        color.color.into_color_unclamped()
    }
}

