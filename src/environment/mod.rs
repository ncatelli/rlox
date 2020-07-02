use crate::object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[cfg(test)]
mod tests;

type SymbolTable = HashMap<String, object::Object>;
type Parent = Box<Rc<Environment>>;

/// Functions as a symbols table for looking up variables assignments.
#[derive(Default, Debug)]
pub struct Environment {
    offset: usize,
    parent: Option<Parent>,
    symbols_table: RefCell<SymbolTable>,
}

impl Environment {
    pub fn new() -> Rc<Self> {
        Rc::new(Environment {
            offset: 0,
            parent: None,
            symbols_table: RefCell::new(SymbolTable::new()),
        })
    }

    pub fn from(parent: &Rc<Environment>) -> Rc<Self> {
        let parent = Box::new(parent.clone());

        Rc::new(Environment {
            offset: parent.offset + 1,
            parent: Some(parent),
            symbols_table: RefCell::new(SymbolTable::new()),
        })
    }

    pub fn assign(&self, name: &str, value: object::Object) -> Option<object::Object> {
        let id = name.to_string();
        let has_key = self.symbols_table.borrow().contains_key(&id);
        match (has_key, self.parent.clone()) {
            (true, _) => self.define(&id, value),
            (false, Some(parent)) => parent.assign(&id, value),
            (false, None) => None,
        }
    }

    pub fn define(&self, name: &str, value: object::Object) -> Option<object::Object> {
        self.symbols_table
            .borrow_mut()
            .insert(name.to_string(), value.clone());

        Some(value)
    }

    pub fn get(&self, name: &str) -> Option<object::Object> {
        let val = self.symbols_table.borrow().get(name).cloned();
        match (val, self.parent.clone()) {
            (Some(v), _) => Some(v),
            (None, Some(parent)) => parent.get(&name),
            (None, None) => None,
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}
