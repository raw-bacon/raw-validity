use super::l_group_term::*;
use super::free_group_term::*;
use super::*;
use std::collections::BTreeSet;

/// reduces the atom by reducing its associated free group term
pub (super) fn atom_reduced(x: FreeGroupTerm) -> LGroupTerm {
    LGroupTerm::Atom(x.reduced())
}

/// recursively absorbs inner meets
pub (super) fn meet_reduced(xs: BTreeSet<LGroupTerm>) -> LGroupTerm {
    let mut new_meetands = xs.clone();
    let mut old_meetands: BTreeSet<LGroupTerm>;
    let mut not_done = contains_meets(&xs);
    while not_done {
        old_meetands = new_meetands.clone();
        new_meetands = BTreeSet::new();
        for x in old_meetands {
            match x {
                LGroupTerm::Meet(ys) => {
                    for y in ys { new_meetands.insert(y.clone().reduced()); }
                },
                term => { new_meetands.insert(term.clone().reduced()); }
            }
        }
        not_done = contains_meets(&new_meetands);
    }
    match new_meetands.len() {
        0 => panic!("Unexpected empty meet"),
        1 => {
            let option = new_meetands.iter().next();
            match option {
                None => panic!("ultra-unexpected empty meet"),
                Some(x) => x.clone()
            }
        }
        _ => LGroupTerm::Meet(new_meetands)
    }
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

/// recursively absorbs inner joins
pub (super) fn join_reduced(xs: BTreeSet<LGroupTerm>) -> LGroupTerm {
    let mut new_joinands = xs.clone();
    let mut old_joinands: BTreeSet<LGroupTerm>;
    let mut not_done = contains_joins(&xs);
    while not_done {
        old_joinands = new_joinands.clone();
        new_joinands = BTreeSet::new();
        for x in old_joinands {
            match x {
                LGroupTerm::Join(ys) => {
                    for y in ys { new_joinands.insert(y.clone().reduced()); }
                },
                term => { new_joinands.insert(term.clone().reduced()); }
            }
        }
        not_done = contains_joins(&new_joinands);
    }
    match new_joinands.len() {
        0 => panic!("Unexpected empty join"),
        1 => {
            let option = new_joinands.iter().next();
            match option {
                None => panic!("ultra-unexpected empty meet"),
                Some(x) => x.clone()
            }
        }
        _ => LGroupTerm::Join(new_joinands)
    }
}

fn contains_joins(xs: &BTreeSet<LGroupTerm>) -> bool {
    for x in xs {
        match x {
            LGroupTerm::Join(_) => { return true; },
            _ => {}
        }
    }
    false
}

/// recursively absorbs products, then multiplies successive atoms as free group terms
pub (super) fn prod_reduced(xs: Vec<LGroupTerm>) -> LGroupTerm {
    let mut new_factors = xs.clone();
    let mut old_factors: Vec<LGroupTerm>;
    let mut not_done = contains_prods(&xs);
    while not_done {
        old_factors = new_factors.clone();
        new_factors = Vec::new();
        for x in old_factors {
            match x {
                LGroupTerm::Prod(ys) => {
                    for y in ys { new_factors.push(y.clone().reduced()); }
                },
                term => { new_factors.push(term.clone().reduced()); }
            }
        }
        not_done = contains_prods(&new_factors);
    }

    // removing adjacent atoms
    let mut index = 0;
    while index < new_factors.len() - 1 {
        if is_atom(&new_factors[index]) && is_atom(&new_factors[index + 1]) {
            let left = new_factors.remove(index);
            let right = new_factors.remove(index);
            new_factors.insert(index, left * right);
        } else {
            index += 1;
        }
    }

    match new_factors.len() {
        0 => LGroupTerm::from(IDENTITY.clone()),
        1 => {
            let option = new_factors.iter().next();
            match option {
                None => panic!(""),
                Some(x) => x.clone()
            }
        }
        _ => LGroupTerm::Prod(new_factors)
    }
}

fn is_atom(x: &LGroupTerm) -> bool {
    match x {
        LGroupTerm::Atom(_) => true,
        _ => false
    }
}

fn contains_prods(xs: &Vec<LGroupTerm>) -> bool {
    for x in xs {
        match x {
            LGroupTerm::Prod(_) => { return true; },
            _ => {}
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::literal::*;
    use super::*;

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

    #[test]
    fn test_prod_reduced() {
        let result = LGroupTerm::Prod(vec![LGroupTerm::from(lit('x')), LGroupTerm::from(lit('y'))]);
        assert_eq!(LGroupTerm::Atom(FreeGroupTerm::new(vec![lit('x'), lit('y')])), result.reduced())
    }
}