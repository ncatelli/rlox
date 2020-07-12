use crate::analyzer::SemanticAnalyzer;
use crate::ast::expression::Expr;
use std::fmt;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug)]
pub enum ScopeAnalyzerErr {
    Undefined,
}

impl fmt::Display for ScopeAnalyzerErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Undefined => write!(f, "undefined error"),
        }
    }
}

#[derive(Default)]
pub struct ScopeAnalyzer {}

impl ScopeAnalyzer {
    pub fn new() -> ScopeAnalyzer {
        ScopeAnalyzer {}
    }
}

type ExprSemanticAnalyzerResult = Result<Expr, ScopeAnalyzerErr>;

/// SemanticAnalyzer<Expr, Expr> begins implemententing the required state
/// for interpreting Expressions in a stateful way.
impl SemanticAnalyzer<Expr, Expr> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, expr: Expr) -> ExprSemanticAnalyzerResult {
        match expr {
            e @ Expr::Grouping(_) => Ok(e),
            e @ Expr::Lambda(_, _) => Ok(e),
            e @ Expr::Variable(_) => Ok(e),
            e @ Expr::Primary(_) => Ok(e),
            e @ Expr::Call(_, _) => Ok(e),
            e @ Expr::Unary(_) => Ok(e),
            e @ Expr::Multiplication(_) => Ok(e),
            e @ Expr::Addition(_) => Ok(e),
            e @ Expr::Comparison(_) => Ok(e),
            e @ Expr::Equality(_) => Ok(e),
            e @ Expr::Logical(_) => Ok(e),
            e @ Expr::Assignment(_, _) => Ok(e),
        }
    }
}

impl SemanticAnalyzer<Box<Expr>, Expr> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;
    fn analyze(&self, expr: Box<Expr>) -> ExprSemanticAnalyzerResult {
        self.analyze(*expr)
    }
}

use crate::ast::statement::Stmt;

#[derive(PartialEq, Debug)]
pub enum StmtScopeAnalyzerErr {
    Unspecified,
}

impl fmt::Display for StmtScopeAnalyzerErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unspecified => write!(f, "unspecified statement error"),
        }
    }
}

pub type StmtSemanticAnalyzerResult = Result<Stmt, StmtScopeAnalyzerErr>;

impl SemanticAnalyzer<Vec<Stmt>, Vec<Stmt>> for ScopeAnalyzer {
    type Error = StmtScopeAnalyzerErr;

    fn analyze(&self, input: Vec<Stmt>) -> Result<Vec<Stmt>, StmtScopeAnalyzerErr> {
        let mut output: Vec<Stmt> = Vec::new();
        for stmt in input {
            match self.analyze(stmt) {
                Ok(s) => output.push(s),
                Err(e) => return Err(e),
            };
        }
        Ok(output)
    }
}

impl SemanticAnalyzer<Stmt, Stmt> for ScopeAnalyzer {
    type Error = StmtScopeAnalyzerErr;

    fn analyze(&self, input: Stmt) -> StmtSemanticAnalyzerResult {
        match input {
            s @ Stmt::Expression(_) => Ok(s),
            s @ Stmt::If(_, _, _) => Ok(s),
            s @ Stmt::While(_, _) => Ok(s),
            s @ Stmt::Print(_) => Ok(s),
            s @ Stmt::Function(_, _, _) => Ok(s),
            s @ Stmt::Declaration(_, _) => Ok(s),
            s @ Stmt::Return(_) => Ok(s),
            s @ Stmt::Block(_) => Ok(s),
        }
    }
}

/// This functions only to unpack an Stmt and dispatch to the upstream SemanticAnalyzer<Stmt, Stmt)> implementation
impl SemanticAnalyzer<Box<Stmt>, Stmt> for ScopeAnalyzer {
    type Error = StmtScopeAnalyzerErr;
    fn analyze(&self, input: Box<Stmt>) -> StmtSemanticAnalyzerResult {
        self.analyze(*input)
    }
}

impl ScopeAnalyzer {}
