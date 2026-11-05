// This file is auto-generated and any manual changes to it will be overwritten.
//
// Run `cargo run -p codegen` from the project root to regenerate it.

use super::Lms;

#[automatically_derived]
impl<M, T, _A> crate::WithAlpha<_A> for Lms<M, T>
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

