
pub trait Term {
    fn inverse(&self) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Literal {
    character: char,
    id: i8,
    is_inverted: bool
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

pub type FreeGroupTerm = Vec<Literal>;


/// Reduces a free group term according to the rule aa^-1 = e.
fn reduce(word: &mut FreeGroupTerm) {
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce() {
        let x = Literal {
            character: 'x',
            id: 0,
            is_inverted: false
        };
        let x_inv = Literal {
            character: 'x',
            id: 0,
            is_inverted: true
        };
        let y = Literal {
            character: 'y',
            id: 0,
            is_inverted: false
        };
        
        let mut result = vec![x, x_inv, y];
        reduce(&mut result);
        assert_eq!(vec![y], result);
    }
}