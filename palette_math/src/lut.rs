//! Lookup tables.

use core::marker::PhantomData;

/// A lookup table that maps values of type `I` to values of type `V`.
///
/// `I` and `V` are the index and value types, and `T` determines the storage
/// type for the table.
pub struct Lut<I, V, T>
where
    T: LutType<V>,
    T::Table: Sized,
{
    table: T::Table,
    lookup: PhantomData<fn(I) -> V>,
}

impl<I, V, T> Lut<I, V, T>
where
    T: LutType<V>,
    T::Table: Sized,
{
    /// Create a new lookup table.
    #[inline]
    pub const fn new(table: T::Table) -> Self
    where
        T::Table: Sized,
    {
        Lut {
            table,
            lookup: PhantomData,
        }
    }

    /// Get the value at an index.
    #[inline]
    pub fn lookup(&self, index: I) -> &V
    where
        T: Lookup<I, V>,
    {
        T::lookup(&self.table, index)
    }

    /// Get a lookup table that uses a reference to this table.
    #[inline]
    pub fn get_ref(&self) -> Lut<I, V, &'_ T> {
        Lut {
            table: &self.table,
            lookup: PhantomData,
        }
    }

    /// Get a lookup table that uses a slice reference to this table.
    #[inline]
    pub fn get_slice(&self) -> Lut<I, V, &'_ SliceTable> {
        Lut {
            table: self.table.as_ref(),
            lookup: PhantomData,
        }
    }
}

impl<I, V, T> AsRef<[V]> for Lut<I, V, T>
where
    T: LutType<V>,
    T::Table: Sized,
{
    fn as_ref(&self) -> &[V] {
        self.table.as_ref()
    }
}

/// Represents the storage method for a lookup table.
pub trait LutType<T> {
    /// The concrete storage type for any value type `T`.
    type Table: AsRef<[T]> + ?Sized;
}

/// Index a lookup table with any value of type `I`.
pub trait Lookup<I, T>: LutType<T> {
    /// Get the value at an index.
    fn lookup(table: &Self::Table, index: I) -> &T;
}

impl<'a, T, V> LutType<V> for &'a T
where
    T: LutType<V>,
    T::Table: 'a,
{
    type Table = &'a T::Table;
}

impl<'a, T, I, V> Lookup<I, V> for &'a T
where
    T: Lookup<I, V>,
    T::Table: 'a,
{
    fn lookup(table: &Self::Table, index: I) -> &V {
        T::lookup(table, index)
    }
}

/// A slice as storage for a lookup table.
pub enum SliceTable {}

impl<T> LutType<T> for SliceTable {
    type Table = [T];
}

impl<I, T> Lookup<I, T> for SliceTable
where
    usize: From<I>,
{
    fn lookup(table: &Self::Table, index: I) -> &T {
        let index = usize::from(index);
        &table[index]
    }
}

/// An array as storage for a lookup table.
pub enum ArrayTable<const N: usize> {}

impl<T, const N: usize> LutType<T> for ArrayTable<N> {
    type Table = [T; N];
}

impl<T> Lookup<u8, T> for ArrayTable<256> {
    fn lookup(table: &Self::Table, index: u8) -> &T {
        let index = usize::from(index);

        debug_assert!(index < table.as_ref().len());

        // SAFETY: A u8 value is never 256 or higher.
        unsafe { table.as_ref().get_unchecked(index) }
    }
}

impl<T> Lookup<u16, T> for ArrayTable<65536> {
    fn lookup(table: &Self::Table, index: u16) -> &T {
        let index = usize::from(index);

        debug_assert!(index < table.as_ref().len());

        // SAFETY: A u16 value is never 65536 or higher.
        unsafe { table.as_ref().get_unchecked(index) }
    }
}

/// A [Vec][alloc::vec::Vec] as storage for a lookup table.
#[cfg(feature = "alloc")]
pub enum VecTable {}

#[cfg(feature = "alloc")]
impl<T> LutType<T> for VecTable {
    type Table = alloc::vec::Vec<T>;
}

#[cfg(feature = "alloc")]
impl<I, T> Lookup<I, T> for VecTable
where
    usize: From<I>,
{
    fn lookup(table: &Self::Table, index: I) -> &T {
        let index = usize::from(index);
        &AsRef::<[T]>::as_ref(table)[index]
    }
}
