
#[derive(Debug, PartialEq, Eq)]
pub enum ParsingError {
    EmptyLiteralError,
    EmptyFreeGroupTermError,
    NoLowerCaseError(char),
    InvalidLiteralError(String),
    NonMatchingBracketsError(String),
    ParsingAtomError(String),
    ParsingInverseError(String, String),
    ParsingMeetError(String, String),
    ParsingJoinError(String, String),
    ParsingProductError(String, String, String)
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParsingError::EmptyLiteralError => 
                write!(f, "Empty literal could not be parsed."),
            ParsingError::EmptyFreeGroupTermError => 
                write!(f, "Empty free group term could not be parsed."),
            ParsingError::NoLowerCaseError(c) => 
                write!(f, "{}", format!("The character {} does not have a lower case.", c.to_string())),
            ParsingError::InvalidLiteralError(s) =>
                write!(f, "{}", format!("The literal {} could not be parsed.", s)),
            ParsingError::NonMatchingBracketsError(s) =>
                write!(f, "{}", format!("The brackets in {} did not match.", s)),
            ParsingError::ParsingAtomError(s) =>
                write!(f, "{}", format!("parsing this atom failed: {}", s)),
            ParsingError::ParsingInverseError(s, e_string) =>
                write!(f, "{}", format!("parsing this inverse failed: {}. That is, {}.", s, e_string)),
            ParsingError::ParsingMeetError(s, e_string) =>
                write!(f, "{}", format!("parsing this meet failed: {}. That is, {}.", s, e_string)),
            ParsingError::ParsingJoinError(s, e_string) =>
                write!(f, "{}", format!("parsing this join failed: {}. That is, {}.", s, e_string)),
            ParsingError::ParsingProductError(s, t, e_string) =>
                write!(f, "{}", format!("parsing the product {} at {} failed. That is, {}.", s, t, e_string))
        }
    }
}
