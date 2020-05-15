mod interpreter;

use crate::interpreter::Interpreter;
use crate::parser::statement::Stmt;
use interpreter::StatementInterpreter;

// Export Error and Result
pub use interpreter::InterpreterResult;
pub use interpreter::StmtInterpreterErr;

#[cfg(tests)]
mod tests;

/// Handles interpreting an arbitrarily Statements
pub fn interpret(stmt: Stmt) -> InterpreterResult {
    StatementInterpreter::new().interpret(stmt)
}
