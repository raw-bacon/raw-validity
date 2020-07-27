use super::Term;
use super::parsing_error::ParsingError;

/// The smallest term (apart from the identity).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Literal {
    pub character: char,
    pub id: usize,
    pub is_inverted: bool
}

impl Literal {
    pub fn new(character: char, id: usize, is_inverted: bool) -> Literal {
        Literal {
            character: character,
            id: id,
            is_inverted: is_inverted
        }
    }
}

/// shorthand constructor for a literal
/// 
/// sets `id` to zero and `is_inverted` to false.
/// 
/// # Examples
/// Basic usage:
/// ```
/// use terms::literal::*;
/// let literal1 = lit('x');
/// let literal2 = Literal::new('x', 0, false);
/// assert_eq!(literal1, literal2);
/// ```
pub fn lit(c: char) -> Literal {
    Literal {
        character: c,
        id: 0,
        is_inverted: false
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
        let mut result = String::from("");
        if !self.is_inverted {
            result.push(self.character);
        } else {
            let upper = self.character.to_uppercase();
            for c in upper {
                result.push(c);
            }
        }
        if self.id != 0 {
            result.push_str(&self.id.to_string());
        }
        return result;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_to_string() {
        assert_eq!("x", lit('x').to_string());
        assert_eq!("X", lit ('x').inverse().to_string());
        let l = Literal::new('x', 31, true);
        assert_eq!("X31", l.to_string());
    }
}


/// parses a literal from a string. 
/// 
/// # Examples
/// Basic usage:
/// Symbols with ids are okay.
/// ```
/// use parsing::parse_literal;
/// use terms::literal::Literal;
/// let string = String::from("X31");
/// let literal = Literal::new('x', 31, true);
/// assert_eq!(Ok(literal), parse_literal::parse(string));
/// ```
/// So are symbols without.
/// ```
/// # use parsing::parse_literal;
/// # use terms::literal::Literal;
/// let string = String::from("y");
/// let literal = Literal::new('y', 0, false);
/// assert_eq!(Ok(literal), parse_literal::parse(string));
/// ```
fn parse(s: &str) -> Result<Literal, ParsingError> {
    let l = s.len();
    if l == 0 {
        return Err(ParsingError::EmptyLiteralError);
    } else {
        let is_inverted: bool;
        let mut lower_case: std::char::ToLowercase;
        match s.chars().next() {
            None => return Err(ParsingError::EmptyLiteralError),
            Some(c) => { is_inverted = c.is_uppercase(); lower_case = c.to_lowercase(); }
        };
        let character: char;
        match lower_case.next() {
            None => return Err(ParsingError::EmptyLiteralError),
            Some(c) => character = c
        };
        if l == 1 { return Ok(Literal::new(character, 0, is_inverted)); }

        let id = without_first(s).parse::<usize>().expect(
            "could not parse ID. a literal should be a character, possibly followed
            by an id (a non-negative integer)"
        );
        return Ok(Literal::new(character, id, is_inverted))
    }
}

fn without_first(string: &str) -> String {
    let mut result = String::new();
    let mut iterator = string.chars();
    iterator.next();
    for c in iterator { result.push(c); }

    return result;
}
