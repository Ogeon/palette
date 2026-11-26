// This file is auto-generated and any manual changes to it will be overwritten.
//
// Run `cargo run -p codegen` from the project root to regenerate it.

use super::{
    Cam16, Cam16Jch, Cam16Jmh, Cam16Jsh, Cam16Qch, Cam16Qmh, Cam16Qsh, Cam16UcsJab, Cam16UcsJmh,
};

#[automatically_derived]
impl<T, _A> crate::WithAlpha<_A> for Cam16<T>
where
    _A: crate::stimulus::Stimulus,
{
    type Color = Self;
    type WithAlpha = crate::Alpha<Self, _A>;
    #[inline]
    fn with_alpha(self, alpha: _A) -> Self::WithAlpha {
        crate::Alpha { color: self, alpha }
    }
    #[inline]
    fn without_alpha(self) -> Self::Color {
        self
    }
    #[inline]
    fn split(self) -> (Self::Color, _A) {
        (self, crate::stimulus::Stimulus::max_intensity())
    }
}

#[automatically_derived]
impl<T, _A> crate::WithAlpha<_A> for Cam16Jch<T>
where
    _A: crate::stimulus::Stimulus,
{
    type Color = Self;
    type WithAlpha = crate::Alpha<Self, _A>;
    #[inline]
    fn with_alpha(self, alpha: _A) -> Self::WithAlpha {
        crate::Alpha { color: self, alpha }
    }
    #[inline]
    fn without_alpha(self) -> Self::Color {
        self
    }
    #[inline]
    fn split(self) -> (Self::Color, _A) {
        (self, crate::stimulus::Stimulus::max_intensity())
    }
}

#[automatically_derived]
impl<T, _A> crate::WithAlpha<_A> for Cam16Jmh<T>
where
    _A: crate::stimulus::Stimulus,
{
    type Color = Self;
    type WithAlpha = crate::Alpha<Self, _A>;
    #[inline]
    fn with_alpha(self, alpha: _A) -> Self::WithAlpha {
        crate::Alpha { color: self, alpha }
    }
    #[inline]
    fn without_alpha(self) -> Self::Color {
        self
    }
    #[inline]
    fn split(self) -> (Self::Color, _A) {
        (self, crate::stimulus::Stimulus::max_intensity())
    }
}

#[automatically_derived]
impl<T, _A> crate::WithAlpha<_A> for Cam16UcsJmh<T>
where
    _A: crate::stimulus::Stimulus,
{
    type Color = Self;
    type WithAlpha = crate::Alpha<Self, _A>;
    #[inline]
    fn with_alpha(self, alpha: _A) -> Self::WithAlpha {
        crate::Alpha { color: self, alpha }
    }
    #[inline]
    fn without_alpha(self) -> Self::Color {
        self
    }
    #[inline]
    fn split(self) -> (Self::Color, _A) {
        (self, crate::stimulus::Stimulus::max_intensity())
    }
}

#[automatically_derived]
impl<T, _A> crate::WithAlpha<_A> for Cam16UcsJab<T>
where
    _A: crate::stimulus::Stimulus,
{
    type Color = Self;
    type WithAlpha = crate::Alpha<Self, _A>;
    #[inline]
    fn with_alpha(self, alpha: _A) -> Self::WithAlpha {
        crate::Alpha { color: self, alpha }
    }
    #[inline]
    fn without_alpha(self) -> Self::Color {
        self
    }
    #[inline]
    fn split(self) -> (Self::Color, _A) {
        (self, crate::stimulus::Stimulus::max_intensity())
    }
}

#[automatically_derived]
impl<T, _A> crate::WithAlpha<_A> for Cam16Jsh<T>
where
    _A: crate::stimulus::Stimulus,
{
    type Color = Self;
    type WithAlpha = crate::Alpha<Self, _A>;
    #[inline]
    fn with_alpha(self, alpha: _A) -> Self::WithAlpha {
        crate::Alpha { color: self, alpha }
    }
    #[inline]
    fn without_alpha(self) -> Self::Color {
        self
    }
    #[inline]
    fn split(self) -> (Self::Color, _A) {
        (self, crate::stimulus::Stimulus::max_intensity())
    }
}

#[automatically_derived]
impl<T, _A> crate::WithAlpha<_A> for Cam16Qch<T>
where
    _A: crate::stimulus::Stimulus,
{
    type Color = Self;
    type WithAlpha = crate::Alpha<Self, _A>;
    #[inline]
    fn with_alpha(self, alpha: _A) -> Self::WithAlpha {
        crate::Alpha { color: self, alpha }
    }
    #[inline]
    fn without_alpha(self) -> Self::Color {
        self
    }
    #[inline]
    fn split(self) -> (Self::Color, _A) {
        (self, crate::stimulus::Stimulus::max_intensity())
    }
}

#[automatically_derived]
impl<T, _A> crate::WithAlpha<_A> for Cam16Qmh<T>
where
    _A: crate::stimulus::Stimulus,
{
    type Color = Self;
    type WithAlpha = crate::Alpha<Self, _A>;
    #[inline]
    fn with_alpha(self, alpha: _A) -> Self::WithAlpha {
        crate::Alpha { color: self, alpha }
    }
    #[inline]
    fn without_alpha(self) -> Self::Color {
        self
    }
    #[inline]
    fn split(self) -> (Self::Color, _A) {
        (self, crate::stimulus::Stimulus::max_intensity())
    }
}

#[automatically_derived]
impl<T, _A> crate::WithAlpha<_A> for Cam16Qsh<T>
where
    _A: crate::stimulus::Stimulus,
{
    type Color = Self;
    type WithAlpha = crate::Alpha<Self, _A>;
    #[inline]
    fn with_alpha(self, alpha: _A) -> Self::WithAlpha {
        crate::Alpha { color: self, alpha }
    }
    #[inline]
    fn without_alpha(self) -> Self::Color {
        self
    }
    #[inline]
    fn split(self) -> (Self::Color, _A) {
        (self, crate::stimulus::Stimulus::max_intensity())
    }
}

