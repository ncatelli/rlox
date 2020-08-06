use crate::ast::identifier::Identifier;
use crate::class::Class;
use crate::environment::Environment;
use crate::object::Object;
use std::fmt;
use std::rc::Rc;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct Instance {
    pub class: Class,
    pub scope: Rc<Environment<Identifier, Object>>,
}

impl Instance {
    pub fn new(cls: &Class) -> Self {
        Self {
            class: cls.clone(),
            scope: Environment::new(),
        }
    }

    pub fn get(&self, id: &Identifier) -> Option<Object> {
        self.scope.clone().get(id)
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Instance({})", self.class)
    }
}

impl PartialEq for Instance {
    fn eq(&self, other: &Instance) -> bool {
        if self.class == other.class {
            true
        } else {
            false
        }
    }
}
