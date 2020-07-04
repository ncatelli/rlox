use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[cfg(test)]
mod tests;

type Locals = HashMap<String, usize>;
type SymbolTable = HashSet<String>;
type Parent = Rc<Node>;

/// Functions as a symbols table for looking up variables assignments.
#[derive(Default, Debug)]
pub struct Node {
    offset: usize,
    parent: Option<Parent>,
    locals: RefCell<Locals>,
    symbols_table: RefCell<SymbolTable>,
}

impl Node {
    pub fn new() -> Rc<Self> {
        Rc::new(Node {
            offset: 0,
            parent: None,
            locals: RefCell::new(Locals::new()),
            symbols_table: RefCell::new(SymbolTable::new()),
        })
    }

    pub fn from(parent: &Rc<Node>) -> Rc<Self> {
        let parent = parent.clone();

        Rc::new(Node {
            offset: parent.offset + 1,
            parent: Some(parent),
            locals: RefCell::new(Locals::new()),
            symbols_table: RefCell::new(SymbolTable::new()),
        })
    }

    /// Adds a new symbol to the Nodes symbol table, returning the scope offset
    pub fn declare(&self, name: &str) -> usize {
        self.symbols_table.borrow_mut().insert(name.to_string());

        self.offset()
    }

    /// Walks up the tree, looking for the first node that contains a
    /// definition for the variable. Returning the nodes offset from the root
    /// node.
    pub fn get(&self, name: &str) -> Option<usize> {
        let val = self.symbols_table.borrow().get(name).cloned();
        match (val, self.parent.as_ref()) {
            (Some(_), _) => Some(self.offset()),
            (None, Some(parent)) => parent.get(&name),
            (None, None) => None,
        }
    }

    /// Gets the first offset declaring name and assigns the pairing to the
    /// locals map.
    pub fn resolve_local(&self, name: &str) -> Option<usize> {
        match self.get(name) {
            Some(offset) => {
                self.locals.borrow_mut().insert(name.to_string(), offset);
                Some(offset)
            }
            None => None,
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}
