use num_traits::{One, Zero};

use crate::blend::{BlendFunction, PreAlpha};
use crate::float::Float;
use crate::{clamp, ComponentWise};

/// A trait for colors that can be blended together.
///
/// Blending can either be performed through the predefined blend modes, or a
/// custom blend functions.
///
/// _Note: The default implementations of the blend modes are meant for color
/// components in the range [0.0, 1.0] and may otherwise produce strange
/// results._
pub trait Blend: Sized
where
    <Self::Color as ComponentWise>::Scalar: Float,
{
    /// The core color type. Typically `Self` for color types without alpha.
    type Color: Blend<Color = Self::Color> + ComponentWise;

    /// Convert the color to premultiplied alpha.
    fn into_premultiplied(self) -> PreAlpha<Self::Color, <Self::Color as ComponentWise>::Scalar>;

    /// Convert the color from premultiplied alpha.
    fn from_premultiplied(
        color: PreAlpha<Self::Color, <Self::Color as ComponentWise>::Scalar>,
    ) -> Self;

    /// Blend self, as the source color, with `destination`, using
    /// `blend_function`. Anything that implements `BlendFunction` is
    /// acceptable, including functions and closures.
    ///
    /// ```
    /// use palette::{LinSrgb, LinSrgba, Blend};
    /// use palette::blend::PreAlpha;
    ///
    /// type PreRgba = PreAlpha<LinSrgb<f32>, f32>;
    ///
    /// fn blend_mode(a: PreRgba, b: PreRgba) -> PreRgba {
    ///    PreAlpha {
    ///        color: LinSrgb::new(a.red * b.green, a.green * b.blue, a.blue * b.red),
    ///        alpha: a.alpha * b.alpha,
    ///    }
    /// }
    ///
    /// let a = LinSrgba::new(0.2, 0.5, 0.1, 0.8);
    /// let b = LinSrgba::new(0.6, 0.3, 0.5, 0.1);
    /// let c = a.blend(b, blend_mode);
    /// ```
    fn blend<F>(self, destination: Self, blend_function: F) -> Self
    where
        F: BlendFunction<Self::Color>,
    {
        Self::from_premultiplied(
            blend_function.apply_to(self.into_premultiplied(), destination.into_premultiplied()),
        )
    }

    /// Place `self` over `other`. This is the good old common alpha
    /// composition equation.
    fn over(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one();
        let zero = <Self::Color as ComponentWise>::Scalar::zero();

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src
                .color
                .component_wise(&dst.color, |a, b| a + b * (one - src.alpha)),
            alpha: clamp(src.alpha + dst.alpha - src.alpha * dst.alpha, zero, one),
        };

        Self::from_premultiplied(result)
    }

    /// Results in the parts of `self` that overlaps the visible parts of
    /// `other`.
    fn inside(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one();
        let zero = <Self::Color as ComponentWise>::Scalar::zero();

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise_self(|a| a * dst.alpha),
            alpha: clamp(src.alpha * dst.alpha, zero, one),
        };

        Self::from_premultiplied(result)
    }

    /// Results in the parts of `self` that lies outside the visible parts of
    /// `other`.
    fn outside(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one();
        let zero = <Self::Color as ComponentWise>::Scalar::zero();

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise_self(|a| a * (one - dst.alpha)),
            alpha: clamp(src.alpha * (one - dst.alpha), zero, one),
        };

        Self::from_premultiplied(result)
    }

    /// Place `self` over only the visible parts of `other`.
    fn atop(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one();
        let zero = <Self::Color as ComponentWise>::Scalar::zero();

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src
                .color
                .component_wise(&dst.color, |a, b| a * dst.alpha + b * (one - src.alpha)),
            alpha: clamp(dst.alpha, zero, one),
        };

        Self::from_premultiplied(result)
    }

    /// Results in either `self` or `other`, where they do not overlap.
    fn xor(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one();
        let zero = <Self::Color as ComponentWise>::Scalar::zero();
        let two = one + one;

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                a * (one - dst.alpha) + b * (one - src.alpha)
            }),
            alpha: clamp(
                src.alpha + dst.alpha - two * src.alpha * dst.alpha,
                zero,
                one,
            ),
        };

        Self::from_premultiplied(result)
    }

    /// Add `self` and `other`. This uses the alpha component to regulate the
    /// effect, so it's not just plain component wise addition.
    fn plus(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one();
        let zero = <Self::Color as ComponentWise>::Scalar::zero();

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| a + b),
            alpha: clamp(src.alpha + dst.alpha, zero, one),
        };

        Self::from_premultiplied(result)
    }

    /// Multiply `self` with `other`. This uses the alpha component to regulate
    /// the effect, so it's not just plain component wise multiplication.
    fn multiply(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one();
        let zero = <Self::Color as ComponentWise>::Scalar::zero();

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                a * b + a * (one - dst.alpha) + b * (one - src.alpha)
            }),
            alpha: clamp(src.alpha + dst.alpha - src.alpha * dst.alpha, zero, one),
        };

        Self::from_premultiplied(result)
    }

    /// Make a color which is at least as light as `self` or `other`.
    fn screen(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one();
        let zero = <Self::Color as ComponentWise>::Scalar::zero();

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| a + b - a * b),
            alpha: clamp(src.alpha + dst.alpha - src.alpha * dst.alpha, zero, one),
        };

        Self::from_premultiplied(result)
    }

    /// Multiply `self` or `other` if other is dark, or screen them if `other`
    /// is light. This results in an S curve.
    fn overlay(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one();
        let zero = <Self::Color as ComponentWise>::Scalar::zero();
        let two = one + one;

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                if b * two <= dst.alpha {
                    two * a * b + a * (one - dst.alpha) + b * (one - src.alpha)
                } else {
                    a * (one + dst.alpha) + b * (one + src.alpha)
                        - two * a * b
                        - src.alpha * dst.alpha
                }
            }),
            alpha: clamp(src.alpha + dst.alpha - src.alpha * dst.alpha, zero, one),
        };

        Self::from_premultiplied(result)
    }

    /// Return the darkest parts of `self` and `other`.
    fn darken(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one();
        let zero = <Self::Color as ComponentWise>::Scalar::zero();

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                (a * dst.alpha).min(b * src.alpha) + a * (one - dst.alpha) + b * (one - src.alpha)
            }),
            alpha: clamp(src.alpha + dst.alpha - src.alpha * dst.alpha, zero, one),
        };

        Self::from_premultiplied(result)
    }

    /// Return the lightest parts of `self` and `other`.
    fn lighten(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one();
        let zero = <Self::Color as ComponentWise>::Scalar::zero();

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                (a * dst.alpha).max(b * src.alpha) + a * (one - dst.alpha) + b * (one - src.alpha)
            }),
            alpha: clamp(src.alpha + dst.alpha - src.alpha * dst.alpha, zero, one),
        };

        Self::from_premultiplied(result)
    }

    /// Lighten `other` to reflect `self`. Results in `other` if `self` is
    /// black.
    fn dodge(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one();
        let zero = <Self::Color as ComponentWise>::Scalar::zero();

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                if a == src.alpha && !b.is_normal() {
                    a * (one - dst.alpha)
                } else if a == src.alpha {
                    src.alpha * dst.alpha + a * (one - dst.alpha) + b * (one - src.alpha)
                } else {
                    src.alpha * dst.alpha * one.min((b / dst.alpha) * src.alpha / (src.alpha - a))
                        + a * (one - dst.alpha)
                        + b * (one - src.alpha)
                }
            }),
            alpha: clamp(src.alpha + dst.alpha - src.alpha * dst.alpha, zero, one),
        };

        Self::from_premultiplied(result)
    }

    /// Darken `other` to reflect `self`. Results in `other` if `self` is
    /// white.
    fn burn(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one();
        let zero = <Self::Color as ComponentWise>::Scalar::zero();

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                if !a.is_normal() && b == dst.alpha {
                    src.alpha * dst.alpha + b * (one - src.alpha)
                } else if !a.is_normal() {
                    b * (one - src.alpha)
                } else {
                    src.alpha * dst.alpha * (one - one.min((one - b / dst.alpha) * src.alpha / a))
                        + a * (one - dst.alpha)
                        + b * (one - src.alpha)
                }
            }),
            alpha: clamp(src.alpha + dst.alpha - src.alpha * dst.alpha, zero, one),
        };

        Self::from_premultiplied(result)
    }

    /// Multiply `self` or `other` if other is dark, or screen them if `self`
    /// is light. This is similar to `overlay`, but depends on `self` instead
    /// of `other`.
    fn hard_light(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one();
        let zero = <Self::Color as ComponentWise>::Scalar::zero();
        let two = one + one;

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                if a * two <= src.alpha {
                    two * a * b + a * (one - dst.alpha) + b * (one - src.alpha)
                } else {
                    a * (one + dst.alpha) + b * (one + src.alpha)
                        - two * a * b
                        - src.alpha * dst.alpha
                }
            }),
            alpha: clamp(src.alpha + dst.alpha - src.alpha * dst.alpha, zero, one),
        };

        Self::from_premultiplied(result)
    }

    /// Lighten `other` if `self` is light, or darken `other` as if it's burned
    /// if `self` is dark. The effect is increased if the components of `self`
    /// is further from 0.5.
    fn soft_light(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one();
        let zero = <Self::Color as ComponentWise>::Scalar::zero();
        let two = one + one;
        let three = two + one;
        let four = two + two;
        let twelve = four + four + four;
        let sixteen = twelve + four;

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                let m = if dst.alpha.is_normal() {
                    b / dst.alpha
                } else {
                    zero
                };

                if a * two <= src.alpha {
                    b * (src.alpha + (two * a - src.alpha) * (one - m))
                        + a * (one - dst.alpha)
                        + b * (one - src.alpha)
                } else if b * four <= dst.alpha {
                    let m2 = m * m;
                    let m3 = m2 * m;

                    dst.alpha * (two * a - src.alpha) * (m3 * sixteen - m2 * twelve - m * three) + a
                        - a * dst.alpha
                        + b
                } else {
                    dst.alpha * (two * a - src.alpha) * (m.sqrt() - m) + a - a * dst.alpha + b
                }
            }),
            alpha: clamp(src.alpha + dst.alpha - src.alpha * dst.alpha, zero, one),
        };

        Self::from_premultiplied(result)
    }

    /// Return the absolute difference between `self` and `other`. It's
    /// basically `abs(self - other)`, but regulated by the alpha component.
    fn difference(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one();
        let zero = <Self::Color as ComponentWise>::Scalar::zero();
        let two = one + one;

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                a + b - two * (a * dst.alpha).min(b * src.alpha)
            }),
            alpha: clamp(src.alpha + dst.alpha - src.alpha * dst.alpha, zero, one),
        };

        Self::from_premultiplied(result)
    }

    /// Similar to `difference`, but appears to result in a lower contrast.
    /// `other` is inverted if `self` is white, and preserved if `self` is
    /// black.
    fn exclusion(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one();
        let zero = <Self::Color as ComponentWise>::Scalar::zero();
        let two = one + one;

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src
                .color
                .component_wise(&dst.color, |a, b| a + b - two * a * b),
            alpha: clamp(src.alpha + dst.alpha - src.alpha * dst.alpha, zero, one),
        };

        Self::from_premultiplied(result)
    }
}
