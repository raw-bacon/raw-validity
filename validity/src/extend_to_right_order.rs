use truncated::truncated_group::TruncatedGroup;
use truncated::truncated_subgroup::TruncatedSubgroup;
use truncated::truncated_group::ElementsExceptIdentity;
use terms::short_free_group_term::ShortFreeGroupTerm;
use std::collections::BTreeSet;
use terms::short_free_group_term::Len;
use terms::Term;
use truncated::truncated_subgroup::Insert;


pub (super) fn extend_to_right_order(elements: Box<BTreeSet<ShortFreeGroupTerm>>, verbose: bool) -> bool {
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


    let ambient_group = TruncatedGroup::new(all_literals.clone());
    let subgroup = TruncatedSubgroup::new(elements, all_literals, false, verbose);

    if verbose {
        let mut subgroup_string = String::new();
        subgroup_string.push('{');
        for x in &*subgroup.elements {
            subgroup_string.push_str(x.to_string().as_str());
            subgroup_string.push_str(", ")
        }
        subgroup_string.pop();
        subgroup_string.pop();
        subgroup_string.push('}');
        println!("The truncated subgroup is {}.", subgroup_string);
    }
    
    let mut terms_and_inverses = subgroup.elements.clone();
    for x in &*subgroup.elements {
        terms_and_inverses.insert(x.inverse());
    }
    
    let mut strong_complement = ambient_group.elements_except_identity();
    for x in *terms_and_inverses {
        strong_complement.remove(&x);
    }
    
    return extends_helper(&ambient_group, &subgroup, &strong_complement, verbose);
}

fn extends_helper(
        ambient_group: &TruncatedGroup, 
        subgroup: &TruncatedSubgroup,
        strong_complement: &BTreeSet<ShortFreeGroupTerm>,
        verbose: bool) -> bool {
    
    if contains_identity(&subgroup) { return false; }
    if contains_all_terms_or_inverses(&ambient_group, &subgroup) { return true; }

    let minimal = &strong_complement.iter().min_by_key(|x| x.len()).unwrap();
    let mut new_complement = strong_complement.clone();
    new_complement.remove(*minimal);
    new_complement.remove(&minimal.inverse());

    for t in &[**minimal, minimal.inverse()] {
        if verbose {
            println!("\nAdding {}.", t.to_string());
        }
        let mut new_subgroup = TruncatedSubgroup::new(subgroup.elements.clone(), ambient_group.generators.clone(), true, verbose);
        new_subgroup.insert(*t);
        if extends_helper(&ambient_group, &new_subgroup, &new_complement, verbose) {
            return true;
        }
    }
    if verbose {
        println!("This didn't extend.")
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
