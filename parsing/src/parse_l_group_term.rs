use super::parse_free_group_term;
use terms::l_group_term::LGroupTerm;
use terms::Term;
use std::collections::BTreeSet;

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
pub fn parse(s: &String) -> Result<LGroupTerm, String> {
    let mut s = s.clone();
    // remove whitespace and outer brackets
    s.retain(|c| !c.is_whitespace());
    while s.len() >= 2 && only_has_outermost_brackets(&s) {
        s = s[1..s.len() - 1].to_string();
    }

    if is_atom(&s) { 
        let parsed_free_group_term = parse_free_group_term::parse(&s);
        match parsed_free_group_term {
            Err(e) => return Err(format!("parsing this atom failed: {}. {}", &s, e)),
            Ok(term) => return Ok(LGroupTerm::Atom(term))
        };
    }
    
    if is_inverse(&s) {
        let result = parse(&s[1..s.len()].to_string());
        match result {
            Ok(term) => return Ok(term.inverse()),
            Err(e) => return Err(format!("Parsing this inverse failed: {}, {}", &s, e))
        };
    }

    if is_meet(&s) {
        let mut meetands = BTreeSet::new();

        for term_string in s.split("^") {
            let result = parse(&term_string.to_string());
            match result {
                Ok(term) => meetands.insert(term),
                Err(e) => return Err(format!("Parsing this meet failed: {}, {}", &s, e))
            };
        }
        return Ok(LGroupTerm::Meet(meetands));
    }

    if is_join(&s) {
        let mut joinands = BTreeSet::new();

        for term_string in s.split("v") {
            let result = parse(&term_string.to_string());
            match result {
                Ok(term) => joinands.insert(term),
                Err(e) => return Err(format!("Parsing this join failed: {}, {}", &s, e))
            };
        }
        return Ok(LGroupTerm::Join(joinands));
    }

    let mut factors = Vec::new();
    let mut current_factor = String::new();
    let mut depth = 0;
    for c in s.chars() {
        match &c {
            '(' => {
                if depth == 0 && current_factor.len() > 0 {
                    let result = parse(&current_factor);
                    match result {
                        Ok(term) => factors.push(term),
                        Err(e) => return Err(format!("Parsing the product {} at {} failed. {}", &s, &current_factor, e))
                    };
                    current_factor = String::new();
                }
                depth += 1;
                current_factor.push(c);
            },
            ')' => {
                match depth {
                    0 => return Err(String::from("Brackets do not match!")),
                    _ => depth -= 1
                };
                current_factor.push(c);
                if depth == 0 {
                    let result = parse(&current_factor);
                    match result {
                        Ok(term) => factors.push(term),
                        Err(e) => return Err(format!("Parsing the product {} at {} failed. {}", &s, &current_factor, e))
                    };
                    current_factor = String::new();
                }
            },
            '-' => {
                if depth == 0 && current_factor.len() > 0 {
                    let result = parse(&current_factor);
                    match result {
                        Ok(term) => factors.push(term),
                        Err(e) => return Err(format!("Parsing the product {} at {} failed. {}", &s, &current_factor, e))
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
                Err(e) => return Err(format!("Parsing the product {} at {} failed. {}", &s, &current_factor, e))
            };
    }
    return Ok(LGroupTerm::Prod(factors));
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


/// returns true if it is of the form `(...(x)...)`, where
/// `x` does not contain any brackets. 
fn only_has_outermost_brackets(s: &String) -> bool {
    #[derive(PartialEq, Eq)]
    enum Position { Left, Middle, Right };
    let mut pos = Position::Left;
    for c in s.chars() {
        match &pos {
            Position::Left => {
                if c != '(' { pos = Position::Middle; }
            },
            Position::Middle => {
                if c == '(' { return false; }
                else if c == ')' { pos = Position::Right; }
            },
            Position::Right => {
                if c != ')' { return false; }
            }
        };
    }
    return pos == Position::Right;
}