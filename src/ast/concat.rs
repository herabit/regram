use std::mem;

use super::{Ast, Children};
use bumpalo::Bump;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Concat<'a> {
    pub children: &'a mut Children<'a>,
}

impl<'a> Concat<'a> {
    #[inline]
    #[must_use]
    pub fn clone_into<'b>(&self, bump: &'b Bump) -> Concat<'b> {
        let iter = self.children.iter().map(|child| child.clone_into(bump));
        let slice = bump.alloc_slice_fill_iter(iter);

        Concat {
            children: slice.try_into().unwrap(),
        }
    }

    #[inline]
    pub fn normalize(this: &mut Ast<'a>) {
        if let Ast::Concat(Concat { children }) = this {
            children.iter_mut().for_each(Ast::normalize);

            if children.len().get() == 1 {
                *this = mem::take(children.first_mut());
            }
        }
    }
}
