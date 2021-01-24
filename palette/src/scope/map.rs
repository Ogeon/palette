use core::ops::{Deref, DerefMut};

use crate::{encoding::pixel::ArrayRepr, pipeline::Pipeline};

/// Experimental Map scope implementation.
pub struct Map<'a, T, R>
where
    T: ArrayRepr + Clone,
    R: Pipeline<T>,
    R::Output: 'a + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>,
{
    buffer: &'a mut [T],
    restore: R,
}

impl<'a, T, R> Map<'a, T, R>
where
    T: ArrayRepr + Clone,
    R: Pipeline<T>,
    R::Output: 'a + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType> + Clone,
{
    /// Creates a new scope to temporarily convert a color buffer.
    pub fn new<C>(buffer: &'a mut [R::Output], convert: C, restore: R) -> Self
    where
        C: Pipeline<R::Output, Output = T>,
    {
        Self {
            buffer: super::map_buffer(buffer, |color| convert.apply(color)),
            restore,
        }
    }
}

impl<'a, T, R> AsRef<[T]> for Map<'a, T, R>
where
    T: ArrayRepr + Clone,
    R: Pipeline<T>,
    R::Output: 'a + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>,
{
    fn as_ref(&self) -> &[T] {
        self.buffer
    }
}

impl<'a, T, R> AsMut<[T]> for Map<'a, T, R>
where
    T: ArrayRepr + Clone,
    R: Pipeline<T>,
    R::Output: 'a + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.buffer
    }
}

impl<'a, T, R> Deref for Map<'a, T, R>
where
    T: ArrayRepr + Clone,
    R: Pipeline<T>,
    R::Output: 'a + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.buffer
    }
}

impl<'a, T, R> DerefMut for Map<'a, T, R>
where
    T: ArrayRepr + Clone,
    R: Pipeline<T>,
    R::Output: 'a + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.buffer
    }
}

impl<'i, 'a, T, R> IntoIterator for &'i mut Map<'a, T, R>
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

impl<'a, T, R> Drop for Map<'a, T, R>
where
    T: ArrayRepr + Clone,
    R: Pipeline<T>,
    R::Output: 'a + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>,
{
    fn drop(&mut self) {
        let restore = &self.restore;
        super::convert_in_place(self.buffer, |color| restore.apply(color))
    }
}
