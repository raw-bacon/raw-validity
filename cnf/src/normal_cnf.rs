use terms::free_group_term::FreeGroupTerm;
use terms::l_group_term::LGroupTerm;
use terms::Reducable;
use std::collections::BTreeSet;

pub struct CNF {
    meetands: BTreeSet<BTreeSet<FreeGroupTerm>>
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
        CNF { meetands: meetands }
    }
}

impl CNF {
    /// Constructs a new CNF from a set of sets of free group terms
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// use terms::literal::lit;
    /// use terms::free_group_term::FreeGroupTerm;
    /// use std::collections::BTreeSet;
    /// use cnf::normal_cnf::CNF;
    /// let mut meetands = BTreeSet::new();
    /// let mut first_meetand = BTreeSet::new();
    /// let mut second_meetand = BTreeSet::new();
    /// let first_joinand = FreeGroupTerm::from(lit('x'));
    /// let second_joinand = FreeGroupTerm::from(lit('y'));
    /// let third_joinand = FreeGroupTerm::from(lit('z'));
    /// let fourth_joinand = FreeGroupTerm::from(lit('w'));
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
        CNF { meetands: meetands }
    }
}

fn to_cnf(term: LGroupTerm) -> LGroupTerm {
    if is_in_cnf(&term) {
        return term;
    }

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
            let mut rest_left = Vec::new();
            for x in xs.clone() {
                match &x {
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
                            new_meetands.insert(to_cnf(LGroupTerm::Prod(vec)));
                        }
                        return to_cnf(LGroupTerm::Meet(new_meetands));
                    },
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
                        // term = prod(rest_left, join(joinands), rest_right)
                        let mut new_joinands = BTreeSet::new();
                        for joinand in joinands {
                            let vec = vec![LGroupTerm::Prod(rest_left.clone()), joinand.clone(), LGroupTerm::Prod(rest_right.clone())];
                            new_joinands.insert(to_cnf(LGroupTerm::Prod(vec)));
                        }
                        return to_cnf(LGroupTerm::Join(new_joinands));
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

fn is_in_cnf(term: &LGroupTerm) -> bool {
    match term {
        LGroupTerm::Prod(_) => return false,
        LGroupTerm::Atom(_) => return true,
        LGroupTerm::Join(xs) => {
            for x in xs {
                match x {
                    LGroupTerm::Atom(_) => {},
                    _ => return false
                };
            }
        }
        LGroupTerm::Meet(xs) => {
            for x in xs {
                match x {
                    LGroupTerm::Meet(_) | LGroupTerm::Prod(_) => return false,
                    LGroupTerm::Atom(_) | LGroupTerm::Join(_) => {
                        if !is_in_cnf(x) { return false };
                    }
                }
            }
        }
    };
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use terms::literal::lit;

    #[test]
    fn test_to_string() {
        let mut meetands = BTreeSet::new();
        let mut first_meetand = BTreeSet::new();
        let mut second_meetand = BTreeSet::new();
        let first_joinand = FreeGroupTerm::from(lit('x'));
        let second_joinand = FreeGroupTerm::from(lit('y'));
        let third_joinand = FreeGroupTerm::from(lit('z'));
        let fourth_joinand = FreeGroupTerm::from(lit('w'));

        first_meetand.insert(first_joinand);
        first_meetand.insert(second_joinand);

        second_meetand.insert(third_joinand);
        second_meetand.insert(fourth_joinand);

        meetands.insert(first_meetand);
        meetands.insert(second_meetand);

        assert_eq!(String::from("(w v z) ^ (x v y)"), CNF::new(meetands).to_string());
    }
}