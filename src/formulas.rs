
pub trait Term {
    fn inverse(&self) -> Self;
}

pub trait Reducable {
    fn reduce(&mut self);
}

pub mod literal {
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
    /// use rsvalidity::formulas::literal::*;
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
}

pub mod free_group_term {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct FreeGroupTerm {
        pub literals: Vec<literal::Literal>
    }

    impl Term for FreeGroupTerm {
        fn inverse(&self) -> FreeGroupTerm {
            let mut result = Vec::new();
            for x in &self.literals {
                result.push(x.inverse())
            }
            FreeGroupTerm {
                literals: result
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
        fn reduce(&mut self) {
            let mut index: usize = 0;
            let mut reduced_at_zero = false;
            let literals = &mut self.literals;
            while index < literals.len() - 1 {
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
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use super::literal::*;
    use super::free_group_term::*;

    #[test]
    fn test_reduce_free_group_term() {
        let x = lit('x');
        let x_inv = lit('x').inverse();
        let y = lit('y');
        
        let mut result = FreeGroupTerm { literals: vec![x, x_inv, y] };
        result.reduce();
        assert_eq!(FreeGroupTerm { literals: vec![y] }, result);
    }

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

    #[test]
    fn test_free_group_term_to_string() {
        let term = FreeGroupTerm {
            literals: vec![lit('x'), lit('y'), lit('z')]
        };
        assert_eq!("xyz", term.to_string());
    }
}