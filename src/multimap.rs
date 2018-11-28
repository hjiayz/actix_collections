use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;
pub struct BTreeMultiMap<K, V>(BTreeMap<K, BTreeSet<V>>);

impl<K: Ord, V: Ord> Default for BTreeMultiMap<K, V> {
    fn default() -> Self {
        BTreeMultiMap(BTreeMap::new())
    }
}

impl<K: Ord, V: Ord> BTreeMultiMap<K, V> {
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

pub struct HashMultiMap<K, V>(HashMap<K, HashSet<V>>);

impl<K: Hash + Eq, V: Hash + Eq> Default for HashMultiMap<K, V> {
    fn default() -> Self {
        HashMultiMap(HashMap::new())
    }
}

impl<K: Hash + Eq, V: Hash + Eq> HashMultiMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> bool {
        if self.0.contains_key(&key) {
            return self.0.get_mut(&key).unwrap().insert(value);
        } else {
            let mut new_set = HashSet::new();
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
    pub fn get(&self, key: &K) -> Option<&HashSet<V>> {
        self.0.get(key)
    }
}
