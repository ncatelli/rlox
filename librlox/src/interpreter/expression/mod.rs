mod interpreter;

use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::parser::expression::{Expr, PrimaryExpr};
use interpreter::ExpressionInterpreter;

// Export Error and Result
pub use interpreter::ExprInterpreterErr;

#[cfg(tests)]
mod tests;

pub type InterpreterResult = Result<(Environment, PrimaryExpr), ExprInterpreterErr>;

/// Handles interpreting an arbitrarily nested Expr into a terminal literal as
/// represented by the PrimaryExpr type. This value is returned as an
/// InterpreterResult containing either an Ok(PrimaryExpr) or an Error.
pub fn interpret(sym_tab: Environment, expr: Expr) -> InterpreterResult {
    match ExpressionInterpreter::new().interpret(expr) {
        Ok(v) => Ok((sym_tab, v)),
        Err(e) => Err(e),
    }
}
