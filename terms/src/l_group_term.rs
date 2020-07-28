use super::free_group_term::*;
use super::*;
use super::literal::*;
use std::collections::BTreeSet;
use std::ops::Mul;
use super::l_group_term_reducing::*;

mod parse_l_group_term;

/// An element the term algebra of l-groups.
/// 
/// # Examples
/// To use this, we want to do some imports:
/// ```
/// use terms::free_group_term::*;
/// use terms::literal::*;
/// use terms::l_group_term::*;
/// ```
/// An `LGroupTerm` be either an `Atom`, i.e.,
/// ```
/// # use terms::free_group_term::*;
/// # use terms::literal::*;
/// # use terms::l_group_term::*;
/// let x = FreeGroupTerm::from('x');
/// let lGroupTerm = LGroupTerm::Atom(x);
/// ```
/// a `Meet`, a `Join`, or a `Product`. `Meet`s and `Join`s take `BTreeSet`s as arguments:
/// ```
/// # use terms::free_group_term::*;
/// # use terms::literal::*;
/// # use terms::l_group_term::*;
/// use std::collections::BTreeSet;
/// let mut meetands = BTreeSet::new();
/// meetands.insert(LGroupTerm::from('x'));
/// meetands.insert(LGroupTerm::from('y'));
/// let meet = LGroupTerm::Meet(meetands);
/// ```
/// whereas `Product`s take `Vec<LGroupTerm>`s:
/// ```
/// # use terms::free_group_term::*;
/// # use terms::l_group_term::*;
/// # use terms::literal::*;
/// let factors = vec![LGroupTerm::from('x'), LGroupTerm::from('y')];
/// let product = LGroupTerm::Prod(factors);
/// ```
/// This models associativity of meets, joins, and products, and takes into
/// account the non-commutativity of the products, but also the commutativity
/// of meets and joins.
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum LGroupTerm {
    Atom(FreeGroupTerm),
    Meet(BTreeSet<LGroupTerm>),
    Join(BTreeSet<LGroupTerm>),
    Prod(Vec<LGroupTerm>)
}

pub const IDENTITY: LGroupTerm = LGroupTerm::Atom(FreeGroupTerm { literals: vec![] } );

impl From<FreeGroupTerm> for LGroupTerm {
    fn from(x: FreeGroupTerm) -> LGroupTerm {
        LGroupTerm::Atom(x).reduced()
    }
}

impl From<Literal> for LGroupTerm {
    fn from(x: Literal) -> LGroupTerm {
        LGroupTerm::Atom(FreeGroupTerm::from(x))
    }
}

impl From<&str> for LGroupTerm {
    /// Parses l-group terms. 
    /// 
    /// l-Group terms are represented in the input as strings,
    /// where literals are of the form `[character][usize]` (without the brackets),
    /// literals are multiplied by writing them next to each other (spaces are ignored),
    /// meets are denoted by `^`, joins by `v`, and inverses by prefix `-`. e.g.,
    /// ```
    /// use terms::l_group_term::LGroupTerm;
    /// use terms::Term;
    /// let term = LGroupTerm::from('x').inverse();
    /// assert_eq!(term, LGroupTerm::from("-x"));
    /// ```
    /// Multiplication of terms bigger than literals is also by writing them next to
    /// each other:
    /// ```
    /// # use terms::l_group_term::LGroupTerm;
    /// # use terms::literal::Literal;
    /// use std::collections::BTreeSet;
    /// let mut meetands = BTreeSet::new();
    /// meetands.insert(LGroupTerm::from('y'));
    /// meetands.insert(LGroupTerm::from('z'));
    /// let term = LGroupTerm::from('x') *  LGroupTerm::Meet(meetands);
    /// assert_eq!(term, LGroupTerm::from("x(y^z)"));
    /// ```
    fn from(s: &str) -> LGroupTerm {
        let result = parse_l_group_term::parse(s);
        match result {
            Ok(term) => term.reduced(),
            Err(e) => panic!(e)
        }
    }
}

impl From<char> for LGroupTerm {
    fn from(c: char) -> LGroupTerm {
        LGroupTerm::Atom(FreeGroupTerm::from(c))
    }
}

impl Mul for LGroupTerm {
    type Output = LGroupTerm;

    fn mul(self, other: LGroupTerm) -> LGroupTerm {
        match (self.clone(), other.clone()) {
            (LGroupTerm::Atom(x), LGroupTerm::Atom(y)) => LGroupTerm::Atom(x * y),
            _ => LGroupTerm::Prod(vec![self, other]).reduced()
        }
    }
}

/*
impl Meet for LGroupTerm {
    type Output = LGroupTerm;

    fn meet(self, other: LGroupTerm) -> LGroupTerm {
        let mut meetands = BTreeSet::new();
        meetands.insert(self);
        meetands.insert(other);LGroupTerm::Meet(meetands).reduced()
    }
}

impl Join for LGroupTerm {
    type Output = LGroupTerm;

    fn join(self, other: LGroupTerm) -> LGroupTerm {
        let mut joinands = BTreeSet::new();
        joinands.insert(self);
        joinands.insert(other);
        LGroupTerm::Join(joinands).reduced()
    }
}
*/

impl Term for LGroupTerm {
    fn inverse(&self) -> LGroupTerm {
        match self {
            LGroupTerm::Atom(x) => LGroupTerm::Atom(x.inverse()),
            LGroupTerm::Meet(xs) => LGroupTerm::Join(xs.iter().map(|x| x.inverse()).collect()),
            LGroupTerm::Join(xs) => LGroupTerm::Meet(xs.iter().map(|x| x.inverse()).collect()),
            LGroupTerm::Prod(xs) => {
                let mut new_xs: Vec<LGroupTerm> = xs.iter().map(|x| x.inverse()).collect();
                new_xs.reverse();
                LGroupTerm::Prod(new_xs)
            }
        }
    }
}

impl Reducable for LGroupTerm {
    fn reduced(self) -> LGroupTerm {
        match self {
            LGroupTerm::Atom(x) => atom_reduced(x),
            LGroupTerm::Meet(xs) => meet_reduced(xs).expect("Reducing failed."),
            LGroupTerm::Join(xs) => join_reduced(xs).expect("Reducing failed."),
            LGroupTerm::Prod(xs) => prod_reduced(xs).expect("Reducing failed.")
        }
    }
}

impl ToString for LGroupTerm {
    fn to_string(&self) -> String {
        let delimiter: char;
        let mut elements = Vec::new();
        match self {
            LGroupTerm::Atom(x) => return x.to_string(),
            LGroupTerm::Meet(xs) => {
                delimiter = '^';
                for x in xs { elements.push(x); }
            },
            LGroupTerm::Join(xs) => {
                delimiter = 'v';
                for x in xs { elements.push(x); }
            },
            LGroupTerm::Prod(xs) => {
                delimiter = '*';
                for x in xs { elements.push(x); }
            }
        }

        if elements.len() == 0 {
            return format!("Empty '{}'", delimiter);
        }

        let mut string = format!("{}", elements[0].to_string());
        for i in 1 .. elements.len() {
            string.push_str(format!(" {} {}", delimiter, elements[i].to_string()).as_str());
        }
        return format!("({})", string);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse_atom() {
        let x = Literal::from('x');
        let y = Literal::from('y');
        let z = Literal::from('z');
        let xyz = FreeGroupTerm { literals: vec![x,y,z] };
        let term = LGroupTerm::Atom(xyz);
        let inverse = LGroupTerm::Atom(FreeGroupTerm { literals: vec![z.inverse(), y.inverse(), x.inverse()]});
        assert_eq!(inverse, term.inverse());
    }

    #[test]
    fn test_inverse_meet() {
        let x = LGroupTerm::Atom(FreeGroupTerm { literals: vec![Literal::from('x')]});
        let y = LGroupTerm::Atom(FreeGroupTerm { literals: vec![Literal::from('y')]});
        let mut meetands = BTreeSet::new();
        meetands.insert(x.clone());
        meetands.insert(y.clone());
        let meet = LGroupTerm::Meet(meetands);
        let mut inverse_meetands = BTreeSet::new();
        inverse_meetands.insert(x.inverse());
        inverse_meetands.insert(y.inverse());
        let inverse = LGroupTerm::Join(inverse_meetands);
        assert_eq!(inverse, meet.inverse());
    }

    #[test]
    fn test_inverse_join() {
        let x = LGroupTerm::Atom(FreeGroupTerm { literals: vec![Literal::from('x')]});
        let y = LGroupTerm::Atom(FreeGroupTerm { literals: vec![Literal::from('y')]});
        let mut joinands = BTreeSet::new();
        joinands.insert(x.clone());
        joinands.insert(y.clone());
        let join = LGroupTerm::Join(joinands);
        let mut inverse_joinands = BTreeSet::new();
        inverse_joinands.insert(x.inverse());
        inverse_joinands.insert(y.inverse());
        let inverse = LGroupTerm::Meet(inverse_joinands);
        assert_eq!(inverse, join.inverse());
    }

    #[test]
    fn test_inverse_recursive() {
        let x = LGroupTerm::Atom(FreeGroupTerm { literals: vec![Literal::from('x')]});
        let y = LGroupTerm::Atom(FreeGroupTerm { literals: vec![Literal::from('y')]});
        let z = LGroupTerm::Atom(FreeGroupTerm { literals: vec![Literal::from('z')]});
        let z_inv = z.inverse();

        let mut meetands = BTreeSet::new();
        meetands.insert(x.clone());
        meetands.insert(y.clone());
        let meet = LGroupTerm::Meet(meetands);
        
        let mut joinands = BTreeSet::new();
        joinands.insert(x.inverse());
        joinands.insert(y.inverse());
        let inverse_of_meet = LGroupTerm::Join(joinands);

        let prod = LGroupTerm::Prod(vec![meet, z]);
        let prod_inverse = LGroupTerm::Prod(vec![z_inv, inverse_of_meet]);

        assert_eq!(prod_inverse, prod.inverse());
    }

    #[test]
    fn test_mul_atoms() {
        let x = LGroupTerm::from(Literal::from('x'));
        let y = LGroupTerm::from(Literal::from('y'));
        assert_eq!(LGroupTerm::Atom(FreeGroupTerm::new(vec![Literal::from('x'), Literal::from('y')])), x * y)
    }

    #[test]
    fn test_to_string() {
        let mut inner_meetands = BTreeSet::new();
        inner_meetands.insert(LGroupTerm::from(Literal::from('x')));
        inner_meetands.insert(LGroupTerm::from(Literal::from('y')));
        let inner_meet = LGroupTerm::Meet(inner_meetands);
        let mut meetands = BTreeSet::new();
        meetands.insert(inner_meet);
        meetands.insert(LGroupTerm::from(Literal::from('z')));
        let meet = LGroupTerm::Meet(meetands);
        assert_eq!(String::from("(z ^ (x ^ y))"), meet.to_string());
    }
}