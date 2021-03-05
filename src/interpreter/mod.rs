use crate::ast::expression::{
    AdditionExpr, ComparisonExpr, EqualityExpr, Expr, LogicalExpr, MultiplicationExpr, UnaryExpr,
};
use crate::ast::identifier::Identifier;
use crate::class;
use crate::environment;
use crate::environment::Environment;
use crate::functions;
use crate::object::{Literal, Object};
use crate::pass::*;
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
    UndefinedFunction,
    CallErr(String),
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
            Self::UndefinedFunction => write!(f, "undefined function"),
            Self::CallErr(o) => write!(f, "{}", o),
        }
    }
}

pub type ExprInterpreterResult = Result<Object, ExprInterpreterErr>;

pub struct StatefulInterpreter {
    pub env: Rc<Environment<Identifier, Object>>,
    pub flat_env: environment::flat_env::Environment<Object>,
}

impl StatefulInterpreter {
    pub fn new() -> StatefulInterpreter {
        StatefulInterpreter {
            env: Environment::new(),
            flat_env: environment::flat_env::Environment::new(),
        }
    }
}

impl Default for StatefulInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Rc<Environment<Identifier, Object>>> for StatefulInterpreter {
    fn from(env: Rc<Environment<Identifier, Object>>) -> StatefulInterpreter {
        let mut si = StatefulInterpreter::new();
        si.env = env;
        si
    }
}

/// Interpreter<Expr, object::Object> begins implemententing the required state
/// for interpreting Expressions in a stateful way.
impl Pass<Expr, Object> for StatefulInterpreter {
    type Error = ExprInterpreterErr;

    fn tree_pass(&self, expr: Expr) -> ExprInterpreterResult {
        match expr {
            Expr::Grouping(expr) => self.tree_pass(expr),
            Expr::Lambda(params, body) => self.interpret_lambda(params, *body),
            Expr::Variable(id) => self.interpret_variable(id),
            Expr::Primary(obj) => self.interpret_primary(obj),
            Expr::Call(callee, args) => self.interpret_call(*callee, args),
            Expr::Get(instance, param) => self.interpret_get(*instance, *param),
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
impl Pass<Box<Expr>, Object> for StatefulInterpreter {
    type Error = ExprInterpreterErr;
    fn tree_pass(&self, expr: Box<Expr>) -> ExprInterpreterResult {
        self.tree_pass(*expr)
    }
}

impl StatefulInterpreter {
    fn interpret_assignment(&self, id: Identifier, expr: Box<Expr>) -> ExprInterpreterResult {
        let lhv = id;
        let rhv = self.tree_pass(expr)?;

        match self.env.assign(&lhv, rhv) {
            Some(v) => Ok(v),
            None => Err(ExprInterpreterErr::UndefinedVariable(format!("{:?}", &lhv))),
        }
    }

    fn interpret_logical(&self, expr: LogicalExpr) -> ExprInterpreterResult {
        match expr {
            LogicalExpr::Or(left, right) => {
                let lho: Object = self.tree_pass(left)?;
                let lho_bool: bool = lho.clone().into();
                if lho_bool {
                    Ok(lho)
                } else {
                    self.tree_pass(right)
                }
            }
            LogicalExpr::And(left, right) => {
                let lho: Object = self.tree_pass(left)?;
                let lho_bool: bool = lho.clone().into();
                if !lho_bool {
                    Ok(lho)
                } else {
                    self.tree_pass(right)
                }
            }
        }
    }

    fn interpret_equality(&self, expr: EqualityExpr) -> ExprInterpreterResult {
        match expr {
            EqualityExpr::Equal(left, right) => match (self.tree_pass(left), self.tree_pass(right))
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
                match (self.tree_pass(left), self.tree_pass(right)) {
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
                match (self.tree_pass(left), self.tree_pass(right)) {
                    (
                        Ok(Object::Literal(Literal::Number(l_val))),
                        Ok(Object::Literal(Literal::Number(r_val))),
                    ) => Ok(obj_bool!(l_val < r_val)),
                    (Ok(l), Ok(r)) => type_error!(l, "<", r),
                    _ => type_error!(),
                }
            }
            ComparisonExpr::LessEqual(left, right) => {
                match (self.tree_pass(left), self.tree_pass(right)) {
                    (
                        Ok(Object::Literal(Literal::Number(l_val))),
                        Ok(Object::Literal(Literal::Number(r_val))),
                    ) => Ok(obj_bool!(l_val <= r_val)),
                    (Ok(l), Ok(r)) => type_error!(l, "<=", r),
                    _ => type_error!(),
                }
            }
            ComparisonExpr::Greater(left, right) => {
                match (self.tree_pass(left), self.tree_pass(right)) {
                    (
                        Ok(Object::Literal(Literal::Number(l_val))),
                        Ok(Object::Literal(Literal::Number(r_val))),
                    ) => Ok(obj_bool!(l_val > r_val)),
                    (Ok(l), Ok(r)) => type_error!(l, ">", r),
                    _ => type_error!(),
                }
            }
            ComparisonExpr::GreaterEqual(left, right) => {
                match (self.tree_pass(left), self.tree_pass(right)) {
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
            AdditionExpr::Add(left, right) => match (self.tree_pass(left), self.tree_pass(right)) {
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
                match (self.tree_pass(left), self.tree_pass(right)) {
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
                match (self.tree_pass(left), self.tree_pass(right)) {
                    (
                        Ok(Object::Literal(Literal::Number(l_val))),
                        Ok(Object::Literal(Literal::Number(r_val))),
                    ) => Ok(obj_number!(l_val * r_val)),
                    (Ok(l), Ok(r)) => type_error!(l, "*", r),
                    _ => type_error!(),
                }
            }
            MultiplicationExpr::Divide(left, right) => {
                match (self.tree_pass(left), self.tree_pass(right)) {
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
            UnaryExpr::Bang(ue) => match self.tree_pass(ue) {
                Ok(obj) => {
                    let ob: bool = obj.into();
                    Ok(obj_bool!(!ob))
                }
                e @ Err(_) => e,
            },
            UnaryExpr::Minus(ue) => match self.tree_pass(ue) {
                Ok(Object::Literal(Literal::Number(n))) => Ok(obj_number!(n * -1.0)),
                e @ Err(_) => e,
                _ => type_error!(),
            },
        }
    }

    fn interpret_call(&self, callee: Expr, args: Vec<Expr>) -> ExprInterpreterResult {
        let fun = self.tree_pass(callee).map(|obj_res| obj_res)?;
        let params: Vec<Object> = args
            .into_iter()
            .map(|expr| self.tree_pass(expr).unwrap())
            .collect();

        let c = match fun {
            Object::Call(c) => Ok(c),
            _ => Err(ExprInterpreterErr::CallErr(format!(
                "object {} is not callable",
                fun
            ))),
        }?;

        match c.call(self.env.clone(), params) {
            Ok(r) => Ok(r),
            Err(e) => Err(ExprInterpreterErr::CallErr(format!("{:?}", e))),
        }
    }

    fn interpret_get(&self, instance: Expr, param: Expr) -> ExprInterpreterResult {
        let i = self.tree_pass(instance).map(|obj_res| match obj_res {
            Object::Instance(i) => Ok(i),
            _ => Err(ExprInterpreterErr::CallErr(format!(
                "object {} is not callable",
                obj_res
            ))),
        })??;

        let param_id = if let Expr::Variable(id) = param {
            Ok(id)
        } else {
            Err(ExprInterpreterErr::Type(
                "Expected identifier for parameter",
            ))
        }?;

        i.get(&param_id).map_or(
            Err(ExprInterpreterErr::UndefinedVariable(param_id.to_string())),
            |obj| Ok(obj),
        )
    }

    fn interpret_lambda(&self, params: Vec<Identifier>, body: Stmt) -> ExprInterpreterResult {
        let func = functions::Function::new(self.env.clone(), params, body);
        let callable = functions::Callable::Func(func);
        let obj = Object::Call(Box::new(callable));

        Ok(obj)
    }

    fn interpret_primary(&self, obj: Object) -> ExprInterpreterResult {
        Ok(obj)
    }

    fn interpret_variable(&self, identifier: Identifier) -> ExprInterpreterResult {
        match self.env.get(&identifier) {
            Some(v) => Ok(v),
            None => Err(ExprInterpreterErr::UndefinedVariable(format!(
                "{:?}",
                &identifier
            ))),
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

pub type StmtInterpreterResult = Result<Option<Object>, StmtInterpreterErr>;

impl Pass<Vec<Stmt>, Option<Object>> for StatefulInterpreter {
    type Error = StmtInterpreterErr;

    fn tree_pass(&self, input: Vec<Stmt>) -> StmtInterpreterResult {
        for stmt in input {
            match self.tree_pass(stmt) {
                Ok(None) => continue,
                rv @ Ok(_) => return rv,
                Err(e) => return Err(e),
            };
        }
        Ok(None)
    }
}

impl Pass<Stmt, Option<Object>> for StatefulInterpreter {
    type Error = StmtInterpreterErr;

    fn tree_pass(&self, input: Stmt) -> StmtInterpreterResult {
        match input {
            Stmt::Expression(expr) => self.interpret_expression_stmt(expr),
            Stmt::If(expr, tb, eb) => self.interpret_if_stmt(expr, tb, eb),
            Stmt::While(cond, body) => self.interpret_while_stmt(cond, body),
            Stmt::Print(expr) => self.interpret_print_stmt(expr),
            Stmt::Function(name, params, body) => {
                self.interpret_function_decl_stmt(name, params, *body)
            }
            Stmt::Declaration(name, expr) => self.interpret_declaration_stmt(name, expr),
            Stmt::Return(expr) => self.interpret_return_stmt(expr),
            Stmt::Class(id, stmts) => self.interpret_class_decl_stmt(id, stmts),
            Stmt::Block(stmts) => self.interpret_block(stmts),
        }
    }
}

/// This functions only to unpack an Stmt and dispatch to the upstream Interpreter<Stmt, Object)> implementation
impl Pass<Box<Stmt>, Option<Object>> for StatefulInterpreter {
    type Error = StmtInterpreterErr;
    fn tree_pass(&self, input: Box<Stmt>) -> StmtInterpreterResult {
        self.tree_pass(*input)
    }
}

impl StatefulInterpreter {
    fn interpret_expression_stmt(&self, expr: Expr) -> StmtInterpreterResult {
        match self.tree_pass(expr) {
            Ok(_) => Ok(None),
            Err(err) => Err(StmtInterpreterErr::Expression(err)),
        }
    }

    fn interpret_print_stmt(&self, expr: Expr) -> StmtInterpreterResult {
        match self.tree_pass(expr) {
            Ok(expr) => {
                println!("{}", expr);
                Ok(None)
            }
            Err(err) => Err(StmtInterpreterErr::Expression(err)),
        }
    }

    fn interpret_declaration_stmt(&self, id: Identifier, expr: Expr) -> StmtInterpreterResult {
        match self.tree_pass(expr) {
            Ok(obj) => {
                self.env.define(&id, obj);
                Ok(None)
            }
            Err(e) => Err(StmtInterpreterErr::Expression(e)),
        }
    }

    fn interpret_function_decl_stmt(
        &self,
        id: Identifier,
        params: Vec<Identifier>,
        body: Stmt,
    ) -> StmtInterpreterResult {
        let func = functions::Function::new(self.env.clone(), params, body);
        let callable = functions::Callable::Func(func);
        let obj = obj_call!(Box::new(callable));

        self.env.define(&id, obj);
        Ok(None)
    }

    fn interpret_class_decl_stmt(
        &self,
        id: Identifier,
        _methods: Vec<Stmt>,
    ) -> StmtInterpreterResult {
        let c = class::Class::new(&id);
        let callable = functions::Callable::Class(c);
        self.env.define(&id, obj_call!(Box::new(callable)));
        Ok(None)
    }

    fn interpret_return_stmt(&self, expr: Expr) -> StmtInterpreterResult {
        match self.tree_pass(expr) {
            Ok(obj) => Ok(Some(obj)),
            Err(e) => Err(StmtInterpreterErr::Expression(e)),
        }
    }

    fn interpret_block(&self, stmts: Vec<Stmt>) -> StmtInterpreterResult {
        let block_interpreter = StatefulInterpreter::from(Environment::from(&self.env));
        block_interpreter.tree_pass(stmts)
    }

    #[allow(clippy::redundant_closure)]
    fn interpret_if_stmt(
        &self,
        cond: Expr,
        tb: Box<Stmt>,
        eb: Option<Box<Stmt>>,
    ) -> StmtInterpreterResult {
        let condition = self
            .tree_pass(cond)
            .map_err(|e| StmtInterpreterErr::Expression(e))?;
        match (condition.into(), eb) {
            (true, _) => self.tree_pass(tb),
            (false, None) => Ok(None),
            (false, Some(stmt)) => self.tree_pass(stmt),
        }
    }

    #[allow(clippy::redundant_closure)]
    fn interpret_while_stmt(&self, cond: Expr, body: Box<Stmt>) -> StmtInterpreterResult {
        while self
            .tree_pass(cond.clone())
            .map_err(|e| StmtInterpreterErr::Expression(e))?
            .into()
        {
            match self.tree_pass(body.clone()) {
                Ok(None) => continue,
                rv @ Ok(_) => return rv,
                Err(e) => return Err(e),
            }
        }

        Ok(None)
    }
}

/// Interpreter<Expr, object::Object> begins implemententing the required state
/// for interpreting Expressions in a stateful way.
impl PassMut<Expr, Object> for StatefulInterpreter {
    type Error = ExprInterpreterErr;

    fn tree_pass_mut(&mut self, expr: Expr) -> ExprInterpreterResult {
        match expr {
            Expr::Grouping(expr) => self.tree_pass_mut(expr),
            Expr::Lambda(params, body) => self.interpret_lambda_mut(params, *body),
            Expr::Variable(id) => self.interpret_variable_mut(id),
            Expr::Primary(obj) => self.interpret_primary_mut(obj),
            Expr::Call(callee, args) => self.interpret_call_mut(*callee, args),
            Expr::Get(instance, param) => self.interpret_get_mut(*instance, *param),
            Expr::Unary(expr) => self.interpret_unary_mut(expr),
            Expr::Multiplication(expr) => self.interpret_multiplication_mut(expr),
            Expr::Addition(expr) => self.interpret_addition_mut(expr),
            Expr::Comparison(expr) => self.interpret_comparison_mut(expr),
            Expr::Equality(expr) => self.interpret_equality_mut(expr),
            Expr::Logical(expr) => self.interpret_logical_mut(expr),
            Expr::Assignment(id, expr) => self.interpret_assignment_mut(id, expr),
        }
    }
}

/// This functions only to unpack an Expr and dispatch to the upstream
/// Interpreter<Expr, object::Object> implementation.
impl PassMut<Box<Expr>, Object> for StatefulInterpreter {
    type Error = ExprInterpreterErr;
    fn tree_pass_mut(&mut self, expr: Box<Expr>) -> ExprInterpreterResult {
        self.tree_pass_mut(*expr)
    }
}

impl StatefulInterpreter {
    fn interpret_assignment_mut(
        &mut self,
        id: Identifier,
        expr: Box<Expr>,
    ) -> ExprInterpreterResult {
        let lhv = id;
        let rhv = self.tree_pass_mut(expr)?;

        match self.env.assign(&lhv, rhv) {
            Some(v) => Ok(v),
            None => Err(ExprInterpreterErr::UndefinedVariable(format!("{:?}", &lhv))),
        }
    }

    fn interpret_logical_mut(&mut self, expr: LogicalExpr) -> ExprInterpreterResult {
        match expr {
            LogicalExpr::Or(left, right) => {
                let lho: Object = self.tree_pass_mut(left)?;
                let lho_bool: bool = lho.clone().into();
                if lho_bool {
                    Ok(lho)
                } else {
                    self.tree_pass_mut(right)
                }
            }
            LogicalExpr::And(left, right) => {
                let lho: Object = self.tree_pass_mut(left)?;
                let lho_bool: bool = lho.clone().into();
                if !lho_bool {
                    Ok(lho)
                } else {
                    self.tree_pass_mut(right)
                }
            }
        }
    }

    fn interpret_equality_mut(&mut self, expr: EqualityExpr) -> ExprInterpreterResult {
        match expr {
            EqualityExpr::Equal(left, right) => {
                match (self.tree_pass_mut(left), self.tree_pass_mut(right)) {
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
                }
            }
            EqualityExpr::NotEqual(left, right) => {
                match (self.tree_pass_mut(left), self.tree_pass_mut(right)) {
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

    fn interpret_comparison_mut(&mut self, expr: ComparisonExpr) -> ExprInterpreterResult {
        match expr {
            ComparisonExpr::Less(left, right) => {
                match (self.tree_pass_mut(left), self.tree_pass_mut(right)) {
                    (
                        Ok(Object::Literal(Literal::Number(l_val))),
                        Ok(Object::Literal(Literal::Number(r_val))),
                    ) => Ok(obj_bool!(l_val < r_val)),
                    (Ok(l), Ok(r)) => type_error!(l, "<", r),
                    _ => type_error!(),
                }
            }
            ComparisonExpr::LessEqual(left, right) => {
                match (self.tree_pass_mut(left), self.tree_pass_mut(right)) {
                    (
                        Ok(Object::Literal(Literal::Number(l_val))),
                        Ok(Object::Literal(Literal::Number(r_val))),
                    ) => Ok(obj_bool!(l_val <= r_val)),
                    (Ok(l), Ok(r)) => type_error!(l, "<=", r),
                    _ => type_error!(),
                }
            }
            ComparisonExpr::Greater(left, right) => {
                match (self.tree_pass_mut(left), self.tree_pass_mut(right)) {
                    (
                        Ok(Object::Literal(Literal::Number(l_val))),
                        Ok(Object::Literal(Literal::Number(r_val))),
                    ) => Ok(obj_bool!(l_val > r_val)),
                    (Ok(l), Ok(r)) => type_error!(l, ">", r),
                    _ => type_error!(),
                }
            }
            ComparisonExpr::GreaterEqual(left, right) => {
                match (self.tree_pass_mut(left), self.tree_pass_mut(right)) {
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

    fn interpret_addition_mut(&mut self, expr: AdditionExpr) -> ExprInterpreterResult {
        match expr {
            AdditionExpr::Add(left, right) => {
                match (self.tree_pass_mut(left), self.tree_pass_mut(right)) {
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
                }
            }
            AdditionExpr::Subtract(left, right) => {
                match (self.tree_pass_mut(left), self.tree_pass_mut(right)) {
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

    fn interpret_multiplication_mut(&mut self, expr: MultiplicationExpr) -> ExprInterpreterResult {
        match expr {
            MultiplicationExpr::Multiply(left, right) => {
                match (self.tree_pass_mut(left), self.tree_pass_mut(right)) {
                    (
                        Ok(Object::Literal(Literal::Number(l_val))),
                        Ok(Object::Literal(Literal::Number(r_val))),
                    ) => Ok(obj_number!(l_val * r_val)),
                    (Ok(l), Ok(r)) => type_error!(l, "*", r),
                    _ => type_error!(),
                }
            }
            MultiplicationExpr::Divide(left, right) => {
                match (self.tree_pass_mut(left), self.tree_pass_mut(right)) {
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

    fn interpret_unary_mut(&mut self, expr: UnaryExpr) -> ExprInterpreterResult {
        match expr {
            UnaryExpr::Bang(ue) => match self.tree_pass_mut(ue) {
                Ok(obj) => {
                    let ob: bool = obj.into();
                    Ok(obj_bool!(!ob))
                }
                e @ Err(_) => e,
            },
            UnaryExpr::Minus(ue) => match self.tree_pass_mut(ue) {
                Ok(Object::Literal(Literal::Number(n))) => Ok(obj_number!(n * -1.0)),
                e @ Err(_) => e,
                _ => type_error!(),
            },
        }
    }

    fn interpret_call_mut(&mut self, callee: Expr, args: Vec<Expr>) -> ExprInterpreterResult {
        let fun = self.tree_pass_mut(callee).map(|obj_res| obj_res)?;
        let params: Vec<Object> = args
            .into_iter()
            .map(|expr| self.tree_pass_mut(expr).unwrap())
            .collect();

        let c = match fun {
            Object::Call(c) => Ok(c),
            _ => Err(ExprInterpreterErr::CallErr(format!(
                "object {} is not callable",
                fun
            ))),
        }?;

        match c.call(self.env.clone(), params) {
            Ok(r) => Ok(r),
            Err(e) => Err(ExprInterpreterErr::CallErr(format!("{:?}", e))),
        }
    }

    fn interpret_get_mut(&mut self, instance: Expr, param: Expr) -> ExprInterpreterResult {
        let i = self.tree_pass_mut(instance).map(|obj_res| match obj_res {
            Object::Instance(i) => Ok(i),
            _ => Err(ExprInterpreterErr::CallErr(format!(
                "object {} is not callable",
                obj_res
            ))),
        })??;

        let param_id = if let Expr::Variable(id) = param {
            Ok(id)
        } else {
            Err(ExprInterpreterErr::Type(
                "Expected identifier for parameter",
            ))
        }?;

        i.get(&param_id).map_or(
            Err(ExprInterpreterErr::UndefinedVariable(param_id.to_string())),
            |obj| Ok(obj),
        )
    }

    fn interpret_lambda_mut(
        &mut self,
        params: Vec<Identifier>,
        body: Stmt,
    ) -> ExprInterpreterResult {
        let func = functions::Function::new(self.env.clone(), params, body);
        let callable = functions::Callable::Func(func);
        let obj = Object::Call(Box::new(callable));

        Ok(obj)
    }

    fn interpret_primary_mut(&mut self, obj: Object) -> ExprInterpreterResult {
        Ok(obj)
    }

    fn interpret_variable_mut(&mut self, identifier: Identifier) -> ExprInterpreterResult {
        match self.env.get(&identifier) {
            Some(v) => Ok(v),
            None => Err(ExprInterpreterErr::UndefinedVariable(format!(
                "{:?}",
                &identifier
            ))),
        }
    }
}

impl PassMut<Vec<Stmt>, Option<Object>> for StatefulInterpreter {
    type Error = StmtInterpreterErr;

    fn tree_pass_mut(&mut self, input: Vec<Stmt>) -> StmtInterpreterResult {
        for stmt in input {
            match self.tree_pass_mut(stmt) {
                Ok(None) => continue,
                rv @ Ok(_) => return rv,
                Err(e) => return Err(e),
            };
        }
        Ok(None)
    }
}

impl PassMut<Stmt, Option<Object>> for StatefulInterpreter {
    type Error = StmtInterpreterErr;

    fn tree_pass_mut(&mut self, input: Stmt) -> StmtInterpreterResult {
        match input {
            Stmt::Expression(expr) => self.interpret_expression_stmt_mut(expr),
            Stmt::If(expr, tb, eb) => self.interpret_if_stmt_mut(expr, tb, eb),
            Stmt::While(cond, body) => self.interpret_while_stmt_mut(cond, body),
            Stmt::Print(expr) => self.interpret_print_stmt_mut(expr),
            Stmt::Function(name, params, body) => {
                self.interpret_function_decl_stmt_mut(name, params, *body)
            }
            Stmt::Declaration(name, expr) => self.interpret_declaration_stmt_mut(name, expr),
            Stmt::Return(expr) => self.interpret_return_stmt_mut(expr),
            Stmt::Class(id, stmts) => self.interpret_class_decl_stmt_mut(id, stmts),
            Stmt::Block(stmts) => self.interpret_block_mut(stmts),
        }
    }
}

/// This functions only to unpack an Stmt and dispatch to the upstream Interpreter<Stmt, Object)> implementation
impl PassMut<Box<Stmt>, Option<Object>> for StatefulInterpreter {
    type Error = StmtInterpreterErr;
    fn tree_pass_mut(&mut self, input: Box<Stmt>) -> StmtInterpreterResult {
        self.tree_pass_mut(*input)
    }
}

impl StatefulInterpreter {
    fn interpret_expression_stmt_mut(&mut self, expr: Expr) -> StmtInterpreterResult {
        match self.tree_pass_mut(expr) {
            Ok(_) => Ok(None),
            Err(err) => Err(StmtInterpreterErr::Expression(err)),
        }
    }

    fn interpret_print_stmt_mut(&mut self, expr: Expr) -> StmtInterpreterResult {
        match self.tree_pass_mut(expr) {
            Ok(expr) => {
                println!("{}", expr);
                Ok(None)
            }
            Err(err) => Err(StmtInterpreterErr::Expression(err)),
        }
    }

    fn interpret_declaration_stmt_mut(
        &mut self,
        id: Identifier,
        expr: Expr,
    ) -> StmtInterpreterResult {
        match self.tree_pass_mut(expr) {
            Ok(obj) => {
                self.env.define(&id, obj);
                Ok(None)
            }
            Err(e) => Err(StmtInterpreterErr::Expression(e)),
        }
    }

    fn interpret_function_decl_stmt_mut(
        &self,
        id: Identifier,
        params: Vec<Identifier>,
        body: Stmt,
    ) -> StmtInterpreterResult {
        let func = functions::Function::new(self.env.clone(), params, body);
        let callable = functions::Callable::Func(func);
        let obj = obj_call!(Box::new(callable));

        self.env.define(&id, obj);
        Ok(None)
    }

    fn interpret_class_decl_stmt_mut(
        &self,
        id: Identifier,
        _methods: Vec<Stmt>,
    ) -> StmtInterpreterResult {
        let c = class::Class::new(&id);
        let callable = functions::Callable::Class(c);
        self.env.define(&id, obj_call!(Box::new(callable)));
        Ok(None)
    }

    fn interpret_return_stmt_mut(&mut self, expr: Expr) -> StmtInterpreterResult {
        match self.tree_pass_mut(expr) {
            Ok(obj) => Ok(Some(obj)),
            Err(e) => Err(StmtInterpreterErr::Expression(e)),
        }
    }

    fn interpret_block_mut(&mut self, stmts: Vec<Stmt>) -> StmtInterpreterResult {
        let mut block_interpreter = StatefulInterpreter::from(Environment::from(&self.env));
        block_interpreter.tree_pass_mut(stmts)
    }

    #[allow(clippy::redundant_closure)]
    fn interpret_if_stmt_mut(
        &mut self,
        cond: Expr,
        tb: Box<Stmt>,
        eb: Option<Box<Stmt>>,
    ) -> StmtInterpreterResult {
        let condition = self
            .tree_pass_mut(cond)
            .map_err(|e| StmtInterpreterErr::Expression(e))?;
        match (condition.into(), eb) {
            (true, _) => self.tree_pass_mut(tb),
            (false, None) => Ok(None),
            (false, Some(stmt)) => self.tree_pass_mut(stmt),
        }
    }

    #[allow(clippy::redundant_closure)]
    fn interpret_while_stmt_mut(&mut self, cond: Expr, body: Box<Stmt>) -> StmtInterpreterResult {
        while self
            .tree_pass_mut(cond.clone())
            .map_err(|e| StmtInterpreterErr::Expression(e))?
            .into()
        {
            match self.tree_pass_mut(body.clone()) {
                Ok(None) => continue,
                rv @ Ok(_) => return rv,
                Err(e) => return Err(e),
            }
        }

        Ok(None)
    }
}
