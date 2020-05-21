use std::fmt;
mod interpreter;

#[cfg(test)]
mod tests;

//use crate::parser::expression::Expr;
pub use interpreter::StatefulInterpreter;

// Export Error and Result
pub use interpreter::ExprInterpreterErr;
pub use interpreter::StmtInterpreterResult;

#[derive(PartialEq, Debug)]
pub enum InterpreterErr {
    TypeErr(String),
}

impl fmt::Display for InterpreterErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TypeErr(e) => write!(f, "invalid type: {}", e),
        }
    }
}

pub trait Interpreter<A, B> {
    type Error;

    fn interpret(&self, input: A) -> Result<B, Self::Error>;
}

pub trait InterpreterMut<A, B> {
    type Error;

    fn interpret(&mut self, input: A) -> Result<B, Self::Error>;
}
