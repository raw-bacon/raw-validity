use super::free_group_term::*;
use super::*;
use super::literal::*;
use std::collections::BTreeSet;
use std::ops::Mul;
use super::l_group_term_reducing::*;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum LGroupTerm {
    Atom(FreeGroupTerm),
    Meet(BTreeSet<LGroupTerm>),
    Join(BTreeSet<LGroupTerm>),
    Prod(Vec<LGroupTerm>)
}

impl From<FreeGroupTerm> for LGroupTerm {
    fn from(x: FreeGroupTerm) -> LGroupTerm {
        LGroupTerm::Atom(x)
    }
}

impl From<Literal> for LGroupTerm {
    fn from(x: Literal) -> LGroupTerm {
        LGroupTerm::Atom(FreeGroupTerm::from(x))
    }
}

impl Mul for LGroupTerm {
    type Output = LGroupTerm;

    fn mul(self, other: LGroupTerm) -> LGroupTerm {
        LGroupTerm::Prod(vec![self, other]).reduced()
    }
}

impl Meet for LGroupTerm {
    type Output = LGroupTerm;

    fn meet(self, other: LGroupTerm) -> LGroupTerm {
        let mut meetands = BTreeSet::new();
        meetands.insert(self);
        meetands.insert(other);LGroupTerm::Meet(meetands).reduced()
    }
}

impl Join for LGroupTerm {
    type Output = LGroupTerm;

    fn join(self, other: LGroupTerm) -> LGroupTerm {
        let mut joinands = BTreeSet::new();
        joinands.insert(self);
        joinands.insert(other);
        LGroupTerm::Join(joinands).reduced()
    }
}

impl Term for LGroupTerm {
    fn inverse(&self) -> LGroupTerm {
        match self {
            LGroupTerm::Atom(x) => LGroupTerm::Atom(x.inverse()),
            LGroupTerm::Meet(xs) => LGroupTerm::Join(xs.iter().map(|x| x.inverse()).collect()),
            LGroupTerm::Join(xs) => LGroupTerm::Meet(xs.iter().map(|x| x.inverse()).collect()),
            LGroupTerm::Prod(xs) => {
                let mut new_xs: Vec<LGroupTerm> = xs.iter().map(|x| x.inverse()).collect();
                new_xs.reverse();
                LGroupTerm::Prod(new_xs)
            }
        }
    }
}

impl Reducable for LGroupTerm {
    fn reduced(self) -> LGroupTerm{
        match self {
            LGroupTerm::Atom(x) => atom_reduced(x),
            LGroupTerm::Meet(xs) => meet_reduced(xs),
            LGroupTerm::Join(xs) => join_reduced(xs),
            LGroupTerm::Prod(xs) => prod_reduced(xs)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse_atom() {
        let x = lit('x');
        let y = lit('y');
        let z = lit('z');
        let xyz = FreeGroupTerm { literals: vec![x,y,z] };
        let term = LGroupTerm::Atom(xyz);
        let inverse = LGroupTerm::Atom(FreeGroupTerm { literals: vec![z.inverse(), y.inverse(), x.inverse()]});
        assert_eq!(inverse, term.inverse());
    }

    #[test]
    fn test_inverse_meet() {
        let x = LGroupTerm::Atom(FreeGroupTerm { literals: vec![lit('x')]});
        let y = LGroupTerm::Atom(FreeGroupTerm { literals: vec![lit('y')]});
        let mut meetands = BTreeSet::new();
        meetands.insert(x.clone());
        meetands.insert(y.clone());
        let meet = LGroupTerm::Meet(meetands);
        let mut inverse_meetands = BTreeSet::new();
        inverse_meetands.insert(x.inverse());
        inverse_meetands.insert(y.inverse());
        let inverse = LGroupTerm::Join(inverse_meetands);
        assert_eq!(inverse, meet.inverse());
    }

    #[test]
    fn test_inverse_join() {
        let x = LGroupTerm::Atom(FreeGroupTerm { literals: vec![lit('x')]});
        let y = LGroupTerm::Atom(FreeGroupTerm { literals: vec![lit('y')]});
        let mut joinands = BTreeSet::new();
        joinands.insert(x.clone());
        joinands.insert(y.clone());
        let join = LGroupTerm::Join(joinands);
        let mut inverse_joinands = BTreeSet::new();
        inverse_joinands.insert(x.inverse());
        inverse_joinands.insert(y.inverse());
        let inverse = LGroupTerm::Meet(inverse_joinands);
        assert_eq!(inverse, join.inverse());
    }

    #[test]
    fn test_inverse_recursive() {
        let x = LGroupTerm::Atom(FreeGroupTerm { literals: vec![lit('x')]});
        let y = LGroupTerm::Atom(FreeGroupTerm { literals: vec![lit('y')]});
        let z = LGroupTerm::Atom(FreeGroupTerm { literals: vec![lit('z')]});
        let z_inv = z.inverse();

        let mut meetands = BTreeSet::new();
        meetands.insert(x.clone());
        meetands.insert(y.clone());
        let meet = LGroupTerm::Meet(meetands);
        
        let mut joinands = BTreeSet::new();
        joinands.insert(x.inverse());
        joinands.insert(y.inverse());
        let inverse_of_meet = LGroupTerm::Join(joinands);

        let prod = LGroupTerm::Prod(vec![meet, z]);
        let prod_inverse = LGroupTerm::Prod(vec![z_inv, inverse_of_meet]);

        assert_eq!(prod_inverse, prod.inverse());
    }
}