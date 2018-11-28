use std::collections::{BTreeMap, BTreeSet};

pub struct MultiMap<K, V>(BTreeMap<K, BTreeSet<V>>);

impl<K: Ord, V: Ord> Default for MultiMap<K, V> {
    fn default() -> Self {
        MultiMap(BTreeMap::new())
    }
}

impl<K: Ord, V: Ord> MultiMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> bool {
        if self.0.contains_key(&key) {
            return self.0.get_mut(&key).unwrap().insert(value);
        } else {
            let mut new_set = BTreeSet::new();
            new_set.insert(value);
            self.0.insert(key, new_set);
            return true;
        }
    }
    pub fn remove(&mut self, key: &K, value: &V) -> bool {
        match self.0.get_mut(key) {
            Some(s) => s.remove(value),
            None => false,
        }
    }
    pub fn contains_key(&self, key: &K) -> bool {
        self.0.contains_key(key)
    }
    pub fn contains(&self, key: &K, value: &V) -> bool {
        match self.0.get(key) {
            Some(s) => s.contains(value),
            None => false,
        }
    }
    pub fn get(&self, key: &K) -> Option<&BTreeSet<V>> {
        self.0.get(key)
    }
}
