//use truncated::truncated_group::TruncatedGroup;
use truncated::tiny_truncated_group::TinyTruncatedGroup;
use truncated::truncated_subgroup::TruncatedSubgroup;
use truncated::tiny_truncated_group::ElementsExceptIdentity;
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

    if verbose {
        println!("Computing the ambient group.");
    }
    let ambient_group = TinyTruncatedGroup::new(all_literals.clone());
    if verbose {
        println!("The ambient group has size {}.", ambient_group.elements.len());
    }
    let subgroup = TruncatedSubgroup::new(elements, all_literals, false, true, verbose);

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
    
    extends_helper(&ambient_group, &subgroup, 1, verbose)
}

fn strong_complement(subgroup: &TruncatedSubgroup, ambient_group: &TinyTruncatedGroup) -> BTreeSet<ShortFreeGroupTerm> {
    let mut terms_and_inverses = subgroup.elements.clone();
    for x in &*subgroup.elements {
        terms_and_inverses.insert(x.inverse());
    }
    
    let mut strong_complement = ambient_group.elements_except_identity();
    for x in *terms_and_inverses {
        strong_complement.remove(&x);
    }
    return strong_complement;
}

fn extends_helper(
        ambient_group: &TinyTruncatedGroup, 
        subgroup: &TruncatedSubgroup,
        recursion_depth: usize,
        verbose: bool) -> bool {
    
    
    if contains_identity(&subgroup) { 
        if verbose { println!("The subgroup contains the identity.\n"); }
        return false; 
    }
    if contains_all_terms_or_inverses(&ambient_group, &subgroup) { 
        if verbose {
            let mut elements_string = String::new();
            elements_string.push('{');
            for x in &*subgroup.elements {
                elements_string.push_str(x.to_string().as_str());
                elements_string.push_str(", ");
            }
            elements_string.pop();
            elements_string.pop();
            elements_string.push('}');
            println!("The order this extends to is {}", elements_string)
        }
        return true; 
    }

    let complement = strong_complement(&subgroup, &ambient_group);
    let minimal = complement.iter().min_by_key(|x| x.len()).unwrap();
    // let minimal = complement.iter().next().unwrap();

    for t in &[*minimal, minimal.inverse()] {
        
        if verbose {
            println!("Currently at recursion depth {}. Adding {}.", recursion_depth, t.to_string());
        }
        let mut new_subgroup = TruncatedSubgroup::new(subgroup.elements.clone(), ambient_group.generators.clone(), true, true, verbose);
        new_subgroup.insert(*t);
        if verbose {
            // println!("Got {} new elements.", new_subgroup.elements.len() - subgroup.elements.len());
        }

        if extends_helper(&ambient_group, &new_subgroup, recursion_depth + 1, verbose) {
            return true;
        }
    }
    if verbose { 
        // println!("This didn't extend.") 
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
