// This file is auto-generated and any manual changes to it will be overwritten.
//
// Run `cargo run -p codegen` from the project root to regenerate it.

use super::{Cam16Jch, Cam16Jmh, Cam16Jsh, Cam16Qch, Cam16Qmh, Cam16Qsh, Cam16UcsJab, Cam16UcsJmh};

#[automatically_derived]
impl<T, _C, _A> crate::convert::FromColorUnclamped<crate::Alpha<_C, _A>> for Cam16Jch<T>
where
    _C: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::Alpha<_C, _A>) -> Self {
        color.color.into_color_unclamped()
    }
}

#[automatically_derived]
impl<T> crate::convert::FromColorUnclamped<crate::cam16::Cam16UcsJab<T>> for Cam16Jmh<T>
where
    crate::cam16::Cam16UcsJmh<T>: crate::convert::FromColorUnclamped<crate::cam16::Cam16UcsJab<T>>,
    crate::cam16::Cam16UcsJmh<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::cam16::Cam16UcsJab<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::cam16::Cam16UcsJmh::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T, _C, _A> crate::convert::FromColorUnclamped<crate::Alpha<_C, _A>> for Cam16Jmh<T>
where
    _C: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::Alpha<_C, _A>) -> Self {
        color.color.into_color_unclamped()
    }
}

#[automatically_derived]
impl<T> crate::convert::FromColorUnclamped<crate::cam16::Cam16<T>> for Cam16UcsJmh<T>
where
    crate::cam16::Cam16Jmh<T>: crate::convert::FromColorUnclamped<crate::cam16::Cam16<T>>,
    crate::cam16::Cam16Jmh<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::cam16::Cam16<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::cam16::Cam16Jmh::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T, _C, _A> crate::convert::FromColorUnclamped<crate::Alpha<_C, _A>> for Cam16UcsJmh<T>
where
    _C: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::Alpha<_C, _A>) -> Self {
        color.color.into_color_unclamped()
    }
}

#[automatically_derived]
impl<T> crate::convert::FromColorUnclamped<crate::cam16::Cam16Jmh<T>> for Cam16UcsJab<T>
where
    crate::cam16::Cam16UcsJmh<T>: crate::convert::FromColorUnclamped<crate::cam16::Cam16Jmh<T>>,
    crate::cam16::Cam16UcsJmh<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::cam16::Cam16Jmh<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::cam16::Cam16UcsJmh::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T> crate::convert::FromColorUnclamped<crate::cam16::Cam16<T>> for Cam16UcsJab<T>
where
    crate::cam16::Cam16UcsJmh<T>: crate::convert::FromColorUnclamped<crate::cam16::Cam16<T>>,
    crate::cam16::Cam16UcsJmh<T>: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::cam16::Cam16<T>) -> Self {
        use crate::convert::IntoColorUnclamped;
        crate::cam16::Cam16UcsJmh::<T>::from_color_unclamped(color).into_color_unclamped()
    }
}
#[automatically_derived]
impl<T, _C, _A> crate::convert::FromColorUnclamped<crate::Alpha<_C, _A>> for Cam16UcsJab<T>
where
    _C: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::Alpha<_C, _A>) -> Self {
        color.color.into_color_unclamped()
    }
}

#[automatically_derived]
impl<T, _C, _A> crate::convert::FromColorUnclamped<crate::Alpha<_C, _A>> for Cam16Jsh<T>
where
    _C: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::Alpha<_C, _A>) -> Self {
        color.color.into_color_unclamped()
    }
}

#[automatically_derived]
impl<T, _C, _A> crate::convert::FromColorUnclamped<crate::Alpha<_C, _A>> for Cam16Qch<T>
where
    _C: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::Alpha<_C, _A>) -> Self {
        color.color.into_color_unclamped()
    }
}

#[automatically_derived]
impl<T, _C, _A> crate::convert::FromColorUnclamped<crate::Alpha<_C, _A>> for Cam16Qmh<T>
where
    _C: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::Alpha<_C, _A>) -> Self {
        color.color.into_color_unclamped()
    }
}

#[automatically_derived]
impl<T, _C, _A> crate::convert::FromColorUnclamped<crate::Alpha<_C, _A>> for Cam16Qsh<T>
where
    _C: crate::convert::IntoColorUnclamped<Self>,
{
    #[inline]
    fn from_color_unclamped(color: crate::Alpha<_C, _A>) -> Self {
        color.color.into_color_unclamped()
    }
}

