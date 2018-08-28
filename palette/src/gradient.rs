//!Types for interpolation between multiple colors.
//!
//!This module is only available if the `std` feature is enabled (this is the
//!default).

use num_traits::{One, Zero};
use float::Float;
use std::cmp::max;
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use cast;

use Mix;

///A linear interpolation between colors.
///
///It's used to smoothly transition between a series of colors, that can be
///either evenly spaced or have customized positions. The gradient is
///continuous between the control points, but it's possible to iterate over a
///number of evenly spaced points using the `take` method. Any point outside
///the domain of the gradient will have the same color as the closest control
///point.
#[derive(Clone, Debug)]
pub struct Gradient<C: Mix + Clone>(Vec<(C::Scalar, C)>);

impl<C: Mix + Clone> Gradient<C> {
    ///Create a gradient of evenly spaced colors with the domain [0.0, 1.0].
    ///There must be at least one color.
    pub fn new<I: IntoIterator<Item = C>>(colors: I) -> Gradient<C> {
        let mut points: Vec<_> = colors.into_iter().map(|c| (C::Scalar::zero(), c)).collect();
        assert!(points.len() > 0);
        let step_size = C::Scalar::one() / cast(max(points.len() - 1, 1) as f64);

        for (i, &mut (ref mut p, _)) in points.iter_mut().enumerate() {
            *p = cast::<C::Scalar, _>(i) * step_size;
        }

        Gradient(points)
    }

    ///Create a gradient of colors with custom spacing and domain. There must be
    ///at least one color and they are expected to be ordered by their
    ///position value.
    pub fn with_domain(colors: Vec<(C::Scalar, C)>) -> Gradient<C> {
        assert!(colors.len() > 0);

        //Maybe sort the colors?
        Gradient(colors)
    }

    ///Get a color from the gradient. The color of the closest control point
    ///will be returned if `i` is outside the domain.
    pub fn get(&self, i: C::Scalar) -> C {
        let &(mut min, ref min_color) = self.0
            .get(0)
            .expect("a Gradient must contain at least one color");
        let mut min_color = min_color;
        let mut min_index = 0;

        if i <= min {
            return min_color.clone();
        }

        let &(mut max, ref max_color) = self.0
            .last()
            .expect("a Gradient must contain at least one color");
        let mut max_color = max_color;
        let mut max_index = self.0.len() - 1;

        if i >= max {
            return max_color.clone();
        }

        while min_index < max_index - 1 {
            let index = min_index + (max_index - min_index) / 2;

            let (p, ref color) = self.0[index];

            if i <= p {
                max = p;
                max_color = color;
                max_index = index;
            } else {
                min = p;
                min_color = color;
                min_index = index;
            }
        }

        let factor = (i - min) / (max - min);

        min_color.mix(max_color, factor)
    }

    ///Take `n` evenly spaced colors from the gradient, as an iterator.
    pub fn take(&self, n: usize) -> Take<C> {
        let (min, max) = self.domain();

        Take {
            gradient: MaybeSlice::NotSlice(self),
            from: min,
            diff: max - min,
            len: n,
            current: 0,
        }
    }

    ///Slice this gradient to limit its domain.
    pub fn slice<R: Into<Range<C::Scalar>>>(&self, range: R) -> Slice<C> {
        Slice {
            gradient: self,
            range: range.into(),
        }
    }

    ///Get the limits of this gradient's domain.
    pub fn domain(&self) -> (C::Scalar, C::Scalar) {
        let &(min, _) = self.0
            .get(0)
            .expect("a Gradient must contain at least one color");
        let &(max, _) = self.0
            .last()
            .expect("a Gradient must contain at least one color");
        (min, max)
    }
}

///An iterator over interpolated colors.
#[derive(Clone)]
pub struct Take<'a, C: Mix + Clone + 'a> {
    gradient: MaybeSlice<'a, C>,
    from: C::Scalar,
    diff: C::Scalar,
    len: usize,
    current: usize,
}

impl<'a, C: Mix + Clone> Iterator for Take<'a, C> {
    type Item = C;

    fn next(&mut self) -> Option<C> {
        if self.current < self.len {
            let i = self.from + (self.diff / cast(self.len)) * cast(self.current);
            self.current += 1;
            Some(self.gradient.get(i))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len - self.current, Some(self.len - self.current))
    }
}

impl<'a, C: Mix + Clone> ExactSizeIterator for Take<'a, C> {}

///A slice of a Gradient that limits its domain.
#[derive(Clone, Debug)]
pub struct Slice<'a, C: Mix + Clone + 'a> {
    gradient: &'a Gradient<C>,
    range: Range<C::Scalar>,
}

impl<'a, C: Mix + Clone> Slice<'a, C> {
    ///Get a color from the gradient slice. The color of the closest domain
    ///limit will be returned if `i` is outside the domain.
    pub fn get(&self, i: C::Scalar) -> C {
        self.gradient.get(self.range.clamp(i))
    }

    ///Take `n` evenly spaced colors from the gradient slice, as an iterator.
    pub fn take(&self, n: usize) -> Take<C> {
        let (min, max) = self.domain();

        Take {
            gradient: MaybeSlice::Slice(self.clone()),
            from: min,
            diff: max - min,
            len: n,
            current: 0,
        }
    }

    ///Slice this gradient slice to further limit its domain. Ranges outside
    ///the domain will be clamped to the nearest domain limit.
    pub fn slice<R: Into<Range<C::Scalar>>>(&self, range: R) -> Slice<C> {
        Slice {
            gradient: self.gradient,
            range: self.range.constrain(&range.into()),
        }
    }

    ///Get the limits of this gradient slice's domain.
    pub fn domain(&self) -> (C::Scalar, C::Scalar) {
        if let Range {
            from: Some(from),
            to: Some(to),
        } = self.range
        {
            (from, to)
        } else {
            let (from, to) = self.gradient.domain();
            (self.range.from.unwrap_or(from), self.range.to.unwrap_or(to))
        }
    }
}

///A domain range for gradient slices.
#[derive(Clone, Debug, PartialEq)]
pub struct Range<T: Float> {
    from: Option<T>,
    to: Option<T>,
}

impl<T: Float> Range<T> {
    fn clamp(&self, mut x: T) -> T {
        x = self.from.unwrap_or(x).max(x);
        self.to.unwrap_or(x).min(x)
    }

    fn constrain(&self, other: &Range<T>) -> Range<T> {
        if let (Some(f), Some(t)) = (other.from, self.to) {
            if f >= t {
                return Range {
                    from: self.to,
                    to: self.to,
                };
            }
        }

        if let (Some(t), Some(f)) = (other.to, self.from) {
            if t <= f {
                return Range {
                    from: self.from,
                    to: self.from,
                };
            }
        }

        Range {
            from: match (self.from, other.from) {
                (Some(s), Some(o)) => Some(s.max(o)),
                (Some(s), None) => Some(s),
                (None, Some(o)) => Some(o),
                (None, None) => None,
            },
            to: match (self.to, other.to) {
                (Some(s), Some(o)) => Some(s.min(o)),
                (Some(s), None) => Some(s),
                (None, Some(o)) => Some(o),
                (None, None) => None,
            },
        }
    }
}

impl<T: Float> From<::std::ops::Range<T>> for Range<T> {
    fn from(range: ::std::ops::Range<T>) -> Range<T> {
        Range {
            from: Some(range.start),
            to: Some(range.end),
        }
    }
}

impl<T: Float> From<::std::ops::RangeFrom<T>> for Range<T> {
    fn from(range: ::std::ops::RangeFrom<T>) -> Range<T> {
        Range {
            from: Some(range.start),
            to: None,
        }
    }
}

impl<T: Float> From<::std::ops::RangeTo<T>> for Range<T> {
    fn from(range: ::std::ops::RangeTo<T>) -> Range<T> {
        Range {
            from: None,
            to: Some(range.end),
        }
    }
}

impl<T: Float> From<::std::ops::RangeFull> for Range<T> {
    fn from(_range: ::std::ops::RangeFull) -> Range<T> {
        Range {
            from: None,
            to: None,
        }
    }
}

impl<T> AbsDiffEq for Range<T>
where
    T: AbsDiffEq + Float,
    T::Epsilon: Copy,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
        let from = match (self.from, other.from) {
            (Some(s), Some(o)) => s.abs_diff_eq(&o, epsilon),
            (None, None) => true,
            _ => false,
        };

        let to = match (self.to, other.to) {
            (Some(s), Some(o)) => s.abs_diff_eq(&o, epsilon),
            (None, None) => true,
            _ => false,
        };

        from && to
    }
}

impl<T> RelativeEq for Range<T>
where
    T: RelativeEq + Float,
    T::Epsilon: Copy,
{
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Range<T>,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        let from = match (self.from, other.from) {
            (Some(s), Some(o)) => s.relative_eq(&o, epsilon, max_relative),
            (None, None) => true,
            _ => false,
        };

        let to = match (self.to, other.to) {
            (Some(s), Some(o)) => s.relative_eq(&o, epsilon, max_relative),
            (None, None) => true,
            _ => false,
        };

        from && to
    }
}

impl<T> UlpsEq for Range<T>
where
    T: UlpsEq + Float,
    T::Epsilon: Copy,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Range<T>, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        let from = match (self.from, other.from) {
            (Some(s), Some(o)) => s.ulps_eq(&o, epsilon, max_ulps),
            (None, None) => true,
            _ => false,
        };

        let to = match (self.to, other.to) {
            (Some(s), Some(o)) => s.ulps_eq(&o, epsilon, max_ulps),
            (None, None) => true,
            _ => false,
        };

        from && to
    }
}

#[derive(Clone)]
enum MaybeSlice<'a, C: Mix + Clone + 'a> {
    NotSlice(&'a Gradient<C>),
    Slice(Slice<'a, C>),
}

impl<'a, C: Mix + Clone> MaybeSlice<'a, C> {
    fn get(&self, i: C::Scalar) -> C {
        match *self {
            MaybeSlice::NotSlice(g) => g.get(i),
            MaybeSlice::Slice(ref s) => s.get(i),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Gradient, Range};
    use LinSrgb;

    #[test]
    fn range_clamp() {
        let range: Range<f64> = (0.0..1.0).into();
        assert_eq!(range.clamp(-1.0), 0.0);
        assert_eq!(range.clamp(2.0), 1.0);
        assert_eq!(range.clamp(0.5), 0.5);
    }

    #[test]
    fn range_constrain() {
        let range: Range<f64> = (0.0..1.0).into();
        assert_eq!(range.constrain(&(-3.0..-5.0).into()), (0.0..0.0).into());
        assert_eq!(range.constrain(&(-3.0..0.8).into()), (0.0..0.8).into());

        assert_eq!(range.constrain(&(3.0..5.0).into()), (1.0..1.0).into());
        assert_eq!(range.constrain(&(0.2..5.0).into()), (0.2..1.0).into());

        assert_eq!(range.constrain(&(0.2..0.8).into()), (0.2..0.8).into());
    }

    #[test]
    fn simple_slice() {
        let g1 = Gradient::new(vec![
            LinSrgb::new(1.0, 0.0, 0.0),
            LinSrgb::new(0.0, 0.0, 1.0),
        ]);
        let g2 = g1.slice(..0.5);

        let v1: Vec<_> = g1.take(10).take(5).collect();
        let v2: Vec<_> = g2.take(5).collect();
        for (t1, t2) in v1.iter().zip(v2.iter()) {
            assert_relative_eq!(t1, t2);
        }
    }
}
