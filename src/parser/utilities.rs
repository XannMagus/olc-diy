use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use anyhow::{bail, Result};
use pretty::{BoxAllocator, DocBuilder};

pub trait ToDoc {
    fn to_doc<'a>(&'a self, allocator: &'a BoxAllocator) -> DocBuilder<'a, BoxAllocator>;
}

pub struct Displayable<T>(pub T);

impl<T: ToDoc> Display for Displayable<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let allocator = BoxAllocator;
        let str = self.0.to_doc(&allocator).pretty(80).to_string();
        write!(f, "{str}")
    }
}

pub struct UniqueKeyHashMap<K, V> {
    map: HashMap<K, V>,
}

impl<K, V> UniqueKeyHashMap<K, V>
where K: Eq + PartialEq + Hash + Display + Clone {

    pub fn new() -> Self {
        Self { map: HashMap::new() }
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