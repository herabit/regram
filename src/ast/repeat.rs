use std::{
    mem,
    ops::{Bound, RangeBounds},
};

use bumpalo::Bump;

use super::Ast;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Repeat<'a> {
    pub kind: RepeatKind,
    pub child: &'a mut Ast<'a>,
}

impl<'a> Repeat<'a> {
    #[inline]
    #[must_use]
    pub fn clone_into<'b>(&self, bump: &'b Bump) -> Repeat<'b> {
        let child = self.child.clone_into(bump);

        Repeat {
            kind: self.kind,
            child: bump.alloc(child),
        }
    }

    #[inline]
    pub fn normalize(this: &mut Ast<'a>) {
        if let Ast::Repeat(Repeat { kind, child }) = this {
            *kind = kind.normalize();

            match kind {
                RepeatKind::Exact(0) => *this = Ast::Empty,
                RepeatKind::Exact(1) => {
                    // Overwrite `this` first to allow a tailcall.
                    *this = mem::take(child);

                    this.normalize()
                }
                _ => child.normalize(),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RepeatKind {
    Exact(u32),
    AtLeast(u32),
    Bounded(u32, u32),
}

impl RepeatKind {
    pub const ZERO_OR_ONE: RepeatKind = RepeatKind::Bounded(0, 1);
    pub const ZERO_OR_MORE: RepeatKind = RepeatKind::AtLeast(0);
    pub const ONE_OR_MORE: RepeatKind = RepeatKind::AtLeast(1);

    #[inline]
    #[must_use]
    pub const fn normalize(self) -> Self {
        match self {
            Self::Bounded(start, end) if start == end => Self::Exact(start),
            this => this,
        }
    }

    #[inline]
    #[must_use]
    pub const fn is_valid(self) -> bool {
        match self {
            Self::Bounded(start, end) => start <= end,
            _ => true,
        }
    }

    #[inline]
    #[must_use]
    pub const fn start(self) -> u32 {
        match self {
            Self::Exact(start) => start,
            Self::AtLeast(start) => start,
            Self::Bounded(start, _) => start,
        }
    }

    #[inline]
    #[must_use]
    pub const fn start_bound(&self) -> Bound<&u32> {
        match self {
            RepeatKind::Exact(start) => Bound::Included(start),
            RepeatKind::AtLeast(start) => Bound::Included(start),
            RepeatKind::Bounded(start, _) => Bound::Included(start),
        }
    }

    #[inline]
    #[must_use]
    pub const fn end(self) -> Option<u32> {
        match self {
            RepeatKind::Exact(end) => Some(end),
            RepeatKind::AtLeast(_) => None,
            RepeatKind::Bounded(_, end) => Some(end),
        }
    }

    #[inline]
    #[must_use]
    pub const fn end_bound(&self) -> Bound<&u32> {
        match self {
            RepeatKind::Exact(end) => Bound::Included(end),
            RepeatKind::AtLeast(_) => Bound::Unbounded,
            RepeatKind::Bounded(_, end) => Bound::Included(end),
        }
    }
}

impl RangeBounds<u32> for RepeatKind {
    #[inline]
    fn start_bound(&self) -> Bound<&u32> {
        <RepeatKind>::start_bound(self)
    }

    #[inline]
    fn end_bound(&self) -> Bound<&u32> {
        <RepeatKind>::end_bound(self)
    }
}
