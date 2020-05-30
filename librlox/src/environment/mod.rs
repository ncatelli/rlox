use crate::object;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

/// Functions as a symbols table for looking up variables assignments.
#[derive(Default, Debug)]
pub struct Environment {
    symbols_table: HashMap<String, object::Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn assign(&mut self, name: String, value: object::Object) -> Option<object::Object> {
        if None == self.symbols_table.get(&name) {
            return None;
        }

        self.define(name, value)
    }

    pub fn define(&mut self, name: String, value: object::Object) -> Option<object::Object> {
        self.symbols_table.insert(name, value.clone());

        Some(value)
    }

    pub fn get(&mut self, name: &String) -> Option<&object::Object> {
        self.symbols_table.get(name)
    }
}
