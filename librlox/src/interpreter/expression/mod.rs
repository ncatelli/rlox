mod interpreter;

use crate::interpreter::Interpreter;
use crate::parser::expression::Expr;
use interpreter::ExpressionInterpreter;

// Export Error and Result
pub use interpreter::ExprInterpreterErr;
pub use interpreter::InterpreterResult;

#[cfg(tests)]
mod tests;

/// Handles interpreting an arbitrarily nested Expr into a terminal literal as
/// represented by the PrimaryExpr type. This value is returned as an
/// InterpreterResult containing either an Ok(PrimaryExpr) or an Error.
pub fn interpret(expr: Expr) -> InterpreterResult {
    ExpressionInterpreter::new().interpret(expr)
}
