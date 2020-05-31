use crate::object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[cfg(test)]
mod tests;

type SymbolTable = HashMap<String, object::Object>;

/// Functions as a symbols table for looking up variables assignments.
#[derive(Default, Debug)]
pub struct Environment {
    parent: Option<Parent>,
    symbols_table: RefCell<SymbolTable>,
}

impl Environment {
    pub fn new() -> Rc<Self> {
        Rc::new(Environment {
            parent: None,
            symbols_table: RefCell::new(SymbolTable::new()),
        })
    }

    pub fn assign(&self, name: &str, value: object::Object) -> Option<object::Object> {
        let id = name.to_string();
        if None == self.symbols_table.borrow().get(&id) {
            return None;
        }

        self.define(&id, value)
    }

    pub fn define(&self, name: &str, value: object::Object) -> Option<object::Object> {
        self.symbols_table
            .borrow_mut()
            .insert(name.to_string(), value.clone());

        Some(value)
    }

    pub fn get(&self, name: &str) -> Option<object::Object> {
        self.symbols_table.borrow().get(name).cloned()
    }
}

// Wraps an environment for nesting
#[derive(Default, Debug)]
struct Parent {
    parent: Rc<Environment>,
}
