use crate::ast::expression::{
    AdditionExpr, ComparisonExpr, EqualityExpr, Expr, MultiplicationExpr, UnaryExpr,
};
use crate::ast::token;
use crate::environment::Environment;
use crate::interpreter::InterpreterMut;
use crate::object::{Literal, Object};
use std::fmt;

macro_rules! type_error {
    () => {
        Err(ExprInterpreterErr::Unspecified)
    };
    ($e:expr) => {
        Err(ExprInterpreterErr::Type($e))
    };
    ($left:expr, $op:literal, $right:expr) => {
        Err(ExprInterpreterErr::BinaryExpr($op, $left, $right))
    };
}

#[derive(PartialEq, Debug)]
pub enum ExprInterpreterErr {
    Unspecified,
    Type(&'static str),
    BinaryExpr(&'static str, Object, Object),
    Lookup(String),
}

impl fmt::Display for ExprInterpreterErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unspecified => write!(f, "unspecified expression error"),
            Self::Type(e) => write!(f, "invalid type: {}", e),
            Self::BinaryExpr(op, left, right) => write!(
                f,
                "invalid operand for operators: {} {} {}",
                left, op, right
            ),
            Self::Lookup(id) => write!(f, "undefined symbol: {}", id),
        }
    }
}

pub type ExprInterpreterResult = Result<Object, ExprInterpreterErr>;

pub struct StatefulInterpreter {
    pub globals: Environment,
}

impl StatefulInterpreter {
    pub fn new() -> StatefulInterpreter {
        StatefulInterpreter {
            globals: Environment::new(),
        }
    }
}

/// InterpreterMut<Expr, object::Object> begins implemententing the required state
/// for interpreting Expressions in a stateful way.
impl InterpreterMut<Expr, Object> for StatefulInterpreter {
    type Error = ExprInterpreterErr;

    fn interpret(&mut self, expr: Expr) -> ExprInterpreterResult {
        match expr {
            Expr::Grouping(expr) => self.interpret(expr),
            Expr::Variable(id) => self.interpret_variable(id),
            Expr::Primary(obj) => self.interpret_primary(obj),
            Expr::Unary(expr) => self.interpret_unary(expr),
            Expr::Multiplication(expr) => self.interpret_multiplication(expr),
            Expr::Addition(expr) => self.interpret_addition(expr),
            Expr::Comparison(expr) => self.interpret_comparison(expr),
            Expr::Equality(expr) => self.interpret_equality(expr),
            Expr::Assignment(id, expr) => self.interpret_assignment(id, expr),
        }
    }
}

/// This functions only to unpack an Expr and dispatch to the upstream
/// Interpreter<Expr, object::Object> implementation.
impl InterpreterMut<Box<Expr>, Object> for StatefulInterpreter {
    type Error = ExprInterpreterErr;
    fn interpret(&mut self, expr: Box<Expr>) -> ExprInterpreterResult {
        self.interpret(*expr)
    }
}

impl StatefulInterpreter {
    fn interpret_assignment(&mut self, id: token::Token, expr: Box<Expr>) -> ExprInterpreterResult {
        let _lhv = id.lexeme.unwrap();
        let _rhv = self.interpret(expr)?;

        todo!()
    }

    fn interpret_equality(&mut self, expr: EqualityExpr) -> ExprInterpreterResult {
        match expr {
            EqualityExpr::Equal(left, right) => match (self.interpret(left), self.interpret(right))
            {
                (
                    Ok(Object::Literal(Literal::Number(l_val))),
                    Ok(Object::Literal(Literal::Number(r_val))),
                ) => Ok(obj_bool!((l_val - r_val).abs() < std::f64::EPSILON)),
                (
                    Ok(Object::Literal(Literal::Str(l_val))),
                    Ok(Object::Literal(Literal::Str(r_val))),
                ) => Ok(obj_bool!(l_val == r_val)),
                (Ok(l), Ok(r)) => type_error!(l, "==", r),
                _ => type_error!(),
            },
            EqualityExpr::NotEqual(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (
                        Ok(Object::Literal(Literal::Number(l_val))),
                        Ok(Object::Literal(Literal::Number(r_val))),
                    ) => Ok(obj_bool!((l_val - r_val).abs() > std::f64::EPSILON)),
                    (
                        Ok(Object::Literal(Literal::Str(l_val))),
                        Ok(Object::Literal(Literal::Str(r_val))),
                    ) => Ok(obj_bool!(l_val != r_val)),
                    (Ok(l), Ok(r)) => type_error!(l, "!=", r),
                    _ => type_error!(),
                }
            }
        }
    }

    fn interpret_comparison(&mut self, expr: ComparisonExpr) -> ExprInterpreterResult {
        match expr {
            ComparisonExpr::Less(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (
                        Ok(Object::Literal(Literal::Number(l_val))),
                        Ok(Object::Literal(Literal::Number(r_val))),
                    ) => Ok(obj_bool!(l_val < r_val)),
                    (Ok(l), Ok(r)) => type_error!(l, "<", r),
                    _ => type_error!(),
                }
            }
            ComparisonExpr::LessEqual(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (
                        Ok(Object::Literal(Literal::Number(l_val))),
                        Ok(Object::Literal(Literal::Number(r_val))),
                    ) => Ok(obj_bool!(l_val <= r_val)),
                    (Ok(l), Ok(r)) => type_error!(l, "<=", r),
                    _ => type_error!(),
                }
            }
            ComparisonExpr::Greater(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (
                        Ok(Object::Literal(Literal::Number(l_val))),
                        Ok(Object::Literal(Literal::Number(r_val))),
                    ) => Ok(obj_bool!(l_val > r_val)),
                    (Ok(l), Ok(r)) => type_error!(l, ">", r),
                    _ => type_error!(),
                }
            }
            ComparisonExpr::GreaterEqual(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (
                        Ok(Object::Literal(Literal::Number(l_val))),
                        Ok(Object::Literal(Literal::Number(r_val))),
                    ) => Ok(obj_bool!(l_val >= r_val)),
                    (Ok(l), Ok(r)) => type_error!(l, ">=", r),
                    _ => type_error!(),
                }
            }
        }
    }

    fn interpret_addition(&mut self, expr: AdditionExpr) -> ExprInterpreterResult {
        match expr {
            AdditionExpr::Add(left, right) => match (self.interpret(left), self.interpret(right)) {
                (
                    Ok(Object::Literal(Literal::Number(l_val))),
                    Ok(Object::Literal(Literal::Number(r_val))),
                ) => Ok(obj_number!(l_val + r_val)),
                (
                    Ok(Object::Literal(Literal::Str(l_val))),
                    Ok(Object::Literal(Literal::Str(r_val))),
                ) => Ok(obj_str!(format!("{}{}", l_val, r_val))),
                (Ok(l), Ok(r)) => type_error!(l, "+", r),
                _ => type_error!(),
            },
            AdditionExpr::Subtract(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (
                        Ok(Object::Literal(Literal::Number(l_val))),
                        Ok(Object::Literal(Literal::Number(r_val))),
                    ) => Ok(obj_number!(l_val - r_val)),
                    (Ok(l), Ok(r)) => type_error!(l, "-", r),
                    _ => type_error!(),
                }
            }
        }
    }

    fn interpret_multiplication(&mut self, expr: MultiplicationExpr) -> ExprInterpreterResult {
        match expr {
            MultiplicationExpr::Multiply(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (
                        Ok(Object::Literal(Literal::Number(l_val))),
                        Ok(Object::Literal(Literal::Number(r_val))),
                    ) => Ok(obj_number!(l_val * r_val)),
                    (Ok(l), Ok(r)) => type_error!(l, "*", r),
                    _ => type_error!(),
                }
            }
            MultiplicationExpr::Divide(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (
                        Ok(Object::Literal(Literal::Number(l_val))),
                        Ok(Object::Literal(Literal::Number(r_val))),
                    ) => Ok(obj_number!(l_val / r_val)),
                    (Ok(l), Ok(r)) => type_error!(l, "/", r),
                    _ => type_error!(),
                }
            }
        }
    }

    fn interpret_unary(&mut self, expr: UnaryExpr) -> ExprInterpreterResult {
        match expr {
            UnaryExpr::Bang(ue) => match self.interpret(ue) {
                Ok(obj) => {
                    let ob: bool = obj.into();
                    Ok(obj_bool!(!ob))
                }
                e @ Err(_) => e,
            },
            UnaryExpr::Minus(ue) => match self.interpret(ue) {
                Ok(Object::Literal(Literal::Number(n))) => Ok(obj_number!(n * -1.0)),
                e @ Err(_) => e,
                _ => type_error!(),
            },
        }
    }

    fn interpret_primary(&mut self, obj: Object) -> ExprInterpreterResult {
        Ok(obj)
    }

    fn interpret_variable(&mut self, identifier: token::Token) -> ExprInterpreterResult {
        let var = identifier.lexeme.unwrap();

        match self.globals.get(&var) {
            Some(v) => Ok(v.to_owned()),
            None => Err(ExprInterpreterErr::Lookup(var.clone())),
        }
    }
}

use crate::ast::statement::Stmt;

#[derive(PartialEq, Debug)]
pub enum StmtInterpreterErr {
    Unspecified,
    Expression(ExprInterpreterErr),
}

impl fmt::Display for StmtInterpreterErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unspecified => write!(f, "unspecified statement error"),
            Self::Expression(e) => write!(f, "Expression Error: {}", e),
        }
    }
}

pub type StmtInterpreterResult = Result<(), StmtInterpreterErr>;

impl InterpreterMut<Vec<Stmt>, ()> for StatefulInterpreter {
    type Error = StmtInterpreterErr;

    fn interpret(&mut self, input: Vec<Stmt>) -> StmtInterpreterResult {
        for stmt in input {
            match self.interpret(stmt) {
                Ok(()) => continue,
                Err(e) => return Err(e),
            };
        }
        Ok(())
    }
}

impl InterpreterMut<Stmt, ()> for StatefulInterpreter {
    type Error = StmtInterpreterErr;

    fn interpret(&mut self, input: Stmt) -> StmtInterpreterResult {
        match input {
            Stmt::Expression(expr) => self.interpret_expression_stmt(expr),
            Stmt::Print(expr) => self.interpret_print_stmt(expr),
            Stmt::Declaration(name, expr) => self.interpret_declaration_stmt(name, expr),
        }
    }
}

/// This functions only to unpack an Stmt and dispatch to the upstream Interpreter<Stmt, ())> implementation
impl InterpreterMut<Box<Stmt>, ()> for StatefulInterpreter {
    type Error = StmtInterpreterErr;
    fn interpret(&mut self, input: Box<Stmt>) -> StmtInterpreterResult {
        self.interpret(*input)
    }
}

impl StatefulInterpreter {
    fn interpret_expression_stmt(&mut self, expr: Expr) -> StmtInterpreterResult {
        match self.interpret(expr) {
            Ok(_) => Ok(()),
            Err(err) => Err(StmtInterpreterErr::Expression(err)),
        }
    }

    fn interpret_print_stmt(&mut self, expr: Expr) -> StmtInterpreterResult {
        match self.interpret(expr) {
            Ok(expr) => {
                println!("{}", expr);
                Ok(())
            }
            Err(err) => Err(StmtInterpreterErr::Expression(err)),
        }
    }

    fn interpret_declaration_stmt(&mut self, name: String, expr: Expr) -> StmtInterpreterResult {
        match self.interpret(expr) {
            Ok(obj) => {
                self.globals.define(name, obj);
                Ok(())
            }
            Err(e) => Err(StmtInterpreterErr::Expression(e)),
        }
    }
}
