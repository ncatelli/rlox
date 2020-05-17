use crate::environment;
use crate::parser::expression::{Expr, Identifier};
use std::collections::HashMap;

/// Functions as a symbols table for looking up variables assignments.
#[derive(Default)]
pub struct Hashmap {
    symbols_table: HashMap<Identifier, Expr>,
}

impl Hashmap {
    pub fn new() -> Self {
        Hashmap::default()
    }
}

impl environment::Environment<Identifier, Expr> for Hashmap {
    fn define(&mut self, name: Identifier, value: Expr) -> Option<Expr> {
        self.symbols_table.insert(name, value)
    }

    fn get(&mut self, name: &Identifier) -> Option<&Expr> {
        self.symbols_table.get(name)
    }
}
