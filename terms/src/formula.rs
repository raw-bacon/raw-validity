use super::l_group_term::LGroupTerm;

#[derive(Debug)]
pub enum LGroupFormula {
    LGroupEquation(LGroupTerm, LGroupTerm),
    LGroupInequation(LGroupTerm, LGroupTerm)
}

impl From<&str> for LGroupFormula {
    fn from(s: &str) -> LGroupFormula {
        let string = s.to_string();
        let delimiter: &str;
        match s.contains('<') {
            true => delimiter = "<=",
            false => delimiter = "="
        };
        let strings: Vec<&str> = string.split(delimiter).collect();
        match delimiter {
            "<=" => LGroupFormula::LGroupInequation(
                LGroupTerm::from(strings[0].to_string().as_str()),
                LGroupTerm::from(strings[1].to_string().as_str())
            ),
            "="  => LGroupFormula::LGroupEquation(
                LGroupTerm::from(strings[0].to_string().as_str()),
                LGroupTerm::from(strings[1].to_string().as_str())
            ),
            _   => panic!("impossible")
        }
    }
}

impl ToString for LGroupFormula {
    fn to_string(&self) -> String {
        match self {
            LGroupFormula::LGroupEquation(lhs, rhs) => format!("{}={}", lhs.to_string(), rhs.to_string()),
            LGroupFormula::LGroupInequation(lhs, rhs) => format!("{}<={}", lhs.to_string(), rhs.to_string())
        }
    }
}