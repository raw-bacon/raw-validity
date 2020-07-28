use super::truncated_subgroup::TruncatedSubgroup;
use std::collections::BTreeSet;
use terms::literal::Literal;
use terms::short_free_group_term::ShortFreeGroupTerm;
use terms::Term;

/// Represents the ball of radius 3 in the Cayley graph of the free
/// group with respect to the standard generating set.
/// 
/// #Examples
/// Basic usage:
/// ```
/// use std::collections::BTreeSet;
/// use terms::literal::Literal;
/// use terms::short_free_group_term::ShortFreeGroupTerm;
/// use truncated::truncated_group::TruncatedGroup;
/// let mut generators = BTreeSet::new();
/// generators.insert(Literal::from('x'));
/// generators.insert(Literal::from('y'));
/// let truncated_group = TruncatedGroup::new(generators);
/// let mut expected: BTreeSet<ShortFreeGroupTerm> = BTreeSet::new();
/// assert_eq!(53, truncated_group.elements.len());
#[derive(Debug)]
pub struct TruncatedGroup {
    pub generators: BTreeSet<Literal>,
    pub elements:   Box<BTreeSet<ShortFreeGroupTerm>>
}


impl TruncatedGroup {
    pub fn new(generators: BTreeSet<Literal>) -> TruncatedGroup {
        let mut sub_elements = BTreeSet::new();
        for x in &generators {
            sub_elements.insert(ShortFreeGroupTerm::from(*x));
            sub_elements.insert(ShortFreeGroupTerm::from(x.inverse()));
        }
        let sub = TruncatedSubgroup::new(Box::new(sub_elements), generators.clone());
        TruncatedGroup {
            generators: generators,
            elements:   sub.elements
        }
    }
}

pub trait ElementsExceptIdentity {
    fn elements_except_identity(&self) -> BTreeSet<ShortFreeGroupTerm>;
}

impl ElementsExceptIdentity for TruncatedGroup {
    fn elements_except_identity(&self) -> BTreeSet<ShortFreeGroupTerm> {
        let mut all_elements = self.elements.clone();
        all_elements.remove(&ShortFreeGroupTerm::new(None, None, None));
        return *all_elements;
    }
}