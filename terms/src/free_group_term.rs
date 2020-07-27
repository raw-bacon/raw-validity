use super::*;
use super::literal::*;
use super::short_free_group_term::ShortFreeGroupTerm;
use std::ops::{Mul, MulAssign};

mod parse_free_group_term;

/// An element of the group algebra.
/// 
/// # Examples
/// To use this, import
/// ```
/// use terms::free_group_term::*;
/// use terms::literal::*;
/// use terms::*;
/// ```
/// Constructing a free group term goes like this.
/// ```
/// # use terms::free_group_term::*;
/// # use terms::literal::*;
/// let term = Literal::from('x') * Literal::from('y');
/// ```
/// In this case, `term` encodes the element xy of a free group.
/// 
/// Terms constructed using `FreeGrouPTerm::new` automatically get reduced:
/// ```
/// # use terms::free_group_term::*;
/// # use terms::literal::*;
/// # use terms::*;
/// let term = Literal::from('x') * Literal::from('x').inverse();
/// assert_eq!(String::from("e"), term.to_string());
/// ```
/// 
/// Can also be constructed from strings:
/// `FreeGroupTerm::from(&str)` parses a free group term from a string.
/// 
/// This ignores all non-alphanumeric characters, such as `*`. Perhaps dangerously,
/// this also ignores symbols like `^`, and treats `v` as the name of a symbol.
/// The input is parsed by 
/// 
/// # Examples
/// Basic usage:
/// ```
/// use terms::literal::Literal;
/// use terms::free_group_term::FreeGroupTerm;
/// // this is equivalent to: 
/// // let string = "X31yz39";
/// let string = "X3 1*yz39 ";
/// let x = Literal::new('x', 31, true);
/// let y = Literal::new('y', 0, false);
/// let z = Literal::new('z', 39, false);
/// let term = FreeGroupTerm::new(vec![x, y, z]);
/// assert_eq!(term, FreeGroupTerm::from(string));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FreeGroupTerm {
    pub literals: Vec<literal::Literal>
}

impl FreeGroupTerm {
    pub fn new(literals: Vec<literal::Literal>) -> FreeGroupTerm {
        FreeGroupTerm { literals: literals }.reduced()
    }
}

pub static IDENTITY: FreeGroupTerm = FreeGroupTerm { literals: Vec::new() };

impl From<Literal> for FreeGroupTerm {
    fn from(x: Literal) -> FreeGroupTerm {
        FreeGroupTerm::new(vec![x])
    }
}

impl From<char> for FreeGroupTerm {
    fn from(c: char) -> FreeGroupTerm {
        FreeGroupTerm::from(Literal::from(c))
    }
}

impl From<&str> for FreeGroupTerm {
    fn from(s: &str) -> FreeGroupTerm {
        let result = parse_free_group_term::parse(s);
        match result {
            Ok(t) => t,
            Err(e) => panic!(e)
        }
    }
}

impl From<ShortFreeGroupTerm> for FreeGroupTerm {
    fn from(t: ShortFreeGroupTerm) -> FreeGroupTerm {
        match (t.left, t.mid, t.right) {
            (None, None, None) => FreeGroupTerm::new(Vec::new()),
            (Some(x), None, None) => FreeGroupTerm::from(x),
            (Some(x), Some(y), None) => x * y,
            (Some(x), Some(y), Some(z)) => x * y * z,
            _ => panic!("Invalid ShortFreeGroupTerm")
        }
    }
}

impl Term for FreeGroupTerm {
    fn inverse(&self) -> FreeGroupTerm {
        let mut result = Vec::new();
        for x in &self.literals {
            result.push(x.inverse())
        }
        result.reverse();
        FreeGroupTerm {
            literals: result
        }
    }
}

impl Mul for FreeGroupTerm {
    type Output = FreeGroupTerm;
    
    fn mul(self, other: FreeGroupTerm) -> FreeGroupTerm {
        if self.literals.len() == 0 { other } 
        else if other.literals.len() == 0 { self } 
        else {
            FreeGroupTerm::new([&self.literals[..], &other.literals[..]].concat())
        }
    }
}

impl Mul<Literal> for FreeGroupTerm {
    type Output = FreeGroupTerm;

    fn mul(self, other: Literal) -> FreeGroupTerm {
        self * FreeGroupTerm::from(other)
    }
}

impl MulAssign for FreeGroupTerm {
    fn mul_assign(&mut self, rhs: FreeGroupTerm) {
        for x in rhs.literals {
            self.literals.push(x);
        }
        *self = FreeGroupTerm::new(self.literals.clone());
    }
}

impl ToString for FreeGroupTerm {
    fn to_string(&self) -> String {
        let mut result = String::from("");
        for l in &self.literals {
            result.push_str(&l.to_string());
        }
        if result == "" {
            return String::from("e");
        }
        return result;
    }
}


impl Reducable for FreeGroupTerm {
    fn reduced(self) -> FreeGroupTerm {
        let mut index: usize = 0;
        let mut literals = self.literals.clone();
        
        enum ReducingState {
            ReducedInBeginning,
            ReducedElsewhere,
            DidNotReduce
        }

        while literals.len() > 0 && index < literals.len() - 1 {
            let mut reducing_state = ReducingState::DidNotReduce;
            if literals[index] == literals[index + 1].inverse() {
                literals.remove(index);
                literals.remove(index);
                match index {
                    0 | 1 => reducing_state = ReducingState::ReducedInBeginning,
                    _     => reducing_state = ReducingState::ReducedElsewhere
                };
            }
            index = match reducing_state {
                ReducingState::ReducedInBeginning => 0,
                ReducingState::ReducedElsewhere   => index - 1,
                ReducingState::DidNotReduce       => index + 1
            };
        }
        return FreeGroupTerm { literals: (literals).to_vec() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

     #[test]
    fn test_reduce() {
        let x = Literal::from('x');
        let x_inv = Literal::from('x').inverse();
        let y = Literal::from('y');
        
        let result = FreeGroupTerm { literals: vec![x, x_inv, y] };
        assert_eq!(FreeGroupTerm { literals: vec![y] }, result.reduced());

        let x = Literal::from('x');
        let y = Literal::from('y');
        let z = Literal::from('z');
        let result = FreeGroupTerm { literals: vec![x, y, z, z.inverse(), y.inverse(), x.inverse()]};
        assert_eq!(FreeGroupTerm::new(vec![]), result.reduced());
    }

    #[test]
    fn test_to_string() {
        let term = FreeGroupTerm::new(vec![Literal::from('x'), Literal::from('y'), Literal::from('z')]);
        assert_eq!("xyz", term.to_string());
    }

    #[test]
    fn test_inverse() {
        let x = Literal::from('x');
        let y = Literal::from('y');
        let z = Literal::from('z');
        let term = FreeGroupTerm { literals: vec![x,y,z] };
        let other_term = FreeGroupTerm { literals: vec![z.inverse(), y.inverse(), x.inverse()] };
        assert_eq!(other_term, term.inverse())
    }

    #[test]
    fn test_mul() {
        let x = FreeGroupTerm::new(vec![Literal::from('x')]);
        let x_inv = FreeGroupTerm::new(vec![Literal::from('x').inverse()]);
        assert_eq!(IDENTITY, x*x_inv);
    }

    #[test]
    fn test_mul_assign() {
        let mut term = FreeGroupTerm::from(Literal::from('x').inverse());
        term *= FreeGroupTerm::from(Literal::from('x'));
        assert_eq!(FreeGroupTerm::new(Vec::new()), term);
    }
}