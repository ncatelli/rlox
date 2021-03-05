use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub mod flat_env;
#[cfg(test)]
mod tests;

type Parent<K, V> = Box<Rc<Environment<K, V>>>;

/// Functions as a symbols table for looking up variables assignments.
#[derive(Default, Debug)]
pub struct Environment<K, V> {
    parent: Option<Parent<K, V>>,
    symbols_table: RefCell<HashMap<K, V>>,
}

impl<K, V> Environment<K, V> {
    pub fn new() -> Rc<Self> {
        Rc::new(Environment {
            parent: None,
            symbols_table: RefCell::new(HashMap::new()),
        })
    }

    pub fn from(parent: &Rc<Environment<K, V>>) -> Rc<Self> {
        Rc::new(Environment {
            parent: Some(Box::new(parent.clone())),
            symbols_table: RefCell::new(HashMap::new()),
        })
    }

    pub fn has_key(&self, name: &K) -> bool
    where
        K: Eq + Hash + Clone,
    {
        self.symbols_table.borrow().contains_key(name)
    }

    pub fn assign(&self, name: &K, value: V) -> Option<V>
    where
        K: Eq + Hash + Clone,
        V: Clone,
    {
        let has_key = self.symbols_table.borrow().contains_key(name);
        match (has_key, self.parent.as_ref()) {
            (true, _) => self.define(name, value),
            (false, Some(parent)) => parent.assign(name, value),
            (false, None) => None,
        }
    }

    pub fn define(&self, name: &K, value: V) -> Option<V>
    where
        K: Eq + Hash + Clone,
        V: Clone,
    {
        self.symbols_table
            .borrow_mut()
            .insert(name.clone(), value.clone());

        Some(value)
    }

    pub fn get(&self, name: &K) -> Option<V>
    where
        K: Eq + Hash + Clone,
        V: Clone,
    {
        let val = self.symbols_table.borrow().get(name).cloned();
        match (val, self.parent.as_ref()) {
            (Some(v), _) => Some(v),
            (None, Some(parent)) => parent.get(name),
            (None, None) => None,
        }
    }
}
