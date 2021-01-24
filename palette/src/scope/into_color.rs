use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{convert, encoding::pixel::ArrayRepr};

/// Prototype IntoColor scope.
pub struct IntoColor<'a, T, U>
where
    T: ArrayRepr,
    U: convert::IntoColor<T>
        + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
        + Clone,
{
    buffer: &'a mut [U],
    phantom: PhantomData<fn(U) -> T>,
}

impl<'a, T, U> IntoColor<'a, T, U>
where
    T: ArrayRepr,
    U: convert::IntoColor<T>
        + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
        + Clone,
{
    /// Creates a scope that temporarily converts a buffer into another color
    /// space. See [convert::IntoColor] for more details.
    pub fn new(buffer: &'a mut [T]) -> Self
    where
        T: convert::IntoColor<U> + Clone,
    {
        Self {
            buffer: super::map_buffer(buffer, T::into_color),
            phantom: PhantomData,
        }
    }

    /// Temporarily converts the buffer into `T` without clamping. This also replaces
    /// the current scope with a scope that will restore directly to `T`, without
    /// first restoring to `U`. See [convert::IntoColor] for more details.
    ///
    /// ```
    /// use palette::{scope::Scope, Shade, Hue, Srgb, Xyz, Lch};
    ///
    /// let mut colors = vec![Srgb::new(1.0f32, 0.5, 0.0), Srgb::new(0.5, 1.0, 0.5)];
    ///
    /// {
    ///     let mut xyz_colors = colors.into_color::<Xyz>();
    ///
    ///     for color in &mut *xyz_colors {
    ///         *color = color.darken(0.5);
    ///     }
    ///
    ///     let mut lch_colors = xyz_colors.replace_into_color::<Lch>();
    ///
    ///     for color in &mut *lch_colors {
    ///         *color = color.shift_hue(30.0);
    ///     }
    /// }
    ///
    /// let srgb_u8: Vec<Srgb<u8>> = colors.into_iter().map(Srgb::into_format).collect();
    /// ```
    pub fn replace_into_color<C>(self) -> IntoColor<'a, T, C>
    where
        U: convert::IntoColor<C>,
        C: convert::IntoColor<T>
            + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
            + Clone,
    {
        IntoColor {
            buffer: super::map_buffer(self.into_buffer(), U::into_color),
            phantom: PhantomData,
        }
    }

    /// Temporarily converts the buffer into `T` without clamping. This also replaces
    /// the current scope with a scope that will restore directly to `T`, without
    /// first restoring to `U`, and without clamping. See
    /// [convert::IntoColorUnclamped] for more details.
    ///
    /// ```
    /// use palette::{scope::Scope, Shade, Hue, Srgb, Xyz, Lch};
    ///
    /// let mut colors = vec![Srgb::new(1.0f32, 0.5, 0.0), Srgb::new(0.5, 1.0, 0.5)];
    ///
    /// {
    ///     let mut xyz_colors = colors.into_color::<Xyz>();
    ///
    ///     for color in &mut *xyz_colors {
    ///         *color = color.darken(0.5);
    ///     }
    ///
    ///     let mut lch_colors = xyz_colors.replace_into_color_unclamped::<Lch>();
    ///
    ///     for color in &mut *lch_colors {
    ///         *color = color.shift_hue(30.0);
    ///     }
    /// }
    ///
    /// let srgb_u8: Vec<Srgb<u8>> = colors.into_iter().map(Srgb::into_format).collect();
    /// ```
    pub fn replace_into_color_unclamped<C>(self) -> IntoColorUnclamped<'a, T, C>
    where
        U: convert::IntoColorUnclamped<C>,
        C: convert::IntoColorUnclamped<T>
            + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
            + Clone,
    {
        IntoColorUnclamped {
            buffer: super::map_buffer(self.into_buffer(), U::into_color_unclamped),
            phantom: PhantomData,
        }
    }

    /// Replaces the current scope with a scope that will restore to `T` without
    /// clamping. See [convert::IntoColorUnclamped] for more details.
    ///
    /// ```
    /// use palette::{scope::Scope, Shade, Srgb, Xyz};
    ///
    /// let mut colors = vec![Srgb::new(1.0f32, 0.5, 0.0), Srgb::new(0.5, 1.0, 0.5)];
    ///
    /// {
    ///     let mut xyz_colors = colors.into_color::<Xyz>().restore_unclamped();
    ///
    ///     for color in &mut *xyz_colors {
    ///         *color = color.darken(0.5);
    ///     }
    /// }
    ///
    /// let srgb_u8: Vec<Srgb<u8>> = colors.into_iter().map(Srgb::into_format).collect();
    /// ```
    pub fn restore_unclamped(self) -> IntoColorUnclamped<'a, T, U>
    where
        U: convert::IntoColorUnclamped<T>,
    {
        IntoColorUnclamped {
            buffer: self.into_buffer(),
            phantom: PhantomData,
        }
    }

    fn into_buffer(self) -> &'a mut [U] {
        core::mem::replace(&mut std::mem::ManuallyDrop::new(self).buffer, &mut [])
    }
}

impl<'a, T, U> super::Scope for IntoColor<'a, T, U>
where
    T: ArrayRepr,
    U: convert::IntoColor<T>
        + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
        + Clone,
{
    type Item = U;
}

impl<'a, T, U> AsRef<[U]> for IntoColor<'a, T, U>
where
    T: ArrayRepr,
    U: convert::IntoColor<T>
        + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
        + Clone,
{
    fn as_ref(&self) -> &[U] {
        self.buffer
    }
}

impl<'a, T, U> AsMut<[U]> for IntoColor<'a, T, U>
where
    T: ArrayRepr,
    U: convert::IntoColor<T>
        + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
        + Clone,
{
    fn as_mut(&mut self) -> &mut [U] {
        self.buffer
    }
}

impl<'a, T, U> Deref for IntoColor<'a, T, U>
where
    T: ArrayRepr,
    U: convert::IntoColor<T>
        + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
        + Clone,
{
    type Target = [U];

    fn deref(&self) -> &Self::Target {
        self.buffer
    }
}

impl<'a, T, U> DerefMut for IntoColor<'a, T, U>
where
    T: ArrayRepr,
    U: convert::IntoColor<T>
        + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
        + Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.buffer
    }
}

impl<'a, T, U> Drop for IntoColor<'a, T, U>
where
    T: ArrayRepr,
    U: convert::IntoColor<T>
        + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
        + Clone,
{
    fn drop(&mut self) {
        super::convert_in_place(self.buffer, convert::IntoColor::<T>::into_color)
    }
}

/// Prototype IntoColorUnclamped scope.
pub struct IntoColorUnclamped<'a, T, U>
where
    T: ArrayRepr,
    U: convert::IntoColorUnclamped<T>
        + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
        + Clone,
{
    buffer: &'a mut [U],
    phantom: PhantomData<fn(U) -> T>,
}

impl<'a, T, U> IntoColorUnclamped<'a, T, U>
where
    T: ArrayRepr,
    U: convert::IntoColorUnclamped<T>
        + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
        + Clone,
{
    /// Creates a scope that temporarily converts a buffer into another color
    /// space without clamping. See [convert::IntoColorUnclamped] for more
    /// details.
    pub fn new(buffer: &'a mut [T]) -> Self
    where
        T: convert::IntoColorUnclamped<U> + Clone,
    {
        Self {
            buffer: super::map_buffer(buffer, T::into_color_unclamped),
            phantom: PhantomData,
        }
    }

    /// Temporarily converts the buffer into `T` without clamping. This also replaces
    /// the current scope with a scope that will restore directly to `T`, without
    /// first restoring to `U`. See [convert::IntoColor] for more details.
    ///
    /// ```
    /// use palette::{scope::Scope, Shade, Hue, Srgb, Xyz, Lch};
    ///
    /// let mut colors = vec![Srgb::new(1.0f32, 0.5, 0.0), Srgb::new(0.5, 1.0, 0.5)];
    ///
    /// {
    ///     let mut xyz_colors = colors.into_color_unclamped::<Xyz>();
    ///
    ///     for color in &mut *xyz_colors {
    ///         *color = color.darken(0.5);
    ///     }
    ///
    ///     let mut lch_colors = xyz_colors.replace_into_color::<Lch>();
    ///
    ///     for color in &mut *lch_colors {
    ///         *color = color.shift_hue(30.0);
    ///     }
    /// }
    ///
    /// let srgb_u8: Vec<Srgb<u8>> = colors.into_iter().map(Srgb::into_format).collect();
    /// ```

    pub fn replace_into_color<C>(self) -> IntoColor<'a, T, C>
    where
        U: convert::IntoColor<C>,
        C: convert::IntoColor<T>
            + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
            + Clone,
    {
        IntoColor {
            buffer: super::map_buffer(self.into_buffer(), U::into_color),
            phantom: PhantomData,
        }
    }

    /// Temporarily converts the buffer into `T` without clamping. This also replaces
    /// the current scope with a scope that will restore directly to `T`, without
    /// first restoring to `U`, and without clamping. See
    /// [convert::IntoColorUnclamped] for more details.
    ///
    /// ```
    /// use palette::{scope::Scope, Shade, Hue, Srgb, Xyz, Lch};
    ///
    /// let mut colors = vec![Srgb::new(1.0f32, 0.5, 0.0), Srgb::new(0.5, 1.0, 0.5)];
    ///
    /// {
    ///     let mut xyz_colors = colors.into_color_unclamped::<Xyz>();
    ///
    ///     for color in &mut *xyz_colors {
    ///         *color = color.darken(0.5);
    ///     }
    ///
    ///     let mut lch_colors = xyz_colors.replace_into_color_unclamped::<Lch>();
    ///
    ///     for color in &mut *lch_colors {
    ///         *color = color.shift_hue(30.0);
    ///     }
    /// }
    ///
    /// let srgb_u8: Vec<Srgb<u8>> = colors.into_iter().map(Srgb::into_format).collect();
    /// ```
    pub fn replace_into_color_unclamped<C>(self) -> IntoColorUnclamped<'a, T, C>
    where
        U: convert::IntoColorUnclamped<C>,
        C: convert::IntoColorUnclamped<T>
            + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
            + Clone,
    {
        IntoColorUnclamped {
            buffer: super::map_buffer(self.into_buffer(), U::into_color_unclamped),
            phantom: PhantomData,
        }
    }

    /// Replaces the current scope with a scope that will clanp after restoring
    /// to `T`. See [convert::IntoColor] for more details.
    ///
    /// ```
    /// use palette::{scope::Scope, Shade, Srgb, Xyz};
    ///
    /// let mut colors = vec![Srgb::new(1.0f32, 0.5, 0.0), Srgb::new(0.5, 1.0, 0.5)];
    ///
    /// {
    ///     let mut xyz_colors = colors.into_color_unclamped::<Xyz>().restore_clamped();
    ///
    ///     for color in &mut *xyz_colors {
    ///         *color = color.lighten(0.5);
    ///     }
    /// }
    ///
    /// let srgb_u8: Vec<Srgb<u8>> = colors.into_iter().map(Srgb::into_format).collect();
    /// ```
    pub fn restore_clamped(self) -> IntoColor<'a, T, U>
    where
        U: convert::IntoColor<T>,
    {
        IntoColor {
            buffer: self.into_buffer(),
            phantom: PhantomData,
        }
    }

    fn into_buffer(self) -> &'a mut [U] {
        core::mem::replace(&mut std::mem::ManuallyDrop::new(self).buffer, &mut [])
    }
}

impl<'a, T, U> super::Scope for IntoColorUnclamped<'a, T, U>
where
    T: ArrayRepr,
    U: convert::IntoColorUnclamped<T>
        + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
        + Clone,
{
    type Item = U;
}

impl<'a, T, U> AsRef<[U]> for IntoColorUnclamped<'a, T, U>
where
    T: ArrayRepr,
    U: convert::IntoColorUnclamped<T>
        + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
        + Clone,
{
    fn as_ref(&self) -> &[U] {
        self.buffer
    }
}

impl<'a, T, U> AsMut<[U]> for IntoColorUnclamped<'a, T, U>
where
    T: ArrayRepr,
    U: convert::IntoColorUnclamped<T>
        + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
        + Clone,
{
    fn as_mut(&mut self) -> &mut [U] {
        self.buffer
    }
}

impl<'a, T, U> Deref for IntoColorUnclamped<'a, T, U>
where
    T: ArrayRepr,
    U: convert::IntoColorUnclamped<T>
        + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
        + Clone,
{
    type Target = [U];

    fn deref(&self) -> &Self::Target {
        self.buffer
    }
}

impl<'a, T, U> DerefMut for IntoColorUnclamped<'a, T, U>
where
    T: ArrayRepr,
    U: convert::IntoColorUnclamped<T>
        + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
        + Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.buffer
    }
}

impl<'a, T, U> Drop for IntoColorUnclamped<'a, T, U>
where
    T: ArrayRepr,
    U: convert::IntoColorUnclamped<T>
        + ArrayRepr<Component = T::Component, ArrayType = T::ArrayType>
        + Clone,
{
    fn drop(&mut self) {
        super::convert_in_place(
            self.buffer,
            convert::IntoColorUnclamped::<T>::into_color_unclamped,
        )
    }
}
