use crate::ast::identifier::Identifier;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum StackErr {
    Undefined,
    InvalidIndex,
}

impl fmt::Display for StackErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Undefined => write!(f, "undefined error"),
            Self::InvalidIndex => write!(f, "invalid index passed to stack"),
        }
    }
}

/// Scope represents an array of Identifiers signifiying a scope
pub type Scope = Vec<Identifier>;

// ScopeStack represents a stack of Scopes implemented using a vector.
#[derive(Clone)]
pub struct ScopeStack {
    stack: Vec<Scope>,
}

impl From<Vec<Scope>> for ScopeStack {
    fn from(source: Vec<Scope>) -> Self {
        ScopeStack { stack: source }
    }
}

impl Into<Vec<Scope>> for ScopeStack {
    fn into(self) -> Vec<Scope> {
        self.stack
    }
}

impl ScopeStack {
    pub fn new() -> Self {
        ScopeStack {
            stack: vec![Scope::new()],
        }
    }

    pub fn push(&mut self, scope: Scope) {
        self.stack.push(scope);
    }

    pub fn pop(&mut self) -> Option<Scope> {
        self.stack.pop()
    }

    pub fn push_elem(&mut self, elem: Identifier) {
        match self.pop() {
            Some(mut scope) => {
                scope.push(elem);
                self.push(scope);
            }
            None => self.push(vec![elem]),
        }
    }

    #[allow(dead_code)]
    pub fn pop_elem(&mut self) -> Option<Identifier> {
        match self.pop() {
            Some(mut scope) => {
                let elem = scope.pop();
                self.push(scope);
                elem
            }
            None => None,
        }
    }

    /// get_offset returns the uzise offset for a given Identifier.
    pub fn get_offset(&self, id: &Identifier) -> Option<usize> {
        let stack_len = self.len();

        let offset = self
            .stack
            .iter()
            .rev()
            .flatten()
            .position(|elem| elem == id);

        offset.map(|i| stack_len - i - 1)
    }

    /// len returns the number of unique IDs exist in the scope, this is useful
    /// for fetching the next Identifier offset.
    pub fn len(&self) -> usize {
        self.stack
            .iter()
            .flatten()
            .collect::<Vec<&Identifier>>()
            .len()
    }
}
