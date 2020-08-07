use l_group_formulas::l_group_term::LGroupTerm;
use std::collections::BTreeSet;
use l_group_formulas::Reducable;

pub (super) fn split_at_meets(term: LGroupTerm) -> BTreeSet<LGroupTerm> {
    let mut terms : BTreeSet<LGroupTerm> = BTreeSet::new();
    match term {
        LGroupTerm::Meet(xs) => {
            for x in xs {
                for y in split_at_meets(x) {
                    terms.insert(y);
                }
            }
        },
        LGroupTerm::Atom(x) => {
            terms.insert(LGroupTerm::Atom(x));
        },
        LGroupTerm::Join(xs) => {
            let mut splits = Vec::new();
            for x in xs {
                splits.push(split_at_meets(x));
            }
            for y in join_all_combinations(splits) {
                terms.insert(y);
            }
        },
        LGroupTerm::Prod(xs) => {
            let mut splits = Vec::new();
            for x in xs {
                splits.push(split_at_meets(x));
            }
            for y in multiply_all_combinations(splits) {
                terms.insert(y);
            }
        }
    };
    terms
}

fn join_all_combinations(sets_of_joinands: Vec<BTreeSet<LGroupTerm>>) -> BTreeSet<LGroupTerm> {
    let mut result = BTreeSet::new();
    let mut iterator = sets_of_joinands.iter();
    if let Some(first_set) = iterator.next() {
        for x in first_set {
            result.insert(x.clone());
        }
    }
    for joinands in iterator {
        let mut tmp = BTreeSet::new();
        for r in &result {
            for j in joinands {
                let mut tmp_joinands = BTreeSet::new();
                tmp_joinands.insert(r.clone());
                tmp_joinands.insert(j.clone());
                let new_r = LGroupTerm::Join(tmp_joinands).reduced();
                tmp.insert(new_r);
            }
        }
        result = tmp;
    }
    result
}

fn multiply_all_combinations(sets_of_factors: Vec<BTreeSet<LGroupTerm>>) -> BTreeSet<LGroupTerm> {
    let mut result = BTreeSet::new();
    let mut iterator = sets_of_factors.iter();
    if let Some(first_set) = iterator.next() {
        for x in first_set {
            result.insert(x.clone());
        }
    }
    for factors in iterator {
        let mut tmp = BTreeSet::new();
        for r in &result {
            for f in factors {
                tmp.insert(r.clone() * f.clone());
            }
        }
        result = tmp;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split() {
        let mut meetands = BTreeSet::new();
        meetands.insert(LGroupTerm::from('y'));
        meetands.insert(LGroupTerm::from('z'));

        let mut joinands = BTreeSet::new();
        joinands.insert(LGroupTerm::Meet(meetands));
        joinands.insert(LGroupTerm::from('x'));

        let join = LGroupTerm::Join(joinands);

        let mut expected = BTreeSet::new();
        let mut expected_joinands1 = BTreeSet::new();
        expected_joinands1.insert(LGroupTerm::from('x'));
        expected_joinands1.insert(LGroupTerm::from('y'));
        let mut expected_joinands2 = BTreeSet::new();
        expected_joinands2.insert(LGroupTerm::from('x'));
        expected_joinands2.insert(LGroupTerm::from('z'));
        expected.insert(LGroupTerm::Join(expected_joinands1));
        expected.insert(LGroupTerm::Join(expected_joinands2));

        for x in split_at_meets(join.clone()) {
            println!("{}", x.to_string())
        }
        assert_eq!(expected, split_at_meets(join));

    }
}
