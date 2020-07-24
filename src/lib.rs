pub mod terms {
    pub trait Term {
        fn inverse(&self) -> Self;
    }

    pub trait Reducable {
        fn reduce(&mut self);
    }

    pub mod free_group_term;
    pub mod literal;
    pub mod l_group_term;
}