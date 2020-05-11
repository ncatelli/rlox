mod interpreter;

use crate::interpreter::Interpreter;
use crate::parser::expression::{Expr, PrimaryExpr};
use interpreter::ExpressionInterpreter;

pub use interpreter::ExprInterpreterErr;

#[cfg(tests)]
mod tests;

pub type InterpreterResult = Result<PrimaryExpr, ExprInterpreterErr>;

pub fn interpret(expr: Expr) -> InterpreterResult {
    ExpressionInterpreter::new().interpret(expr)
}
