use super::Ast;
use bumpalo::Bump;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Group<'a> {
    pub child: &'a mut Ast<'a>,
}

impl<'a> Group<'a> {
    #[inline]
    #[must_use]
    pub fn clone_into<'b>(&self, bump: &'b Bump) -> Group<'b> {
        let child = self.child.clone_into(bump);

        Group {
            child: bump.alloc(child),
        }
    }

    #[inline]
    pub fn normalize(this: &mut Ast<'a>) {
        if let Ast::Group(Group { child }) = this {
            child.normalize();
        }
    }
}
