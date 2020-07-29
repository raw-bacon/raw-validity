use super::literal::*;
use super::free_group_term::FreeGroupTerm;
use super::Term;
use std::ops::Mul;

/// Short means length at most three
#[derive(Eq, PartialOrd, PartialEq, Ord, Debug, Clone, Copy)]
pub struct ShortFreeGroupTerm {
    pub left:  Option<Literal>,
    pub mid:   Option<Literal>,
    pub right: Option<Literal>
}

impl ShortFreeGroupTerm {
    pub fn new(
        left: Option<Literal>, 
        mid: Option<Literal>, 
        right: Option<Literal>) -> ShortFreeGroupTerm {
        ShortFreeGroupTerm {
            left: left,
            mid: mid,
            right: right
        }
    }
}

impl From<FreeGroupTerm> for ShortFreeGroupTerm {
    /// ignores everything after the third symbol
    fn from(term: FreeGroupTerm) -> ShortFreeGroupTerm {
        let literals = term.literals;
        match literals.len() {
            0 => ShortFreeGroupTerm::new(None, None, None),
            1 => ShortFreeGroupTerm::new(Some(literals[0]), None, None),
            2 => ShortFreeGroupTerm::new(Some(literals[0]), Some(literals[1]), None),
            _ => ShortFreeGroupTerm::new(Some(literals[0]), Some(literals[1]), Some(literals[2]))
        }
    }
}

impl From<&str> for ShortFreeGroupTerm {
    fn from(s: &str) -> ShortFreeGroupTerm {
        ShortFreeGroupTerm::from(FreeGroupTerm::from(s))
    }
}

impl From<char> for ShortFreeGroupTerm {
    fn from(c: char) -> ShortFreeGroupTerm {
        ShortFreeGroupTerm::from(FreeGroupTerm::from(c))
    }
}

impl From<Literal> for ShortFreeGroupTerm {
    fn from(x: Literal) -> ShortFreeGroupTerm {
        ShortFreeGroupTerm::from(FreeGroupTerm::from(x))
    }
}

impl ToString for ShortFreeGroupTerm {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for option in &[self.left, self.mid, self.right] {
            match option {
                Some(literal) => string.push_str(literal.to_string().as_str()),
                None => {}
            };
        }
        if string.len() == 0 {
            string.push('e');
        }
        return string;
    }
}

impl Term for ShortFreeGroupTerm {
    fn inverse(&self) -> ShortFreeGroupTerm {
        match (self.left, self.mid, self.right) {
            (Some(x), Some(y), Some(z)) => {
                ShortFreeGroupTerm {
                    left:  Some(x.inverse()),
                    mid:   Some(y.inverse()),
                    right: Some(z.inverse())
                }
            },
            (Some(x), Some(y), None) => {
                ShortFreeGroupTerm {
                    left:  Some(y.inverse()),
                    mid:   Some(x.inverse()),
                    right: None
                }
            },
            (Some(x), None, None) => {
                ShortFreeGroupTerm {
                    left:  Some(x.inverse()),
                    mid:   None,
                    right: None,
                }
            },
            (None, None, None) => {
                ShortFreeGroupTerm {
                    left:  None,
                    mid:   None,
                    right: None
                }
            }
            _ => panic!("invalid short term ...")
        }
    }
}

pub trait Len {
    fn len(&self) -> usize;
}

impl Len for ShortFreeGroupTerm {
    fn len(&self) -> usize {
        match (self.left, self.mid, self.right) {
            (None, None, None)          => 0,
            (Some(_), None, None)       => 1,
            (Some(_), Some(_), None)    => 2,
            (Some(_), Some(_), Some(_)) => 3,
            _                           => panic!("Invalid short free group term!")
        }
    }
}

pub struct LongFreeGroupTermError;

impl Mul for ShortFreeGroupTerm {
    type Output = ShortFreeGroupTerm;

    /// Warning: This does not check whether the product is indeed short.
    fn mul(self, other: ShortFreeGroupTerm) -> ShortFreeGroupTerm {
        ShortFreeGroupTerm::from(FreeGroupTerm::from(self) * FreeGroupTerm::from(other))
    }
}
