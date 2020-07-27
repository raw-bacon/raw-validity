/// All term-like objects in our context have an inverse.
pub trait Term {
    fn inverse(&self) -> Self;
}

/// If terms can be cleaned up in a non-expensive way, they
/// implement this trait.
pub trait Reducable {
    fn reduced(self) -> Self;
}

/*
pub trait Meet<Rhs=Self> {
    type Output;
    fn meet(self, other: Rhs) -> Self::Output;
}

pub trait Join<Rhs=Self> {
    type Output;
    fn join(self, other: Rhs) -> Self::Output;
}
*/

/// The module containing everything about free group terms,
/// in particular the struct `FreeGroupTerm`.
pub mod free_group_term;

/// The module containing everything about literals,
/// in particular the struct `Literal`.
pub mod literal;

/// The module containing everything about l-group-terms
/// (but not their cnfs), in particular the struct `LGroupTerm`.
pub mod l_group_term;
mod l_group_term_reducing;

/// The module containing everything about 'short' (shorter than three)
/// free group terms, in particular the struct `ShortFreeGroupTerm`.
pub mod short_free_group_term;


pub mod parsing_error;