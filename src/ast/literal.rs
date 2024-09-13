use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use bumpalo::Bump;

use crate::util::{slice::NonEmpty, BytesExt};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Lit<'a> {
    pub bytes: &'a mut NonEmpty<u8>,
}

impl<'a> Lit<'a> {
    #[inline]
    #[must_use]
    pub fn clone_into<'b>(&self, bump: &'b Bump) -> Lit<'b> {
        let slice = bump.alloc_slice_copy(self);

        Lit {
            bytes: slice.try_into().unwrap(),
        }
    }
}

impl Deref for Lit<'_> {
    type Target = NonEmpty<u8>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.bytes
    }
}

impl DerefMut for Lit<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.bytes
    }
}

impl fmt::Debug for Lit<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.byte_str().fmt(f)
    }
}

impl fmt::Display for Lit<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.byte_str().fmt(f)
    }
}
