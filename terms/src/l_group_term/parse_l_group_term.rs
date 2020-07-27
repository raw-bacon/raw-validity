use super::super::free_group_term::FreeGroupTerm;
use super::super::l_group_term::LGroupTerm;
use super::super::Term;
use std::collections::BTreeSet;
use super::super::parsing_error::ParsingError;


/// Parses l-group terms. 
/// 
/// l-Group terms are represented in the input as strings,
/// where literals are of the form `[character][usize]` (without the brackets),
/// literals are multiplied by writing them next to each other (spaces are ignored),
/// meets are denoted by `^`, joins by `v`, and inverses by prefix `-`. e.g.,
/// ```
/// use parsing::parse_l_group_term;
/// use terms::l_group_term::LGroupTerm;
/// use terms::literal::lit;
/// use terms::Term;
/// let term = LGroupTerm::from(lit('x').inverse());
/// assert_eq!(Ok(term), parse_l_group_term::parse(&String::from("-x")))
/// ```
/// Multiplication of terms bigger than literals is also by writing them next to
/// each other:
/// ```
/// # use parsing::parse_l_group_term;
/// # use terms::l_group_term::LGroupTerm;
/// # use terms::literal::lit;
/// use std::collections::BTreeSet;
/// let mut meetands = BTreeSet::new();
/// meetands.insert(LGroupTerm::from(lit('y')));
/// meetands.insert(LGroupTerm::from(lit('z')));
/// let term = LGroupTerm::from(lit('x')) *  LGroupTerm::Meet(meetands);
/// assert_eq!(Ok(term), parse_l_group_term::parse(&String::from("x(y^z)")));
/// ```
pub fn parse(s: &str) -> Result<LGroupTerm, ParsingError> {
    let mut string = s.to_string();
    // remove whitespace and outer brackets
    string.retain(|c| !c.is_whitespace());
    loop {
        let result = has_outermost_brackets(&string);
        match result {
            Err(e) => return Err(e),
            Ok(can_be_stripped) => {
                if string.len() >= 2 && can_be_stripped {
                    string = string[1..string.len() - 1].to_string();
                }
                else {
                    break;
                }
            }
        }
    }

    if is_atom(&string) { 
        let parsed_free_group_term = std::panic::catch_unwind(|| FreeGroupTerm::from(string.as_str()));
        match parsed_free_group_term {
            Err(e) => return Err(ParsingError::ParsingAtomError(string)),
            Ok(term) => return Ok(LGroupTerm::Atom(term))
        };
    }
    
    if is_inverse(&string) {
        let result = parse(&string[1..string.len()].to_string());
        match result {
            Ok(term) => return Ok(term.inverse()),
            Err(e) => return Err(ParsingError::ParsingInverseError(string, e.to_string()))
        };
    }

    if is_meet(&string) {
        let mut meetands = BTreeSet::new();
        let result = split_at_outermost_meet(&string);
        match result {
            Err(e) => return Err(e),
            Ok(strings) => {
                for term_string in strings {
                    let result = parse(&term_string);
                    match result {
                        Ok(term) => meetands.insert(term),
                        Err(e) => return Err(ParsingError::ParsingMeetError(string, e.to_string()))
                    };
                }
            }
        };
        return Ok(LGroupTerm::Meet(meetands));
    }

    if is_join(&string) {
        let mut joinands = BTreeSet::new();
        let result = split_at_outermost_join(&string);
        match result {
            Err(e) => return Err(e),
            Ok(strings) => {
                for term_string in strings {
                    let result = parse(&term_string);
                    match result {
                        Ok(term) => joinands.insert(term),
                        Err(e) => return Err(ParsingError::ParsingJoinError(string, e.to_string()))
                    };
                }
            }
        };
        return Ok(LGroupTerm::Join(joinands));
    }

    let mut factors = Vec::new();
    let mut current_factor = String::new();
    let mut depth = 0;
    for c in string.chars() {
        match &c {
            '(' => {
                if depth == 0 && current_factor.len() > 0 {
                    let result = parse(&current_factor);
                    match result {
                        Ok(term) => factors.push(term),
                        Err(e) => return Err(ParsingError::ParsingProductError(string, current_factor, e.to_string()))
                    };
                    current_factor = String::new();
                }
                depth += 1;
                current_factor.push(c);
            },
            ')' => {
                match depth {
                    0 => return Err(ParsingError::NonMatchingBracketsError(string.to_string())),
                    _ => depth -= 1
                };
                current_factor.push(c);
                if depth == 0 {
                    let result = parse(&current_factor);
                    match result {
                        Ok(term) => factors.push(term),
                        Err(e) => return Err(ParsingError::ParsingProductError(string, current_factor, e.to_string()))
                    };
                    current_factor = String::new();
                }
            },
            '-' => {
                if depth == 0 && current_factor.len() > 0 {
                    let result = parse(&current_factor);
                    match result {
                        Ok(term) => factors.push(term),
                        Err(e) => return Err(ParsingError::ParsingProductError(string, current_factor, e.to_string()))
                    };
                    current_factor = String::new();
                }
                current_factor.push(c);
            }
            _ => current_factor.push(c)
        };
    }
    if current_factor.len() > 0 {
        let result = parse(&current_factor);
            match result {
                Ok(term) => factors.push(term),
                Err(e) => return Err(ParsingError::ParsingProductError(string, current_factor, e.to_string()))
            };
    }
    return Ok(LGroupTerm::Prod(factors));
}

fn split_at_outermost_join(s: &String) -> Result<Vec<String>, ParsingError> {
    let mut depth = 0;
    let mut strings = Vec::new();
    let mut current_string = String::new();
    for c in s.chars() {
        match c {
            '(' => {
                depth += 1;
                current_string.push(c);
            },
            ')' => {
                match depth {
                    0 => return Err(ParsingError::NonMatchingBracketsError(*s)),
                    _ => depth -= 1
                };
                current_string.push(c);
            },
            'v' => {
                if depth == 0 {
                    strings.push(current_string.clone());
                    current_string = String::new();
                }
                else {
                    current_string.push(c);
                }
            },
            _ => current_string.push(c)
        };
    }
    strings.push(current_string);
    return Ok(strings);
}

fn split_at_outermost_meet(s: &String) -> Result<Vec<String>, ParsingError> {
    let mut depth = 0;
    let mut strings = Vec::new();
    let mut current_string = String::new();
    for c in s.chars() {
        match c {
            '(' => depth += 1,
            ')' => {
                match depth {
                    0 => return Err(ParsingError::NonMatchingBracketsError(*s)),
                    _ => depth -= 1
                };
            },
            '^' => {
                if depth == 0 {
                    strings.push(current_string.clone());
                    current_string = String::new();
                }
                else {
                    current_string.push(c);
                }
            },
            _ => current_string.push(c)
        };
    }
    strings.push(current_string);
    return Ok(strings);
}

/// Returns whether the input contains `^`, `v`, or `-`
fn is_atom(s: &String) -> bool {
    for c in s.chars() {
        if c == '^' || c == 'v' || c == '-' { return false; }
    }
    return true;
}

/// Warning: only save to call if it is known not to be an atom
fn is_inverse(s: &String) -> bool {
    let mut chars = s.chars();
    let c = chars.next();
    if c != Some('-') { return false };
    let mut depth = 0;
    for c in chars {
        match &c {
            '(' => depth += 1,
            ')' => depth -= 1,
            'v' | '^' | '-'  => { if depth == 0 { return false; }},
            _ => {}
        };
    }
    return true;
}

/// Warning: only save to call if it is known not to be an atom or an inverse
fn is_meet(s: &String) -> bool {
    let mut depth = 0;
    for c in s.chars() {
        match &c {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        };
        if depth == 0 && c == '^' { return true };
    }
    return false;
}

/// Warning: only save to call if it is known not to be an atom or an inverse
fn is_join(s: &String) -> bool {
    let mut depth = 0;
    for c in s.chars() {
        match &c {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        };
        if depth == 0 && c == 'v' { return true };
    }
    return false;
}


/// returns true if it its outermost brackets are totally left and right,
/// and are redundant.
fn has_outermost_brackets(s: &String) -> Result<bool, ParsingError> {
    let mut depth = 0;
    let s = s[0 .. s.len() - 1].to_string();
    for c in s.chars() {
        match c {
            '(' => depth += 1,
            ')' => { 
                match depth {
                    0 => return Err(ParsingError::NonMatchingBracketsError(s)),
                    _ => depth -= 1
                }
            },
            _ => {}
        };
        if depth == 0 { return Ok(false); }
    }
    return Ok(true);
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn test_does_not_crash() {
        let string = String::from("(x v (z v (x ^ y)))");
        assert_eq!(string, super::parse(&string).expect("crashed ...").to_string());
    }
}