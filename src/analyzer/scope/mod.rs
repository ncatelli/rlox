use crate::analyzer::SemanticAnalyzer;
use crate::ast::statement::Stmt;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum ScopeAnalyzerErr {
    Unspecified,
}

impl fmt::Display for ScopeAnalyzerErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unspecified => write!(f, "unspecified resolver error"),
        }
    }
}

pub struct ScopeAnalyzer {}

impl SemanticAnalyzer<Vec<Stmt>, Vec<Stmt>> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, input: Vec<Stmt>) -> Result<Vec<Stmt>, Self::Error> {
        input.into_iter().map(|stmt| self.analyze(stmt)).collect()
    }
}

impl SemanticAnalyzer<Stmt, Stmt> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, input: Stmt) -> Result<Stmt, Self::Error> {
        match input {
            _ => Err(ScopeAnalyzerErr::Unspecified),
        }
    }
}
