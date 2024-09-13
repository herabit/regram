use crate::util::slice::NonEmpty;
use bumpalo::Bump;
use std::{
    fmt,
    iter::FusedIterator,
    ops::{Deref, DerefMut, Range, RangeInclusive},
};

use super::Ast;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Class<'a> {
    pub ranges: &'a mut NonEmpty<ByteRange>,
}

impl<'a> Class<'a> {
    #[inline]
    #[must_use]
    pub fn clone_into<'b>(&self, bump: &'b Bump) -> Class<'b> {
        let slice = bump.alloc_slice_copy(self);

        Class {
            ranges: slice.try_into().unwrap(),
        }
    }

    #[inline]
    pub fn normalize(this: &mut Ast<'a>) {
        if let Ast::Class(Class { ranges }) = this {
            // NOTE: preserving initial order is not necessary
            ranges.sort_unstable_by_key(|range| range.start);
            // TODO: Merge overlapping ranges.
        }
    }
}

impl Deref for Class<'_> {
    type Target = NonEmpty<ByteRange>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.ranges
    }
}

impl DerefMut for Class<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ranges
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ByteRange {
    pub start: u8,
    pub end: u8,
}

impl ByteRange {
    #[inline]
    #[must_use]
    pub const fn merge(self, _other: ByteRange) -> Option<ByteRange> {
        todo!()
    }

    #[inline]
    #[must_use]
    pub const fn to_inclusive(self) -> RangeInclusive<u8> {
        self.start..=self.end
    }

    #[inline]
    #[must_use]
    pub const fn iter(self) -> ByteRangeIter {
        ByteRangeIter {
            iter: (self.start as u16)..(self.end as u16 + 1),
        }
    }
}

impl fmt::Debug for ByteRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..={}", self.start, self.end)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ByteRangeIter {
    iter: Range<u16>,
}

impl Iterator for ByteRangeIter {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|b| b as u8)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(|b| b as u8)
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.iter.last().map(|b| b as u8)
    }

    #[inline]
    fn min(mut self) -> Option<Self::Item> {
        self.next()
    }

    #[inline]
    fn max(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    #[inline]
    fn is_sorted(self) -> bool {
        false
    }
}

impl DoubleEndedIterator for ByteRangeIter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|b| b as u8)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(|b| b as u8)
    }
}

impl ExactSizeIterator for ByteRangeIter {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl FusedIterator for ByteRangeIter {}

impl IntoIterator for ByteRange {
    type Item = u8;
    type IntoIter = ByteRangeIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
