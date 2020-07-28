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
    if meetands.len() == 0 {
        return false;
    }
    for meetand in meetands {
        if extend_to_right_order(Box::new(meetand)) {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn check_valid(string: &str) {
        assert_eq!(true, is_valid(LGroupFormula::from(string)));
    }

    fn check_invalid(string: &str) {
        assert_eq!(false, is_valid(LGroupFormula::from(string)));
    }

    #[test]
    fn test_distributive() {
        check_valid( "x ^ (y v z) = (x ^ y) v (x ^ z) ");
    }

    #[test]
    fn test_mul_distributive() {
        check_valid("x(y v z)w = xyw v xzw");
        check_valid("x(y ^ z)w = xyw ^ xzw ");
    }

    #[test]
    fn test_de_morgan() {
        check_valid("X ^ Y = -(x v y)");
        check_valid("X v Y = -(x ^ y)");
    }

    #[test]
    fn test_metcalfe_exercise18() {
        check_valid("e <= x v X");
        check_valid("xy ^ e <= x v y");
    }

    #[test]
    fn test_colacito_example_1point3point6() {
        check_valid("e <= xx v yy v XY");
    }

    #[test]
    fn test_prelinearity() {
        check_valid("(Xy ^ e) v (Yx ^ e) = e");
        check_valid("(xY ^ e) v (yX ^ e) = e");
    }

    #[test]
    fn test_commutativity() {
        check_invalid("xy = yx");
    }

    /*
    #[test]
    fn test_colacito_example_1point3point7() {
        check_invalid("e <= xx v xy v yX ");
    }

    #[test]
    fn test_representable_l_groups() {
        check_invalid("e <= x v yXY");
    }
    
    #[test]
    fn test_weakly_abelian() {
        check_invalid("(x ^ e)(x ^ e) <= Y(x ^ e)y");
    }

    #[test]
    fn test_representable_l_monoids() {
        check_invalid("xyz ^ rst <= xsz v ryt");
    }
    */
        
    #[test]
    fn test_pyvalidity_bug() {
        check_valid("e <= xY v yZ v zX");
    }
}