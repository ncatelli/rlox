use crate::class::Class;
use crate::functions::Callable;
use std::fmt;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Literal(Literal),
    Call(Box<Callable>),
    Class(Class),
}

impl Into<bool> for Object {
    fn into(self) -> bool {
        match self {
            Self::Literal(l) => l.into(),
            Self::Call(_) => true,
            Self::Class(_) => true,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(l) => write!(f, "{}", &l),
            Self::Call(c) => write!(f, "{:?}", &c),
            Self::Class(ref c) => write!(f, "{}", c),
        }
    }
}

/// Literal functions to encapsulate values to be embedded in their
/// corresponding
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Nil,
    Bool(bool),
    Str(String),
    Number(f64),
}

impl std::convert::From<bool> for Literal {
    fn from(b: bool) -> Self {
        Literal::Bool(b)
    }
}

impl std::convert::From<String> for Literal {
    fn from(s: String) -> Self {
        Literal::Str(s)
    }
}

impl std::convert::From<f64> for Literal {
    fn from(f: f64) -> Self {
        Literal::Number(f)
    }
}

impl Into<bool> for Literal {
    fn into(self) -> bool {
        match self {
            Self::Nil => false,
            Self::Bool(b) => b,
            Self::Str(s) => !s.is_empty(),
            Self::Number(n) => n.abs() > std::f64::EPSILON,
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Nil => write!(f, "nil"),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Str(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{}", n),
        }
    }
}

#[allow(unused_macros)]
macro_rules! obj_class {
    ($c:expr) => {
        $crate::object::Object::Class($c)
    };
}

#[allow(unused_macros)]
macro_rules! obj_call {
    ($c:expr) => {
        $crate::object::Object::Call($c)
    };
}

#[allow(unused_macros)]
macro_rules! obj_number {
    ($n:expr) => {
        $crate::object::Object::Literal($crate::object::Literal::Number($n))
    };
}

#[allow(unused_macros)]
macro_rules! obj_str {
    ($s:expr) => {
        $crate::object::Object::Literal($crate::object::Literal::Str($s))
    };
}

#[allow(unused_macros)]
macro_rules! obj_bool {
    ($b:expr) => {
        $crate::object::Object::Literal($crate::object::Literal::Bool($b))
    };
}

#[allow(unused_macros)]
macro_rules! obj_nil {
    () => {
        $crate::object::Object::Literal($crate::object::Literal::Nil)
    };
}
