use truncated::truncated_group::TruncatedGroup;
use truncated::truncated_subgroup::TruncatedSubgroup;
use terms::short_free_group_term::ShortFreeGroupTerm;
use std::collections::BTreeSet;

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
    let ambient_group = TruncatedGroup::new(all_literals);
    todo!()
    return true;
}