use crate::analyzer::SemanticAnalyzerMut;
use crate::ast::expression::{
    AdditionExpr, ComparisonExpr, EqualityExpr, Expr, LogicalExpr, MultiplicationExpr, UnaryExpr,
};
use crate::ast::identifier::Identifier;
use std::fmt;

mod stack;
use stack::{Scope, ScopeStack};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug)]
pub enum ScopeAnalyzerErr {
    Undefined,
    TypeMismatch,
}

impl fmt::Display for ScopeAnalyzerErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Undefined => write!(f, "undefined error"),
            Self::TypeMismatch => write!(f, "invalid type passed to analyzer method"),
        }
    }
}

pub struct ScopeAnalyzer {
    stack: ScopeStack,
}

impl ScopeAnalyzer {
    pub fn new() -> ScopeAnalyzer {
        ScopeAnalyzer {
            stack: ScopeStack::new(),
        }
    }
}

type ExprSemanticAnalyzerResult = Result<Expr, ScopeAnalyzerErr>;

impl SemanticAnalyzerMut<Expr, Expr> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&mut self, expr: Expr) -> ExprSemanticAnalyzerResult {
        match expr {
            Expr::Grouping(e) => Ok(Expr::Grouping(Box::new(self.analyze(e)?))),
            Expr::Lambda(params, body) => self.analyze_lambda(params, *body),
            Expr::Variable(id) => self.analyze_variable(id),
            e @ Expr::Primary(_) => Ok(e),
            Expr::Call(callee, args) => self.analyze_call(*callee, args),
            Expr::Unary(expr) => self.analyze_unary(expr),
            Expr::Multiplication(me) => self.analyze_multiplication(me),
            Expr::Addition(ae) => self.analyze_addition(ae),
            Expr::Comparison(ce) => self.analyze_comparison(ce),
            Expr::Equality(ee) => self.analyze_equality(ee),
            Expr::Logical(le) => self.analyze_logical(le),
            Expr::Assignment(id, v) => self.analyze_assignment(id, v),
        }
    }
}

impl ScopeAnalyzer {
    fn analyze_assignment(
        &mut self,
        id: Identifier,
        expr: Box<Expr>,
    ) -> ExprSemanticAnalyzerResult {
        let rhv = self.analyze(expr)?;

        match self.stack.get_offset(&id) {
            Some(offset) => Ok(Expr::Assignment(Identifier::Id(offset), Box::new(rhv))),
            None => Err(ScopeAnalyzerErr::Undefined),
        }
    }

    fn analyze_logical(&mut self, expr: LogicalExpr) -> ExprSemanticAnalyzerResult {
        Ok(Expr::Logical(match expr {
            LogicalExpr::Or(left, right) => LogicalExpr::Or(
                Box::new(self.analyze(left)?),
                Box::new(self.analyze(right)?),
            ),
            LogicalExpr::And(left, right) => LogicalExpr::And(
                Box::new(self.analyze(left)?),
                Box::new(self.analyze(right)?),
            ),
        }))
    }

    fn analyze_equality(&mut self, expr: EqualityExpr) -> ExprSemanticAnalyzerResult {
        Ok(Expr::Equality(match expr {
            EqualityExpr::Equal(left, right) => EqualityExpr::Equal(
                Box::new(self.analyze(left)?),
                Box::new(self.analyze(right)?),
            ),
            EqualityExpr::NotEqual(left, right) => EqualityExpr::NotEqual(
                Box::new(self.analyze(left)?),
                Box::new(self.analyze(right)?),
            ),
        }))
    }

    fn analyze_comparison(&mut self, expr: ComparisonExpr) -> ExprSemanticAnalyzerResult {
        Ok(Expr::Comparison(match expr {
            ComparisonExpr::Greater(left, right) => ComparisonExpr::Greater(
                Box::new(self.analyze(left)?),
                Box::new(self.analyze(right)?),
            ),
            ComparisonExpr::GreaterEqual(left, right) => ComparisonExpr::GreaterEqual(
                Box::new(self.analyze(left)?),
                Box::new(self.analyze(right)?),
            ),
            ComparisonExpr::Less(left, right) => ComparisonExpr::Less(
                Box::new(self.analyze(left)?),
                Box::new(self.analyze(right)?),
            ),
            ComparisonExpr::LessEqual(left, right) => ComparisonExpr::LessEqual(
                Box::new(self.analyze(left)?),
                Box::new(self.analyze(right)?),
            ),
        }))
    }

    fn analyze_addition(&mut self, expr: AdditionExpr) -> ExprSemanticAnalyzerResult {
        Ok(Expr::Addition(match expr {
            AdditionExpr::Add(left, right) => AdditionExpr::Add(
                Box::new(self.analyze(left)?),
                Box::new(self.analyze(right)?),
            ),
            AdditionExpr::Subtract(left, right) => AdditionExpr::Subtract(
                Box::new(self.analyze(left)?),
                Box::new(self.analyze(right)?),
            ),
        }))
    }

    fn analyze_multiplication(&mut self, expr: MultiplicationExpr) -> ExprSemanticAnalyzerResult {
        Ok(Expr::Multiplication(match expr {
            MultiplicationExpr::Multiply(left, right) => MultiplicationExpr::Multiply(
                Box::new(self.analyze(left)?),
                Box::new(self.analyze(right)?),
            ),
            MultiplicationExpr::Divide(left, right) => MultiplicationExpr::Divide(
                Box::new(self.analyze(left)?),
                Box::new(self.analyze(right)?),
            ),
        }))
    }

    fn analyze_unary(&mut self, expr: UnaryExpr) -> ExprSemanticAnalyzerResult {
        Ok(Expr::Unary(match expr {
            UnaryExpr::Bang(expr) => UnaryExpr::Bang(Box::new(self.analyze(expr)?)),
            UnaryExpr::Minus(expr) => UnaryExpr::Minus(Box::new(self.analyze(expr)?)),
        }))
    }

    fn analyze_call(&mut self, callee: Expr, args: Vec<Expr>) -> ExprSemanticAnalyzerResult {
        let analyzed_callee = self.analyze(callee)?;
        let mut analyzed_args: Vec<Expr> = Vec::new();

        for arg in args {
            analyzed_args.push(self.analyze(arg)?);
        }

        Ok(Expr::Call(Box::new(analyzed_callee), analyzed_args))
    }

    fn analyze_lambda(
        &mut self,
        params: Vec<Identifier>,
        body: Stmt,
    ) -> ExprSemanticAnalyzerResult {
        // enter scope
        self.stack.push(Scope::new());
        let param_ids: Vec<Identifier> = params
            .into_iter()
            .map(|param| self.declare_or_assign(param))
            .collect();

        let analyzed_body = self.analyze(body)?;

        // exit scope
        self.stack.pop();

        Ok(Expr::Lambda(param_ids, Box::new(analyzed_body)))
    }

    fn analyze_variable(&mut self, id: Identifier) -> ExprSemanticAnalyzerResult {
        match self.stack.get_offset(&id) {
            Some(offset) => Ok(Expr::Variable(Identifier::Id(offset))),
            None => Err(ScopeAnalyzerErr::Undefined),
        }
    }
}

impl SemanticAnalyzerMut<Box<Expr>, Expr> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;
    fn analyze(&mut self, expr: Box<Expr>) -> ExprSemanticAnalyzerResult {
        self.analyze(*expr)
    }
}

use crate::ast::statement::Stmt;

pub type StmtSemanticAnalyzerResult = Result<Stmt, ScopeAnalyzerErr>;

impl SemanticAnalyzerMut<Vec<Stmt>, Vec<Stmt>> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&mut self, input: Vec<Stmt>) -> Result<Vec<Stmt>, ScopeAnalyzerErr> {
        input.into_iter().map(|s| self.analyze(s)).collect()
    }
}

impl SemanticAnalyzerMut<Stmt, Stmt> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&mut self, input: Stmt) -> StmtSemanticAnalyzerResult {
        match input {
            Stmt::Expression(e) => Ok(Stmt::Expression(self.analyze(e)?)),
            Stmt::If(cond, tb, eb) => self.analyze_if(cond, tb, eb),
            Stmt::While(e, b) => Ok(Stmt::While(self.analyze(e)?, Box::new(self.analyze(b)?))),
            Stmt::Print(e) => Ok(Stmt::Print(self.analyze(e)?)),
            Stmt::Function(name, params, body) => self.analyze_function(name, params, body),
            Stmt::Declaration(id, expr) => self.analyze_declaration(id, expr),
            Stmt::Return(e) => Ok(Stmt::Return(self.analyze(e)?)),
            Stmt::Block(stmts) => self.analyze_block(stmts),
        }
    }
}

/// This functions only to unpack an Stmt and dispatch to the upstream SemanticAnalyzer<Stmt, Stmt)> implementation
impl SemanticAnalyzerMut<Box<Stmt>, Stmt> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;
    fn analyze(&mut self, input: Box<Stmt>) -> StmtSemanticAnalyzerResult {
        self.analyze(*input)
    }
}

impl ScopeAnalyzer {
    fn declare_or_assign(&mut self, id: Identifier) -> Identifier {
        match self.stack.get_offset(&id) {
            Some(offset) => Identifier::Id(offset),
            None => {
                self.stack.push_elem(id);
                Identifier::Id(self.stack.len() - 1)
            }
        }
    }

    fn analyze_block(&mut self, stmts: Vec<Stmt>) -> StmtSemanticAnalyzerResult {
        // enter scope
        self.stack.push(Scope::new());
        let analyzed_block = self.analyze(stmts)?;
        // leave scope
        self.stack.pop();

        Ok(Stmt::Block(analyzed_block))
    }

    fn analyze_function(
        &mut self,
        fname: Identifier,
        params: Vec<Identifier>,
        body: Box<Stmt>,
    ) -> StmtSemanticAnalyzerResult {
        let fid = self.declare_or_assign(fname);

        // enter scope
        self.stack.push(Scope::new());

        let param_ids: Vec<Identifier> = params
            .into_iter()
            .map(|param| self.declare_or_assign(param))
            .collect();
        let analyzed_body = self.analyze(body)?;
        // leave scope
        self.stack.pop();

        Ok(Stmt::Function(fid, param_ids, Box::new(analyzed_body)))
    }

    fn analyze_if(
        &mut self,
        cond: Expr,
        tb: Box<Stmt>,
        eb: Option<Box<Stmt>>,
    ) -> StmtSemanticAnalyzerResult {
        let c = self.analyze(cond)?;
        let then_branch = Box::new(self.analyze(tb)?);
        let else_branch = match eb {
            Some(branch) => Some(Box::new(self.analyze(branch)?)),
            None => None,
        };

        Ok(Stmt::If(c, then_branch, else_branch))
    }

    fn analyze_declaration(&mut self, id: Identifier, expr: Expr) -> StmtSemanticAnalyzerResult {
        match self.analyze(expr) {
            Ok(e) => Ok(Stmt::Declaration(self.declare_or_assign(id), e)),
            Err(e) => Err(e),
        }
    }
}
