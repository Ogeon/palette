// This file is auto-generated and any manual changes to it will be overwritten.
//
// Run `cargo run -p codegen` from the project root to regenerate it.

use super::Luma;

#[automatically_derived]
impl<S, T, _S> crate::convert::FromColorUnclamped<crate::rgb::Rgb<_S, T>> for Luma<S, T>
where
    _S: crate::rgb::RgbStandard,
    _S::Space: crate::rgb::RgbSpace<WhitePoint = <S as crate::luma::LumaStandard>::WhitePoint>,
    S: crate::luma::LumaStandard,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<crate::rgb::Rgb<_S, T>>,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::rgb::Rgb<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<<S as crate::luma::LumaStandard>::WhitePoint, T>::from_color_unclamped(
            color,
        )
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T, _S> crate::convert::FromColorUnclamped<crate::hsl::Hsl<_S, T>> for Luma<S, T>
where
    _S: crate::rgb::RgbStandard,
    _S::Space: crate::rgb::RgbSpace<WhitePoint = <S as crate::luma::LumaStandard>::WhitePoint>,
    S: crate::luma::LumaStandard,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<crate::hsl::Hsl<_S, T>>,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hsl::Hsl<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<<S as crate::luma::LumaStandard>::WhitePoint, T>::from_color_unclamped(
            color,
        )
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T>
    crate::convert::FromColorUnclamped<
        crate::hsluv::Hsluv<<S as crate::luma::LumaStandard>::WhitePoint, T>,
    > for Luma<S, T>
where
    S: crate::luma::LumaStandard,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<
            crate::hsluv::Hsluv<<S as crate::luma::LumaStandard>::WhitePoint, T>,
        >,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(
        color: crate::hsluv::Hsluv<<S as crate::luma::LumaStandard>::WhitePoint, T>,
    ) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<<S as crate::luma::LumaStandard>::WhitePoint, T>::from_color_unclamped(
            color,
        )
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T, _S> crate::convert::FromColorUnclamped<crate::hsv::Hsv<_S, T>> for Luma<S, T>
where
    _S: crate::rgb::RgbStandard,
    _S::Space: crate::rgb::RgbSpace<WhitePoint = <S as crate::luma::LumaStandard>::WhitePoint>,
    S: crate::luma::LumaStandard,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<crate::hsv::Hsv<_S, T>>,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hsv::Hsv<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<<S as crate::luma::LumaStandard>::WhitePoint, T>::from_color_unclamped(
            color,
        )
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T, _S> crate::convert::FromColorUnclamped<crate::hwb::Hwb<_S, T>> for Luma<S, T>
where
    _S: crate::rgb::RgbStandard,
    _S::Space: crate::rgb::RgbSpace<WhitePoint = <S as crate::luma::LumaStandard>::WhitePoint>,
    S: crate::luma::LumaStandard,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<crate::hwb::Hwb<_S, T>>,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::hwb::Hwb<_S, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<<S as crate::luma::LumaStandard>::WhitePoint, T>::from_color_unclamped(
            color,
        )
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T>
    crate::convert::FromColorUnclamped<
        crate::lab::Lab<<S as crate::luma::LumaStandard>::WhitePoint, T>,
    > for Luma<S, T>
where
    S: crate::luma::LumaStandard,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<
            crate::lab::Lab<<S as crate::luma::LumaStandard>::WhitePoint, T>,
        >,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(
        color: crate::lab::Lab<<S as crate::luma::LumaStandard>::WhitePoint, T>,
    ) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<<S as crate::luma::LumaStandard>::WhitePoint, T>::from_color_unclamped(
            color,
        )
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T>
    crate::convert::FromColorUnclamped<
        crate::lch::Lch<<S as crate::luma::LumaStandard>::WhitePoint, T>,
    > for Luma<S, T>
where
    S: crate::luma::LumaStandard,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<
            crate::lch::Lch<<S as crate::luma::LumaStandard>::WhitePoint, T>,
        >,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(
        color: crate::lch::Lch<<S as crate::luma::LumaStandard>::WhitePoint, T>,
    ) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<<S as crate::luma::LumaStandard>::WhitePoint, T>::from_color_unclamped(
            color,
        )
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T>
    crate::convert::FromColorUnclamped<
        crate::lchuv::Lchuv<<S as crate::luma::LumaStandard>::WhitePoint, T>,
    > for Luma<S, T>
where
    S: crate::luma::LumaStandard,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<
            crate::lchuv::Lchuv<<S as crate::luma::LumaStandard>::WhitePoint, T>,
        >,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(
        color: crate::lchuv::Lchuv<<S as crate::luma::LumaStandard>::WhitePoint, T>,
    ) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<<S as crate::luma::LumaStandard>::WhitePoint, T>::from_color_unclamped(
            color,
        )
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T, _LmsM> crate::convert::FromColorUnclamped<crate::lms::Lms<_LmsM, T>> for Luma<S, T>
where
    _LmsM: crate::xyz::meta::HasXyzMeta<XyzMeta = <S as crate::luma::LumaStandard>::WhitePoint>,
    S: crate::luma::LumaStandard,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<crate::lms::Lms<_LmsM, T>>,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::lms::Lms<_LmsM, T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<<S as crate::luma::LumaStandard>::WhitePoint, T>::from_color_unclamped(
            color,
        )
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T>
    crate::convert::FromColorUnclamped<
        crate::luv::Luv<<S as crate::luma::LumaStandard>::WhitePoint, T>,
    > for Luma<S, T>
where
    S: crate::luma::LumaStandard,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<
            crate::luv::Luv<<S as crate::luma::LumaStandard>::WhitePoint, T>,
        >,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(
        color: crate::luv::Luv<<S as crate::luma::LumaStandard>::WhitePoint, T>,
    ) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<<S as crate::luma::LumaStandard>::WhitePoint, T>::from_color_unclamped(
            color,
        )
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T> crate::convert::FromColorUnclamped<crate::oklab::Oklab<T>> for Luma<S, T>
where
    S: crate::luma::LumaStandard,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<crate::oklab::Oklab<T>>,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::oklab::Oklab<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<<S as crate::luma::LumaStandard>::WhitePoint, T>::from_color_unclamped(
            color,
        )
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T> crate::convert::FromColorUnclamped<crate::oklch::Oklch<T>> for Luma<S, T>
where
    S: crate::luma::LumaStandard,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<crate::oklch::Oklch<T>>,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::oklch::Oklch<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<<S as crate::luma::LumaStandard>::WhitePoint, T>::from_color_unclamped(
            color,
        )
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T> crate::convert::FromColorUnclamped<crate::okhsl::Okhsl<T>> for Luma<S, T>
where
    S: crate::luma::LumaStandard,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<crate::okhsl::Okhsl<T>>,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::okhsl::Okhsl<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<<S as crate::luma::LumaStandard>::WhitePoint, T>::from_color_unclamped(
            color,
        )
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T> crate::convert::FromColorUnclamped<crate::okhsv::Okhsv<T>> for Luma<S, T>
where
    S: crate::luma::LumaStandard,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<crate::okhsv::Okhsv<T>>,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::okhsv::Okhsv<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<<S as crate::luma::LumaStandard>::WhitePoint, T>::from_color_unclamped(
            color,
        )
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T> crate::convert::FromColorUnclamped<crate::okhwb::Okhwb<T>> for Luma<S, T>
where
    S: crate::luma::LumaStandard,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::FromColorUnclamped<crate::okhwb::Okhwb<T>>,
    crate::xyz::Xyz<<S as crate::luma::LumaStandard>::WhitePoint, T>:
        crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::okhwb::Okhwb<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::xyz::Xyz::<<S as crate::luma::LumaStandard>::WhitePoint, T>::from_color_unclamped(
            color,
        )
        .into_color_unclamped()
    }
}
#[automatically_derived]
impl<S, T, _C, _A> crate::convert::FromColorUnclamped<crate::Alpha<_C, _A>> for Luma<S, T>
where
    _C: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::Alpha<_C, _A>) -> Self {
        color.color.into_color_unclamped()
    }
}

