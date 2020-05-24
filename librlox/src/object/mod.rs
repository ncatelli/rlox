use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Literal(Literal),
    Identifier(String),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Literal(l) => write!(f, "{}", &l),
            Object::Identifier(i) => write!(f, "{}", &i),
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
