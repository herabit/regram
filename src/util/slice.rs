use core::{fmt, slice};
use std::{
    array::TryFromSliceError,
    borrow::{Borrow, BorrowMut},
    hash::Hash,
    hint::assert_unchecked,
    num::NonZeroUsize,
    ops::{Deref, DerefMut, Index, IndexMut},
};

/// Thin wrapper around a slice that always has at least one element.
#[repr(transparent)]
pub struct NonEmpty<T> {
    inner: [T],
}

impl<T> NonEmpty<T> {
    /// Get a reference to a non-empty slice from a single reference to an element.
    #[inline]
    #[must_use]
    pub const fn from_ref(r: &T) -> &NonEmpty<T> {
        NonEmpty::new(slice::from_ref(r))
    }

    /// Get a mutable reference to a non-empty slice from a single reference to an element.
    #[inline]
    #[must_use]
    pub fn from_mut(r: &mut T) -> &mut NonEmpty<T> {
        NonEmpty::new_mut(slice::from_mut(r))
    }

    /// Get a reference to a non-empty slice without doing any checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `slice` is not empty.
    #[inline]
    #[must_use]
    #[track_caller]
    pub const unsafe fn new_unchecked(slice: &[T]) -> &NonEmpty<T> {
        unsafe {
            assert_unchecked(slice.len() > 0);

            &*(slice as *const [T] as *const NonEmpty<T>)
        }
    }

    /// Try to get a reference to a non-empty slice.
    #[inline]
    #[must_use]
    pub const fn try_new(slice: &[T]) -> Result<&NonEmpty<T>, &[T]> {
        if slice.len() > 0 {
            // SAFETY: slice is not empty
            Ok(unsafe { NonEmpty::new_unchecked(slice) })
        } else {
            Err(slice)
        }
    }

    /// Get a reference to a non-empty slice.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is empty.
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn new(slice: &[T]) -> &NonEmpty<T> {
        match NonEmpty::try_new(slice) {
            Ok(slice) => slice,
            Err(..) => panic!("slice is empty"),
        }
    }

    /// Get a mutable reference to a non-empty slice without doing any checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `slice` is not empty.
    #[inline]
    #[must_use]
    #[track_caller]
    pub unsafe fn new_unchecked_mut(slice: &mut [T]) -> &mut NonEmpty<T> {
        unsafe {
            assert_unchecked(slice.len() > 0);

            &mut *(slice as *mut [T] as *mut NonEmpty<T>)
        }
    }

    /// Try to get a mutable reference to a non-empty slice.
    #[inline]
    #[must_use]
    pub fn try_new_mut(slice: &mut [T]) -> Result<&mut NonEmpty<T>, &mut [T]> {
        if slice.len() > 0 {
            // SAFETY: slice is not empty
            Ok(unsafe { NonEmpty::new_unchecked_mut(slice) })
        } else {
            Err(slice)
        }
    }

    /// Get a mutable reference to a non-empty slice.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is empty.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new_mut(slice: &mut [T]) -> &mut NonEmpty<T> {
        match NonEmpty::try_new_mut(slice) {
            Ok(slice) => slice,
            Err(..) => panic!("slice is empty"),
        }
    }

    /// Get the length of this slice (guaranteed to always be nonzero).
    #[inline]
    #[must_use]
    pub const fn len(&self) -> NonZeroUsize {
        // SAFETY: Constructing a `NonEmpty` requires that the slice is not empty.
        unsafe { NonZeroUsize::new_unchecked(self.inner.len()) }
    }

    /// Get a reference to the slice.
    #[inline]
    #[must_use]
    pub const fn as_slice(&self) -> &[T] {
        // Doing this instead of `&self.inner` hints to the compiler
        // that the length is nonzero, allowing for optimizations.
        let len = self.len();
        let ptr = self.inner.as_ptr();

        // SAFETY: `ptr` and `len` are guaranteed to be valid.
        unsafe { slice::from_raw_parts(ptr, len.get()) }
    }

    /// Get a mutable reference to the slice.
    #[inline]
    #[must_use]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        // See `as_slice` for why this is done instead of `&mut self.inner`.
        let len = self.len();
        let ptr = self.inner.as_mut_ptr();

        // SAFETY: `ptr` and `len` are guaranteed to be valid.
        unsafe { slice::from_raw_parts_mut(ptr, len.get()) }
    }

    /// Get a reference to the first element.
    #[inline]
    #[must_use]
    pub const fn first(&self) -> &T {
        match self.as_slice().first() {
            Some(first) => first,
            _ => unreachable!(),
        }
    }

    /// Get a mutable reference to the first element.
    #[inline]
    #[must_use]
    pub fn first_mut(&mut self) -> &mut T {
        match self.as_slice_mut().first_mut() {
            Some(first) => first,
            None => unreachable!(),
        }
    }

    /// Split the slice at the first element.
    #[inline]
    #[must_use]
    pub const fn split_first(&self) -> (&T, &[T]) {
        match self.as_slice().split_first() {
            Some(split) => split,
            None => unreachable!(),
        }
    }

    /// Split the slice mutably at the first element.
    #[inline]
    #[must_use]
    pub fn split_first_mut(&mut self) -> (&mut T, &mut [T]) {
        match self.as_slice_mut().split_first_mut() {
            Some(split) => split,
            None => unreachable!(),
        }
    }

    /// Get a reference to the last element.
    #[inline]
    #[must_use]
    pub const fn last(&self) -> &T {
        match self.as_slice().last() {
            Some(last) => last,
            None => unreachable!(),
        }
    }

    /// Get a mutable reference to the last element.
    #[inline]
    #[must_use]
    pub fn last_mut(&mut self) -> &mut T {
        match self.as_slice_mut().last_mut() {
            Some(last) => last,
            None => unreachable!(),
        }
    }

    /// Split the slice at the last element.
    #[inline]
    #[must_use]
    pub const fn split_last(&self) -> (&T, &[T]) {
        match self.as_slice().split_last() {
            Some(split) => split,
            None => unreachable!(),
        }
    }

    /// Split the slice mutably at the last element.
    #[inline]
    #[must_use]
    pub fn split_last_mut(&mut self) -> (&mut T, &mut [T]) {
        match self.as_slice_mut().split_last_mut() {
            Some(split) => split,
            None => unreachable!(),
        }
    }
}

impl<'a, T> TryFrom<&'a [T]> for &'a NonEmpty<T> {
    type Error = &'a [T];

    #[inline]
    fn try_from(value: &'a [T]) -> Result<Self, Self::Error> {
        NonEmpty::try_new(value)
    }
}

impl<'a, T> From<&'a NonEmpty<T>> for &'a [T] {
    #[inline]
    fn from(value: &'a NonEmpty<T>) -> Self {
        value.as_slice()
    }
}

impl<'a, T> TryFrom<&'a mut [T]> for &'a mut NonEmpty<T> {
    type Error = &'a mut [T];

    #[inline]
    fn try_from(value: &'a mut [T]) -> Result<Self, Self::Error> {
        NonEmpty::try_new_mut(value)
    }
}

impl<'a, T> From<&'a mut NonEmpty<T>> for &'a mut [T] {
    #[inline]
    fn from(value: &'a mut NonEmpty<T>) -> Self {
        value.as_slice_mut()
    }
}

impl<T> AsRef<[T]> for NonEmpty<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> AsMut<[T]> for NonEmpty<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self.as_slice_mut()
    }
}

impl<T> Borrow<[T]> for NonEmpty<T> {
    #[inline]
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> BorrowMut<[T]> for NonEmpty<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_slice_mut()
    }
}

impl<T> Deref for NonEmpty<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> DerefMut for NonEmpty<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}

impl<T: Hash> Hash for NonEmpty<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl<T: PartialEq<U>, U> PartialEq<[U]> for NonEmpty<T> {
    #[inline]
    fn eq(&self, other: &[U]) -> bool {
        self.as_slice().eq(other)
    }

    #[inline]
    fn ne(&self, other: &[U]) -> bool {
        self.as_slice().ne(other)
    }
}

impl<T: PartialEq<U>, U> PartialEq<U> for NonEmpty<T> {
    #[inline]
    fn eq(&self, other: &U) -> bool {
        self.eq(slice::from_ref(other))
    }

    #[inline]
    fn ne(&self, other: &U) -> bool {
        self.ne(slice::from_ref(other))
    }
}

impl<T: PartialEq> PartialEq for NonEmpty<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.eq(other.as_slice())
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.ne(other.as_slice())
    }
}

impl<T: Eq> Eq for NonEmpty<T> {}

impl<T: PartialOrd> PartialOrd<[T]> for NonEmpty<T> {
    #[inline]
    fn partial_cmp(&self, other: &[T]) -> Option<std::cmp::Ordering> {
        self.as_slice().partial_cmp(other)
    }

    #[inline]
    fn lt(&self, other: &[T]) -> bool {
        self.as_slice().lt(other)
    }

    #[inline]
    fn le(&self, other: &[T]) -> bool {
        self.as_slice().le(other)
    }

    #[inline]
    fn gt(&self, other: &[T]) -> bool {
        self.as_slice().gt(other)
    }

    #[inline]
    fn ge(&self, other: &[T]) -> bool {
        self.as_slice().ge(other)
    }
}

impl<T: PartialOrd> PartialOrd for NonEmpty<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.as_slice())
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        self.lt(other.as_slice())
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        self.le(other.as_slice())
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        self.gt(other.as_slice())
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        self.ge(other.as_slice())
    }
}

impl<T: PartialOrd> PartialOrd<T> for NonEmpty<T> {
    #[inline]
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.partial_cmp(slice::from_ref(other))
    }

    #[inline]
    fn lt(&self, other: &T) -> bool {
        self.lt(slice::from_ref(other))
    }

    #[inline]
    fn le(&self, other: &T) -> bool {
        self.le(slice::from_ref(other))
    }

    #[inline]
    fn gt(&self, other: &T) -> bool {
        self.gt(slice::from_ref(other))
    }

    #[inline]
    fn ge(&self, other: &T) -> bool {
        self.ge(slice::from_ref(other))
    }
}

impl<T: Ord> Ord for NonEmpty<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<T: fmt::Debug> fmt::Debug for NonEmpty<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<T> fmt::Display for NonEmpty<T>
where
    [T]: fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<T, I> Index<I> for NonEmpty<T>
where
    [T]: Index<I>,
{
    type Output = <[T] as Index<I>>::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.as_slice().index(index)
    }
}

impl<T, I> IndexMut<I> for NonEmpty<T>
where
    [T]: IndexMut<I>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.as_slice_mut().index_mut(index)
    }
}

impl<T: Copy, const N: usize> TryFrom<&NonEmpty<T>> for [T; N] {
    type Error = TryFromSliceError;

    #[inline]
    fn try_from(value: &NonEmpty<T>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

impl<'a, T> IntoIterator for &'a NonEmpty<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut NonEmpty<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
