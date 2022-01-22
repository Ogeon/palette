use crate::{
    blend::{BlendFunction, PreAlpha},
    clamp,
    num::{Arithmetics, IsValidDivisor, MinMax, One, Real, Sqrt, Zero},
    ComponentWise,
};

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
    <Self::Color as ComponentWise>::Scalar:
        Real + One + Zero + MinMax + Sqrt + IsValidDivisor + Arithmetics + PartialOrd + Clone,
{
    /// The core color type. Typically `Self` for color types without alpha.
    type Color: Blend<Color = Self::Color> + ComponentWise;

    /// Convert the color to premultiplied alpha.
    #[must_use]
    fn into_premultiplied(self) -> PreAlpha<Self::Color, <Self::Color as ComponentWise>::Scalar>;

    /// Convert the color from premultiplied alpha.
    #[must_use]
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
    #[must_use]
    #[inline]
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
    #[must_use]
    #[inline]
    fn over(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one;
        let zero = <Self::Color as ComponentWise>::Scalar::zero;

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src
                .color
                .component_wise(&dst.color, |a, b| a + b * (one() - &src.alpha)),
            alpha: clamp(
                src.alpha.clone() + &dst.alpha - src.alpha * dst.alpha,
                zero(),
                one(),
            ),
        };

        Self::from_premultiplied(result)
    }

    /// Results in the parts of `self` that overlaps the visible parts of
    /// `other`.
    #[must_use]
    #[inline]
    fn inside(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one();
        let zero = <Self::Color as ComponentWise>::Scalar::zero();

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise_self(|a| a * &dst.alpha),
            alpha: clamp(src.alpha * dst.alpha, zero, one),
        };

        Self::from_premultiplied(result)
    }

    /// Results in the parts of `self` that lies outside the visible parts of
    /// `other`.
    #[must_use]
    #[inline]
    fn outside(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one;
        let zero = <Self::Color as ComponentWise>::Scalar::zero;

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise_self(|a| a * (one() - &dst.alpha)),
            alpha: clamp(src.alpha * (one() - dst.alpha), zero(), one()),
        };

        Self::from_premultiplied(result)
    }

    /// Place `self` over only the visible parts of `other`.
    #[must_use]
    #[inline]
    fn atop(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one;
        let zero = <Self::Color as ComponentWise>::Scalar::zero;

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src
                .color
                .component_wise(&dst.color, |a, b| a * &dst.alpha + b * (one() - &src.alpha)),
            alpha: clamp(dst.alpha, zero(), one()),
        };

        Self::from_premultiplied(result)
    }

    /// Results in either `self` or `other`, where they do not overlap.
    #[must_use]
    #[inline]
    fn xor(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one;
        let zero = <Self::Color as ComponentWise>::Scalar::zero;
        let two = || one() + one();

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                a * (one() - &dst.alpha) + b * (one() - &src.alpha)
            }),
            alpha: clamp(
                src.alpha.clone() + &dst.alpha - two() * src.alpha * dst.alpha,
                zero(),
                one(),
            ),
        };

        Self::from_premultiplied(result)
    }

    /// Add `self` and `other`. This uses the alpha component to regulate the
    /// effect, so it's not just plain component wise addition.
    #[must_use]
    #[inline]
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
    #[must_use]
    #[inline]
    fn multiply(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one;
        let zero = <Self::Color as ComponentWise>::Scalar::zero;

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                a.clone() * &b + a * (one() - &dst.alpha) + b * (one() - &src.alpha)
            }),
            alpha: clamp(
                src.alpha.clone() + &dst.alpha - src.alpha * dst.alpha,
                zero(),
                one(),
            ),
        };

        Self::from_premultiplied(result)
    }

    /// Make a color which is at least as light as `self` or `other`.
    #[must_use]
    #[inline]
    fn screen(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one();
        let zero = <Self::Color as ComponentWise>::Scalar::zero();

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src
                .color
                .component_wise(&dst.color, |a, b| a.clone() + &b - a * b),
            alpha: clamp(
                src.alpha.clone() + &dst.alpha - src.alpha * dst.alpha,
                zero,
                one,
            ),
        };

        Self::from_premultiplied(result)
    }

    /// Multiply `self` or `other` if other is dark, or screen them if `other`
    /// is light. This results in an S curve.
    #[must_use]
    #[inline]
    fn overlay(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one;
        let zero = <Self::Color as ComponentWise>::Scalar::zero;
        let two = || one() + one();

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                if two() * &b <= dst.alpha {
                    two() * &a * &b + a * (one() - &dst.alpha) + b * (one() - &src.alpha)
                } else {
                    a.clone() * (one() + &dst.alpha) + b.clone() * (one() + &src.alpha)
                        - two() * a * b
                        - src.alpha.clone() * &dst.alpha
                }
            }),
            alpha: clamp(
                src.alpha.clone() + &dst.alpha - src.alpha * dst.alpha,
                zero(),
                one(),
            ),
        };

        Self::from_premultiplied(result)
    }

    /// Return the darkest parts of `self` and `other`.
    #[must_use]
    #[inline]
    fn darken(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one;
        let zero = <Self::Color as ComponentWise>::Scalar::zero;

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                (a.clone() * &dst.alpha).min(b.clone() * &src.alpha)
                    + a * (one() - &dst.alpha)
                    + b * (one() - &src.alpha)
            }),
            alpha: clamp(
                src.alpha.clone() + &dst.alpha - src.alpha * dst.alpha,
                zero(),
                one(),
            ),
        };

        Self::from_premultiplied(result)
    }

    /// Return the lightest parts of `self` and `other`.
    #[must_use]
    #[inline]
    fn lighten(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one;
        let zero = <Self::Color as ComponentWise>::Scalar::zero;

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                (a.clone() * &dst.alpha).max(b.clone() * &src.alpha)
                    + a * (one() - &dst.alpha)
                    + b * (one() - &src.alpha)
            }),
            alpha: clamp(
                src.alpha.clone() + &dst.alpha - src.alpha * dst.alpha,
                zero(),
                one(),
            ),
        };

        Self::from_premultiplied(result)
    }

    /// Lighten `other` to reflect `self`. Results in `other` if `self` is
    /// black.
    #[must_use]
    #[inline]
    fn dodge(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one;
        let zero = <Self::Color as ComponentWise>::Scalar::zero;

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                if a == src.alpha && !b.is_valid_divisor() {
                    a * (one() - &dst.alpha)
                } else if a == src.alpha {
                    src.alpha.clone() * &dst.alpha
                        + a * (one() - &dst.alpha)
                        + b * (one() - &src.alpha)
                } else {
                    src.alpha.clone()
                        * &dst.alpha
                        * one()
                            .min((b.clone() / &dst.alpha) * &src.alpha / (src.alpha.clone() - &a))
                        + a * (one() - &dst.alpha)
                        + b * (one() - &src.alpha)
                }
            }),
            alpha: clamp(
                src.alpha.clone() + &dst.alpha - src.alpha * dst.alpha,
                zero(),
                one(),
            ),
        };

        Self::from_premultiplied(result)
    }

    /// Darken `other` to reflect `self`. Results in `other` if `self` is
    /// white.
    #[must_use]
    #[inline]
    fn burn(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one;
        let zero = <Self::Color as ComponentWise>::Scalar::zero;

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                if !a.is_valid_divisor() && b == dst.alpha {
                    src.alpha.clone() * &dst.alpha + b * (one() - &src.alpha)
                } else if !a.is_valid_divisor() {
                    b * (one() - &src.alpha)
                } else {
                    src.alpha.clone()
                        * &dst.alpha
                        * (one() - one().min((one() - b.clone() / &dst.alpha) * &src.alpha / &a))
                        + a * (one() - &dst.alpha)
                        + b * (one() - &src.alpha)
                }
            }),
            alpha: clamp(
                src.alpha.clone() + &dst.alpha - src.alpha * dst.alpha,
                zero(),
                one(),
            ),
        };

        Self::from_premultiplied(result)
    }

    /// Multiply `self` or `other` if other is dark, or screen them if `self`
    /// is light. This is similar to `overlay`, but depends on `self` instead
    /// of `other`.
    #[must_use]
    #[inline]
    fn hard_light(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one;
        let zero = <Self::Color as ComponentWise>::Scalar::zero;
        let two = || one() + one();

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                if two() * &a <= src.alpha {
                    two() * &a * &b + a * (one() - &dst.alpha) + b * (one() - &src.alpha)
                } else {
                    a.clone() * (one() + &dst.alpha) + b.clone() * (one() + &src.alpha)
                        - two() * a * b
                        - src.alpha.clone() * &dst.alpha
                }
            }),
            alpha: clamp(
                src.alpha.clone() + &dst.alpha - src.alpha * dst.alpha,
                zero(),
                one(),
            ),
        };

        Self::from_premultiplied(result)
    }

    /// Lighten `other` if `self` is light, or darken `other` as if it's burned
    /// if `self` is dark. The effect is increased if the components of `self`
    /// is further from 0.5.
    #[must_use]
    #[inline]
    fn soft_light(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one;
        let zero = <Self::Color as ComponentWise>::Scalar::zero;
        let two = || one() + one();
        let three = || two() + one();
        let four = || two() + two();
        let twelve = || four() + four() + four();
        let sixteen = || twelve() + four();

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                let m = if dst.alpha.is_valid_divisor() {
                    b.clone() / &dst.alpha
                } else {
                    zero()
                };

                if two() * &a <= src.alpha {
                    b.clone() * (src.alpha.clone() + (two() * &a - &src.alpha) * (one() - m))
                        + a * (one() - &dst.alpha)
                        + b * (one() - &src.alpha)
                } else if four() * &b <= dst.alpha {
                    let m2 = m.clone() * &m;
                    let m3 = m2.clone() * &m;

                    dst.alpha.clone()
                        * (two() * &a - &src.alpha)
                        * (m3 * sixteen() - m2 * twelve() - m * three())
                        + &a
                        - a * &dst.alpha
                        + b
                } else {
                    dst.alpha.clone() * (two() * &a - &src.alpha) * (m.clone().sqrt() - m) + &a
                        - a * &dst.alpha
                        + b
                }
            }),
            alpha: clamp(
                src.alpha.clone() + &dst.alpha - src.alpha * dst.alpha,
                zero(),
                one(),
            ),
        };

        Self::from_premultiplied(result)
    }

    /// Return the absolute difference between `self` and `other`. It's
    /// basically `abs(self - other)`, but regulated by the alpha component.
    #[must_use]
    #[inline]
    fn difference(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one;
        let zero = <Self::Color as ComponentWise>::Scalar::zero;
        let two = || one() + one();

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src.color.component_wise(&dst.color, |a, b| {
                a.clone() + &b - two() * (a * &dst.alpha).min(b * &src.alpha)
            }),
            alpha: clamp(
                src.alpha.clone() + &dst.alpha - src.alpha * dst.alpha,
                zero(),
                one(),
            ),
        };

        Self::from_premultiplied(result)
    }

    /// Similar to `difference`, but appears to result in a lower contrast.
    /// `other` is inverted if `self` is white, and preserved if `self` is
    /// black.
    #[must_use]
    #[inline]
    fn exclusion(self, other: Self) -> Self {
        let one = <Self::Color as ComponentWise>::Scalar::one;
        let zero = <Self::Color as ComponentWise>::Scalar::zero;
        let two = || one() + one();

        let src = self.into_premultiplied();
        let dst = other.into_premultiplied();

        let result = PreAlpha {
            color: src
                .color
                .component_wise(&dst.color, |a, b| a.clone() + &b - two() * a * b),
            alpha: clamp(
                src.alpha.clone() + &dst.alpha - src.alpha * dst.alpha,
                zero(),
                one(),
            ),
        };

        Self::from_premultiplied(result)
    }
}
