use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Literal {
    pub character: char,
    pub id: usize,
    pub is_inverted: bool
}

/// shorthand constructor for a literal
/// 
/// sets `id` to zero and `is_inverted` to false.
/// 
/// # Example
/// ```
/// use rsvalidity::terms::literal::*;
/// let literal1 = lit('x');
/// let literal2 = Literal {
///     character: 'x',
///     id: 0,
///     is_inverted: false
/// };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_to_string() {
        assert_eq!("x", lit('x').to_string());
        assert_eq!("X", lit ('x').inverse().to_string());
        let l = Literal {
            character: 'x',
            id: 31,
            is_inverted: true
        };
        assert_eq!("X31", l.to_string());
    }
}