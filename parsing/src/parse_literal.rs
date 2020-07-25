use terms::literal::*;

/// parses a literal from a string. 
/// 
/// # Examples
/// Basic usage:
/// Symbols with ids are okay.
/// ```
/// use parsing::parse_literal;
/// use terms::literal::Literal;
/// let string = String::from("X31");
/// let literal = Literal::new('x', 31, true);
/// assert_eq!(Ok(literal), parse_literal::parse(string));
/// ```
/// So are symbols without.
/// ```
/// # use parsing::parse_literal;
/// # use terms::literal::Literal;
/// let string = String::from("y");
/// let literal = Literal::new('y', 0, false);
/// assert_eq!(Ok(literal), parse_literal::parse(string));
/// ```
pub fn parse(s: String) -> Result<Literal, String> {
    let l = s.len();
    if l == 0 {
        return Err(String::from("empty literal does not exist"));
    } else {
        let is_inverted: bool;
        let mut lower_case: std::char::ToLowercase;
        match s.chars().next() {
            None => return Err(String::from("absurd empty literal does not exist")),
            Some(c) => { is_inverted = c.is_uppercase(); lower_case = c.to_lowercase(); }
        };
        let character: char;
        match lower_case.next() {
            None => return Err(String::from("this character does not have a reasonable lower case")),
            Some(c) => character = c
        };
        if l == 1 { return Ok(Literal::new(character, 0, is_inverted)); }

        let id = without_first(s).parse::<usize>().expect(
            "could not parse ID. a literal should be a character, possibly followed
            by an id (a non-negative integer)"
        );
        return Ok(Literal::new(character, id, is_inverted))
    }
}

fn without_first(string: String) -> String {
    let mut result = String::new();
    let mut iterator = string.chars();
    iterator.next();
    for c in iterator { result.push(c); }

    // remove this
    println!("{}", result);
    
    return result;
}
