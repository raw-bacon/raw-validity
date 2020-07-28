use std::collections::{BTreeSet, BTreeMap};
use terms::short_free_group_term::*;
use terms::literal::Literal;
use terms::Term;
use super::Closable;

/// Represents the closed ball of radius 3 around e in the Cayley
/// graph of a free group with respect to the standard free generating set.
/// 
/// # Examples
/// Basic usage:
/// ```
/// use truncated::truncated_subgroup::TruncatedSubgroup;
/// use terms::short_free_group_term::ShortFreeGroupTerm;
/// use terms::literal::Literal;
/// use std::collections::BTreeSet;
/// let s = ShortFreeGroupTerm::from("xY");
/// let t = ShortFreeGroupTerm::from("yz");
/// let mut set = BTreeSet::new();
/// set.insert(s);
/// set.insert(t);
/// let mut gens = BTreeSet::new();
/// gens.insert(Literal::from('x'));
/// gens.insert(Literal::from('y'));
/// gens.insert(Literal::from('z'));
/// let truncated = TruncatedSubgroup::new(set, gens);
/// let mut expected = BTreeSet::new();
/// expected.insert(s);
/// expected.insert(t);
/// expected.insert(ShortFreeGroupTerm::from("xz"));
/// assert_eq!(expected, truncated.elements.clone());
/// ```
#[derive(Debug)]
pub struct TruncatedSubgroup {
    pub elements:                BTreeSet<ShortFreeGroupTerm>,
    // pub gens_of_ambient_group:   BTreeSet<Literal>,
    pub starts_with_single:      BTreeMap<Literal, BTreeSet<ShortFreeGroupTerm>>,
    pub ends_with_single:        BTreeMap<Literal, BTreeSet<ShortFreeGroupTerm>>,
    pub starts_with_pair:        BTreeMap<(Literal, Literal), BTreeSet<ShortFreeGroupTerm>>,
    pub ends_with_pair:          BTreeMap<(Literal, Literal), BTreeSet<ShortFreeGroupTerm>>,
    pub length_one:              BTreeSet<ShortFreeGroupTerm>,
    pub length_two:              BTreeSet<ShortFreeGroupTerm>,
    pub length_three:            BTreeSet<ShortFreeGroupTerm>
}

impl TruncatedSubgroup {
    pub fn new(
        elements: BTreeSet<ShortFreeGroupTerm>, 
        gens:     BTreeSet<Literal>
    ) -> TruncatedSubgroup {
        // close gens under inversion
        let mut gens_of_ambient_group = BTreeSet::new();
        for g in gens {
            gens_of_ambient_group.insert(g);
            gens_of_ambient_group.insert(g.inverse());
        }

        // initialize categories
        let mut length_one   = BTreeSet::new();
        let mut length_two   = BTreeSet::new();
        let mut length_three = BTreeSet::new();

        let mut starts_with_single = BTreeMap::new();
        let mut starts_with_pair   = BTreeMap::new();
        let mut ends_with_single   = BTreeMap::new();
        let mut ends_with_pair     = BTreeMap::new();

        for g in &gens_of_ambient_group {
            starts_with_single.insert(*g, BTreeSet::new());
            ends_with_single.insert(*g, BTreeSet::new());
            for h in &gens_of_ambient_group {
                starts_with_pair.insert((*g, *h), BTreeSet::new());
                ends_with_pair.insert((*g, *h), BTreeSet::new());
            }
        }

        for y in &elements {
            match (y.left, y.mid, y.right) {
                (Some(a), None, None) => {
                    length_one.insert(*y);
                    starts_with_single.get_mut(&a).unwrap().insert(*y);
                    ends_with_single.get_mut(&a).unwrap().insert(*y);
                },
                (Some(a), Some(b), None) => {
                    length_two.insert(*y);
                    starts_with_single.get_mut(&a).unwrap().insert(*y);
                    ends_with_single.get_mut(&b).unwrap().insert(*y);
                    starts_with_pair.get_mut(&(a, b)).unwrap().insert(*y);
                    ends_with_pair.get_mut(&(a, b)).unwrap().insert(*y);
                },
                (Some(a), Some(b), Some(c)) => {
                    length_three.insert(*y);
                    starts_with_single.get_mut(&a).unwrap().insert(*y);
                    ends_with_single.get_mut(&c).unwrap().insert(*y);
                    starts_with_pair.get_mut(&(a, b)).unwrap().insert(*y);
                    ends_with_pair.get_mut(&(b, c)).unwrap().insert(*y);
                }
                _ => {} // is identity
            };
        }

        let mut sub = TruncatedSubgroup {
            elements:              elements,
            // gens_of_ambient_group: gens_of_ambient_group,
            starts_with_single:    starts_with_single,
            starts_with_pair:      starts_with_pair,
            ends_with_single:      ends_with_single,
            ends_with_pair:        ends_with_pair,
            length_one:            length_one,
            length_two:            length_two,
            length_three:          length_three
        };
        sub.close();
        return sub;
    }
}

impl Closable for TruncatedSubgroup {
    fn close(&mut self) {
        let mut found_new_element = true;
        let mut new_elements_buffer: BTreeSet<ShortFreeGroupTerm> = BTreeSet::new();
        while found_new_element {
            found_new_element = false;
            for y in new_elements_buffer {
                self.elements.insert(y.clone());
                match (y.left, y.mid, y.right) {
                    (Some(a), None, None) => {
                        self.length_one.insert(y);
                        self.starts_with_single.get_mut(&a).unwrap().insert(y);
                        self.ends_with_single.get_mut(&a).unwrap().insert(y);
                    },
                    (Some(a), Some(b), None) => {
                        self.length_two.insert(y);
                        self.starts_with_single.get_mut(&a).unwrap().insert(y);
                        self.ends_with_single.get_mut(&b).unwrap().insert(y);
                        self.starts_with_pair.get_mut(&(a, b)).unwrap().insert(y);
                        self.ends_with_pair.get_mut(&(a, b)).unwrap().insert(y);
                    },
                    (Some(a), Some(b), Some(c)) => {
                        self.length_three.insert(y);
                        self.starts_with_single.get_mut(&a).unwrap().insert(y);
                        self.ends_with_single.get_mut(&c).unwrap().insert(y);
                        self.starts_with_pair.get_mut(&(a, b)).unwrap().insert(y);
                        self.ends_with_pair.get_mut(&(b, c)).unwrap().insert(y);
                    }
                    _ => {} // is identity
                };
            }
            new_elements_buffer = BTreeSet::new();
            for x in &self.elements {
                match x.len() {
                    0 => {},
                    1 => {
                        // x * y is shorter than 3 if either y is shorter 
                        // than 2 or there is cancellation, i.e., y begins
                        // with x^-1. Same for y * x, except then y ends
                        // with x^-1.
                        for y in &self.length_one {
                            let maybe_new = *x * *y;
                            if !self.elements.contains(&maybe_new) {
                                found_new_element = true;
                                new_elements_buffer.insert(maybe_new);
                            }
                            let maybe_new = *y * *x;
                            if !self.elements.contains(&maybe_new) {
                                found_new_element = true;
                                new_elements_buffer.insert(maybe_new);
                            }
                        }
                        for y in &self.length_two {
                            let maybe_new = *x * *y;
                            if !self.elements.contains(&maybe_new) {
                                found_new_element = true;
                                new_elements_buffer.insert(maybe_new);
                            }
                            let maybe_new = *y * *x;
                            if !self.elements.contains(&maybe_new) {
                                found_new_element = true;
                                new_elements_buffer.insert(maybe_new);
                            }
                        }
                        let lit = x.left.unwrap();
                        let cancelling_candidates_start = self.starts_with_single.get(&lit.inverse()).unwrap();
                        for y in cancelling_candidates_start {
                            let maybe_new = *x * *y;
                            if !self.elements.contains(&maybe_new) {
                                found_new_element = true;
                                new_elements_buffer.insert(maybe_new);
                            }
                        }
                        let cancelling_candidates_end = self.ends_with_single.get(&lit.inverse()).unwrap();
                        for y in cancelling_candidates_end {
                            let maybe_new = *y * *x;
                            if !self.elements.contains(&maybe_new) {
                                found_new_element = true;
                                new_elements_buffer.insert(maybe_new);
                            }
                        }
                    },
                    2 => {
                        // if y has length 1, then x * y and y * x are of length <= 3
                        // if y has length 2 or 3, then 
                        //      x * y has length <= 3 if y starts with x.mid.inverse()
                        //      y * x has length <= 3 if y ends with x.left.inverse()
                        for y in &self.length_one {
                            let maybe_new = *x * *y;
                            if !self.elements.contains(&maybe_new) {
                                found_new_element = true;
                                new_elements_buffer.insert(maybe_new);
                            }
                            let maybe_new = *y * *x;
                            if !self.elements.contains(&maybe_new) {
                                found_new_element = true;
                                new_elements_buffer.insert(maybe_new);
                            }
                        }
                        let cancelling_candidates_start = self.starts_with_single.get(&x.mid.unwrap().inverse()).unwrap();
                        for y in cancelling_candidates_start {
                            let maybe_new = *x * *y;
                            if !self.elements.contains(&maybe_new) {
                                found_new_element = true;
                                new_elements_buffer.insert(maybe_new);
                            }
                        }
                        let cancelling_candidates_end = self.ends_with_single.get(&x.left.unwrap().inverse()).unwrap();
                        for y in cancelling_candidates_end {
                            let maybe_new = *y * *x;
                            if !self.elements.contains(&maybe_new) {
                                found_new_element = true;
                                new_elements_buffer.insert(maybe_new);
                            }
                        }
                    },
                    3 => {
                        // if y has length 1, then x * y is of length <= 3 if y.left == x.right.inverse(),
                        // and                     y * x is of length <= 3 if y.left == x.left.inverse().
                        // if y has length 2 or 3, then x * y is of length <= 3 if y starts with (x.right.inverse(), x.mid.inverse())
                        // and                          y * x is of length <= 3 if y ends with (x.mid.inverse(), x.left.inverse())
                        for y in &self.length_one {
                            if y.left.unwrap() == x.right.unwrap().inverse() {
                                let maybe_new = *x * *y;
                                if !self.elements.contains(&maybe_new) {
                                    found_new_element = true;
                                    new_elements_buffer.insert(maybe_new);
                                }
                            }
                            if y.left.unwrap() == x.left.unwrap().inverse() {
                                let maybe_new = *y * *x;
                                if !self.elements.contains(&maybe_new) {
                                    found_new_element = true;
                                    new_elements_buffer.insert(maybe_new);
                                }
                            }
                        }
                        let cancelling_candidates_start = self.starts_with_pair.get(&(x.right.unwrap().inverse(), x.mid.unwrap().inverse())).unwrap();
                        for y in cancelling_candidates_start {
                            let maybe_new = *x * *y;
                            if !self.elements.contains(&maybe_new) {
                                found_new_element = true;
                                new_elements_buffer.insert(maybe_new);
                            }
                        }
                        let cancelling_candidates_end = self.ends_with_pair.get(&(x.mid.unwrap().inverse(), x.left.unwrap().inverse())).unwrap();
                        for y in cancelling_candidates_end {
                            let maybe_new = *y * *x;
                            if !self.elements.contains(&maybe_new) {
                                found_new_element = true;
                                new_elements_buffer.insert(maybe_new);
                            }
                        }
                    },
                    _ => { panic!("Invalid short free group term!") }
                };
            }
        }
    }
}