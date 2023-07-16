use core::{convert::Infallible, fmt::Debug};

use crate::ArrayExt;

use super::{
    from_array_array, from_array_slice, from_array_slice_mut, from_component_array,
    into_array_array, into_array_slice, into_array_slice_mut, into_component_array,
    into_component_slice, into_component_slice_mut, try_from_component_slice,
    try_from_component_slice_mut, ArrayCast, SliceCastError,
};

#[cfg(feature = "std")]
use super::{
    from_array_slice_box, from_array_vec, into_array_slice_box, into_array_vec,
    into_component_slice_box, into_component_vec, try_from_component_slice_box,
    try_from_component_vec, BoxedSliceCastError, VecCastError,
};

/// Trait for trying to cast a collection of colors from a collection of color
/// components without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Errors
///
/// The cast will return an error if the cast fails, such as when the length of
/// the input is not a multiple of the color's array length.
///
/// ## Examples
///
/// ```
/// use palette::{cast::TryFromComponents, Srgb};
///
/// let array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let slice: &[_] = &[64, 139, 10, 93, 18, 214];
/// let slice_mut: &mut [_] = &mut [64, 139, 10, 93, 18, 214];
/// let vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// assert_eq!(
///     <[Srgb<u8>; 2]>::try_from_components(array),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)])
/// );
///
/// assert_eq!(
///     <&[Srgb<u8>]>::try_from_components(slice),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].as_ref())
/// );
///
/// assert_eq!(
///     <&mut [Srgb<u8>]>::try_from_components(slice_mut),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].as_mut())
/// );
///
/// assert_eq!(
///     Vec::<Srgb<u8>>::try_from_components(vec),
///     Ok(vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)])
/// );
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::TryFromComponents, Srgb};
///
/// let array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let mut vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// assert_eq!(
///     <&[Srgb<u8>]>::try_from_components(&array),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].as_ref())
/// );
///
/// assert_eq!(
///     <&mut [Srgb<u8>]>::try_from_components(&mut vec),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].as_mut())
/// );
/// ```
///
/// This produces an error:
///
/// ```
/// use palette::{cast::TryFromComponents, Srgb};
///
/// let components = &[64, 139, 10, 93, 18]; // Not a multiple of 3
/// assert!(<&[Srgb<u8>]>::try_from_components(components).is_err());
/// ```
pub trait TryFromComponents<C>: Sized {
    /// The error for when `try_from_components` fails to cast.
    type Error;

    /// Try to cast a collection of color components into an collection of
    /// colors of type `Self::Color`.
    ///
    /// Return an error if the conversion can't be done, such as when the number
    /// of items in `components` isn't a multiple of the number of components in
    /// `Self::Color`.
    fn try_from_components(components: C) -> Result<Self, Self::Error>;
}

impl<T, C, const N: usize, const M: usize> TryFromComponents<[T; N]> for [C; M]
where
    C: ArrayCast,
    C::Array: ArrayExt<Item = T>,
{
    type Error = Infallible; // We don't provide a `try_*` option for arrays.

    #[inline]
    fn try_from_components(components: [T; N]) -> Result<Self, Self::Error> {
        Ok(from_component_array(components))
    }
}

macro_rules! impl_try_from_components_slice {
    ($($owning:ty $(where ($($ty_input:tt)+))?),*) => {
        $(
            impl<'a, T, C $(, $($ty_input)+)?> TryFromComponents<&'a $owning> for &'a [C]
            where
                T: 'a,
                C: ArrayCast,
                C::Array: ArrayExt<Item = T>,
            {
                type Error = SliceCastError;

                #[inline]
                fn try_from_components(components: &'a $owning) -> Result<Self, Self::Error> {
                    try_from_component_slice(components)
                }
            }

            impl<'a, T, C $(, $($ty_input)+)?> TryFromComponents<&'a mut $owning> for &'a mut [C]
            where
                T: 'a,
                C: ArrayCast,
                C::Array: ArrayExt<Item = T>,
            {
                type Error = SliceCastError;

                #[inline]
                fn try_from_components(components: &'a mut $owning) -> Result<Self, Self::Error> {
                    try_from_component_slice_mut(components)
                }
            }
        )*
    };
}

impl_try_from_components_slice!([T], [T; N] where (const N: usize));

#[cfg(feature = "std")]
impl_try_from_components_slice!(Box<[T]>, Vec<T>);

#[cfg(feature = "std")]
impl<T, C> TryFromComponents<Box<[T]>> for Box<[C]>
where
    C: ArrayCast,
    C::Array: ArrayExt<Item = T>,
{
    type Error = BoxedSliceCastError<T>;

    #[inline]
    fn try_from_components(components: Box<[T]>) -> Result<Self, Self::Error> {
        try_from_component_slice_box(components)
    }
}

#[cfg(feature = "std")]
impl<T, C> TryFromComponents<Vec<T>> for Vec<C>
where
    C: ArrayCast,
    C::Array: ArrayExt<Item = T>,
{
    type Error = VecCastError<T>;

    #[inline]
    fn try_from_components(components: Vec<T>) -> Result<Self, Self::Error> {
        try_from_component_vec(components)
    }
}

/// Trait for casting a collection of colors from a collection of color
/// components without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Panics
///
/// The cast will panic if the cast fails, such as when the length of the input
/// is not a multiple of the color's array length.
///
/// ## Examples
///
/// ```
/// use palette::{cast::FromComponents, Srgb};
///
/// let array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let slice: &[_] = &[64, 139, 10, 93, 18, 214];
/// let slice_mut: &mut [_] = &mut [64, 139, 10, 93, 18, 214];
/// let vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// assert_eq!(
///     <[Srgb<u8>; 2]>::from_components(array),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
///
/// assert_eq!(
///     <&[Srgb<u8>]>::from_components(slice),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
///
/// assert_eq!(
///     <&mut [Srgb<u8>]>::from_components(slice_mut),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
///
/// assert_eq!(
///     Vec::<Srgb<u8>>::from_components(vec),
///     vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::FromComponents, Srgb};
///
/// let array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let mut vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// assert_eq!(
///     <&[Srgb<u8>]>::from_components(&array),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
///
/// assert_eq!(
///     <&mut [Srgb<u8>]>::from_components(&mut vec),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
/// ```
///
/// This panics:
///
/// ```should_panic
/// use palette::{cast::FromComponents, Srgb};
///
/// let components = &[64, 139, 10, 93, 18, 214, 0, 123]; // Not a multiple of 3
/// <&[Srgb<u8>]>::from_components(components);
/// ```
pub trait FromComponents<C> {
    /// Cast a collection of color components into an collection of colors of
    /// type `Self::Color`.
    ///
    /// ## Panics
    /// If the conversion can't be done, such as when the number of items in
    /// `components` isn't a multiple of the number of components in
    /// `Self::Color`.
    fn from_components(components: C) -> Self;
}

impl<T, C> FromComponents<C> for T
where
    T: TryFromComponents<C>,
    T::Error: Debug,
{
    #[inline]
    fn from_components(components: C) -> Self {
        Self::try_from_components(components).unwrap()
    }
}

/// Trait for casting a collection of colors into a collection of color
/// components without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::IntoComponents, Srgb};
///
/// let array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice: &[_] = &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice_mut: &mut [_] = &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!(array.into_components(), [64, 139, 10, 93, 18, 214]);
/// assert_eq!(slice.into_components(), [64, 139, 10, 93, 18, 214]);
/// assert_eq!(slice_mut.into_components(), [64, 139, 10, 93, 18, 214]);
/// assert_eq!(vec.into_components(), vec![64, 139, 10, 93, 18, 214]);
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::IntoComponents, Srgb};
///
/// let array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let mut vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!((&array).into_components(), [64, 139, 10, 93, 18, 214]);
/// assert_eq!((&mut vec).into_components(), [64, 139, 10, 93, 18, 214]);
/// ```
pub trait IntoComponents<C> {
    /// Cast this collection of colors into a collection of color components.
    fn into_components(self) -> C;
}

impl<T, C, const N: usize, const M: usize> IntoComponents<[T; M]> for [C; N]
where
    C: ArrayCast,
    C::Array: ArrayExt<Item = T>,
{
    #[inline]
    fn into_components(self) -> [T; M] {
        into_component_array(self)
    }
}

macro_rules! impl_into_components_slice {
    ($($owning:ty $(where ($($ty_input:tt)+))?),*) => {
        $(
            impl<'a, T, C $(, $($ty_input)+)?> IntoComponents<&'a [T]> for &'a $owning
            where
                T: 'a,
                C: ArrayCast,
                C::Array: ArrayExt<Item = T>,
            {
                #[inline]
                fn into_components(self) -> &'a [T]  {
                    into_component_slice(self)
                }
            }

            impl<'a, T, C $(, $($ty_input)+)?> IntoComponents<&'a mut [T]> for &'a mut $owning
            where
                T: 'a,
                C: ArrayCast,
                C::Array: ArrayExt<Item = T>,
            {
                #[inline]
                fn into_components(self) -> &'a mut [T] {
                    into_component_slice_mut(self)
                }
            }
        )*
    };
}

impl_into_components_slice!([C], [C; N] where (const N: usize));

#[cfg(feature = "std")]
impl_into_components_slice!(Box<[C]>, Vec<C>);

#[cfg(feature = "std")]
impl<T, C> IntoComponents<Box<[T]>> for Box<[C]>
where
    C: ArrayCast,
    C::Array: ArrayExt<Item = T>,
{
    #[inline]
    fn into_components(self) -> Box<[T]> {
        into_component_slice_box(self)
    }
}

#[cfg(feature = "std")]
impl<T, C> IntoComponents<Vec<T>> for Vec<C>
where
    C: ArrayCast,
    C::Array: ArrayExt<Item = T>,
{
    #[inline]
    fn into_components(self) -> Vec<T> {
        into_component_vec(self)
    }
}

/// Trait for casting a collection of colors from a collection of arrays without
/// copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::FromArrays, Srgb};
///
/// let array: [_; 2] = [[64, 139, 10], [93, 18, 214]];
/// let slice: &[_] = &[[64, 139, 10], [93, 18, 214]];
/// let slice_mut: &mut [_] = &mut [[64, 139, 10], [93, 18, 214]];
/// let vec: Vec<_> = vec![[64, 139, 10], [93, 18, 214]];
///
/// assert_eq!(
///     <[Srgb<u8>; 2]>::from_arrays(array),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
///
/// assert_eq!(
///     <&[Srgb<u8>]>::from_arrays(slice),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
///
/// assert_eq!(
///     <&mut [Srgb<u8>]>::from_arrays(slice_mut),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
///
/// assert_eq!(
///     Vec::<Srgb<u8>>::from_arrays(vec),
///     vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::FromArrays, Srgb};
///
/// let array: [_; 2] = [[64, 139, 10], [93, 18, 214]];
/// let mut vec: Vec<_> = vec![[64, 139, 10], [93, 18, 214]];
///
/// assert_eq!(
///     <&[Srgb<u8>]>::from_arrays(&array),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
///
/// assert_eq!(
///     <&mut [Srgb<u8>]>::from_arrays(&mut vec),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
/// ```
pub trait FromArrays<A> {
    /// Cast a collection of arrays into an collection of colors of type
    /// `Self::Color`.
    fn from_arrays(arrays: A) -> Self;
}

impl<T, C, const N: usize, const M: usize> FromArrays<[[T; N]; M]> for [C; M]
where
    C: ArrayCast<Array = [T; N]>,
{
    #[inline]
    fn from_arrays(arrays: [[T; N]; M]) -> Self {
        from_array_array(arrays)
    }
}

macro_rules! impl_from_arrays_slice {
    ($($owning:ty $(where ($($ty_input:tt)+))?),*) => {
        $(
            impl<'a, T, C, const N: usize $(, $($ty_input)+)?> FromArrays<&'a $owning> for &'a [C]
            where
                C: ArrayCast<Array = [T; N]>,
            {
                #[inline]
                fn from_arrays(arrays: &'a $owning) -> Self {
                    from_array_slice(arrays)
                }
            }

            impl<'a, T, C, const N: usize $(, $($ty_input)+)?> FromArrays<&'a mut $owning> for &'a mut [C]
            where
                C: ArrayCast<Array = [T; N]>,
            {
                #[inline]
                fn from_arrays(arrays: &'a mut $owning) -> Self {
                    from_array_slice_mut(arrays)
                }
            }
        )*
    };
}

impl_from_arrays_slice!([[T; N]], [[T; N]; M] where (const M: usize));

#[cfg(feature = "std")]
impl_from_arrays_slice!(Box<[[T; N]]>, Vec<[T; N]>);

#[cfg(feature = "std")]
impl<T, C, const N: usize> FromArrays<Box<[[T; N]]>> for Box<[C]>
where
    C: ArrayCast<Array = [T; N]>,
{
    #[inline]
    fn from_arrays(arrays: Box<[[T; N]]>) -> Self {
        from_array_slice_box(arrays)
    }
}

#[cfg(feature = "std")]
impl<T, C, const N: usize> FromArrays<Vec<[T; N]>> for Vec<C>
where
    C: ArrayCast<Array = [T; N]>,
{
    #[inline]
    fn from_arrays(arrays: Vec<[T; N]>) -> Self {
        from_array_vec(arrays)
    }
}

/// Trait for casting a collection of colors into a collection of arrays without
/// copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::IntoArrays, Srgb};
///
/// let array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice: &[_] = &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice_mut: &mut [_] = &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!(array.into_arrays(), [[64, 139, 10], [93, 18, 214]]);
/// assert_eq!(slice.into_arrays(), [[64, 139, 10], [93, 18, 214]]);
/// assert_eq!(slice_mut.into_arrays(), [[64, 139, 10], [93, 18, 214]]);
/// assert_eq!(vec.into_arrays(), vec![[64, 139, 10], [93, 18, 214]]);
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::IntoArrays, Srgb};
///
/// let array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let mut vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!((&array).into_arrays(), [[64, 139, 10], [93, 18, 214]]);
/// assert_eq!((&mut vec).into_arrays(), [[64, 139, 10], [93, 18, 214]]);
/// ```
pub trait IntoArrays<A> {
    /// Cast this collection of colors into a collection of arrays.
    fn into_arrays(self) -> A;
}

impl<T, C, const N: usize, const M: usize> IntoArrays<[[T; N]; M]> for [C; M]
where
    C: ArrayCast<Array = [T; N]>,
{
    #[inline]
    fn into_arrays(self) -> [[T; N]; M] {
        into_array_array(self)
    }
}

macro_rules! impl_into_arrays_slice {
    ($($owning:ty $(where ($($ty_input:tt)+))?),*) => {
        $(
            impl<'a, T, C, const N: usize $(, $($ty_input)+)?> IntoArrays<&'a [[T; N]]> for &'a $owning
            where
                C: ArrayCast<Array = [T; N]>,
            {
                #[inline]
                fn into_arrays(self) -> &'a [[T; N]]  {
                    into_array_slice(self)
                }
            }

            impl<'a, T, C, const N: usize $(, $($ty_input)+)?> IntoArrays<&'a mut [[T; N]]> for &'a mut $owning
            where
                C: ArrayCast<Array = [T; N]>,
            {
                #[inline]
                fn into_arrays(self) -> &'a mut [[T; N]] {
                    into_array_slice_mut(self)
                }
            }
        )*
    };
}

impl_into_arrays_slice!([C], [C; M] where (const M: usize));

#[cfg(feature = "std")]
impl_into_arrays_slice!(Box<[C]>, Vec<C>);

#[cfg(feature = "std")]
impl<T, C, const N: usize> IntoArrays<Box<[[T; N]]>> for Box<[C]>
where
    C: ArrayCast<Array = [T; N]>,
{
    #[inline]
    fn into_arrays(self) -> Box<[[T; N]]> {
        into_array_slice_box(self)
    }
}

#[cfg(feature = "std")]
impl<T, C, const N: usize> IntoArrays<Vec<[T; N]>> for Vec<C>
where
    C: ArrayCast<Array = [T; N]>,
{
    #[inline]
    fn into_arrays(self) -> Vec<[T; N]> {
        into_array_vec(self)
    }
}

/// Trait for casting a collection of color components into a collection of
/// colors without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::ComponentsFrom, Srgb};
///
/// let array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice: &[_] = &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice_mut: &mut [_] = &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!(<[_; 6]>::components_from(array), [64, 139, 10, 93, 18, 214]);
/// assert_eq!(<&[_]>::components_from(slice), [64, 139, 10, 93, 18, 214]);
/// assert_eq!(<&mut [_]>::components_from(slice_mut), [64, 139, 10, 93, 18, 214]);
/// assert_eq!(Vec::<_>::components_from(vec), vec![64, 139, 10, 93, 18, 214]);
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::ComponentsFrom, Srgb};
///
/// let array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let mut vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!(<&[_]>::components_from(&array), [64, 139, 10, 93, 18, 214]);
/// assert_eq!(<&mut [_]>::components_from(&mut vec), [64, 139, 10, 93, 18, 214]);
/// ```
pub trait ComponentsFrom<C> {
    /// Cast a collection of colors of type `C` into a collection of color
    /// components.
    fn components_from(colors: C) -> Self;
}

impl<T, C> ComponentsFrom<C> for T
where
    C: IntoComponents<T>,
{
    #[inline]
    fn components_from(colors: C) -> Self {
        colors.into_components()
    }
}

/// Trait for trying to cast a collection of color components from a collection
/// of colors without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Errors
///
/// The cast will return an error if the cast fails, such as when the length of
/// the input is not a multiple of the color's array length.
///
/// ## Examples
///
/// ```
/// use palette::{cast::TryComponentsInto, Srgb};
///
/// let array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let slice: &[_] = &[64, 139, 10, 93, 18, 214];
/// let slice_mut: &mut [_] = &mut [64, 139, 10, 93, 18, 214];
/// let vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// assert_eq!(
///     array.try_components_into(),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)])
/// );
///
/// assert_eq!(
///     slice.try_components_into(),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].as_ref())
/// );
///
/// assert_eq!(
///     slice_mut.try_components_into(),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].as_mut())
/// );
///
/// assert_eq!(
///     vec.try_components_into(),
///     Ok(vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)])
/// );
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::TryComponentsInto, Srgb};
///
/// let array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let mut vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// assert_eq!(
///     (&array).try_components_into(),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].as_ref())
/// );
///
/// assert_eq!(
///     (&mut vec).try_components_into(),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].as_mut())
/// );
/// ```
///
/// This produces an error:
///
/// ```
/// use palette::{cast::TryComponentsInto, Srgb};
///
/// let components = &[64, 139, 10, 93, 18]; // Not a multiple of 3
/// let colors: Result<&[Srgb<u8>], _> = components.try_components_into();
/// assert!(colors.is_err());
/// ```
pub trait TryComponentsInto<C>: Sized {
    /// The error for when `try_into_colors` fails to cast.
    type Error;

    /// Try to cast this collection of color components into a collection of
    /// colors of type `C`.
    ///
    /// Return an error if the conversion can't be done, such as when the number
    /// of items in `self` isn't a multiple of the number of components in `C`.
    fn try_components_into(self) -> Result<C, Self::Error>;
}

/// Trait for casting a collection of color components from a collection of
/// colors without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Panics
///
/// The cast will panic if the cast fails, such as when the length of the input
/// is not a multiple of the color's array length.
///
/// ## Examples
///
/// ```
/// use palette::{cast::ComponentsInto, Srgb};
///
/// let array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let slice: &[_] = &[64, 139, 10, 93, 18, 214];
/// let slice_mut: &mut [_] = &mut [64, 139, 10, 93, 18, 214];
/// let vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// let colors: [Srgb<u8>; 2] = array.components_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &[Srgb<u8>] = slice.components_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &mut [Srgb<u8>] = slice_mut.components_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: Vec<Srgb<u8>> = vec.components_into();
/// assert_eq!(colors, vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::ComponentsInto, Srgb};
///
/// let array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let mut vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// let colors: &[Srgb<u8>] = (&array).components_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &mut [Srgb<u8>] = (&mut vec).components_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
/// ```
///
/// This panics:
///
/// ```should_panic
/// use palette::{cast::ComponentsInto, Srgb};
///
/// let components = &[64, 139, 10, 93, 18, 214, 0, 123]; // Not a multiple of 3
/// let colors: &[Srgb<u8>] = components.components_into();
/// ```
pub trait ComponentsInto<C> {
    /// Cast this collection of color components into a collection of colors of
    /// type `C`.
    ///
    /// ## Panics
    /// If the conversion can't be done, such as when the number of items in
    /// `self` isn't a multiple of the number of components in `C`.
    fn components_into(self) -> C;
}

impl<T, C> ComponentsInto<C> for T
where
    T: TryComponentsInto<C>,
    T::Error: Debug,
{
    #[inline]
    fn components_into(self) -> C {
        self.try_components_into().unwrap()
    }
}

impl<'a, T, C> TryComponentsInto<C> for T
where
    C: TryFromComponents<T>,
{
    type Error = C::Error;

    #[inline]
    fn try_components_into(self) -> Result<C, Self::Error> {
        C::try_from_components(self)
    }
}

/// Trait for casting a collection of arrays from a collection of colors without
/// copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::ArraysFrom, Srgb};
///
/// let array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice: &[_] = &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice_mut: &mut [_] = &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!(<[_; 2]>::arrays_from(array), [[64, 139, 10], [93, 18, 214]]);
/// assert_eq!(<&[_]>::arrays_from(slice), [[64, 139, 10], [93, 18, 214]]);
/// assert_eq!(<&mut [_]>::arrays_from(slice_mut), [[64, 139, 10], [93, 18, 214]]);
/// assert_eq!(Vec::<_>::arrays_from(vec), vec![[64, 139, 10], [93, 18, 214]]);
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::ArraysFrom, Srgb};
///
/// let array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let mut vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!(<&[_]>::arrays_from(&array), [[64, 139, 10], [93, 18, 214]]);
/// assert_eq!(<&mut [_]>::arrays_from(&mut vec), [[64, 139, 10], [93, 18, 214]]);
/// ```
pub trait ArraysFrom<C> {
    /// Cast a collection of colors into a collection of arrays.
    fn arrays_from(colors: C) -> Self;
}

impl<T, C> ArraysFrom<C> for T
where
    C: IntoArrays<T>,
{
    #[inline]
    fn arrays_from(colors: C) -> Self {
        colors.into_arrays()
    }
}

/// Trait for casting a collection of arrays into a collection of colors
/// without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::ArraysInto, Srgb};
///
/// let array: [_; 2] = [[64, 139, 10], [93, 18, 214]];
/// let slice: &[_] = &[[64, 139, 10], [93, 18, 214]];
/// let slice_mut: &mut [_] = &mut [[64, 139, 10], [93, 18, 214]];
/// let vec: Vec<_> = vec![[64, 139, 10], [93, 18, 214]];
///
/// let colors: [Srgb<u8>; 2] = array.arrays_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &[Srgb<u8>] = slice.arrays_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &mut [Srgb<u8>] = slice_mut.arrays_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: Vec<Srgb<u8>> = vec.arrays_into();
/// assert_eq!(colors, vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::ArraysInto, Srgb};
///
/// let array: [_; 2] = [[64, 139, 10], [93, 18, 214]];
/// let mut vec: Vec<_> = vec![[64, 139, 10], [93, 18, 214]];
///
/// let colors: &[Srgb<u8>] = (&array).arrays_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &mut [Srgb<u8>] = (&mut vec).arrays_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
/// ```
pub trait ArraysInto<C> {
    /// Cast this collection of arrays into a collection of colors of type `C`.
    fn arrays_into(self) -> C;
}

impl<T, C> ArraysInto<C> for T
where
    C: FromArrays<T>,
{
    #[inline]
    fn arrays_into(self) -> C {
        C::from_arrays(self)
    }
}

#[cfg(test)]
mod test {
    use crate::Srgb;

    use super::{
        ArraysFrom, ArraysInto, ComponentsFrom, ComponentsInto, FromArrays, FromComponents,
        IntoArrays, IntoComponents, TryComponentsInto, TryFromComponents,
    };

    #[test]
    fn try_from_components() {
        let slice: &[u8] = &[1, 2, 3, 4, 5, 6];
        let slice_mut: &mut [u8] = &mut [1, 2, 3, 4, 5, 6];
        let mut slice_box: Box<[u8]> = vec![1, 2, 3, 4, 5, 6].into_boxed_slice();
        let mut vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let mut array: [u8; 6] = [1, 2, 3, 4, 5, 6];

        let _ = <&[Srgb<u8>]>::try_from_components(slice).unwrap();
        let _ = <&[Srgb<u8>]>::try_from_components(&slice_box).unwrap();
        let _ = <&[Srgb<u8>]>::try_from_components(&vec).unwrap();
        let _ = <&[Srgb<u8>]>::try_from_components(&array).unwrap();

        let _ = <&mut [Srgb<u8>]>::try_from_components(slice_mut).unwrap();
        let _ = <&mut [Srgb<u8>]>::try_from_components(&mut slice_box).unwrap();
        let _ = <&mut [Srgb<u8>]>::try_from_components(&mut vec).unwrap();
        let _ = <&mut [Srgb<u8>]>::try_from_components(&mut array).unwrap();

        let _ = Box::<[Srgb<u8>]>::try_from_components(slice_box).unwrap();
        let _ = Vec::<Srgb<u8>>::try_from_components(vec).unwrap();
        let _ = <[Srgb<u8>; 2]>::try_from_components(array).unwrap();
    }

    #[test]
    fn try_components_into() {
        let slice: &[u8] = &[1, 2, 3, 4, 5, 6];
        let slice_mut: &mut [u8] = &mut [1, 2, 3, 4, 5, 6];
        let mut slice_box: Box<[u8]> = vec![1, 2, 3, 4, 5, 6].into_boxed_slice();
        let mut vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let mut array: [u8; 6] = [1, 2, 3, 4, 5, 6];

        let _: &[Srgb<u8>] = slice.try_components_into().unwrap();
        let _: &[Srgb<u8>] = (&slice_box).try_components_into().unwrap();
        let _: &[Srgb<u8>] = (&vec).try_components_into().unwrap();
        let _: &[Srgb<u8>] = (&array).try_components_into().unwrap();

        let _: &mut [Srgb<u8>] = slice_mut.try_components_into().unwrap();
        let _: &mut [Srgb<u8>] = (&mut slice_box).try_components_into().unwrap();
        let _: &mut [Srgb<u8>] = (&mut vec).try_components_into().unwrap();
        let _: &mut [Srgb<u8>] = (&mut array).try_components_into().unwrap();

        let _: Box<[Srgb<u8>]> = slice_box.try_components_into().unwrap();
        let _: Vec<Srgb<u8>> = vec.try_components_into().unwrap();
        let _: [Srgb<u8>; 2] = array.try_components_into().unwrap();
    }

    #[test]
    fn from_components() {
        let slice: &[u8] = &[1, 2, 3, 4, 5, 6];
        let slice_mut: &mut [u8] = &mut [1, 2, 3, 4, 5, 6];
        let mut slice_box: Box<[u8]> = vec![1, 2, 3, 4, 5, 6].into_boxed_slice();
        let mut vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let mut array: [u8; 6] = [1, 2, 3, 4, 5, 6];

        let _ = <&[Srgb<u8>]>::from_components(slice);
        let _ = <&[Srgb<u8>]>::from_components(&slice_box);
        let _ = <&[Srgb<u8>]>::from_components(&vec);
        let _ = <&[Srgb<u8>]>::from_components(&array);

        let _ = <&mut [Srgb<u8>]>::from_components(slice_mut);
        let _ = <&mut [Srgb<u8>]>::from_components(&mut slice_box);
        let _ = <&mut [Srgb<u8>]>::from_components(&mut vec);
        let _ = <&mut [Srgb<u8>]>::from_components(&mut array);

        let _ = Box::<[Srgb<u8>]>::from_components(slice_box);
        let _ = Vec::<Srgb<u8>>::from_components(vec);
        let _ = <[Srgb<u8>; 2]>::from_components(array);
    }

    #[test]
    fn components_into() {
        let slice: &[u8] = &[1, 2, 3, 4, 5, 6];
        let slice_mut: &mut [u8] = &mut [1, 2, 3, 4, 5, 6];
        let mut slice_box: Box<[u8]> = vec![1, 2, 3, 4, 5, 6].into_boxed_slice();
        let mut vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let mut array: [u8; 6] = [1, 2, 3, 4, 5, 6];

        let _: &[Srgb<u8>] = slice.components_into();
        let _: &[Srgb<u8>] = (&slice_box).components_into();
        let _: &[Srgb<u8>] = (&vec).components_into();
        let _: &[Srgb<u8>] = (&array).components_into();

        let _: &mut [Srgb<u8>] = slice_mut.components_into();
        let _: &mut [Srgb<u8>] = (&mut slice_box).components_into();
        let _: &mut [Srgb<u8>] = (&mut vec).components_into();
        let _: &mut [Srgb<u8>] = (&mut array).components_into();

        let _: Box<[Srgb<u8>]> = slice_box.components_into();
        let _: Vec<Srgb<u8>> = vec.components_into();
        let _: [Srgb<u8>; 2] = array.components_into();
    }

    #[test]
    fn into_components() {
        let slice: &[Srgb<u8>] = &[Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let slice_mut: &mut [Srgb<u8>] = &mut [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let mut slice_box: Box<[Srgb<u8>]> =
            vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)].into_boxed_slice();
        let mut vec: Vec<Srgb<u8>> = vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let mut array: [Srgb<u8>; 2] = [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];

        let _: &[u8] = slice.into_components();
        let _: &[u8] = (&slice_box).into_components();
        let _: &[u8] = (&vec).into_components();
        let _: &[u8] = (&array).into_components();

        let _: &mut [u8] = slice_mut.into_components();
        let _: &mut [u8] = (&mut slice_box).into_components();
        let _: &mut [u8] = (&mut vec).into_components();
        let _: &mut [u8] = (&mut array).into_components();

        let _: Box<[u8]> = slice_box.into_components();
        let _: Vec<u8> = vec.into_components();
        let _: [u8; 6] = array.into_components();
    }

    #[test]
    fn components_from() {
        let slice: &[Srgb<u8>] = &[Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let slice_mut: &mut [Srgb<u8>] = &mut [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let mut slice_box: Box<[Srgb<u8>]> =
            vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)].into_boxed_slice();
        let mut vec: Vec<Srgb<u8>> = vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let mut array: [Srgb<u8>; 2] = [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];

        let _ = <&[u8]>::components_from(slice);
        let _ = <&[u8]>::components_from(&slice_box);
        let _ = <&[u8]>::components_from(&vec);
        let _ = <&[u8]>::components_from(&array);

        let _ = <&mut [u8]>::components_from(slice_mut);
        let _ = <&mut [u8]>::components_from(&mut slice_box);
        let _ = <&mut [u8]>::components_from(&mut vec);
        let _ = <&mut [u8]>::components_from(&mut array);

        let _ = Box::<[u8]>::components_from(slice_box);
        let _ = Vec::<u8>::components_from(vec);
        let _ = <[u8; 6]>::components_from(array);
    }

    #[test]
    fn from_arrays() {
        let slice: &[[u8; 3]] = &[[1, 2, 3], [4, 5, 6]];
        let slice_mut: &mut [[u8; 3]] = &mut [[1, 2, 3], [4, 5, 6]];
        let mut slice_box: Box<[[u8; 3]]> = vec![[1, 2, 3], [4, 5, 6]].into_boxed_slice();
        let mut vec: Vec<[u8; 3]> = vec![[1, 2, 3], [4, 5, 6]];
        let mut array: [[u8; 3]; 2] = [[1, 2, 3], [4, 5, 6]];

        let _ = <&[Srgb<u8>]>::from_arrays(slice);
        let _ = <&[Srgb<u8>]>::from_arrays(&slice_box);
        let _ = <&[Srgb<u8>]>::from_arrays(&vec);
        let _ = <&[Srgb<u8>]>::from_arrays(&array);

        let _ = <&mut [Srgb<u8>]>::from_arrays(slice_mut);
        let _ = <&mut [Srgb<u8>]>::from_arrays(&mut slice_box);
        let _ = <&mut [Srgb<u8>]>::from_arrays(&mut vec);
        let _ = <&mut [Srgb<u8>]>::from_arrays(&mut array);

        let _ = Box::<[Srgb<u8>]>::from_arrays(slice_box);
        let _ = Vec::<Srgb<u8>>::from_arrays(vec);
        let _ = <[Srgb<u8>; 2]>::from_arrays(array);
    }

    #[test]
    fn arrays_into() {
        let slice: &[[u8; 3]] = &[[1, 2, 3], [4, 5, 6]];
        let slice_mut: &mut [[u8; 3]] = &mut [[1, 2, 3], [4, 5, 6]];
        let mut slice_box: Box<[[u8; 3]]> = vec![[1, 2, 3], [4, 5, 6]].into_boxed_slice();
        let mut vec: Vec<[u8; 3]> = vec![[1, 2, 3], [4, 5, 6]];
        let mut array: [[u8; 3]; 2] = [[1, 2, 3], [4, 5, 6]];

        let _: &[Srgb<u8>] = slice.arrays_into();
        let _: &[Srgb<u8>] = (&slice_box).arrays_into();
        let _: &[Srgb<u8>] = (&vec).arrays_into();
        let _: &[Srgb<u8>] = (&array).arrays_into();

        let _: &mut [Srgb<u8>] = slice_mut.arrays_into();
        let _: &mut [Srgb<u8>] = (&mut slice_box).arrays_into();
        let _: &mut [Srgb<u8>] = (&mut vec).arrays_into();
        let _: &mut [Srgb<u8>] = (&mut array).arrays_into();

        let _: Box<[Srgb<u8>]> = slice_box.arrays_into();
        let _: Vec<Srgb<u8>> = vec.arrays_into();
        let _: [Srgb<u8>; 2] = array.arrays_into();
    }

    #[test]
    fn into_arrays() {
        let slice: &[Srgb<u8>] = &[Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let slice_mut: &mut [Srgb<u8>] = &mut [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let mut slice_box: Box<[Srgb<u8>]> =
            vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)].into_boxed_slice();
        let mut vec: Vec<Srgb<u8>> = vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let mut array: [Srgb<u8>; 2] = [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];

        let _: &[[u8; 3]] = slice.into_arrays();
        let _: &[[u8; 3]] = (&slice_box).into_arrays();
        let _: &[[u8; 3]] = (&vec).into_arrays();
        let _: &[[u8; 3]] = (&array).into_arrays();

        let _: &mut [[u8; 3]] = slice_mut.into_arrays();
        let _: &mut [[u8; 3]] = (&mut slice_box).into_arrays();
        let _: &mut [[u8; 3]] = (&mut vec).into_arrays();
        let _: &mut [[u8; 3]] = (&mut array).into_arrays();

        let _: Box<[[u8; 3]]> = slice_box.into_arrays();
        let _: Vec<[u8; 3]> = vec.into_arrays();
        let _: [[u8; 3]; 2] = array.into_arrays();
    }

    #[test]
    fn arrays_from() {
        let slice: &[Srgb<u8>] = &[Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let slice_mut: &mut [Srgb<u8>] = &mut [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let mut slice_box: Box<[Srgb<u8>]> =
            vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)].into_boxed_slice();
        let mut vec: Vec<Srgb<u8>> = vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let mut array: [Srgb<u8>; 2] = [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];

        let _ = <&[[u8; 3]]>::arrays_from(slice);
        let _ = <&[[u8; 3]]>::arrays_from(&slice_box);
        let _ = <&[[u8; 3]]>::arrays_from(&vec);
        let _ = <&[[u8; 3]]>::arrays_from(&array);

        let _ = <&mut [[u8; 3]]>::arrays_from(slice_mut);
        let _ = <&mut [[u8; 3]]>::arrays_from(&mut slice_box);
        let _ = <&mut [[u8; 3]]>::arrays_from(&mut vec);
        let _ = <&mut [[u8; 3]]>::arrays_from(&mut array);

        let _ = Box::<[[u8; 3]]>::arrays_from(slice_box);
        let _ = Vec::<[u8; 3]>::arrays_from(vec);
        let _ = <[[u8; 3]; 2]>::arrays_from(array);
    }
}
