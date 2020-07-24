use crate::ast::identifier::Identifier;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    id: Identifier,
}

impl Class {
    pub fn new(id: Identifier) -> Self {
        Class { id }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Class {}", self.id)
    }
}
