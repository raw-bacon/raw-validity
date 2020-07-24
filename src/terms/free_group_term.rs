use super::*;
use super::literal::*;
use std::ops::Mul;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FreeGroupTerm {
    pub literals: Vec<literal::Literal>
}

impl FreeGroupTerm {
    fn new(literals: Vec<literal::Literal>) -> FreeGroupTerm {
        FreeGroupTerm { literals: literals }.reduced()
    }
}

pub static IDENTITY: FreeGroupTerm = FreeGroupTerm { literals: Vec::new() };

impl From<Literal> for FreeGroupTerm {
    fn from(x: Literal) -> FreeGroupTerm {
        FreeGroupTerm::new(vec![x])
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
        if self.literals.len() == 0 {
            return other;
        } else if other.literals.len() == 0 {
            return self;
        } else {
            return FreeGroupTerm::new([&self.literals[..], &other.literals[..]].concat());
        }
    }
}

impl ToString for FreeGroupTerm {
    fn to_string(&self) -> String {
        let mut result = String::from("");
        for l in &self.literals {
            result.push_str(&l.to_string());
        }
        return result;
    }
}


impl Reducable for FreeGroupTerm {
    /// Reduces a free group term according to the rule aa^-1 = e.
    fn reduced(self) -> FreeGroupTerm {
        let mut index: usize = 0;
        let mut reduced_at_zero = false;
        let mut literals = self.literals.clone();
        while literals.len() > 0 && index < literals.len() - 1 {
            if literals[index] == literals[index + 1].inverse() {
                literals.remove(index);
                literals.remove(index);
                reduced_at_zero = index <= 1;
            }
            index = match reduced_at_zero {
                true => 0,
                false => index + 1
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
        let x = lit('x');
        let x_inv = lit('x').inverse();
        let y = lit('y');
        
        let result = FreeGroupTerm { literals: vec![x, x_inv, y] };
        assert_eq!(FreeGroupTerm { literals: vec![y] }, result.reduced());
    }

    #[test]
    fn test_to_string() {
        let term = FreeGroupTerm::new(vec![lit('x'), lit('y'), lit('z')]);
        assert_eq!("xyz", term.to_string());
    }

    #[test]
    fn test_inverse() {
        let x = lit('x');
        let y = lit('y');
        let z = lit('z');
        let term = FreeGroupTerm { literals: vec![x,y,z] };
        let other_term = FreeGroupTerm { literals: vec![z.inverse(), y.inverse(), x.inverse()] };
        assert_eq!(other_term, term.inverse())
    }

    #[test]
    fn test_mul() {
        let x = FreeGroupTerm::new(vec![lit('x')]);
        let x_inv = FreeGroupTerm::new(vec![lit('x').inverse()]);
        assert_eq!(IDENTITY, x*x_inv);
    }
}