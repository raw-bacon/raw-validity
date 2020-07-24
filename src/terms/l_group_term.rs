use super::free_group_term::*;
use super::*;
use super::literal::*;
use std::collections::BTreeSet;
use std::ops::Mul;

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

fn atom_reduced(x: FreeGroupTerm) -> LGroupTerm {
    LGroupTerm::Atom(x.reduced())
}

fn meet_reduced(xs: BTreeSet<LGroupTerm>) -> LGroupTerm {
    let mut new_meetands = xs.clone();
    let mut old_meetands: BTreeSet<LGroupTerm>;
    let mut not_done = contains_meets(&xs);
    while not_done {
        old_meetands = new_meetands.clone();
        new_meetands = BTreeSet::new();
        for x in old_meetands.iter() {
            match x {
                LGroupTerm::Meet(ys) => {
                    for y in ys.iter() { new_meetands.insert(y.clone().reduced()); }
                },
                term => { new_meetands.insert(term.clone().reduced()); }
            }
        }
        not_done = contains_meets(&new_meetands);
    }
    LGroupTerm::Meet(new_meetands)
}

fn contains_meets(xs: &BTreeSet<LGroupTerm>) -> bool {
    for x in xs {
        match x {
            LGroupTerm::Meet(_) => { return true; },
            _ => {}
        }
    }
    false
}

fn join_reduced(xs: BTreeSet<LGroupTerm>) -> LGroupTerm {
    // TODO
    LGroupTerm::Join(xs)
}

fn prod_reduced(xs: Vec<LGroupTerm>) -> LGroupTerm {
    // TODO
    LGroupTerm::Prod(xs)
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
    fn test_meet_reduced() {
        // term = x ^ (y ^ (z ^ w))
        let x = LGroupTerm::from(lit('x'));
        let y = LGroupTerm::from(lit('y'));
        let z = LGroupTerm::from(lit('z'));
        let w = LGroupTerm::from(lit('w'));
        let mut zw_meetands = BTreeSet::new();
        zw_meetands.insert(z.clone());
        zw_meetands.insert(w.clone());
        let zw = LGroupTerm::Meet(zw_meetands);
        let mut yzw_meetands = BTreeSet::new();
        yzw_meetands.insert(zw);
        yzw_meetands.insert(y.clone());
        let yzw = LGroupTerm::Meet(yzw_meetands);
        let mut meetands = BTreeSet::new();
        meetands.insert(yzw);
        meetands.insert(x.clone());
        let meet = LGroupTerm::Meet(meetands);

        let mut goal_meetands = BTreeSet::new();
        goal_meetands.insert(x);
        goal_meetands.insert(y);
        goal_meetands.insert(z);
        goal_meetands.insert(w);
        let goal = LGroupTerm::Meet(goal_meetands);
        assert_eq!(goal, meet.reduced());
    }
}