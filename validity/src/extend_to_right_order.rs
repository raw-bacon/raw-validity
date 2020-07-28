use truncated::truncated_group::TruncatedGroup;
use truncated::truncated_subgroup::TruncatedSubgroup;
use truncated::truncated_group::ElementsExceptIdentity;
use terms::short_free_group_term::ShortFreeGroupTerm;
use std::collections::BTreeSet;
use terms::short_free_group_term::Len;
use terms::Term;


pub (super) fn extend_to_right_order(elements: BTreeSet<ShortFreeGroupTerm>) -> bool {
    let mut all_literals = BTreeSet::new();
    for x in &elements {
        match (x.left, x.mid, x.right) {
            (None, None, None) => { return false; }
            (Some(a), None, None) => { all_literals.insert(a); }
            (Some(a), Some(b), None) => {
                all_literals.insert(a);
                all_literals.insert(b);
            },
            (Some(a), Some(b), Some(c)) => {
                all_literals.insert(a);
                all_literals.insert(b);
                all_literals.insert(c);
            },
            _ => {}
        };
    }


    let ambient_group = TruncatedGroup::new(all_literals.clone());
    let subgroup = TruncatedSubgroup::new(elements, all_literals);

    return extends_helper(&ambient_group, &subgroup);
}

fn extends_helper(
        ambient_group: &TruncatedGroup, 
        subgroup: &TruncatedSubgroup) -> bool {
    
    if contains_identity(&subgroup) { return false; }
    if contains_all_terms_or_inverses(&ambient_group, &subgroup) { return true; }

    let elements_except_identity = ambient_group.elements_except_identity();
    let minimal = elements_except_identity.difference(&subgroup.elements)
                    .min_by_key(|x| x.len())
                    .unwrap();

    for t in &[*minimal, minimal.inverse()] {
        let mut new_elements = subgroup.elements.clone();
        new_elements.insert(*t);
        let new_subgroup = TruncatedSubgroup::new(new_elements, ambient_group.generators.clone());
        if extends_helper(&ambient_group, &new_subgroup) {
            return true;
        }
    }
    return false;
}

fn contains_all_terms_or_inverses(
        ambient_group: &TruncatedGroup, 
        subgroup: &TruncatedSubgroup) -> bool {
    for x in &ambient_group.elements_except_identity() {
        if !subgroup.elements.contains(x) && !subgroup.elements.contains(&x.inverse()) {
            return false;
        }
    }
    return true;
}

fn contains_identity(subgroup: &TruncatedSubgroup) -> bool {
    return subgroup.elements.contains(&ShortFreeGroupTerm::new(None, None, None));
}
