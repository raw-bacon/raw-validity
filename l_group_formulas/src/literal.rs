use super::Term;
use super::parsing_error::ParsingError;
use super::free_group_term::FreeGroupTerm;
use std::ops::Mul;

/// The smallest term (apart from the identity).
/// 
/// # Examples
/// Literals can be parsed from a string.
/// Basic usage:
/// Symbols with ids are okay.
/// ```
/// use l_group_formulas::literal::Literal;
/// let literal = Literal::new('x', 31, true);
/// assert_eq!(literal, Literal::from("X31"));
/// ```
/// So are symbols without.
/// ```
/// # use l_group_formulas::literal::Literal;
/// let literal = Literal::new('y', 0, false);
/// assert_eq!(literal, Literal::from("y"));
/// ```
/// 
/// Alternatively, literals can be constructed from characters.
/// This sets `id` to zero and `is_inverted` to false.
/// ```
/// use l_group_formulas::literal::*;
/// let literal1 = Literal::from('x');
/// let literal2 = Literal::new('x', 0, false);
/// assert_eq!(literal1, literal2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Literal {
    pub character: char,
    pub id: usize,
    pub is_inverted: bool
}

impl Literal {
    pub fn new(character: char, id: usize, is_inverted: bool) -> Literal {
        Literal {
            character,
            id,
            is_inverted
        }
    }
}

impl From<char> for Literal {
    fn from(c: char) -> Literal {
        Literal {
            character: c,
            id: 0,
            is_inverted: false
        }
    }
}

impl Term for Literal {
    fn inverse(&self) -> Literal {
        Literal {
            character: self.character,
            id: self.id,
            is_inverted: !self.is_inverted
        }
    }
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        let mut result = String::new();
        match self.is_inverted {
            false => result.push(self.character),
            true => {
                let upper = self.character.to_uppercase();
                for c in upper { result.push(c); }
            }
        }
        if self.id != 0 {
            result.push_str(&self.id.to_string());
        }
        return result;
    }
}

impl Mul for Literal {
    type Output = FreeGroupTerm;
    fn mul(self, other: Literal) -> FreeGroupTerm {
        FreeGroupTerm::new(vec![self, other])
    }
}

impl From<&str> for Literal {
    fn from(s: &str) -> Literal {
        let result = parse(s);
        match result {
            Ok(literal) => literal,
            Err(e) => panic!(e)
        }
    }
}


fn parse(s: &str) -> Result<Literal, ParsingError> {
    let l = s.len();
    if l == 0 {
        return Err(ParsingError::EmptyLiteralError);
    } else {
        let is_inverted: bool;
        let mut lower_case: std::char::ToLowercase;
        match s.chars().next() {
            None => return Err(ParsingError::EmptyLiteralError),
            Some(c) => { 
                is_inverted = c.is_uppercase(); 
                lower_case = c.to_lowercase(); 
            }
        };
        let character: char;
        match lower_case.next() {
            None => return Err(ParsingError::EmptyLiteralError),
            Some(c) => character = c
        };
        if l == 1 { return Ok(Literal::new(character, 0, is_inverted)); }

        let result = without_first(s).parse::<usize>();
        match result {
            Ok(id) => Ok(Literal::new(character, id, is_inverted)),
            Err(e) => Err(ParsingError::InvalidLiteralError(e.to_string()))
        }
    }
}

fn without_first(string: &str) -> String {
    let mut result = String::new();
    let mut iterator = string.chars();
    iterator.next();
    for c in iterator { result.push(c); }

    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_to_string() {
        assert_eq!("x", Literal::from('x').to_string());
        assert_eq!("X", Literal::from('x').inverse().to_string());
        let l = Literal::new('x', 31, true);
        assert_eq!("X31", l.to_string());
    }
}
