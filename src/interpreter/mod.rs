use crate::ast::expression::{
    AdditionExpr, ComparisonExpr, EqualityExpr, Expr, LogicalExpr, MultiplicationExpr, UnaryExpr,
};
use crate::ast::token;
use crate::environment::Environment;
use crate::object::{Literal, Object};
use std::fmt;
use std::rc::Rc;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug)]
pub enum InterpreterErr {
    TypeErr(String),
}

impl fmt::Display for InterpreterErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TypeErr(e) => write!(f, "invalid type: {}", e),
        }
    }
}

pub trait Interpreter<A, B> {
    type Error;

    fn interpret(&self, input: A) -> Result<B, Self::Error>;
}

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
    UndefinedVariable(String),
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
            Self::UndefinedVariable(id) => write!(f, "undefined symbol: {}", id),
        }
    }
}

pub type ExprInterpreterResult = Result<Object, ExprInterpreterErr>;

#[derive(Default)]
pub struct StatefulInterpreter {
    pub env: Rc<Environment>,
}

impl StatefulInterpreter {
    pub fn new() -> StatefulInterpreter {
        StatefulInterpreter {
            env: Environment::new(),
        }
    }

    pub fn from(env: Rc<Environment>) -> StatefulInterpreter {
        StatefulInterpreter { env }
    }
}

/// Interpreter<Expr, object::Object> begins implemententing the required state
/// for interpreting Expressions in a stateful way.
impl Interpreter<Expr, Object> for StatefulInterpreter {
    type Error = ExprInterpreterErr;

    fn interpret(&self, expr: Expr) -> ExprInterpreterResult {
        match expr {
            Expr::Grouping(expr) => self.interpret(expr),
            Expr::Variable(id) => self.interpret_variable(id),
            Expr::Primary(obj) => self.interpret_primary(obj),
            Expr::Call(callee, args) => self.interpret_call(*callee, args),
            Expr::Unary(expr) => self.interpret_unary(expr),
            Expr::Multiplication(expr) => self.interpret_multiplication(expr),
            Expr::Addition(expr) => self.interpret_addition(expr),
            Expr::Comparison(expr) => self.interpret_comparison(expr),
            Expr::Equality(expr) => self.interpret_equality(expr),
            Expr::Logical(expr) => self.interpret_logical(expr),
            Expr::Assignment(id, expr) => self.interpret_assignment(id, expr),
        }
    }
}

/// This functions only to unpack an Expr and dispatch to the upstream
/// Interpreter<Expr, object::Object> implementation.
impl Interpreter<Box<Expr>, Object> for StatefulInterpreter {
    type Error = ExprInterpreterErr;
    fn interpret(&self, expr: Box<Expr>) -> ExprInterpreterResult {
        self.interpret(*expr)
    }
}

impl StatefulInterpreter {
    fn interpret_assignment(&self, id: token::Token, expr: Box<Expr>) -> ExprInterpreterResult {
        let lhv = id.lexeme.unwrap();
        let rhv = self.interpret(expr)?;

        match self.env.assign(&lhv, rhv) {
            Some(v) => Ok(v),
            None => Err(ExprInterpreterErr::UndefinedVariable(lhv.to_string())),
        }
    }

    fn interpret_logical(&self, expr: LogicalExpr) -> ExprInterpreterResult {
        match expr {
            LogicalExpr::Or(left, right) => {
                let lho: Object = self.interpret(left)?;
                let lho_bool: bool = lho.clone().into();
                if lho_bool {
                    Ok(lho)
                } else {
                    self.interpret(right)
                }
            }
            LogicalExpr::And(left, right) => {
                let lho: Object = self.interpret(left)?;
                let lho_bool: bool = lho.clone().into();
                if !lho_bool {
                    Ok(lho)
                } else {
                    self.interpret(right)
                }
            }
        }
    }

    fn interpret_equality(&self, expr: EqualityExpr) -> ExprInterpreterResult {
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

    fn interpret_comparison(&self, expr: ComparisonExpr) -> ExprInterpreterResult {
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

    fn interpret_addition(&self, expr: AdditionExpr) -> ExprInterpreterResult {
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

    fn interpret_multiplication(&self, expr: MultiplicationExpr) -> ExprInterpreterResult {
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

    fn interpret_unary(&self, expr: UnaryExpr) -> ExprInterpreterResult {
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

    fn interpret_call(&self, _callee: Expr, _args: Vec<Expr>) -> ExprInterpreterResult {
        /*let fun = self.interpret(callee).map(|obj_res| obj_res)?;
        let callable = Callable::new(fun);
        let _params: Vec<Object> = args
            .into_iter()
            .map(|expr| self.interpret(expr).unwrap())
            .collect();
        */
        todo!()
    }

    fn interpret_primary(&self, obj: Object) -> ExprInterpreterResult {
        Ok(obj)
    }

    fn interpret_variable(&self, identifier: token::Token) -> ExprInterpreterResult {
        let var = identifier.lexeme.unwrap();

        match self.env.get(&var) {
            Some(v) => Ok(v),
            None => Err(ExprInterpreterErr::UndefinedVariable(var.clone())),
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

impl Interpreter<Vec<Stmt>, ()> for StatefulInterpreter {
    type Error = StmtInterpreterErr;

    fn interpret(&self, input: Vec<Stmt>) -> StmtInterpreterResult {
        for stmt in input {
            match self.interpret(stmt) {
                Ok(()) => continue,
                Err(e) => return Err(e),
            };
        }
        Ok(())
    }
}

impl Interpreter<Stmt, ()> for StatefulInterpreter {
    type Error = StmtInterpreterErr;

    fn interpret(&self, input: Stmt) -> StmtInterpreterResult {
        match input {
            Stmt::Expression(expr) => self.interpret_expression_stmt(expr),
            Stmt::If(expr, tb, eb) => self.interpret_if_stmt(expr, tb, eb),
            Stmt::While(cond, body) => self.interpret_while_stmt(cond, body),
            Stmt::Print(expr) => self.interpret_print_stmt(expr),
            Stmt::Declaration(name, expr) => self.interpret_declaration_stmt(name, expr),
            Stmt::Block(stmts) => self.interpret_block(stmts),
        }
    }
}

/// This functions only to unpack an Stmt and dispatch to the upstream Interpreter<Stmt, ())> implementation
impl Interpreter<Box<Stmt>, ()> for StatefulInterpreter {
    type Error = StmtInterpreterErr;
    fn interpret(&self, input: Box<Stmt>) -> StmtInterpreterResult {
        self.interpret(*input)
    }
}

impl StatefulInterpreter {
    fn interpret_expression_stmt(&self, expr: Expr) -> StmtInterpreterResult {
        match self.interpret(expr) {
            Ok(_) => Ok(()),
            Err(err) => Err(StmtInterpreterErr::Expression(err)),
        }
    }

    fn interpret_print_stmt(&self, expr: Expr) -> StmtInterpreterResult {
        match self.interpret(expr) {
            Ok(expr) => {
                println!("{}", expr);
                Ok(())
            }
            Err(err) => Err(StmtInterpreterErr::Expression(err)),
        }
    }

    fn interpret_declaration_stmt(&self, name: String, expr: Expr) -> StmtInterpreterResult {
        match self.interpret(expr) {
            Ok(obj) => {
                self.env.define(&name, obj);
                Ok(())
            }
            Err(e) => Err(StmtInterpreterErr::Expression(e)),
        }
    }

    fn interpret_block(&self, stmts: Vec<Stmt>) -> StmtInterpreterResult {
        let block_interpreter = StatefulInterpreter::from(Environment::from(&self.env));
        block_interpreter.interpret(stmts)
    }

    #[allow(clippy::redundant_closure)]
    fn interpret_if_stmt(
        &self,
        cond: Expr,
        tb: Box<Stmt>,
        eb: Option<Box<Stmt>>,
    ) -> StmtInterpreterResult {
        let condition = self
            .interpret(cond)
            .map_err(|e| StmtInterpreterErr::Expression(e))?;
        match (condition.into(), eb) {
            (true, _) => self.interpret(tb),
            (false, None) => Ok(()),
            (false, Some(stmt)) => self.interpret(stmt),
        }
    }

    #[allow(clippy::redundant_closure)]
    fn interpret_while_stmt(&self, cond: Expr, body: Box<Stmt>) -> StmtInterpreterResult {
        while self
            .interpret(cond.clone())
            .map_err(|e| StmtInterpreterErr::Expression(e))?
            .into()
        {
            if let Err(e) = self.interpret(body.clone()) {
                return Err(e);
            }
        }

        Ok(())
    }
}
