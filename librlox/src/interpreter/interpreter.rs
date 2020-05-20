use crate::environment::Environment;
use crate::interpreter::InterpreterMut;
use crate::parser::expression::{
    AdditionExpr, ComparisonExpr, EqualityExpr, Expr, MultiplicationExpr, PrimaryExpr, UnaryExpr,
};
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
    BinaryExpr(&'static str, PrimaryExpr, PrimaryExpr),
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
        }
    }
}

pub type ExprInterpreterResult = Result<PrimaryExpr, ExprInterpreterErr>;

pub struct StatefulInterpreter {
    globals: Environment,
}

impl StatefulInterpreter {
    pub fn new() -> StatefulInterpreter {
        StatefulInterpreter {
            globals: Environment::new(),
        }
    }
}

/// InterpreterMut<Expr, PrimaryExpr> begins implemententing the required state
/// for interpreting Expressions in a stateful way.
impl InterpreterMut<Expr, PrimaryExpr> for StatefulInterpreter {
    type Error = ExprInterpreterErr;

    fn interpret(&mut self, expr: Expr) -> ExprInterpreterResult {
        match expr {
            Expr::Grouping(expr) => self.interpret(expr),
            Expr::Primary(expr) => self.interpret_primary(expr),
            Expr::Unary(expr) => self.interpret_unary(expr),
            Expr::Multiplication(expr) => self.interpret_multiplication(expr),
            Expr::Addition(expr) => self.interpret_addition(expr),
            Expr::Comparison(expr) => self.interpret_comparison(expr),
            Expr::Equality(expr) => self.interpret_equality(expr),
        }
    }
}

/// This functions only to unpack an Expr and dispatch to the upstream Interpreter<Expr, PrimaryExpr> implementation
impl InterpreterMut<Box<Expr>, PrimaryExpr> for StatefulInterpreter {
    type Error = ExprInterpreterErr;
    fn interpret(&mut self, expr: Box<Expr>) -> ExprInterpreterResult {
        self.interpret(*expr)
    }
}

impl StatefulInterpreter {
    fn interpret_equality(&mut self, expr: EqualityExpr) -> ExprInterpreterResult {
        match expr {
            EqualityExpr::Equal(left, right) => match (self.interpret(left), self.interpret(right))
            {
                (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => Ok(
                    PrimaryExpr::from((left_val - right_val).abs() < std::f64::EPSILON),
                ),
                (Ok(PrimaryExpr::Str(left_val)), Ok(PrimaryExpr::Str(right_val))) => {
                    Ok(PrimaryExpr::from(left_val == right_val))
                }
                (Ok(l), Ok(r)) => type_error!(l, "==", r),
                _ => type_error!(),
            },
            EqualityExpr::NotEqual(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => Ok(
                        PrimaryExpr::from((left_val - right_val).abs() > std::f64::EPSILON),
                    ),
                    (Ok(PrimaryExpr::Str(left_val)), Ok(PrimaryExpr::Str(right_val))) => {
                        Ok(PrimaryExpr::from(left_val != right_val))
                    }
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
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(PrimaryExpr::from(left_val < right_val))
                    }
                    (Ok(l), Ok(r)) => type_error!(l, "<", r),
                    _ => type_error!(),
                }
            }
            ComparisonExpr::LessEqual(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(PrimaryExpr::from(left_val <= right_val))
                    }
                    (Ok(l), Ok(r)) => type_error!(l, "<=", r),
                    _ => type_error!(),
                }
            }
            ComparisonExpr::Greater(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(PrimaryExpr::from(left_val > right_val))
                    }
                    (Ok(l), Ok(r)) => type_error!(l, ">", r),
                    _ => type_error!(),
                }
            }
            ComparisonExpr::GreaterEqual(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(PrimaryExpr::from(left_val >= right_val))
                    }
                    (Ok(l), Ok(r)) => type_error!(l, ">=", r),
                    _ => type_error!(),
                }
            }
        }
    }

    fn interpret_addition(&mut self, expr: AdditionExpr) -> ExprInterpreterResult {
        match expr {
            AdditionExpr::Add(left, right) => match (self.interpret(left), self.interpret(right)) {
                (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                    Ok(PrimaryExpr::Number(left_val + right_val))
                }
                (Ok(PrimaryExpr::Str(left_val)), Ok(PrimaryExpr::Str(right_val))) => {
                    Ok(PrimaryExpr::Str(format!("{}{}", left_val, right_val)))
                }
                (Ok(l), Ok(r)) => type_error!(l, "+", r),
                _ => type_error!(),
            },
            AdditionExpr::Subtract(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(PrimaryExpr::Number(left_val - right_val))
                    }
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
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(PrimaryExpr::Number(left_val * right_val))
                    }
                    (Ok(l), Ok(r)) => type_error!(l, "*", r),
                    _ => type_error!(),
                }
            }
            MultiplicationExpr::Divide(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(PrimaryExpr::Number(left_val / right_val))
                    }
                    (Ok(l), Ok(r)) => type_error!(l, "/", r),
                    _ => type_error!(),
                }
            }
        }
    }

    fn interpret_unary(&mut self, expr: UnaryExpr) -> ExprInterpreterResult {
        match expr {
            UnaryExpr::Bang(ue) => match self.interpret(ue) {
                Ok(expr) => Ok(PrimaryExpr::from(!is_true(expr))),
                e @ Err(_) => e,
            },
            UnaryExpr::Minus(ue) => match self.interpret(ue) {
                Ok(PrimaryExpr::Number(v)) => Ok(PrimaryExpr::Number(v * -1.0)),
                e @ Err(_) => e,
                _ => type_error!(),
            },
        }
    }

    fn interpret_primary(&self, expr: PrimaryExpr) -> ExprInterpreterResult {
        Ok(expr)
    }
}

fn is_true(expr: PrimaryExpr) -> bool {
    match expr {
        PrimaryExpr::Nil => false,
        PrimaryExpr::False => false,
        _ => true,
    }
}

use crate::parser::statement::Stmt;

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

impl InterpreterMut<Stmt, ()> for StatefulInterpreter {
    type Error = StmtInterpreterErr;

    fn interpret(&mut self, input: Stmt) -> StmtInterpreterResult {
        match input {
            Stmt::Expression(expr) => self.interpret_expression_stmt(expr),
            Stmt::Print(expr) => self.interpret_print_stmt(expr),
            Stmt::Declaration(_, _) => todo!(),
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
}
