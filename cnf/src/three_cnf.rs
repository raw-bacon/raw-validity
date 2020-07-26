use std::collections::BTreeSet;
use terms::literal::Literal;
use terms::free_group_term::FreeGroupTerm;
use terms::short_free_group_term::ShortFreeGroupTerm;
use terms::l_group_term::LGroupTerm;
use super::normal_cnf::CNF;



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

fn split(term: FreeGroupTerm, counter: &mut usize) -> BTreeSet<ShortFreeGroupTerm> {
    let mut output = BTreeSet::new();
    if term.literals.len() <= 3 {
        output.insert(ShortFreeGroupTerm::from(term.clone()));
        return output;
    }
    let mut first_literals = Vec::new();
    first_literals.push(term.literals[0]);
    first_literals.push(term.literals[1]);
    first_literals.push(Literal::new('v', *counter, false));

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