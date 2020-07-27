use super::l_group_term::LGroupTerm;

pub struct LGroupInequation {
    pub lhs: LGroupTerm,
    pub rhs: LGroupTerm
}

pub struct LGroupEquation {
    pub lhs: LGroupTerm,
    pub rhs: LGroupTerm
}

impl From<&str> for LGroupInequation {
    fn from(s: &str) -> LGroupInequation {
        let string = s.to_string();
        let strings: Vec<&str> = string.split("<=").collect();
        LGroupInequation {
            lhs: LGroupTerm::from(strings[0].to_string().as_str()),
            rhs: LGroupTerm::from(strings[1].to_string().as_str())
        }
    }
}


impl From<&str> for LGroupEquation {
    fn from(s: &str) -> LGroupEquation {
        let string = s.to_string();
        let strings: Vec<&str> = string.split("=").collect();
        LGroupEquation {
            lhs: LGroupTerm::from(strings[0].to_string().as_str()),
            rhs: LGroupTerm::from(strings[1].to_string().as_str())
        }
    }
}