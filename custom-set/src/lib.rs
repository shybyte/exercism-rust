use std::collections::HashMap;
use std::hash::{Hash};
use std::clone::{Clone};

#[derive(Debug, PartialEq, Clone)]
pub struct CustomSet<T: Hash + Eq + Clone> {
    hash_map: HashMap<T, bool>
}

impl<T: Hash + Eq + Clone> CustomSet<T> {
    pub fn new<>(vec: Vec<T>) -> CustomSet<T> {
        let mut new_set = CustomSet { hash_map: HashMap::new() };
        for el in vec {
            new_set.add(el);
        }
        new_set
    }

    pub fn add(&mut self, el: T) {
        self.hash_map.insert(el, true);
    }

    pub fn contains(&self, el: &T) -> bool {
        self.hash_map.contains_key(el)
    }

    pub fn is_empty(&self) -> bool {
        self.hash_map.is_empty()
    }

    pub fn is_subset(&self, other: &CustomSet<T>) -> bool {
        self.hash_map.keys().all(|el| other.contains(el))
    }

    pub fn is_disjoint(&self, other: &CustomSet<T>) -> bool {
        self.hash_map.keys().all(|el| !other.contains(el))
    }

    pub fn intersection(&self, other: &CustomSet<T>) -> CustomSet<T> {
        CustomSet::new(self.hash_map.keys()
            .filter(|el| other.contains(el))
            .map(|el| el.clone())
            .collect()
        )
    }

    pub fn difference(&self, other: &CustomSet<T>) -> CustomSet<T> {
        CustomSet::new(self.hash_map.keys()
            .filter(|el| !other.contains(el))
            .map(|el| el.clone())
            .collect()
        )
    }

    pub fn union(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let mut union_set = self.clone();
        for el in other.hash_map.keys() {
            union_set.add(el.clone());
        }

        union_set
    }
}


