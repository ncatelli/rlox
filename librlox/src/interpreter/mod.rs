use std::fmt;
mod interpreter;

//use crate::parser::expression::Expr;
use crate::parser::statement::Stmt;
use interpreter::StatefulInterpreter;

// Export Error and Result
pub use interpreter::ExprInterpreterErr;
pub use interpreter::StmtInterpreterResult;

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

pub trait InterpreterMut<A, B> {
    type Error;

    fn interpret(&mut self, input: A) -> Result<B, Self::Error>;
}

/// Handles interpreting an arbitrarily Statements
pub fn interpret(statements: Vec<Stmt>) -> StmtInterpreterResult {
    let mut interpreter = StatefulInterpreter::new();

    for stmt in statements {
        match interpreter.interpret(stmt) {
            Ok(()) => continue,
            Err(e) => return Err(e),
        };
    }

    Ok(())
}
