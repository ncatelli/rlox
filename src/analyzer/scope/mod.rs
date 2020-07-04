use crate::analyzer::SemanticAnalyzer;
use crate::ast::expression::Expr;
use crate::ast::token;
use std::fmt;
use std::rc::Rc;

pub mod tree;
use tree::Node;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug)]
pub enum ScopeAnalyzerErr {
    Unspecified,
    Unimplemented, // Eventually remove after completed
    Undefined(String),
}

impl fmt::Display for ScopeAnalyzerErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unspecified => write!(f, "unspecified resolver error"),
            Self::Unimplemented => write!(f, "this endpoint was not implemented yet"),
            Self::Undefined(id) => write!(f, "undefined token: {}", &id),
        }
    }
}

#[derive(Default)]
pub struct ScopeAnalyzer {
    pub scopes: Rc<Node>,
}

impl ScopeAnalyzer {
    pub fn new() -> ScopeAnalyzer {
        let glbls = Node::new();

        ScopeAnalyzer {
            scopes: Node::from(&glbls),
        }
    }
}

impl From<Rc<Node>> for ScopeAnalyzer {
    fn from(scopes: Rc<Node>) -> ScopeAnalyzer {
        let mut si = ScopeAnalyzer::new();
        si.scopes = scopes;
        si
    }
}

/// SemanticAnalyzer<&Expr, usize> Implements the requirements
/// for an analyzer pass over Expr
impl SemanticAnalyzer<&Expr, usize> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, expr: &Expr) -> Result<usize, Self::Error> {
        match expr {
            Expr::Variable(id) => self.interpret_variable(id.clone()),
            _ => todo!(),
        }
    }
}

impl ScopeAnalyzer {
    fn interpret_variable(&self, identifier: token::Token) -> Result<usize, ScopeAnalyzerErr> {
        let var = identifier.lexeme.unwrap();

        match self.scopes.resolve_local(&var) {
            Some(v) => Ok(v),
            None => Err(ScopeAnalyzerErr::Undefined(var.to_string())),
        }
    }
}

use crate::ast::statement::Stmt;

impl SemanticAnalyzer<&Vec<Stmt>, ()> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, input: &Vec<Stmt>) -> Result<(), Self::Error> {
        for stmt in input {
            match self.analyze(stmt) {
                Ok(_) => continue,
                Err(e) => return Err(e),
            };
        }
        Ok(())
    }
}

impl SemanticAnalyzer<&Stmt, ()> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, input: &Stmt) -> Result<(), Self::Error> {
        match input {
            _ => todo!(),
        }
    }
}

impl ScopeAnalyzer {}
