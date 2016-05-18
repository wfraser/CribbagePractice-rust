// Utility functions.
//
// Copyright (c) 2016 by William R. Fraser
//

use std::collections::btree_map::{BTreeMap, Entry};

pub struct PowerSet<'a, T: 'a> {
    items: &'a [T],
    current: u64,
}

impl<'a, T> Iterator for PowerSet<'a, T> {
    type Item = Vec<&'a T>;
    fn next(&mut self) -> Option<Vec<&'a T>> {
        if self.current == (1 << self.items.len()) {
            None
        } else {
            let mut set = vec![];
            for (idx, item) in self.items.iter().enumerate() {
                if self.current & (1 << idx) != 0 {
                    set.push(item);
                }
            }
            self.current += 1;
            Some(set)
        }
    }
}

pub fn power_set<'a, T>(items: &'a [T]) -> PowerSet<'a, T> {
    // This method uses one bit per item, plus one to signal that it's done.
    // Of course, you'll probably run out of patience long before you run out of bits. :)
    assert!(items.len() < 64);

    PowerSet {
        items: items,
        current: 0,
    }
}

#[test]
fn test_power_set() {
    let set = vec![1, 2, 3];
    let mut pset = power_set(&set);
    assert!(pset.next().unwrap().is_empty());
    assert_eq!(pset.next().unwrap(), vec![&1]);
    assert_eq!(pset.next().unwrap(), vec![&2]);
    assert_eq!(pset.next().unwrap(), vec![&1, &2]);
    assert_eq!(pset.next().unwrap(), vec![&3]);
    assert_eq!(pset.next().unwrap(), vec![&1, &3]);
    assert_eq!(pset.next().unwrap(), vec![&2, &3]);
    assert_eq!(pset.next().unwrap(), vec![&1, &2, &3]);
    assert!(pset.next().is_none());
}

pub fn group_by<'a, T, K, F>(items: &'a [T], f: F) -> BTreeMap<K, Vec<&'a T>>
        where F: Fn(&T) -> K, K: Ord {
    let mut map: BTreeMap<K, Vec<&'a T>> = BTreeMap::new();
    for item in items {
        let key = f(item);
        match map.entry(key) {
            Entry::Occupied(entry) => { entry.into_mut().push(item); },
            Entry::Vacant(entry) => { entry.insert(vec![item]); },
        }
    }
    map
}

#[test]
fn test_group_by() {
    let items = vec![1, 2, 3, 10, 11, 20];
    let by_tens = group_by(&items, |n| n / 10);
    assert_eq!(by_tens.get(&0).unwrap(), &vec![&1, &2, &3]);
    assert_eq!(by_tens.get(&1).unwrap(), &vec![&10, &11]);
    assert_eq!(by_tens.get(&2).unwrap(), &vec![&20]);
    assert!(by_tens.get(&3).is_none());
}

pub fn factorial(n: i8) -> i8 {
    let mut total = 1;
    for i in 2 .. n + 1 {
        total *= i;
    }
    total
}
