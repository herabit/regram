mod alt;
mod class;
mod concat;
mod group;
mod literal;
mod repeat;

pub use alt::*;
pub use class::*;
pub use concat::*;
pub use group::*;
pub use literal::*;
pub use repeat::*;

use bumpalo::Bump;

use crate::util::slice::NonEmpty;

pub type Children<'a> = NonEmpty<Ast<'a>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Ast<'a> {
    #[default]
    Empty,
    Dot,
    Lit(Lit<'a>),
    Class(Class<'a>),
    Alt(Alt<'a>),
    Concat(Concat<'a>),
    Group(Group<'a>),
    Repeat(Repeat<'a>),
}

impl<'a> Ast<'a> {
    #[inline]
    #[must_use]
    pub fn clone_into<'b>(&self, bump: &'b Bump) -> Ast<'b> {
        match self {
            Ast::Empty => Ast::Empty,
            Ast::Dot => Ast::Dot,
            Ast::Lit(l) => Ast::Lit(l.clone_into(bump)),
            Ast::Class(c) => Ast::Class(c.clone_into(bump)),
            Ast::Alt(a) => Ast::Alt(a.clone_into(bump)),
            Ast::Concat(c) => Ast::Concat(c.clone_into(bump)),
            Ast::Group(g) => Ast::Group(g.clone_into(bump)),
            Ast::Repeat(r) => Ast::Repeat(r.clone_into(bump)),
        }
    }

    #[inline]
    #[must_use]
    pub fn children(&self) -> Option<&Children<'a>> {
        match self {
            Ast::Empty => None,
            Ast::Dot => None,
            Ast::Lit(_) => None,
            Ast::Class(_) => None,
            Ast::Alt(a) => Some(a.children),
            Ast::Concat(c) => Some(c.children),
            Ast::Group(g) => Some(Children::from_ref(g.child)),
            Ast::Repeat(r) => Some(Children::from_ref(r.child)),
        }
    }

    #[inline]
    #[must_use]
    pub fn children_mut(&mut self) -> Option<&mut Children<'a>> {
        match self {
            Ast::Empty => None,
            Ast::Dot => None,
            Ast::Lit(_) => None,
            Ast::Class(_) => None,
            Ast::Alt(a) => Some(a.children),
            Ast::Concat(c) => Some(c.children),
            Ast::Group(g) => Some(Children::from_mut(g.child)),
            Ast::Repeat(r) => Some(Children::from_mut(r.child)),
        }
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
        }
    }

    #[inline]
    #[must_use]
    pub fn is_dot(&self) -> bool {
        match self {
            Self::Dot => true,
            _ => false,
        }
    }

    pub fn normalize(&mut self) {
        match self {
            // Ast::Lit(_) => todo!(),
            Ast::Class(_) => Class::normalize(self),
            Ast::Alt(_) => Alt::normalize(self),
            Ast::Concat(_) => Concat::normalize(self),
            Ast::Group(_) => Group::normalize(self),
            Ast::Repeat(_) => Repeat::normalize(self),
            _ => {}
        }
    }
}

macro_rules! methods {
    ($(
        $kind:ident(
        $is_kind:ident,
        $as_kind:ident,
        $as_kind_mut:ident,
        $take_kind:ident
        $(,)?)

    ),* $(,)?) => {
        $(
            #[inline]
            #[must_use]
            pub fn $is_kind(&self) -> bool {
                match self {
                    Self::$kind(..) => true,
                    _ => false,
                }
            }

            #[inline]
            #[must_use]
            pub fn $as_kind(&self) -> Option<&$kind<'a>> {
                match self {
                    Self::$kind(value) => Some(value),
                    _ => None,
                }
            }

            #[inline]
            #[must_use]
            pub fn $as_kind_mut(&mut self) -> Option<&mut $kind<'a>> {
                match self {
                    Self::$kind(value) => Some(value),
                    _ => None,
                }
            }

            #[inline]
            #[must_use]
            pub fn $take_kind(&mut self) -> Option<$kind<'a>> {
                if !self.$is_kind() {
                    return None;
                }

                match ::core::mem::take(self) {
                    Self::$kind(value) => Some(value),
                    _ => unreachable!(),
                }
            }
        )*
    };
}

impl<'a> Ast<'a> {
    methods! {
        Lit(
            is_lit,
            as_lit,
            as_lit_mut,
            take_lit,
        ),
        Class(
            is_class,
            as_class,
            as_class_mut,
            take_class,
        ),
        Alt(
            is_alt,
            as_alt,
            as_alt_mut,
            take_alt,
        ),
        Concat(
            is_concat,
            as_concat,
            as_concat_mut,
            take_concat,
        ),
        Group(
            is_group,
            as_group,
            as_group_mut,
            take_group,
        ),
        Repeat(
            is_repeat,
            as_repeat,
            as_repeat_mut,
            take_repeat,
        ),
    }
}
