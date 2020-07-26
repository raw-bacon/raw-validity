use terms::free_group_term::FreeGroupTerm;
use std::collections::BTreeSet;

pub struct CNF {
    meetands: BTreeSet<BTreeSet<FreeGroupTerm>>
}

impl ToString for CNF {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for meetand in &self.meetands {
            string.push('(');
            for joinand in meetand {
                string.push_str(joinand.to_string().as_str());
                string.push_str(" v ");
            }
            string = string[0 .. string.len() - 3].to_string();
            string.push_str(") ^ ");
        }
        if string.len() == 0 {
            return String::from("(())")
        }
        string[0..string.len() - 3].to_string()
    }
}

impl CNF {
    /// Constructs a new CNF from a set of sets of free group terms
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// use terms::literal::lit;
    /// use terms::free_group_term::FreeGroupTerm;
    /// use std::collections::BTreeSet;
    /// use cnf::normal_cnf::CNF;
    /// let mut meetands = BTreeSet::new();
    /// let mut first_meetand = BTreeSet::new();
    /// let mut second_meetand = BTreeSet::new();
    /// let first_joinand = FreeGroupTerm::from(lit('x'));
    /// let second_joinand = FreeGroupTerm::from(lit('y'));
    /// let third_joinand = FreeGroupTerm::from(lit('z'));
    /// let fourth_joinand = FreeGroupTerm::from(lit('w'));
    ///
    /// first_meetand.insert(first_joinand);
    /// first_meetand.insert(second_joinand);
    ///
    /// second_meetand.insert(third_joinand);
    /// second_meetand.insert(fourth_joinand);
    ///
    /// meetands.insert(first_meetand);
    /// meetands.insert(second_meetand);
    ///
    /// assert_eq!(String::from("(w v z) ^ (x v y)"), CNF::new(meetands).to_string());
    /// ```
    pub fn new(meetands: BTreeSet<BTreeSet<FreeGroupTerm>>) -> CNF {
        CNF { meetands: meetands }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use terms::literal::lit;

    #[test]
    fn test_to_string() {
        let mut meetands = BTreeSet::new();
        let mut first_meetand = BTreeSet::new();
        let mut second_meetand = BTreeSet::new();
        let first_joinand = FreeGroupTerm::from(lit('x'));
        let second_joinand = FreeGroupTerm::from(lit('y'));
        let third_joinand = FreeGroupTerm::from(lit('z'));
        let fourth_joinand = FreeGroupTerm::from(lit('w'));

        first_meetand.insert(first_joinand);
        first_meetand.insert(second_joinand);

        second_meetand.insert(third_joinand);
        second_meetand.insert(fourth_joinand);

        meetands.insert(first_meetand);
        meetands.insert(second_meetand);

        assert_eq!(String::from("(w v z) ^ (x v y)"), CNF::new(meetands).to_string());
    }
}