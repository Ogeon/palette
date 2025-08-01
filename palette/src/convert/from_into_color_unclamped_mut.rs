use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::cast::{self, ArrayCast};

use super::{FromColorMut, FromColorMutGuard, FromColorUnclamped, IntoColorUnclamped};

/// Temporarily convert colors in place, without clamping.
///
/// It allows colors to be converted without using more additional memory than
/// what is necessary for the conversion, itself. The conversion will however
/// have to be reverted at some point, since the memory space is borrowed and
/// has to be restored to its original format. This is enforced by a scope guard
/// that does the opposite conversion when it's dropped.
///
/// See also [`IntoColorUnclampedMut`] and [`FromColorMut`].
///
/// ```
/// use palette::{convert::FromColorUnclampedMut, ShiftHueAssign, Srgb, Hsv};
///
/// let mut rgb = [
///     Srgb::new(1.0, 0.0, 0.0),
///     Srgb::new(0.0, 1.0, 0.0),
///     Srgb::new(0.0, 0.0, 1.0),
/// ];
///
/// {
///     let mut hsv = <[Hsv]>::from_color_unclamped_mut(&mut rgb);
///
///     // All of the colors in `rgb` have been converted to `Hsv`:
///     assert_eq!(
///         *hsv,
///         [
///             Hsv::new(0.0, 1.0, 1.0),
///             Hsv::new(120.0, 1.0, 1.0),
///             Hsv::new(240.0, 1.0, 1.0),
///         ]
///     );
///
///     hsv.shift_hue_assign(60.0);
///
/// } // The guard is dropped here and the colors are restored to `Srgb`.
///
/// // Notice how the colors in `rgb` have changed:
/// assert_eq!(
///     rgb,
///     [
///         Srgb::new(1.0, 1.0, 0.0),
///         Srgb::new(0.0, 1.0, 1.0),
///         Srgb::new(1.0, 0.0, 1.0),
///     ]
/// );
/// ```
///
/// The scope guard, [`FromColorUnclampedMutGuard`], has a few extra methods
/// that can make multiple conversion steps more efficient. One of those is
/// [`FromColorUnclampedMutGuard::then_into_color_unclamped_mut`], which works
/// like [`IntoColorUnclampedMut::into_color_unclamped_mut`], but does not add
/// an extra step when restoring to the original color type. This example will
/// convert `Srgb → Hsv → Hsl → Srgb` instead of `Srgb → Hsv → Hsl → Hsv →
/// Srgb`:
///
/// ```
/// use palette::{convert::FromColorUnclampedMut, ShiftHueAssign, LightenAssign, Srgb, Hsv, Hsl};
///
/// let mut rgb = [
///     Srgb::new(1.0, 0.0, 0.0),
///     Srgb::new(0.0, 1.0, 0.0),
///     Srgb::new(0.0, 0.0, 1.0),
/// ];
///
/// {
///     let mut hsv = <[Hsv]>::from_color_unclamped_mut(&mut rgb);
///     hsv.shift_hue_assign(60.0);
///
///     let mut hsl = hsv.then_into_color_unclamped_mut::<[Hsl]>();
///     hsl.lighten_assign(0.5);
///
/// } // `then_into_color_unclamped_mut` makes the guard restore directly to `Srgb` here.
///
/// // Notice how the colors in `rgb` have changed:
/// assert_eq!(
///     rgb,
///     [
///         Srgb::new(1.0, 1.0, 0.5),
///         Srgb::new(0.5, 1.0, 1.0),
///         Srgb::new(1.0, 0.5, 1.0),
///     ]
/// );
/// ```
///
/// # Note
///
/// The reused memory space could end up with unexpected values if the
/// conversion panics or if the scope guard's `drop` function doesn't run. The
/// default implementations of `FromColorUnclampedMut` uses [`ArrayCast`], which
/// is only implemented for color types that can safely accept and recover from
/// any value. Other color types will have to provide their own implementations
/// that can handle this case.
pub trait FromColorUnclampedMut<T>
where
    T: ?Sized + FromColorUnclampedMut<Self>,
{
    /// Temporarily convert from another color type in place, without clamping.
    ///
    /// This reuses the memory space, and the returned scope guard will restore
    /// the converted colors to their original type when it's dropped.
    #[must_use]
    fn from_color_unclamped_mut(color: &mut T) -> FromColorUnclampedMutGuard<'_, Self, T>;
}

impl<T, U> FromColorUnclampedMut<U> for T
where
    T: FromColorUnclamped<U> + ArrayCast + Clone,
    U: FromColorUnclamped<T> + ArrayCast<Array = T::Array> + Clone,
{
    #[inline]
    fn from_color_unclamped_mut(color: &mut U) -> FromColorUnclampedMutGuard<'_, Self, U> {
        let color_clone = color.clone();

        let result: &mut Self = cast::from_array_mut(cast::into_array_mut(color));

        *result = color_clone.into_color_unclamped();

        FromColorUnclampedMutGuard {
            current: Some(result),
            original: PhantomData,
        }
    }
}

impl<T, U> FromColorUnclampedMut<[U]> for [T]
where
    T: FromColorUnclampedMut<U> + ArrayCast,
    U: FromColorUnclampedMut<T> + ArrayCast<Array = T::Array>,
{
    #[inline]
    fn from_color_unclamped_mut(colors: &mut [U]) -> FromColorUnclampedMutGuard<'_, Self, [U]> {
        for color in &mut *colors {
            // Forgetting the guard leaves the colors in the converted state.
            core::mem::forget(T::from_color_unclamped_mut(color));
        }

        FromColorUnclampedMutGuard {
            current: Some(cast::from_array_slice_mut(cast::into_array_slice_mut(
                colors,
            ))),
            original: PhantomData,
        }
    }
}

/// Temporarily convert colors in place. The `Into` counterpart to
/// [`FromColorUnclampedMut`].
///
/// See [`FromColorUnclampedMut`] for more details and examples.
///
/// ```
/// use palette::{convert::IntoColorUnclampedMut, ShiftHueAssign, Srgb, Hsv};
///
/// let mut rgb = [
///     Srgb::new(1.0, 0.0, 0.0),
///     Srgb::new(0.0, 1.0, 0.0),
///     Srgb::new(0.0, 0.0, 1.0),
/// ];
///
/// {
///     let hsv: &mut [Hsv] = &mut rgb.into_color_unclamped_mut(); // The guard is coerced into a slice.
///
///     // All of the colors in `rgb` have been converted to `Hsv`:
///     assert_eq!(
///         hsv,
///         [
///             Hsv::new(0.0, 1.0, 1.0),
///             Hsv::new(120.0, 1.0, 1.0),
///             Hsv::new(240.0, 1.0, 1.0),
///         ]
///     );
///
///     hsv.shift_hue_assign(60.0);
///
/// } // The guard is dropped here and the colors are restored to `Srgb`.
///
/// // Notice how the colors in `rgb` have changed:
/// assert_eq!(
///     rgb,
///     [
///         Srgb::new(1.0, 1.0, 0.0),
///         Srgb::new(0.0, 1.0, 1.0),
///         Srgb::new(1.0, 0.0, 1.0),
///     ]
/// );
/// ```
pub trait IntoColorUnclampedMut<T>: FromColorUnclampedMut<T>
where
    T: ?Sized + FromColorUnclampedMut<Self>,
{
    /// Temporarily convert to another color type in place, without clamping.
    ///
    /// This reuses the memory space, and the returned scope guard will restore
    /// the converted colors to their original type when it's dropped.
    #[allow(clippy::wrong_self_convention)]
    #[must_use]
    fn into_color_unclamped_mut(&mut self) -> FromColorUnclampedMutGuard<'_, T, Self>;
}

impl<T, U> IntoColorUnclampedMut<T> for U
where
    T: FromColorUnclampedMut<U> + ?Sized,
    U: FromColorUnclampedMut<T> + ?Sized,
{
    #[inline]
    fn into_color_unclamped_mut(&mut self) -> FromColorUnclampedMutGuard<'_, T, Self> {
        T::from_color_unclamped_mut(self)
    }
}

/// A scope guard that restores the guarded colors to their original type,
/// without clamping, when dropped.
#[repr(transparent)]
pub struct FromColorUnclampedMutGuard<'a, T, U>
where
    T: FromColorUnclampedMut<U> + ?Sized,
    U: FromColorUnclampedMut<T> + ?Sized,
{
    // `Option` lets us move out without triggering `Drop`.
    pub(super) current: Option<&'a mut T>,
    pub(super) original: PhantomData<&'a mut U>,
}

impl<'a, T, U> FromColorUnclampedMutGuard<'a, T, U>
where
    T: FromColorUnclampedMut<U> + ?Sized,
    U: FromColorUnclampedMut<T> + ?Sized,
{
    /// Convert the colors to another type and replace this guard.
    ///
    /// The colors will not be converted back to the current color type before
    /// being restored, as opposed to when `into_color_mut` is called. Instead,
    /// they are restored directly to their original type.
    #[must_use]
    #[inline]
    pub fn then_into_color_mut<C>(mut self) -> FromColorMutGuard<'a, C, U>
    where
        T: FromColorMut<C>,
        C: FromColorMut<U> + FromColorMut<T> + ?Sized,
        U: FromColorMut<C>,
    {
        FromColorMutGuard {
            current: self
                .current
                .take()
                .map(C::from_color_mut)
                .and_then(|mut guard| guard.current.take()),
            original: PhantomData,
        }
    }

    /// Convert the colors to another type, without clamping, and replace this
    /// guard.
    ///
    /// The colors will not be converted back to the current color type before
    /// being restored, as opposed to when `into_color_unclamped_mut` is called.
    /// Instead, they are restored directly to their original type.
    #[must_use]
    #[inline]
    pub fn then_into_color_unclamped_mut<C>(mut self) -> FromColorUnclampedMutGuard<'a, C, U>
    where
        T: FromColorUnclampedMut<C>,
        C: FromColorUnclampedMut<U> + FromColorUnclampedMut<T> + ?Sized,
        U: FromColorUnclampedMut<C>,
    {
        FromColorUnclampedMutGuard {
            current: self
                .current
                .take()
                .map(C::from_color_unclamped_mut)
                .and_then(|mut guard| guard.current.take()),
            original: PhantomData,
        }
    }

    /// Replace this guard with a guard that clamps the colors after restoring.
    #[must_use]
    #[inline]
    pub fn into_clamped_guard(mut self) -> FromColorMutGuard<'a, T, U>
    where
        T: FromColorMut<U>,
        U: FromColorMut<T>,
    {
        FromColorMutGuard {
            current: self.current.take(),
            original: PhantomData,
        }
    }

    /// Immediately restore the colors to their original type.
    ///
    /// This happens automatically when the guard is dropped, but there may be
    /// situations where it's better or more convenient to call `restore`
    /// directly.
    #[inline]
    pub fn restore(mut self) -> &'a mut U {
        let restored = self
            .current
            .take()
            .map(U::from_color_unclamped_mut)
            .and_then(|mut guard| guard.current.take());

        if let Some(restored) = restored {
            restored
        } else {
            unreachable!()
        }
    }
}

impl<T, U> Deref for FromColorUnclampedMutGuard<'_, T, U>
where
    T: FromColorUnclampedMut<U> + ?Sized,
    U: FromColorUnclampedMut<T> + ?Sized,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        if let Some(current) = self.current.as_ref() {
            current
        } else {
            unreachable!()
        }
    }
}

impl<T, U> DerefMut for FromColorUnclampedMutGuard<'_, T, U>
where
    T: FromColorUnclampedMut<U> + ?Sized,
    U: FromColorUnclampedMut<T> + ?Sized,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let Some(current) = self.current.as_mut() {
            current
        } else {
            unreachable!()
        }
    }
}

impl<T, U> Drop for FromColorUnclampedMutGuard<'_, T, U>
where
    T: FromColorUnclampedMut<U> + ?Sized,
    U: FromColorUnclampedMut<T> + ?Sized,
{
    #[inline]
    fn drop(&mut self) {
        // Forgetting the guard leaves the colors in the converted state.
        core::mem::forget(self.current.take().map(U::from_color_unclamped_mut));
    }
}
