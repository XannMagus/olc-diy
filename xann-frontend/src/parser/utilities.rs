use anyhow::{bail, Result};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

pub struct UniqueKeyHashMap<K, V> {
    map: HashMap<K, V>,
}

impl<K, V> UniqueKeyHashMap<K, V>
where
    K: Eq + PartialEq + Hash + Display + Clone,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<()> {
        match self.map.entry(key.clone()) {
            Entry::Occupied(_) => {
                bail!("Key {key} already defined")
            }
            Entry::Vacant(entry) => {
                entry.insert(value);
                Ok(())
            }
        }
    }

    pub fn to_vec(self) -> Vec<V> {
        self.map.into_values().collect()
    }
}
