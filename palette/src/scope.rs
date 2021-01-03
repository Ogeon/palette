//! Experimental scope.

use crate::{encoding::pixel::ArrayRepr, pipeline::Pipeline, IntoColor, Pixel};

/// Experimental scope implementation.
pub struct Scope<'a, T, R>
where
    T: ArrayRepr + Clone,
    R: Pipeline<T>,
    R::Output: 'a + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>,
{
    buffer: &'a mut [T],
    restore: R,
}

impl<'a, T, R> Scope<'a, T, R>
where
    T: ArrayRepr + Clone,
    R: Pipeline<T>,
    R::Output: 'a + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType> + Clone,
{
    /// Creates a new scope to temporarily convert a color buffer.
    pub fn new<B: 'a, C>(buffer: &'a mut B, convert: C, restore: R) -> Self
    where
        B: AsMut<[R::Output]>,
        C: Pipeline<R::Output, Output = T>,
    {
        let buffer = buffer.as_mut();

        for color in &mut *buffer {
            let source = color.clone();
            let destination = T::from_raw_mut(color.as_raw_mut::<T::ArrayType>());
            *destination = convert.apply(source);
        }

        Self {
            buffer: T::from_raw_slice_mut(Pixel::into_raw_slice_mut(buffer)),
            restore,
        }
    }
}

impl<'a, T, R> Drop for Scope<'a, T, R>
where
    T: ArrayRepr + Clone,
    R: Pipeline<T>,
    R::Output: 'a + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>,
{
    fn drop(&mut self) {
        for color in &mut *self.buffer {
            let source = color.clone();
            let destination = R::Output::from_raw_mut(color.as_raw_mut::<T::ArrayType>());
            *destination = self.restore.apply(source);
        }
    }
}

impl<'a, T, R> AsRef<[T]> for Scope<'a, T, R>
where
    T: ArrayRepr + Clone,
    R: Pipeline<T>,
    R::Output: 'a + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>,
{
    fn as_ref(&self) -> &[T] {
        self.buffer
    }
}

impl<'a, T, R> AsMut<[T]> for Scope<'a, T, R>
where
    T: ArrayRepr + Clone,
    R: Pipeline<T>,
    R::Output: 'a + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.buffer
    }
}

impl<'i, 'a, T, R> IntoIterator for &'i mut Scope<'a, T, R>
where
    T: ArrayRepr + Clone,
    R: Pipeline<T>,
    R::Output: 'a + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>,
{
    type Item = &'i mut T;

    type IntoIter = core::slice::IterMut<'i, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.into_iter()
    }
}

/// Temporarily converts a buffer of `T` into a buffer of `U`.
///
/// ```
/// use palette::{scope, Shade, Hue, Srgb, Xyz, Lch};
///
/// let mut colors = vec![Srgb::new(1.0f32, 0.5, 0.0), Srgb::new(0.5, 1.0, 0.5)];
///
/// {
///     let mut xyz_colors = palette::scope::into_color::<Xyz, _, _>(&mut colors);
///
///     for color in &mut xyz_colors {
///         *color = color.darken(0.5);
///     }
///
///     let mut lch_colors = palette::scope::into_color::<Lch, _, _>(&mut xyz_colors);
///
///     for color in &mut lch_colors {
///         *color = color.shift_hue(30.0);
///     }
/// }
///
/// let srgb_u8: Vec<Srgb<u8>> = colors.into_iter().map(Srgb::into_format).collect();
/// ```
pub fn into_color<'a, U, B, T>(buffer: &'a mut B) -> Scope<'a, U, fn(U) -> T>
where
    B: AsMut<[T]>,
    T: 'a + IntoColor<U> + ArrayRepr + Clone,
    U: IntoColor<T> + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType> + Clone,
{
    Scope::new(buffer, T::into_color, U::into_color)
}

#[cfg(test)]
mod test {
    use crate::{Hsv, LinSrgb, Shade};

    #[test]
    fn one_layer() {
        let mut colors = [LinSrgb::new(0.5f32, 0.0, 0.5), LinSrgb::new(0.5, 1.0, 0.5)];

        {
            let mut hsv_colors = super::into_color::<Hsv<_>, _, _>(&mut colors);

            for color in &mut hsv_colors {
                *color = color.darken(0.5);
            }
        }

        assert_eq!(
            &colors,
            &[
                LinSrgb::new(0.0f32, 0.0, 0.0),
                LinSrgb::new(0.25, 0.5, 0.25)
            ]
        );
    }
}
