//!Pixel encodings and pixel format conversion.

pub use self::raw::*;
mod raw;

/// Represents colors that can be serialized and deserialized from raw color values.
///
/// This uses bit by bit conversion, so make sure that anything that implements it can be
/// represented as a contiguous sequence of a single type `T`.
pub unsafe trait Pixel<T>: Sized {
    /// The number of color channels.
    const CHANNELS: usize;

    /// Cast as a reference to raw color values.
    #[inline]
    fn as_raw<P: RawPixel<T> + ?Sized>(&self) -> &P {
        unsafe { P::from_raw_parts(self as *const Self as *const T, Self::CHANNELS) }
    }

    /// Cast as a mutable reference to raw color values.
    #[inline]
    fn as_raw_mut<P: RawPixel<T> + ?Sized>(&mut self) -> &mut P {
        unsafe { P::from_raw_parts_mut(self as *mut Self as *mut T, Self::CHANNELS) }
    }

    /// Convert from raw color values.
    #[inline]
    fn into_raw<P: RawPixelSized<T>>(self) -> P {
        assert_eq!(P::CHANNELS, Self::CHANNELS);
        assert_eq!(::std::mem::size_of::<P>(), ::std::mem::size_of::<Self>());
        assert_eq!(::std::mem::align_of::<P>(), ::std::mem::align_of::<Self>());

        let converted = unsafe { ::std::ptr::read(&self as *const Self as *const P) };

        // Just to be sure...
        ::std::mem::forget(self);

        converted
    }

    /// Cast from a reference to raw color values.
    #[inline]
    fn from_raw<P: RawPixel<T> + ?Sized>(pixel: &P) -> &Self {
        assert!(
            pixel.channels() >= Self::CHANNELS,
            "not enough color channels"
        );
        unsafe { &*(pixel.as_ptr() as *const Self) }
    }

    /// Cast from a mutable reference to raw color values.
    #[inline]
    fn from_raw_mut<P: RawPixel<T> + ?Sized>(pixel: &mut P) -> &mut Self {
        assert!(pixel.channels() >= Self::CHANNELS);
        unsafe { &mut *(pixel.as_mut_ptr() as *mut Self) }
    }

    /// Cast a slice of raw color values to a slice of colors.
    ///
    /// ```rust
    /// use palette::{Srgb, Pixel};
    ///
    /// let raw = &[255u8, 128, 64, 10, 20, 30];
    /// let colors = Srgb::from_raw_slice(raw);
    ///
    /// assert_eq!(colors.len(), 2);
    /// assert_eq!(colors[0].blue, 64);
    /// assert_eq!(colors[1].red, 10);
    /// ```
    #[inline]
    fn from_raw_slice(slice: &[T]) -> &[Self] {
        assert_eq!(slice.len() % Self::CHANNELS, 0);
        let new_length = slice.len() / Self::CHANNELS;
        unsafe { ::std::slice::from_raw_parts(slice.as_ptr() as *const Self, new_length) }
    }

    /// Cast a mutable slice of raw color values to a mutable slice of colors.
    ///
    /// ```rust
    /// use palette::{Srgb, Pixel};
    ///
    /// let raw = &mut [255u8, 128, 64, 10, 20, 30];
    /// {
    ///     let colors = Srgb::from_raw_slice_mut(raw);
    ///     assert_eq!(colors.len(), 2);
    ///
    ///     // These changes affects the raw slice, since they are the same data
    ///     colors[0].blue = 100;
    ///     colors[1].red = 200;
    /// }
    ///
    /// // Notice the two values in the middle:
    /// assert_eq!(raw, &[255, 128, 100, 200, 20, 30]);
    /// ```
    #[inline]
    fn from_raw_slice_mut(slice: &mut [T]) -> &mut [Self] {
        assert_eq!(slice.len() % Self::CHANNELS, 0);
        let new_length = slice.len() / Self::CHANNELS;
        unsafe { ::std::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut Self, new_length) }
    }
}
