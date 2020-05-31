use crate::object;
use std::cell::RefCell;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

type SymbolTable = HashMap<String, object::Object>;

/// Functions as a symbols table for looking up variables assignments.
#[derive(Default, Debug)]
pub struct Environment {
    symbols_table: RefCell<SymbolTable>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn assign(&mut self, name: &str, value: object::Object) -> Option<object::Object> {
        let id = name.to_string();
        if None == self.symbols_table.borrow().get(&id) {
            return None;
        }

        self.define(&id, value)
    }

    pub fn define(&mut self, name: &str, value: object::Object) -> Option<object::Object> {
        self.symbols_table
            .borrow_mut()
            .insert(name.to_string(), value.clone());

        Some(value)
    }

    pub fn get(&mut self, name: &str) -> Option<object::Object> {
        self.symbols_table.borrow().get(name).cloned()
    }
}
