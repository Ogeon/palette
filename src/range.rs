//!Types for interpolation between multiple colors.

use std::cmp::max;

use Mix;

///An interpolated range of colors.
#[derive(Clone)]
pub struct Range<C: Mix + Clone>(Vec<(f32, C)>);

impl<C: Mix + Clone> Range<C> {
    ///Create a range of evenly spaced colors with the domain [0.0, 1.0].
    ///There must be at least one color.
    pub fn new<I: IntoIterator<Item=C>>(colors: I) -> Range<C> {
        let mut points: Vec<_> = colors.into_iter().map(|c| (0.0, c)).collect();
        assert!(points.len() > 0);
        let step_size = 1.0 / max(points.len() - 1, 1) as f32;

        for (i, &mut (ref mut p, _)) in points.iter_mut().enumerate() {
            *p = i as f32 * step_size;
        }

        Range(points)
    }

    ///Create a range of colors with custom spacing and domain. There must be
    ///at least one color and they are expected to be ordered by their
    ///position value.
    pub fn with_domain(colors: Vec<(f32, C)>) -> Range<C> {
        assert!(colors.len() > 0);

        //Maybe sort the colors?
        Range(colors)
    }

    ///Get a color from the range. The nearest color will be returned if `i`
    ///is outside the domain.
    pub fn get(&self, i: f32) -> C {
        let &(mut min, ref min_color) = self.0.get(0).expect("a Range must contain at least one color");
        let mut min_color = min_color;
        let mut min_index = 0;

        if i <= min {
            return min_color.clone();
        }

        let &(mut max, ref max_color) = self.0.last().expect("a Range must contain at least one color");
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

    ///Take `n` evenly spaced colors from the range.
    pub fn take(&self, n: usize) -> Take<C> {
        let &(min, _) = self.0.get(0).expect("a Range must contain at least one color");
        let &(max, _) = self.0.last().expect("a Range must contain at least one color");

        Take {
            range: self,
            from: min,
            diff: max - min,
            len: n,
            current: 0,
        }
    }
}

///An iterator over interpolated colors.
pub struct Take<'a, C: Mix + Clone + 'a> {
    range: &'a Range<C>,
    from: f32,
    diff: f32,
    len: usize,
    current: usize,
}

impl<'a, C: Mix + Clone> Iterator for Take<'a, C> {
    type Item = C;

    fn next(&mut self) -> Option<C> {
        if self.current < self.len {
            let i = self.from + self.current as f32 * (self.diff / self.len as f32);
            self.current += 1;
            Some(self.range.get(i))
        } else {
            None
        }
    }
}
