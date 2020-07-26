use std::collections::BTreeSet;
use terms::literal::Literal;
use terms::free_group_term::FreeGroupTerm;
use terms::short_free_group_term::ShortFreeGroupTerm;
use terms::l_group_term::LGroupTerm;
use super::normal_cnf::CNF;

/// Represents a meet of joins of free group terms of length at most three.
/// 
/// The term obtained is not provably equal to the term it is representing!
/// But, as it turns out, it satisfies that `ThreeCNF::from(term)` is always
/// non-negative, if, and only if, `term` is always non-negative.
/// 
/// # Examples
/// Sometimes, you get empty `ThreeCNF`s from non-empty terms:
/// ```
/// use terms::l_group_term::LGroupTerm;
/// use terms::free_group_term::FreeGroupTerm;
/// use terms::literal::lit;
/// use cnf::three_cnf::ThreeCNF;
/// use std::collections::BTreeSet;
/// let term = LGroupTerm::from(FreeGroupTerm::new(vec![lit('x'), lit('y'), lit('z'), lit('w')]));
/// let three_cnf = ThreeCNF::from(term);
/// let empty_three_cnf = ThreeCNF { meetands: BTreeSet::new() };
/// assert_eq!(empty_three_cnf, three_cnf);
/// ```
/// But, as soon as there is at least one join symbol in the `CNF`, we get shortening of literals
/// using the trick `e <= r v st` iff `e <= r v sX v xt`, where `x` is a variable that
/// does not appear in the formula.
/// ```
/// # use terms::l_group_term::LGroupTerm;
/// # use terms::free_group_term::FreeGroupTerm;
/// # use terms::literal::lit;
/// # use cnf::three_cnf::ThreeCNF;
/// # use std::collections::BTreeSet;
/// let joinand1 = LGroupTerm::from(FreeGroupTerm::new(vec![lit('x'), lit('y'), lit('z'), lit('w')]));
/// let joinand2 = LGroupTerm::from(lit('u'));
/// let mut joinands = BTreeSet::new();
/// joinands.insert(joinand1);
/// joinands.insert(joinand2);
/// let three_cnf = ThreeCNF::from(LGroupTerm::Join(joinands));
/// let expected_three_cnf = ThreeCNF { meetands: BTreeSet::new() };
/// assert_eq!(String::from("(u v V1zw v xyv1)"), three_cnf.to_string());
/// ```
#[derive(PartialEq, Eq, Debug)]
pub struct ThreeCNF {
    pub meetands: BTreeSet<BTreeSet<ShortFreeGroupTerm>>
}

impl From<LGroupTerm> for ThreeCNF {
    fn from(term: LGroupTerm) -> ThreeCNF {
        let normal_cnf = CNF::from(term);
        let mut new_meetands = BTreeSet::new();
        for meetand in normal_cnf.meetands {
            let mut count = 1;
            match meetand.len() {
                0 => panic!("empty meet!"),
                1 => {},  // this always extends to a partial order, so we leave out long individual atoms (they cannot be split anyway)
                _ => {
                    let mut joinands = BTreeSet::new();
                    for term in meetand {
                        for new_term in split(term, &mut count) {
                            joinands.insert(new_term);
                        }
                    }
                    new_meetands.insert(joinands);
                }
            };
        }
        ThreeCNF { meetands: new_meetands }
    }
}

impl ToString for ThreeCNF {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for meetand in &self.meetands {
            string.push('(');
            for joinand in meetand {
                string.push_str(joinand.to_string().as_str());
                string.push_str(" v ");
            }
            string = string[0 .. string.len() - 3].to_string();
            string.push_str(") ^ ");
        }
        if string.len() == 0 {
            return String::from("(())")
        }
        string[0..string.len() - 3].to_string()
    }
}

fn split(term: FreeGroupTerm, counter: &mut usize) -> BTreeSet<ShortFreeGroupTerm> {
    let mut output = BTreeSet::new();
    if term.literals.len() <= 3 {
        output.insert(ShortFreeGroupTerm::from(term.clone()));
        return output;
    }
    output.insert(ShortFreeGroupTerm {
        left:  Some(term.literals[0]),
        mid:   Some(term.literals[1]),
        right: Some(Literal::new('v', *counter, false))
    });

    let mut rest_literals = Vec::new();
    rest_literals.push(Literal::new('v', *counter + term.literals.len() - 4, true));
    for x in &term.literals[2 .. term.literals.len()] {
        rest_literals.push(*x);
    }
    let rest_term = FreeGroupTerm { literals: rest_literals };
    for x in split(rest_term, counter) {
        output.insert(x);
    }
    return output;
}