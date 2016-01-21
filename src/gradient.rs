//! Types for interpolation between multiple colors.

use num::traits::Float;
use std::cmp::max;

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
pub struct Gradient<T: Float, C: Mix<T> + Clone>(Vec<(T, C)>);

impl<T: Float, C: Mix<T> + Clone> Gradient<T, C> {
    ///Create a gradient of evenly spaced colors with the domain [0.0, 1.0].
    ///There must be at least one color.
    pub fn new<I: IntoIterator<Item = C>>(colors: I) -> Gradient<T, C> {
        let mut points: Vec<_> = colors.into_iter().map(|c| (T::zero(), c)).collect();
        assert!(points.len() > 0);
        let step_size = T::one() / T::from(max(points.len() - 1, 1) as f64).unwrap();

        for (i, &mut (ref mut p, _)) in points.iter_mut().enumerate() {
            *p = T::from(i).unwrap() * step_size;
        }

        Gradient(points)
    }

    ///Create a gradient of colors with custom spacing and domain. There must be
    ///at least one color and they are expected to be ordered by their
    ///position value.
    pub fn with_domain(colors: Vec<(T, C)>) -> Gradient<T, C> {
        assert!(colors.len() > 0);

        // Maybe sort the colors?
        Gradient(colors)
    }

    ///Get a color from the gradient. The color of the closest control point
    ///will be returned if `i` is outside the domain.
    pub fn get(&self, i: T) -> C {
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
    pub fn slice<R: Into<Range>>(&self, range: R) -> Slice<C> {
        Slice {
            gradient: self,
            range: range.into(),
        }
    }

    ///Get the limits of this gradient's domain.
    pub fn domain(&self) -> (f32, f32) {
        let &(min, _) = self.0.get(0).expect("a Gradient must contain at least one color");
        let &(max, _) = self.0.last().expect("a Gradient must contain at least one color");
        (min, max)
    }
}

///An iterator over interpolated colors.
pub struct Take<'a, T: Float + 'a, C: Mix<T> + Clone + 'a> {
    gradient: MaybeSlice<'a, T, C>,
    from: T,
    diff: T,
    len: usize,
    current: usize,
}

impl<'a, T: Float, C: Mix<T> + Clone> Iterator for Take<'a, T, C> {
    type Item = C;

    fn next(&mut self) -> Option<C> {
        if self.current < self.len {
            let i = self.from +
                    T::from(self.current).unwrap() * (self.diff / T::from(self.len).unwrap());
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
    range: Range,
}

impl<'a, C: Mix + Clone> Slice<'a, C> {
    ///Get a color from the gradient slice. The color of the closest domain
    ///limit will be returned if `i` is outside the domain.
    pub fn get(&self, i: f32) -> C {
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
    pub fn slice<R: Into<Range>>(&self, range: R) -> Slice<C> {
        Slice {
            gradient: self.gradient,
            range: self.range.constrain(&range.into()),
        }
    }

    ///Get the limits of this gradient slice's domain.
    pub fn domain(&self) -> (f32, f32) {
        if let Range { from: Some(from), to: Some(to) } = self.range {
            (from, to)
        } else {
            let (from, to) = self.gradient.domain();
            (self.range.from.unwrap_or(from), self.range.to.unwrap_or(to))
        }
    }
}

///A domain range for gradient slices.
#[derive(Clone, Debug, PartialEq)]
pub struct Range {
    from: Option<f32>,
    to: Option<f32>,
}

impl Range {
    fn clamp(&self, mut x: f32) -> f32 {
        x = self.from.unwrap_or(x).max(x);
        self.to.unwrap_or(x).min(x)
    }

    fn constrain(&self, other: &Range) -> Range {
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

impl From<::std::ops::Range<f32>> for Range {
    fn from(range: ::std::ops::Range<f32>) -> Range {
        Range {
            from: Some(range.start),
            to: Some(range.end),
        }
    }
}

impl From<::std::ops::RangeFrom<f32>> for Range {
    fn from(range: ::std::ops::RangeFrom<f32>) -> Range {
        Range {
            from: Some(range.start),
            to: None,
        }
    }
}

impl From<::std::ops::RangeTo<f32>> for Range {
    fn from(range: ::std::ops::RangeTo<f32>) -> Range {
        Range {
            from: None,
            to: Some(range.end),
        }
    }
}

impl From<::std::ops::RangeFull> for Range {
    fn from(_range: ::std::ops::RangeFull) -> Range {
        Range {
            from: None,
            to: None,
        }
    }
}

enum MaybeSlice<'a, C: Mix + Clone + 'a> {
    NotSlice(&'a Gradient<C>),
    Slice(Slice<'a, C>),
}

impl<'a, C: Mix + Clone> MaybeSlice<'a, C> {
    fn get(&self, i: f32) -> C {
        match *self {
            MaybeSlice::NotSlice(g) => g.get(i),
            MaybeSlice::Slice(ref s) => s.get(i),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Range, Gradient};
    use Rgb;

    #[test]
    fn range_clamp() {
        let range: Range = (0.0..1.0).into();
        assert_eq!(range.clamp(-1.0), 0.0);
        assert_eq!(range.clamp(2.0), 1.0);
        assert_eq!(range.clamp(0.5), 0.5);
    }

    #[test]
    fn range_constrain() {
        let range: Range = (0.0..1.0).into();
        assert_eq!(range.constrain(&(-3.0..-5.0).into()), (0.0..0.0).into());
        assert_eq!(range.constrain(&(-3.0..0.8).into()), (0.0..0.8).into());

        assert_eq!(range.constrain(&(3.0..5.0).into()), (1.0..1.0).into());
        assert_eq!(range.constrain(&(0.2..5.0).into()), (0.2..1.0).into());

        assert_eq!(range.constrain(&(0.2..0.8).into()), (0.2..0.8).into());
    }

    #[test]
    fn simple_slice() {
        let g1 = Gradient::new(vec![Rgb::linear_rgb(1.0, 0.0, 0.0),
                                    Rgb::linear_rgb(0.0, 0.0, 1.0)]);
        let g2 = g1.slice(..0.5);

        let v1: Vec<_> = g1.take(10).take(5).collect();
        let v2: Vec<_> = g2.take(5).collect();

        assert_eq!(v1, v2);
    }
}
