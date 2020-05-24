use crate::ast::expression::Expr;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

/// Functions as a symbols table for looking up variables assignments.
#[derive(Default, Debug)]
pub struct Environment {
    symbols_table: HashMap<String, Expr>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, name: String, value: Expr) -> Option<Expr> {
        self.symbols_table.insert(name, value)
    }

    pub fn get(&mut self, name: &String) -> Option<&Expr> {
        self.symbols_table.get(name)
    }
}
