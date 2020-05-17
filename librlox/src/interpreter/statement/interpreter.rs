use crate::environment::Environment;
use crate::interpreter::expression;
use crate::interpreter::Interpreter;
use crate::parser::expression::{Expr, Identifier};
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

pub type InterpreterInput = (Environment, Stmt);
pub type InterpreterResult = Result<Environment, StmtInterpreterErr>;

#[derive(Default)]
pub struct StatementInterpreter {}

impl Interpreter<InterpreterInput, Environment> for StatementInterpreter {
    type Error = StmtInterpreterErr;

    fn interpret(&self, input: InterpreterInput) -> InterpreterResult {
        let (sym_tab, stmt) = input;
        match stmt {
            Stmt::Expression(expr) => self.interpret_expression_stmt(sym_tab, expr),
            Stmt::Print(expr) => self.interpret_print_stmt(sym_tab, expr),
            Stmt::Declaration(name, expr) => self.interpret_declaration_stmt(sym_tab, name, expr),
        }
    }
}

/// This functions only to unpack an Stmt and dispatch to the upstream Interpreter<Stmt, ())> implementation
impl Interpreter<Box<InterpreterInput>, Environment> for StatementInterpreter {
    type Error = StmtInterpreterErr;
    fn interpret(&self, input: Box<InterpreterInput>) -> InterpreterResult {
        let (sym_tab, stmt) = *input;
        self.interpret((sym_tab, stmt))
    }
}

impl StatementInterpreter {
    pub fn new() -> StatementInterpreter {
        StatementInterpreter::default()
    }

    fn interpret_expression_stmt(&self, sym_tab: Environment, expr: Expr) -> InterpreterResult {
        match expression::interpret(sym_tab, expr) {
            Ok((st, _)) => Ok(st),
            Err(err) => Err(StmtInterpreterErr::Expression(err)),
        }
    }

    fn interpret_print_stmt(&self, sym_tab: Environment, expr: Expr) -> InterpreterResult {
        match expression::interpret(sym_tab, expr) {
            Ok((st, expr)) => {
                println!("{}", expr);
                Ok(st)
            }
            Err(err) => Err(StmtInterpreterErr::Expression(err)),
        }
    }

    fn interpret_declaration_stmt(
        &self,
        sym_tab: Environment,
        name: Identifier,
        expr: Expr,
    ) -> InterpreterResult {
        match expression::interpret(sym_tab, expr) {
            Ok((mut st, expr)) => {
                st.define(name, Expr::Primary(expr));
                println!("state: {:?}", &st);
                Ok(st)
            }
            Err(err) => Err(StmtInterpreterErr::Expression(err)),
        }
    }
}
