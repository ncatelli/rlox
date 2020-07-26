use crate::class::Class;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    class: Class,
}

impl Instance {
    pub fn new(cls: &Class) -> Self {
        Self { class: cls.clone() }
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Instance({})", self.class)
    }
}
