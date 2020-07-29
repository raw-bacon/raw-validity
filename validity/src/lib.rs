use terms::formula::LGroupFormula;
use std::collections::BTreeSet;
use terms::short_free_group_term::ShortFreeGroupTerm;
use cnf::three_cnf::ThreeCNF;
use cnf::normal_cnf::CNF;
use extend_to_right_order::extend_to_right_order;
use terms::Term;

mod extend_to_right_order;

/// Returns whether an `LGroupFormula` holds in all l-groups.
pub fn is_valid(eq: LGroupFormula, verbose: bool) -> bool {
    let meetands: BTreeSet<BTreeSet<ShortFreeGroupTerm>>;
    meetands = match eq {
        LGroupFormula::LGroupInequation(lhs, rhs) => {
            let three_cnf = ThreeCNF::from(rhs.clone() * lhs.inverse());
            if verbose {
                println!("\nThe CNF of the inequality you entered is {}.", CNF::from(rhs.clone() * lhs.inverse()).to_string());
                println!("\nThe 3-CNF of the inequality you entered is {}.", three_cnf.to_string());
            }
            three_cnf.meetands
        },
        LGroupFormula::LGroupEquation(lhs, rhs) => {
            let three_cnf_one = ThreeCNF::from(rhs.clone() * lhs.inverse());
            let three_cnf_two = ThreeCNF::from(lhs.clone() * rhs.inverse());
            
            if verbose {
                println!("\nThe CNFs of the equality you entered are\n{}\nand\n{}", 
                         CNF::from(rhs.clone() * lhs.inverse()).to_string(), 
                         CNF::from(lhs * rhs.inverse()).to_string());
                println!("\nThe 3-CNFs of the equality you entered are\n{}\nand\n{}.", three_cnf_one.to_string(), three_cnf_two.to_string());
            }

            three_cnf_one.meetands.union(&three_cnf_two.meetands).cloned().collect()
        }
    };
    if meetands.len() == 0 {
        return false;
    }
    for meetand in meetands {
        if verbose {
            let mut meetand_string = String::new();
            meetand_string.push('{');
            for x in &meetand {
                meetand_string.push_str(x.to_string().as_str());
                meetand_string.push_str(", ");
            }
            meetand_string.pop();
            meetand_string.pop();
            meetand_string.push('}');
            println!("\nChecking whether {} extends to a right order.", meetand_string);
        }

        if extend_to_right_order(Box::new(meetand), verbose) {
            return false;
        }
    }
    return true;
}

pub fn is_valid_from_string(s: &str) -> bool {
    is_valid(LGroupFormula::from(s), false)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn check_valid(string: &str) {
        assert_eq!(true, is_valid(LGroupFormula::from(string), false));
    }

    fn check_invalid(string: &str) {
        assert_eq!(false, is_valid(LGroupFormula::from(string), false));
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