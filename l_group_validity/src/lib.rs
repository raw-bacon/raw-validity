use l_group_formulas::formula::LGroupFormula;
use std::collections::BTreeSet;
use l_group_formulas::short_free_group_term::ShortFreeGroupTerm;
use l_group_cnf::three_cnf::ThreeCNF;
use extend_to_right_order::extend_to_right_order;
use l_group_formulas::Term;
use split_at_meets::split_at_meets;

mod extend_to_right_order;
mod split_at_meets;

/// Returns whether an `LGroupFormula` holds in all l-groups.
pub fn is_valid(eq: LGroupFormula) -> bool {
    let mut meetands: BTreeSet<BTreeSet<ShortFreeGroupTerm>> = BTreeSet::new();
    match eq {
        LGroupFormula::LGroupInequation(lhs, rhs) => {
            let split = split_at_meets(rhs.clone() * lhs.inverse());
            println!("Split up the inequation e <= {} to form {} meetands.", (rhs.clone() * lhs.inverse()).to_string(), split.len());
            for x in split {
                println!("Computing the short normal form of {}", x.to_string());
                let three_cnf = ThreeCNF::from(x);
                println!("The three-normal form is {}.\n", three_cnf.to_string());

                if three_cnf.meetands.len() == 0 {
                    return false
                }

                for meetand in three_cnf.meetands {
                    meetands.insert(meetand);
                }
            }
        },
        LGroupFormula::LGroupEquation(lhs, rhs) => {
            let split1 = split_at_meets(rhs.clone() * lhs.inverse());
            let split2 = split_at_meets(lhs.clone() * rhs.inverse());
            println!("Split up the two inequations into {} meetands.", split1.len() + split2.len());
            for x in split1.union(&split2) {
                let three_cnf = ThreeCNF::from(x.clone());
                println!("Constructed the formula {}.", three_cnf.to_string());
                for meetand in three_cnf.meetands {
                    meetands.insert(meetand);
                }
            }
        }
    };

    if meetands.len() == 0 {
        return false;
    }
    
    println!("Checking all meetands.");
    for meetand in meetands {
        // verbosity
        let mut print_string = String::new();
        for t in &meetand {
            print_string.push_str(t.to_string().as_str());
            print_string.push_str(", ");
        }
        print_string.pop();
        print_string.pop();
        println!("Checking whether {} extends to a right order.", print_string);
        // end verbosity
       
        if extend_to_right_order(Box::new(meetand)) {
            return false;
        }
    }
    return true;
}

pub fn is_valid_from_string(s: &str) -> bool {
    is_valid(LGroupFormula::from(s))
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
        
    #[test]
    fn test_pyvalidity_bug() {
        check_valid("e <= xY v yZ v zX");
    }

    #[test]
    fn test_trivial() {
        check_valid("e=e");
    }
}
