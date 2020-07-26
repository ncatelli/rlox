use crate::ast::identifier::Identifier;
use crate::environment::Environment;
use crate::functions::CallResult;
use crate::instance::Instance;
use crate::object::Object;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    id: Identifier,
}

impl Class {
    pub fn new(id: &Identifier) -> Self {
        Class { id: id.to_owned() }
    }

    pub fn arity(&self) -> usize {
        0
    }

    pub fn call(
        &self,
        _env: Rc<Environment<Identifier, Object>>,
        _args: Vec<Object>,
    ) -> CallResult {
        Ok(obj_instance!(Instance::new(self)))
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Class {}", self.id)
    }
}
