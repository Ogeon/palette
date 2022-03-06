use crate::{
    cast::{self, ArrayCast},
    num::{Abs, Arithmetics, MinMax, One, Real, Sqrt, Zero},
    stimulus::{Stimulus, StimulusColor},
    Alpha,
};

use super::{blend_alpha, PreAlpha, Premultiply};

/// A trait for different ways of mixing colors together.
///
/// This implements the classic separable blend modes, [as described by
/// W3C](https://www.w3.org/TR/compositing-1/#blending).
///
/// _Note: The default implementations of the blend modes are meant for color
/// components in the range [0.0, 1.0] and may otherwise produce strange
/// results._
pub trait Blend {
    /// Multiply `self` with `other`. This uses the alpha component to regulate
    /// the effect, so it's not just plain component wise multiplication.
    #[must_use]
    fn multiply(self, other: Self) -> Self;

    /// Make a color which is at least as light as `self` or `other`.
    #[must_use]
    fn screen(self, other: Self) -> Self;

    /// Multiply `self` or `other` if other is dark, or screen them if `other`
    /// is light. This results in an S curve.
    #[must_use]
    fn overlay(self, other: Self) -> Self;

    /// Return the darkest parts of `self` and `other`.
    #[must_use]
    fn darken(self, other: Self) -> Self;

    /// Return the lightest parts of `self` and `other`.
    #[must_use]
    fn lighten(self, other: Self) -> Self;

    /// Lighten `other` to reflect `self`. Results in `other` if `self` is
    /// black.
    #[must_use]
    fn dodge(self, other: Self) -> Self;

    /// Darken `other` to reflect `self`. Results in `other` if `self` is
    /// white.
    #[must_use]
    fn burn(self, other: Self) -> Self;

    /// Multiply `self` or `other` if other is dark, or screen them if `self`
    /// is light. This is similar to `overlay`, but depends on `self` instead
    /// of `other`.
    #[must_use]
    fn hard_light(self, other: Self) -> Self;

    /// Lighten `other` if `self` is light, or darken `other` as if it's burned
    /// if `self` is dark. The effect is increased if the components of `self`
    /// is further from 0.5.
    #[must_use]
    fn soft_light(self, other: Self) -> Self;

    /// Return the absolute difference between `self` and `other`. It's
    /// basically `abs(self - other)`, but regulated by the alpha component.
    #[must_use]
    fn difference(self, other: Self) -> Self;

    /// Similar to `difference`, but appears to result in a lower contrast.
    /// `other` is inverted if `self` is white, and preserved if `self` is
    /// black.
    #[must_use]
    fn exclusion(self, other: Self) -> Self;
}

impl<C, T, const N: usize> Blend for PreAlpha<C>
where
    C: Premultiply<Scalar = T> + StimulusColor + ArrayCast<Array = [T; N]> + Clone,
    T: Real + Zero + One + MinMax + Sqrt + Abs + Arithmetics + PartialOrd + Clone,
{
    #[inline]
    fn multiply(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), multiply_blend)
    }

    #[inline]
    fn screen(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), screen_blend)
    }

    #[inline]
    fn overlay(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), overlay_blend)
    }

    #[inline]
    fn darken(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), darken_blend)
    }

    #[inline]
    fn lighten(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), lighten_blend)
    }

    #[inline]
    fn dodge(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), dodge_blend)
    }

    #[inline]
    fn burn(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), burn_blend)
    }

    #[inline]
    fn hard_light(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), hard_light_blend)
    }

    #[inline]
    fn soft_light(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), soft_light_blend)
    }

    #[inline]
    fn difference(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), difference_blend)
    }

    #[inline]
    fn exclusion(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), exclusion_blend)
    }
}

impl<C, T, const N: usize> Blend for C
where
    C: Premultiply<Scalar = T> + StimulusColor + ArrayCast<Array = [T; N]> + Clone,
    T: Real + Zero + One + MinMax + Sqrt + Abs + Arithmetics + PartialOrd + Clone,
{
    fn multiply(self, other: Self) -> Self {
        let src = BlendInput::new_opaque(self);
        let dst = BlendInput::new_opaque(other);
        blend_separable(src, dst, multiply_blend)
            .unpremultiply()
            .color
    }

    fn screen(self, other: Self) -> Self {
        let src = BlendInput::new_opaque(self);
        let dst = BlendInput::new_opaque(other);
        blend_separable(src, dst, screen_blend)
            .unpremultiply()
            .color
    }

    fn overlay(self, other: Self) -> Self {
        let src = BlendInput::new_opaque(self);
        let dst = BlendInput::new_opaque(other);
        blend_separable(src, dst, overlay_blend)
            .unpremultiply()
            .color
    }

    fn darken(self, other: Self) -> Self {
        let src = BlendInput::new_opaque(self);
        let dst = BlendInput::new_opaque(other);
        blend_separable(src, dst, darken_blend)
            .unpremultiply()
            .color
    }

    fn lighten(self, other: Self) -> Self {
        let src = BlendInput::new_opaque(self);
        let dst = BlendInput::new_opaque(other);
        blend_separable(src, dst, lighten_blend)
            .unpremultiply()
            .color
    }

    fn dodge(self, other: Self) -> Self {
        let src = BlendInput::new_opaque(self);
        let dst = BlendInput::new_opaque(other);
        blend_separable(src, dst, dodge_blend).unpremultiply().color
    }

    fn burn(self, other: Self) -> Self {
        let src = BlendInput::new_opaque(self);
        let dst = BlendInput::new_opaque(other);
        blend_separable(src, dst, burn_blend).unpremultiply().color
    }

    fn hard_light(self, other: Self) -> Self {
        let src = BlendInput::new_opaque(self);
        let dst = BlendInput::new_opaque(other);
        blend_separable(src, dst, hard_light_blend)
            .unpremultiply()
            .color
    }

    fn soft_light(self, other: Self) -> Self {
        let src = BlendInput::new_opaque(self);
        let dst = BlendInput::new_opaque(other);
        blend_separable(src, dst, soft_light_blend)
            .unpremultiply()
            .color
    }

    fn difference(self, other: Self) -> Self {
        let src = BlendInput::new_opaque(self);
        let dst = BlendInput::new_opaque(other);
        blend_separable(src, dst, difference_blend)
            .unpremultiply()
            .color
    }

    fn exclusion(self, other: Self) -> Self {
        let src = BlendInput::new_opaque(self);
        let dst = BlendInput::new_opaque(other);
        blend_separable(src, dst, exclusion_blend)
            .unpremultiply()
            .color
    }
}

impl<C, T, const N: usize> Blend for Alpha<C, T>
where
    C: Premultiply<Scalar = T> + StimulusColor + ArrayCast<Array = [T; N]> + Clone,
    T: Real + Zero + One + MinMax + Sqrt + Abs + Arithmetics + PartialOrd + Clone,
{
    #[inline]
    fn multiply(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), multiply_blend).unpremultiply()
    }

    #[inline]
    fn screen(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), screen_blend).unpremultiply()
    }

    #[inline]
    fn overlay(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), overlay_blend).unpremultiply()
    }

    #[inline]
    fn darken(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), darken_blend).unpremultiply()
    }

    #[inline]
    fn lighten(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), lighten_blend).unpremultiply()
    }

    #[inline]
    fn dodge(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), dodge_blend).unpremultiply()
    }

    #[inline]
    fn burn(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), burn_blend).unpremultiply()
    }

    #[inline]
    fn hard_light(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), hard_light_blend).unpremultiply()
    }

    #[inline]
    fn soft_light(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), soft_light_blend).unpremultiply()
    }

    #[inline]
    fn difference(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), difference_blend).unpremultiply()
    }

    #[inline]
    fn exclusion(self, other: Self) -> Self {
        blend_separable(self.into(), other.into(), exclusion_blend).unpremultiply()
    }
}

struct BlendInput<C: Premultiply> {
    color: C,
    color_pre: C,
    alpha: C::Scalar,
}

impl<C> BlendInput<C>
where
    C: Premultiply + Clone,
{
    fn new_opaque(color: C) -> Self {
        BlendInput {
            color_pre: color.clone(),
            color,
            alpha: C::Scalar::max_intensity(),
        }
    }
}

impl<C> From<Alpha<C, C::Scalar>> for BlendInput<C>
where
    C: Premultiply + Clone,
{
    fn from(color: Alpha<C, C::Scalar>) -> Self {
        let color_pre: PreAlpha<C> = color.color.clone().premultiply(color.alpha);
        BlendInput {
            color: color.color,
            color_pre: color_pre.color,
            alpha: color_pre.alpha,
        }
    }
}

impl<C> From<PreAlpha<C>> for BlendInput<C>
where
    C: Premultiply + Clone,
{
    fn from(color: PreAlpha<C>) -> Self {
        let color_pre = color.color.clone();
        let (color, alpha) = C::unpremultiply(color);
        BlendInput {
            color,
            color_pre,
            alpha,
        }
    }
}

#[inline]
fn multiply_blend<T>(src: T, dst: T) -> T
where
    T: Arithmetics,
{
    src * dst
}

#[inline]
fn screen_blend<T>(src: T, dst: T) -> T
where
    T: Arithmetics + Clone,
{
    src.clone() + &dst - src * dst
}

#[inline]
fn overlay_blend<T>(src: T, dst: T) -> T
where
    T: One + Arithmetics + PartialOrd + Clone,
{
    hard_light_blend(dst, src)
}

#[inline]
fn darken_blend<T>(src: T, dst: T) -> T
where
    T: MinMax,
{
    src.min(dst)
}

#[inline]
fn lighten_blend<T>(src: T, dst: T) -> T
where
    T: MinMax,
{
    src.max(dst)
}

#[inline]
fn dodge_blend<T>(src: T, dst: T) -> T
where
    T: One + Zero + MinMax + Arithmetics + PartialOrd,
{
    // The original algorithm assumes values within [0, 1], but we check for
    // values outside it and clamp.
    if dst <= T::zero() {
        T::zero()
    } else if src >= T::one() {
        T::one()
    } else {
        T::one().min(dst / (T::one() - src))
    }
}

#[inline]
fn burn_blend<T>(src: T, dst: T) -> T
where
    T: One + Zero + MinMax + Arithmetics + PartialOrd,
{
    // The original algorithm assumes values within [0, 1], but we check for
    // values outside it and clamp.
    if dst >= T::one() {
        T::one()
    } else if src <= T::zero() {
        T::zero()
    } else {
        T::one() - T::one().min((T::one() - dst) / src)
    }
}

#[inline]
fn hard_light_blend<T>(src: T, dst: T) -> T
where
    T: One + Arithmetics + PartialOrd + Clone,
{
    let two_src = src.clone() + src;

    if two_src <= T::one() {
        multiply_blend(two_src, dst)
    } else {
        screen_blend(two_src - T::one(), dst)
    }
}

#[inline]
fn soft_light_blend<T>(src: T, dst: T) -> T
where
    T: Real + One + Arithmetics + Sqrt + PartialOrd + Clone,
{
    let four = T::from_f64(4.0);
    let twelve = T::from_f64(12.0);

    let four_dst = dst.clone() * &four;
    let two_src = src.clone() + &src;

    let d_dst = if four_dst <= T::one() {
        let sixteen_dst = four_dst * &four;
        ((sixteen_dst - twelve) * &dst + four) * &dst
    } else {
        dst.clone().sqrt()
    };

    if two_src <= T::one() {
        dst.clone() - (T::one() - two_src) * &dst * (T::one() - dst)
    } else {
        dst.clone() + (two_src - T::one()) * (d_dst - dst)
    }
}

#[inline]
fn difference_blend<T>(src: T, dst: T) -> T
where
    T: Arithmetics + Abs,
{
    (dst - src).abs()
}

#[inline]
fn exclusion_blend<T>(src: T, dst: T) -> T
where
    T: Arithmetics + Clone,
{
    dst.clone() + &src - (dst.clone() + dst) * src
}

#[inline]
fn blend_separable<C, T, F, const N: usize>(
    src: BlendInput<C>,
    mut dst: BlendInput<C>,
    mut blend: F,
) -> PreAlpha<C>
where
    C: ArrayCast<Array = [T; N]> + Premultiply<Scalar = T>,
    T: One + Zero + Arithmetics + PartialOrd + Clone,
    F: FnMut(T, T) -> T,
{
    let src_alpha = src.alpha.clone();
    let zipped_input = zip_input(src, dst.color, &mut dst.color_pre, dst.alpha.clone());

    for (src, src_pre, src_alpha, dst, dst_pre, dst_alpha) in zipped_input {
        *dst_pre = src_pre * (T::one() - &dst_alpha)
            + blend(src, dst) * &src_alpha * dst_alpha
            + (T::one() - src_alpha) * &*dst_pre;
    }

    PreAlpha {
        color: dst.color_pre,
        alpha: blend_alpha(src_alpha, dst.alpha),
    }
}

fn zip_input<'a, C, T, const N: usize>(
    src: BlendInput<C>,
    dst: C,
    dst_pre: &'a mut C,
    dst_alpha: T,
) -> impl Iterator<Item = (T, T, T, T, &'a mut T, T)>
where
    C: ArrayCast<Array = [T; N]> + Premultiply<Scalar = T>,
    T: 'a + Clone,
{
    let src_alpha = src.alpha;
    IntoIterator::into_iter(cast::into_array(src.color))
        .zip(cast::into_array(src.color_pre))
        .zip(cast::into_array(dst))
        .zip(cast::into_array_mut(dst_pre))
        .map(move |(((src_color, src_pre), dst_color), dst_pre)| {
            (
                src_color,
                src_pre,
                src_alpha.clone(),
                dst_color,
                dst_pre,
                dst_alpha.clone(),
            )
        })
}
