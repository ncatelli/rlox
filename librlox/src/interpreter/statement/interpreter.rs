use crate::interpreter::expression;
use crate::interpreter::Interpreter;
use crate::parser::expression::Expr;
use crate::parser::statement::Stmt;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum StmtInterpreterErr {
    Unspecified,
    Expression(expression::ExprInterpreterErr),
}

impl fmt::Display for StmtInterpreterErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unspecified => write!(f, "unspecified statement error"),
            Self::Expression(e) => write!(f, "Expression Error: {}", e),
        }
    }
}

pub type InterpreterResult = Result<(), StmtInterpreterErr>;

#[derive(Default)]
pub struct StatementInterpreter {}

impl Interpreter<Stmt, ()> for StatementInterpreter {
    type Error = StmtInterpreterErr;

    fn interpret(&self, stmt: Stmt) -> InterpreterResult {
        match stmt {
            Stmt::Expression(expr) => self.interpret_expression_stmt(expr),
            Stmt::Print(expr) => self.interpret_print_stmt(expr),
        }
    }
}

/// This functions only to unpack an Stmt and dispatch to the upstream Interpreter<Stmt, ())> implementation
impl Interpreter<Box<Stmt>, ()> for StatementInterpreter {
    type Error = StmtInterpreterErr;
    fn interpret(&self, stmt: Box<Stmt>) -> InterpreterResult {
        self.interpret(*stmt)
    }
}

impl StatementInterpreter {
    pub fn new() -> StatementInterpreter {
        StatementInterpreter::default()
    }

    fn interpret_expression_stmt(&self, expr: Expr) -> InterpreterResult {
        match expression::interpret(expr) {
            Ok(_) => Ok(()),
            Err(err) => Err(StmtInterpreterErr::Expression(err)),
        }
    }

    fn interpret_print_stmt(&self, expr: Expr) -> InterpreterResult {
        match expression::interpret(expr) {
            Ok(expr) => {
                println!("{}", expr);
                Ok(())
            }
            Err(err) => Err(StmtInterpreterErr::Expression(err)),
        }
    }
}
