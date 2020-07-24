use super::free_group_term::*;
use super::*;
use super::literal::*;
use std::collections::BTreeSet;
use std::ops::Mul;
use super::l_group_term_reducing::*;

/// An element the term algebra of l-groups.
/// 
/// # Examples
/// To use this, we want to do some imports:
/// ```
/// use rsvalidity::terms::free_group_term::*;
/// use rsvalidity::terms::literal::*;
/// use rsvalidity::terms::l_group_term::*;
/// ```
/// An `LGroupTerm` be either an `Atom`, i.e.,
/// ```
/// # use rsvalidity::terms::free_group_term::*;
/// # use rsvalidity::terms::literal::*;
/// # use rsvalidity::terms::l_group_term::*;
/// let x = FreeGroupTerm::from(lit('x'));
/// let lGroupTerm = LGroupTerm::Atom(x);
/// ```
/// a `Meet`, a `Join`, or a `Product`. `Meet`s and `Join`s take `BTreeSet`s as arguments:
/// ```
/// # use rsvalidity::terms::free_group_term::*;
/// # use rsvalidity::terms::literal::*;
/// # use rsvalidity::terms::l_group_term::*;
/// use std::collections::BTreeSet;
/// let mut meetands = BTreeSet::new();
/// meetands.insert(LGroupTerm::from(lit('x')));
/// meetands.insert(LGroupTerm::from(lit('y')));
/// let meet = LGroupTerm::Meet(meetands);
/// ```
/// whereas `Product`s take `Vec<LGroupTerm>`s:
/// ```
/// # use rsvalidity::terms::free_group_term::*;
/// # use rsvalidity::terms::literal::*;
/// # use rsvalidity::terms::l_group_term::*;
/// let factors = vec![LGroupTerm::from(lit('x')), LGroupTerm::from(lit('y'))];
/// let product = LGroupTerm::Prod(factors);
/// ```
/// This models associativity of meets, joins, and products, and takes into
/// account the non-commutativity of the products, but also the commutativity
/// of meets and joins.
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
        match (self.clone(), other.clone()) {
            (LGroupTerm::Atom(x), LGroupTerm::Atom(y)) => LGroupTerm::Atom(x * y),
            _ => LGroupTerm::Prod(vec![self, other]).reduced()
        }
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
    fn reduced(self) -> LGroupTerm {
        match self {
            LGroupTerm::Atom(x) => atom_reduced(x),
            LGroupTerm::Meet(xs) => meet_reduced(xs).expect("Reducing failed."),
            LGroupTerm::Join(xs) => join_reduced(xs).expect("Reducing failed."),
            LGroupTerm::Prod(xs) => prod_reduced(xs).expect("Reducing failed.")
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

    #[test]
    fn test_mul_atoms() {
        let x = LGroupTerm::from(lit('x'));
        let y = LGroupTerm::from(lit('y'));
        assert_eq!(LGroupTerm::Atom(FreeGroupTerm::new(vec![lit('x'), lit('y')])), x * y)
    }
}