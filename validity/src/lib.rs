use terms::formula::LGroupFormula;
use std::collections::BTreeSet;
use terms::short_free_group_term::ShortFreeGroupTerm;
use cnf::three_cnf::ThreeCNF;
use extend_to_right_order::extend_to_right_order;
use terms::Term;

mod extend_to_right_order;

/// Returns whether an `LGroupFormula` holds in all l-groups.
pub fn is_valid(eq: LGroupFormula) -> bool {
    let meetands: BTreeSet<BTreeSet<ShortFreeGroupTerm>>;
    meetands = match eq {
        LGroupFormula::LGroupInequation(lhs, rhs) => ThreeCNF::from(rhs * lhs.inverse()).meetands,
        LGroupFormula::LGroupEquation(lhs, rhs) => ThreeCNF::from(rhs.clone() * lhs.inverse()).meetands.union(
                                                        &ThreeCNF::from(lhs * rhs.inverse()).meetands
                                                    ).cloned().collect()
    };
    if meetands == BTreeSet::new() {
        return false;
    }
    for meetand in meetands {
        if extend_to_right_order(meetand) {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn test_valid(string: &str) {
        assert_eq!(true, is_valid(LGroupFormula::from(string)));
    }

    fn test_invalid(string: &str) {
        assert_eq!(false, is_valid(LGroupFormula::from(string)));
    }

    #[test]
    fn test_distributive() {
        test_valid( "x ^ (y v z) = (x ^ y) v (x ^ z) ");
    }

    #[test]
    fn test_commutativity() {
        test_invalid("xy = yx");
    }

}