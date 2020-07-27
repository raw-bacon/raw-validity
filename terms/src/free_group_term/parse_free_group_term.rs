use super::super::free_group_term::FreeGroupTerm;
use super::super::literal::Literal;
use super::parsing_error::ParsingError;



pub (super) fn parse(s: &str) -> Result<FreeGroupTerm, ParsingError> {
    if s == "e" {
        return Ok(FreeGroupTerm::new(Vec::new()));
    }
    let mut literals = Vec::new();
    let mut iterator = s.chars();
    let first = iterator.next();
    let mut current_literal_string = String::new();
    let first_char: char;
    match first {
        None => return Err(ParsingError::EmptyFreeGroupTermError),
        Some(c) => first_char = c
    };
    current_literal_string.push(first_char);
    for c in iterator {
        if c.is_numeric() {
            current_literal_string.push(c);
        } else if c.is_alphabetic() {
            let parsed_literal = std::panic::catch_unwind(|| Literal::from(current_literal_string.as_str()));
            match parsed_literal {
                Ok(literal) => {
                    if literal != Literal::new('e', 0, false) {
                        literals.push(literal);
                    }
                },
                Err(_) => return Err(ParsingError::InvalidLiteralError(current_literal_string))
            }
            current_literal_string = String::new();
            current_literal_string.push(c);
        } 
    }
    if current_literal_string != String::new() {
        let parsed_literal = std::panic::catch_unwind(|| Literal::from(current_literal_string.as_str()));
        match parsed_literal {
            Ok(literal) => {
                if literal != Literal::new('e', 0, false) {
                    literals.push(literal);
                }
            },
            Err(_) => return Err(ParsingError::InvalidLiteralError(current_literal_string))
        }
    }
    return Ok(FreeGroupTerm::new(literals));
}

#[cfg(test)]
mod tests {
    use super::super::super::literal::Literal;
    use super::super::super::free_group_term::FreeGroupTerm;

    #[test]
    fn test_parse() {
        let string = String::from("X3 1*yz39 ");
        let x = Literal::new('x', 31, true);
        let y = Literal::new('y', 0, false);
        let z = Literal::new('z', 39, false);
        let term = FreeGroupTerm::new(vec![x, y, z]);
        assert_eq!(Ok(term), super::parse(&string));
    }
}