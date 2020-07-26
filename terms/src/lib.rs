pub trait Term {
    fn inverse(&self) -> Self;
}

pub trait Reducable {
    fn reduced(self) -> Self;
}

pub trait Meet<Rhs=Self> {
    type Output;
    fn meet(self, other: Rhs) -> Self::Output;
}

pub trait Join<Rhs=Self> {
    type Output;
    fn join(self, other: Rhs) -> Self::Output;
}

pub mod free_group_term;
pub mod literal;
pub mod l_group_term;
pub mod l_group_term_reducing;
pub mod short_free_group_term;
