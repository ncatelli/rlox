use std::fmt;
mod interpreter;

use crate::parser::expression::Expr;
use interpreter::ExpressionInterpreter;

// Export Error and Result
pub use interpreter::ExprInterpreterErr;
pub use interpreter::InterpreterResult;

#[cfg(tests)]
mod tests;

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

/// Handles interpreting an arbitrarily nested Expr into a terminal literal as
/// represented by the PrimaryExpr type. This value is returned as an
/// InterpreterResult containing either an Ok(PrimaryExpr) or an Error.
pub fn interpret(expr: Expr) -> InterpreterResult {
    ExpressionInterpreter::new().interpret(expr)
}
