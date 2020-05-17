mod interpreter;

use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::parser::statement::Stmt;
use interpreter::StatementInterpreter;

// Export Error and Result
pub use interpreter::InterpreterResult;
pub use interpreter::StmtInterpreterErr;

#[cfg(tests)]
mod tests;

/// Handles interpreting an arbitrarily Statements
pub fn interpret(sym_tab: Environment, statements: Vec<Stmt>) -> InterpreterResult {
    let interpreter = StatementInterpreter::new();
    let mut st = sym_tab;

    for stmt in statements {
        match interpreter.interpret((st, stmt)) {
            Ok(s) => {
                st = s;
                continue;
            }
            Err(e) => return Err(e),
        };
    }

    Ok(st)
}
