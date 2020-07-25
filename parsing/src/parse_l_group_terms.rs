use super::parse_free_group_term;
use terms::l_group_term::LGroupTerm;

pub fn parse(s: String) -> Result<LGroupTerm, String> {
    if is_atom(&s) { 
        let parsed_free_group_term = parse_free_group_term::parse(&s);
        match parsed_free_group_term {
            Err(e) => return Err(format!("parsing this term failed: {}. {}", &s, e)),
            Ok(term) => return Ok(LGroupTerm::Atom(term))
        };
    }
    todo!()
}

/// Returns whether the input contains `^`, `v`, or `-`
fn is_atom(s: &String) -> bool {
    for c in s.chars() {
        if c == '^' || c == 'v' || c == '-' { return false; }
    }
    return true;
}