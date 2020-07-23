
pub trait Term {
    fn inverse(&self) -> Self;
}

pub mod literal {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Literal {
        pub character: char,
        pub id: i8,
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
}

pub mod free_group_term {
    use super::*;

    pub type FreeGroupTerm = Vec<literal::Literal>;

    impl Term for FreeGroupTerm {
        fn inverse(&self) -> FreeGroupTerm {
            let mut result = Vec::new();
            for x in self {
                result.push(x.inverse())
            }
            return result;
        }
    }

    /// Reduces a free group term according to the rule aa^-1 = e.
    pub fn reduce(word: &mut FreeGroupTerm) {
        let mut index: usize = 0;
        let mut reduced_at_zero = false;
        while index < word.len() - 1 {
            if word[index] == word[index + 1].inverse() {
                word.remove(index);
                word.remove(index);
                reduced_at_zero = index <= 1;
            }
            index = match reduced_at_zero {
                true => 0,
                false => index + 1
            };
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use super::literal::*;
    use super::free_group_term::*;

    #[test]
    fn test_reduce() {
        let x = lit('x');
        let x_inv = lit('x').inverse();
        let y = lit('y');
        
        let mut result = vec![x, x_inv, y];
        reduce(&mut result);
        assert_eq!(vec![y], result);
    }
}