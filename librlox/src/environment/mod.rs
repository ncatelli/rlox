use crate::parser::expression::{Expr, Identifier};
use std::collections::HashMap;

#[cfg(test)]
mod tests;

/// Functions as a symbols table for looking up variables assignments.
pub struct Environment {
    symbols_table: HashMap<Identifier, Expr>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            symbols_table: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: Identifier, value: Expr) -> Option<Expr> {
        self.symbols_table.insert(name, value)
    }

    pub fn get(&mut self, name: &Identifier) -> Option<&Expr> {
        self.symbols_table.get(name)
    }
}
