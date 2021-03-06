use l_group_formulas::free_group_term::len;
use l_group_formulas::free_group_term::FreeGroupTerm;
use l_group_formulas::l_group_term::LGroupTerm;
use l_group_formulas::literal::Literal;
use l_group_formulas::Reducable;
use std::collections::BTreeSet;
use l_group_formulas::Term;
use rand::Rng;

/// Represents a meet of joins of free group terms.
/// 
/// Can be constructed from LGroupTerms as follows.
/// ```
/// use l_group_formulas::l_group_term::LGroupTerm;
/// use l_group_cnf::normal_cnf::CNF;
/// let l_group_term = LGroupTerm::from("((x^y)v(x^z))(-(x^(yvz)))");
/// println!("The CNF of {} is {}", l_group_term.to_string(), CNF::from(l_group_term).to_string());
/// ```
pub struct CNF {
    pub meetands: BTreeSet<BTreeSet<FreeGroupTerm>>
}

impl ToString for CNF {
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

impl From<LGroupTerm> for CNF {
    fn from(term: LGroupTerm) -> CNF {
        let cnf_term = to_cnf(term);
        let mut meetands = BTreeSet::new();
        match cnf_term {
            LGroupTerm::Meet(xs) => {
                for x in xs {
                    match x {
                        LGroupTerm::Meet(_) | LGroupTerm::Prod(_) => panic!("CNF failed"),
                        LGroupTerm::Join(ys) => { 
                            // should be ys, but freegroupterms instead of lgroupterms
                            let mut new_meetand = BTreeSet::new();
                            for y in ys {
                                match y {
                                    LGroupTerm::Atom(t) => new_meetand.insert(t),
                                    _                   => panic!("CNF failed")
                                };
                            }
                            meetands.insert(new_meetand); 
                        },
                        LGroupTerm::Atom(y) => {
                            let mut single_term = BTreeSet::new();
                            single_term.insert(y);
                            meetands.insert(single_term);
                        }
                    };
                }
            },
            LGroupTerm::Join(xs) => {
                let mut single_meetand = BTreeSet::new();
                for x in xs {
                    match x {
                        LGroupTerm::Atom(t) => single_meetand.insert(t),
                        _                   => panic!("CNF failed")
                    };
                };
                meetands.insert(single_meetand);
            },
            LGroupTerm::Atom(x) => {
                let mut single_meetand = BTreeSet::new();
                single_meetand.insert(x);
                meetands.insert(single_meetand);
            }
            LGroupTerm::Prod(_) => panic!("CNF failed")
        };
        CNF { meetands }
    }
}

impl CNF {
    /// Constructs a new CNF from a set of sets of free group terms
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// use l_group_formulas::literal::Literal;
    /// use l_group_formulas::free_group_term::FreeGroupTerm;
    /// use std::collections::BTreeSet;
    /// use l_group_cnf::normal_cnf::CNF;
    /// let mut meetands = BTreeSet::new();
    /// let mut first_meetand = BTreeSet::new();
    /// let mut second_meetand = BTreeSet::new();
    /// let first_joinand = FreeGroupTerm::from(Literal::from('x'));
    /// let second_joinand = FreeGroupTerm::from(Literal::from('y'));
    /// let third_joinand = FreeGroupTerm::from(Literal::from('z'));
    /// let fourth_joinand = FreeGroupTerm::from(Literal::from('w'));
    ///
    /// first_meetand.insert(first_joinand);
    /// first_meetand.insert(second_joinand);
    ///
    /// second_meetand.insert(third_joinand);
    /// second_meetand.insert(fourth_joinand);
    ///
    /// meetands.insert(first_meetand);
    /// meetands.insert(second_meetand);
    ///
    /// assert_eq!(String::from("(w v z) ^ (x v y)"), CNF::new(meetands).to_string());
    /// ```
    pub fn new(meetands: BTreeSet<BTreeSet<FreeGroupTerm>>) -> CNF {
        CNF { meetands }
    }
}

fn to_cnf(term: LGroupTerm) -> LGroupTerm {
    // if is_in_cnf(&term) {
    //     return term;
    // }

    match term {
        LGroupTerm::Atom(_) => { return term }
        LGroupTerm::Meet(xs) => { 
            let mut meetands = BTreeSet::new();
            for x in xs {
                meetands.insert(to_cnf(x));
            }
            return LGroupTerm::Meet(meetands).reduced();
        },
        LGroupTerm::Join(xs) => {
            let mut rest = BTreeSet::new();
            for x in xs.clone() {
                match &x {
                    LGroupTerm::Meet(meetands) => {
                        for y in xs {
                            if x != y { rest.insert(y); }
                        }
                        // term = join(meet(meetands), rest)
                        let mut new_meetands = BTreeSet::new();
                        for meetand in meetands {
                            let mut set = rest.clone();
                            set.insert(meetand.clone());
                            new_meetands.insert(to_cnf(LGroupTerm::Join(set)).reduced());
                        }
                        return LGroupTerm::Meet(new_meetands).reduced();
                    },
                    _ => {
                        rest.insert(x);
                    }
                };
            }
            // term doesn't contain any meets
            let mut new_joinands = BTreeSet::new();
            for x in xs { new_joinands.insert(to_cnf(x)); }
            return LGroupTerm::Join(new_joinands).reduced();
        },
        LGroupTerm::Prod(xs) => {
            let mut rest_left : Vec<LGroupTerm> = Vec::new();
            for x in xs.clone() {
                match &x {
                    // TODO: prioritize this over meets.
                    // this is not very urgent because the main executable will never
                    // pass meets anyway.
                    LGroupTerm::Join(joinands) => {
                        let mut rest_right = Vec::new();
                        enum Position { Left, Right }
                        let mut pos = Position::Left;
                        for y in xs {
                            match pos {
                                Position::Left => {
                                    if x == y { pos = Position::Right; }
                                }
                                Position::Right => {
                                    rest_right.push(y);
                                }
                            }
                        }

                        // distribute if rest_left and rest_right are both length 1 atoms
                        if rest_left.len() == 1 && rest_right.len() == 1 {
                            let left = rest_left.first().unwrap();
                            let right = rest_right.first().unwrap();
                            match (&left, &right) {
                                (LGroupTerm::Atom(literals_left), LGroupTerm::Atom(literals_right)) => {
                                    if len(literals_left) == 1 && len(literals_right) == 1 {
                                        let mut new_joinands = BTreeSet::new();
                                        for joinand in joinands {
                                            let vec = vec![LGroupTerm::Prod(rest_left.clone()),
                                                           joinand.clone(),
                                                           LGroupTerm::Prod(rest_right.clone())];
                                            new_joinands.insert(to_cnf(LGroupTerm::Prod(vec).reduced()));
                                        }
                                        return to_cnf(LGroupTerm::Join(new_joinands));
                                   }
                                },
                                _ => {}
                            }
                        }
                        // The term
                        // rest_left * Join(joinands) * rest_right
                        // is transformed to
                        // Join(rest_left * x, X * joinand1 * y, X * joinand2 * y, ..., X * joinandn * y, Y * rest_right).
                        // Here, x and y are new variables not appearing in the whole term.
                        let mut rng = rand::thread_rng();
                        let x = Literal::new('v', rng.gen::<usize>(), false);
                        let y = Literal::new('v', rng.gen::<usize>(), false);
                        let mut new_joinands = BTreeSet::new();
                        rest_left.push(LGroupTerm::Atom(FreeGroupTerm::from(x)));
                        new_joinands.insert(to_cnf(LGroupTerm::Prod(rest_left).reduced()));
                        let mut new_rest_right = vec![LGroupTerm::Atom(FreeGroupTerm::from(y.inverse()))];
                        for t in rest_right {
                            new_rest_right.push(t);
                        }
                        new_joinands.insert(to_cnf(LGroupTerm::Prod(new_rest_right).reduced()));
                        for joinand in joinands {
                            let new_factors = vec![LGroupTerm::from(x.inverse()),
                                                   joinand.clone(),
                                                   LGroupTerm::from(y)];
                            new_joinands.insert(to_cnf(LGroupTerm::Prod(new_factors).reduced()));
                        }
                        return to_cnf(LGroupTerm::Join(new_joinands).reduced())
                    },
                    LGroupTerm::Meet(meetands) => {
                        let mut rest_right = Vec::new();
                        enum Position { Left, Right }
                        let mut pos = Position::Left;
                        for y in xs {
                            match pos {
                                Position::Left => {
                                    if x == y { pos = Position::Right; }
                                }
                                Position::Right => {
                                    rest_right.push(y);
                                }
                            }
                        }
                        // term = prod(rest_left, meet(meetands), rest_right)
                        let mut new_meetands = BTreeSet::new();
                        for meetand in meetands {
                            let vec = vec![LGroupTerm::Prod(rest_left.clone()), meetand.clone(), LGroupTerm::Prod(rest_right.clone())];
                            new_meetands.insert(to_cnf(LGroupTerm::Prod(vec).reduced()));
                        }
                        return to_cnf(LGroupTerm::Meet(new_meetands));
                    },
                    _ => {
                        rest_left.push(x);
                    }
                };
            }
            // term doesn't contain any meets
            let mut new_joinands = BTreeSet::new();
            for x in xs { new_joinands.insert(to_cnf(x)); }
            return LGroupTerm::Join(new_joinands).reduced();
        }
    };
}

// fn is_in_cnf(term: &LGroupTerm) -> bool {
//     match term {
//         LGroupTerm::Prod(_) => return false,
//         LGroupTerm::Atom(_) => return true,
//         LGroupTerm::Join(xs) => {
//             for x in xs {
//                 match x {
//                     LGroupTerm::Atom(_) => {},
//                     _ => return false
//                 };
//             }
//         }
//         LGroupTerm::Meet(xs) => {
//             for x in xs {
//                 match x {
//                     LGroupTerm::Meet(_) | LGroupTerm::Prod(_) => return false,
//                     LGroupTerm::Atom(_) | LGroupTerm::Join(_) => {
//                         if !is_in_cnf(x) { return false };
//                     }
//                 }
//             }
//         }
//     };
//     return true;
// }

#[cfg(test)]
mod tests {
    use super::*;
    use l_group_formulas::literal::Literal;

    #[test]
    fn test_to_string() {
        let mut meetands = BTreeSet::new();
        let mut first_meetand = BTreeSet::new();
        let mut second_meetand = BTreeSet::new();
        let first_joinand = FreeGroupTerm::from(Literal::from('x'));
        let second_joinand = FreeGroupTerm::from(Literal::from('y'));
        let third_joinand = FreeGroupTerm::from(Literal::from('z'));
        let fourth_joinand = FreeGroupTerm::from(Literal::from('w'));

        first_meetand.insert(first_joinand);
        first_meetand.insert(second_joinand);

        second_meetand.insert(third_joinand);
        second_meetand.insert(fourth_joinand);

        meetands.insert(first_meetand);
        meetands.insert(second_meetand);

        assert_eq!(String::from("(w v z) ^ (x v y)"), CNF::new(meetands).to_string());
    }
}
