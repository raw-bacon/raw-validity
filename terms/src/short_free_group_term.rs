use super::literal::Literal;
use super::free_group_term::FreeGroupTerm;

/// Short means length at most three
#[derive(Eq, PartialOrd, PartialEq, Ord)]
pub struct ShortFreeGroupTerm {
    left:  Option<Literal>,
    mid:   Option<Literal>,
    right: Option<Literal>
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