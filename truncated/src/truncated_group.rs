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
        let mut literals = BTreeSet::new();
        let mut elements = BTreeSet::new();
        for x in &generators {
            literals.insert(ShortFreeGroupTerm::from(*x));
            literals.insert(ShortFreeGroupTerm::from(x.inverse()));
            elements.insert(ShortFreeGroupTerm::from(*x));
            elements.insert(ShortFreeGroupTerm::from(x.inverse()));
        }

        for _ in &[1, 2] {
            let mut new_elements = BTreeSet::new();
            for literal in &literals {
                for t in &elements {
                    new_elements.insert(*t * *literal);
                }
            }
            for x in &new_elements { elements.insert(*x); }
        }

        TruncatedGroup {
            generators: generators,
            elements:   Box::new(elements)
        }
    }
}
