use std::collections::BTreeSet;
use l_group_formulas::literal::Literal;
use l_group_formulas::short_free_group_term::ShortFreeGroupTerm;
use l_group_formulas::Term;


#[derive(Debug)]
pub struct TinyTruncatedGroup {
    pub generators: BTreeSet<Literal>,
    pub elements:   Box<BTreeSet<ShortFreeGroupTerm>>
}


impl TinyTruncatedGroup {
    pub fn new(generators: BTreeSet<Literal>) -> TinyTruncatedGroup {
        let mut literals = BTreeSet::new();
        let mut elements = BTreeSet::new();
        for x in &generators {
            literals.insert(ShortFreeGroupTerm::from(*x));
            literals.insert(ShortFreeGroupTerm::from(x.inverse()));
            elements.insert(ShortFreeGroupTerm::from(*x));
            elements.insert(ShortFreeGroupTerm::from(x.inverse()));
        }

        let mut new_elements = BTreeSet::new();
        for literal in &literals {
            for t in &elements {
                new_elements.insert(*t * *literal);
            }
        }
        for x in &new_elements { elements.insert(*x); }

        TinyTruncatedGroup {
            generators,
            elements:   Box::new(elements)
        }
    }
}


pub trait ElementsExceptIdentity {
    fn elements_except_identity(&self) -> BTreeSet<ShortFreeGroupTerm>;
}

impl ElementsExceptIdentity for TinyTruncatedGroup {
    fn elements_except_identity(&self) -> BTreeSet<ShortFreeGroupTerm> {
        let mut all_elements = self.elements.clone();
        all_elements.remove(&ShortFreeGroupTerm::new(None, None, None));
        return *all_elements;
    }
}
