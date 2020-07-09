use crate::ast::expression::Identifier;
use crate::object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[cfg(test)]
mod tests;

type SymbolTable = HashMap<Identifier, object::Object>;
type Parent = Box<Rc<Environment>>;

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

    pub fn from(parent: &Rc<Environment>) -> Rc<Self> {
        Rc::new(Environment {
            parent: Some(Box::new(parent.clone())),
            symbols_table: RefCell::new(SymbolTable::new()),
        })
    }

    pub fn assign(&self, name: &Identifier, value: object::Object) -> Option<object::Object> {
        let id = name;
        let has_key = self.symbols_table.borrow().contains_key(&id);
        match (has_key, self.parent.as_ref()) {
            (true, _) => self.define(&id, value),
            (false, Some(parent)) => parent.assign(&id, value),
            (false, None) => None,
        }
    }

    pub fn define(&self, name: &Identifier, value: object::Object) -> Option<object::Object> {
        self.symbols_table
            .borrow_mut()
            .insert(name.clone(), value.clone());

        Some(value)
    }

    pub fn get(&self, name: &Identifier) -> Option<object::Object> {
        let val = self.symbols_table.borrow().get(name).cloned();
        match (val, self.parent.as_ref()) {
            (Some(v), _) => Some(v),
            (None, Some(parent)) => parent.get(&name),
            (None, None) => None,
        }
    }
}
