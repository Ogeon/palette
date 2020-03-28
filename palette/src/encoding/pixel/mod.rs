//! Pixel encodings and pixel format conversion.

#[doc(hidden)]
pub use palette_derive::Pixel;

pub use self::raw::*;
mod raw;

/// Represents colors that can be serialized and deserialized from raw color
/// components.
///
/// This uses bit by bit conversion, so make sure that anything that implements
/// it can be represented as a contiguous sequence of a single type `T`. This is
/// most safely done using `#[derive(Pixel)]`.
///
/// # Deriving
///
/// `Pixel` can be automatically derived. The only requirements are that the
/// type is a `struct`, that it has a `#[repr(C)]` attribute, and that all of
/// its fields have the same types. It stays on the conservative side and will
/// show an error if any of those requirements are not fulfilled. If some fields
/// have different types, but the same memory layout, or are zero-sized, they
/// can be marked with attributes to show that their types are safe to use.
///
/// ## Field Attributes
///
/// * `#[palette_unsafe_same_layout_as = "SomeType"]`: Mark the field as having
///   the same memory
/// layout as `SomeType`.
///
///   **Unsafety:** corrupt data and undefined behavior may occur if this is not
/// true!
///
/// * `#[palette_unsafe_zero_sized]`: Mark the field as being zero-sized, and
///   thus not taking up
/// any memory space. This means that it can be ignored.
///
///   **Unsafety:** corrupt data and undefined behavior may occur if this is not
/// true!
///
/// ## Examples
///
/// Basic use:
///
/// ```rust
/// use palette::Pixel;
///
/// #[derive(PartialEq, Debug, Pixel)]
/// #[repr(C)]
/// struct MyCmyk {
///     cyan: f32,
///     magenta: f32,
///     yellow: f32,
///     key: f32,
/// }
///
/// let buffer = [0.1, 0.2, 0.3, 0.4];
/// let color = MyCmyk::from_raw(&buffer);
///
/// assert_eq!(
///     color,
///     &MyCmyk {
///         cyan: 0.1,
///         magenta: 0.2,
///         yellow: 0.3,
///         key: 0.4,
///     }
/// );
/// ```
///
/// Heterogenous field types:
///
/// ```rust
/// use std::marker::PhantomData;
///
/// use palette::{Pixel, RgbHue};
/// use palette::rgb::RgbStandard;
/// use palette::encoding::Srgb;
///
/// #[derive(PartialEq, Debug, Pixel)]
/// #[repr(C)]
/// struct MyCoolColor<S: RgbStandard> {
///     #[palette(unsafe_zero_sized)]
///     standard: PhantomData<S>,
///     // RgbHue is a wrapper with `#[repr(C)]`, so it can safely
///     // be converted straight from `f32`.
///     #[palette(unsafe_same_layout_as = "f32")]
///     hue: RgbHue<f32>,
///     lumen: f32,
///     chroma: f32,
/// }
///
/// let buffer = [172.0, 100.0, 0.3];
/// let color = MyCoolColor::<Srgb>::from_raw(&buffer);
///
/// assert_eq!(
///     color,
///     &MyCoolColor {
///         hue: 172.0.into(),
///         lumen: 100.0,
///         chroma: 0.3,
///         standard: PhantomData,
///     }
/// );
/// ```
pub unsafe trait Pixel<T>: Sized {
    /// The number of color channels.
    const CHANNELS: usize;

    /// Cast as a reference to raw color components.
    #[inline]
    fn as_raw<P: RawPixel<T> + ?Sized>(&self) -> &P {
        unsafe { P::from_raw_parts(self as *const Self as *const T, Self::CHANNELS) }
    }

    /// Cast as a mutable reference to raw color components.
    #[inline]
    fn as_raw_mut<P: RawPixel<T> + ?Sized>(&mut self) -> &mut P {
        unsafe { P::from_raw_parts_mut(self as *mut Self as *mut T, Self::CHANNELS) }
    }

    /// Convert into raw color components.
    #[inline]
    fn into_raw<P: RawPixelSized<T>>(self) -> P {
        assert_eq!(P::CHANNELS, Self::CHANNELS);
        assert_eq!(::core::mem::size_of::<P>(), ::core::mem::size_of::<Self>());
        assert_eq!(
            ::core::mem::align_of::<P>(),
            ::core::mem::align_of::<Self>()
        );

        let converted = unsafe { ::core::ptr::read(&self as *const Self as *const P) };

        // Just to be sure...
        ::core::mem::forget(self);

        converted
    }

    /// Cast from a reference to raw color components.
    #[inline]
    fn from_raw<P: RawPixel<T> + ?Sized>(pixel: &P) -> &Self {
        assert!(
            pixel.channels() >= Self::CHANNELS,
            "not enough color channels"
        );
        unsafe { &*(pixel.as_ptr() as *const Self) }
    }

    /// Cast from a mutable reference to raw color components.
    #[inline]
    fn from_raw_mut<P: RawPixel<T> + ?Sized>(pixel: &mut P) -> &mut Self {
        assert!(pixel.channels() >= Self::CHANNELS);
        unsafe { &mut *(pixel.as_mut_ptr() as *mut Self) }
    }

    /// Cast a slice of raw color components to a slice of colors.
    ///
    /// ```rust
    /// use palette::{Pixel, Srgb};
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
        unsafe { ::core::slice::from_raw_parts(slice.as_ptr() as *const Self, new_length) }
    }

    /// Cast a mutable slice of raw color components to a mutable slice of
    /// colors.
    ///
    /// ```rust
    /// use palette::{Pixel, Srgb};
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
        unsafe { ::core::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut Self, new_length) }
    }

    /// Cast a slice of colors to a slice of raw color components.
    ///
    /// ```rust
    /// use palette::{Pixel, Srgb};
    ///
    /// let colors = &[Srgb::new(255u8, 128, 64), Srgb::new(10, 20, 30)];
    /// let raw = Srgb::into_raw_slice(colors);
    ///
    /// assert_eq!(raw.len(), 6);
    /// assert_eq!(raw, &[255u8, 128, 64, 10, 20, 30]);
    /// ```
    #[inline]
    fn into_raw_slice(slice: &[Self]) -> &[T] {
        let new_length = slice.len() * Self::CHANNELS;
        unsafe { ::core::slice::from_raw_parts(slice.as_ptr() as *const T, new_length) }
    }

    /// Cast a mutable slice of colors to a mutable slice of raw color
    /// components.
    ///
    /// ```rust
    /// use palette::{Pixel, Srgb};
    ///
    /// let colors = &mut [Srgb::new(255u8, 128, 64), Srgb::new(10, 20, 30)];
    /// {
    ///     let raw = Srgb::into_raw_slice_mut(colors);
    ///     assert_eq!(raw.len(), 6);
    ///
    ///     // These changes affects the color slice, since they are the same data
    ///     raw[2] = 100;
    ///     raw[3] = 200;
    /// }
    ///
    /// assert_eq!(colors[0].blue, 100);
    /// assert_eq!(colors[1].red, 200);
    /// ```
    #[inline]
    fn into_raw_slice_mut(slice: &mut [Self]) -> &mut [T] {
        let new_length = slice.len() * Self::CHANNELS;
        unsafe { ::core::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut T, new_length) }
    }
}
