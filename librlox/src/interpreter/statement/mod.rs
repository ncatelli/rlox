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
pub fn interpret(statements: Vec<Stmt>) -> InterpreterResult {
    let interpreter = StatementInterpreter::new();

    for stmt in statements {
        match interpreter.interpret(stmt) {
            Ok(_) => continue,
            Err(e) => return Err(e),
        };
    }

    Ok(())
}
