//use truncated_free_groups::truncated_group::TruncatedGroup;
use truncated_free_groups::tiny_truncated_group::TinyTruncatedGroup;
use truncated_free_groups::truncated_subgroup::TruncatedSubgroup;
use truncated_free_groups::tiny_truncated_group::ElementsExceptIdentity;
use l_group_formulas::short_free_group_term::ShortFreeGroupTerm;
use std::collections::BTreeSet;
// use l_group_formulas::short_free_group_term::Len;
use l_group_formulas::Term;
use truncated_free_groups::truncated_subgroup::Insert;


pub (super) fn extend_to_right_order(elements: Box<BTreeSet<ShortFreeGroupTerm>>) -> bool {
    let mut all_literals = BTreeSet::new();
    for x in &*elements {
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

    let ambient_group = TinyTruncatedGroup::new(all_literals.clone());
    let subgroup = TruncatedSubgroup::new(elements, all_literals, false, true);

   
    let mut terms_and_inverses = subgroup.elements.clone();
    for x in &*subgroup.elements {
        terms_and_inverses.insert(x.inverse());
    }
    
    let mut strong_complement = ambient_group.elements_except_identity();
    for x in *terms_and_inverses {
        strong_complement.remove(&x);
    }

    extends_helper(&ambient_group, &subgroup, &mut strong_complement, 1)
}

fn extends_helper(
        ambient_group: &TinyTruncatedGroup, 
        subgroup: &TruncatedSubgroup,
        complement: &mut BTreeSet<ShortFreeGroupTerm>,
        recursion_depth: usize) -> bool {
    
    
    if contains_identity(&subgroup) { 
        return false;
    }
    if contains_all_terms_or_inverses(&ambient_group, &subgroup) { 
        return true;
    }

    // let complement = strong_complement(&subgroup, &ambient_group);
    // let minimal = complement.iter().min_by_key(|x| x.len()).unwrap();
    let mut minimal: ShortFreeGroupTerm;
    
    let mut complement_iter = complement.iter();
    minimal = *complement_iter.next().unwrap();
    minimal = minimal.clone();
    

    let mut new_subgroup = TruncatedSubgroup::new(subgroup.elements.clone(), ambient_group.generators.clone(), true, true);
    let newly_added = new_subgroup.insert(minimal);

    for t in &newly_added {
        complement.remove(&t);
        complement.remove(&t.inverse());
    }

    if extends_helper(&ambient_group, &new_subgroup, complement, recursion_depth + 1) {
        return true;
    }

    for t in &newly_added {
        complement.insert(*t);
        complement.insert(t.inverse());
    }

    let mut new_subgroup = TruncatedSubgroup::new(subgroup.elements.clone(), ambient_group.generators.clone(), true, true);
    let newly_added = new_subgroup.insert(minimal.inverse());

    for t in &newly_added {
        complement.remove(&t);
        complement.remove(&t.inverse());
    }

    if extends_helper(&ambient_group, &new_subgroup, complement, recursion_depth + 1) {
        return true;
    }

    for t in &newly_added {
        complement.insert(*t);
        complement.insert(t.inverse());
    }
    return false;
}

fn contains_all_terms_or_inverses(
        ambient_group: &TinyTruncatedGroup, 
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
