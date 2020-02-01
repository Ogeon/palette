/// A contiguous sequence of pixel channels with a known length.
///
/// It's used when converting to and from raw pixel data and should only be
/// implemented for types with either a suitable in-memory representation.
pub unsafe trait RawPixelSized<T>: Sized {
    /// The guaranteed number of channels in the sequence.
    const CHANNELS: usize;
}

unsafe impl<T> RawPixelSized<T> for [T; 1] {
    const CHANNELS: usize = 1;
}

unsafe impl<T> RawPixelSized<T> for [T; 2] {
    const CHANNELS: usize = 2;
}

unsafe impl<T> RawPixelSized<T> for [T; 3] {
    const CHANNELS: usize = 3;
}

unsafe impl<T> RawPixelSized<T> for [T; 4] {
    const CHANNELS: usize = 4;
}

/// A contiguous sequence of pixel channels.
///
/// It's used when converting to and from raw pixel data and should only be
/// implemented for types with a suitable in-memory representation.
pub unsafe trait RawPixel<T> {
    /// The length of the sequence.
    fn channels(&self) -> usize;

    /// Convert from a pointer and a length.
    unsafe fn from_raw_parts<'a>(pointer: *const T, length: usize) -> &'a Self;

    /// Convert from a mutable pointer and a length.
    unsafe fn from_raw_parts_mut<'a>(pointer: *mut T, length: usize) -> &'a mut Self;

    /// Convert to a pointer.
    fn as_ptr(&self) -> *const T;

    /// Convert to a mutable pointer.
    fn as_mut_ptr(&mut self) -> *mut T;
}

unsafe impl<P: RawPixelSized<T>, T> RawPixel<T> for P {
    #[inline]
    fn channels(&self) -> usize {
        P::CHANNELS
    }

    #[inline]
    unsafe fn from_raw_parts<'a>(pointer: *const T, length: usize) -> &'a Self {
        assert_eq!(length, Self::CHANNELS);
        &*(pointer as *const Self)
    }

    #[inline]
    unsafe fn from_raw_parts_mut<'a>(pointer: *mut T, length: usize) -> &'a mut Self {
        assert_eq!(length, Self::CHANNELS);
        &mut *(pointer as *mut Self)
    }

    #[inline]
    fn as_ptr(&self) -> *const T {
        self as *const Self as *const T
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut Self as *mut T
    }
}

unsafe impl<T> RawPixel<T> for [T] {
    #[inline]
    fn channels(&self) -> usize {
        self.len()
    }

    #[inline]
    unsafe fn from_raw_parts<'a>(pointer: *const T, length: usize) -> &'a Self {
        ::core::slice::from_raw_parts(pointer, length)
    }

    #[inline]
    unsafe fn from_raw_parts_mut<'a>(pointer: *mut T, length: usize) -> &'a mut Self {
        ::core::slice::from_raw_parts_mut(pointer, length)
    }

    #[inline]
    fn as_ptr(&self) -> *const T {
        self.as_ptr()
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut T {
        self.as_mut_ptr()
    }
}
