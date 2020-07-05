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

        ScopeAnalyzer { scopes: glbls }
    }
}

impl From<Rc<Node>> for ScopeAnalyzer {
    fn from(scopes: Rc<Node>) -> ScopeAnalyzer {
        let mut si = ScopeAnalyzer::new();
        si.scopes = scopes.clone();
        si
    }
}

/// SemanticAnalyzer<&Box<Expr>, usize> unpacks the boxed expr.
impl SemanticAnalyzer<&Box<Expr>, usize> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, expr: &Box<Expr>) -> Result<usize, Self::Error> {
        self.analyze(&(**expr))
    }
}

/// SemanticAnalyzer<&Expr, usize> Implements the requirements
/// for an analyzer pass over Expr
impl SemanticAnalyzer<&Expr, usize> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, expr: &Expr) -> Result<usize, Self::Error> {
        match expr {
            Expr::Variable(id) => self.analyze_variable(id.clone()),
            Expr::Assignment(id, expr) => self.analyze_assignment(id.clone(), expr),
            Expr::Primary(_) => Ok(self.scopes.offset()),
            _ => todo!(),
        }
    }
}

impl ScopeAnalyzer {
    fn analyze_variable(&self, identifier: token::Token) -> Result<usize, ScopeAnalyzerErr> {
        let var = identifier.lexeme.unwrap();

        match self.scopes.resolve_local(&var) {
            Some(v) => Ok(v),
            None => Err(ScopeAnalyzerErr::Undefined(var.to_string())),
        }
    }

    fn analyze_assignment(
        &self,
        identifier: token::Token,
        expr: &Expr,
    ) -> Result<usize, ScopeAnalyzerErr> {
        let var = identifier.lexeme.unwrap();

        self.analyze(expr)?;
        match self.scopes.resolve_local(&var) {
            Some(v) => Ok(v),
            None => Err(ScopeAnalyzerErr::Undefined(var.to_string())),
        }
    }
}

use crate::ast::statement::Stmt;

impl SemanticAnalyzer<&Vec<Stmt>, Vec<Rc<Node>>> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, input: &Vec<Stmt>) -> Result<Vec<Rc<Node>>, Self::Error> {
        let mut scopes: Vec<Rc<Node>> = vec![self.scopes.clone()];

        for stmt in input {
            match self.analyze(stmt) {
                Ok(None) => continue,
                Ok(Some(s)) => scopes.extend(s.into_iter()),
                Err(e) => return Err(e),
            };
        }

        Ok(scopes)
    }
}

/// SemanticAnalyzer<&Box<Stmt>, Option<Vec<Rc<Node>>>> unpacks the boxed expr.
impl SemanticAnalyzer<&Box<Stmt>, Option<Vec<Rc<Node>>>> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, stmt: &Box<Stmt>) -> Result<Option<Vec<Rc<Node>>>, Self::Error> {
        self.analyze(&(**stmt))
    }
}

impl SemanticAnalyzer<&Stmt, Option<Vec<Rc<Node>>>> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, input: &Stmt) -> Result<Option<Vec<Rc<Node>>>, Self::Error> {
        match input {
            Stmt::Block(stmts) => self.analyze_block(stmts),
            Stmt::Declaration(id, _) => self.analyze_declaration(id),
            Stmt::Expression(expr) => self.analyze(expr).map(|_| None),
            Stmt::Function(id, _, stmt) => self.analyze_function(id, stmt),
            Stmt::If(expr, tb, eb) => self.analyze_if(expr, tb, eb),
            _ => Ok(None),
        }
    }
}

impl ScopeAnalyzer {
    fn analyze_block(&self, stmts: &Vec<Stmt>) -> Result<Option<Vec<Rc<Node>>>, ScopeAnalyzerErr> {
        let block_analyzer = ScopeAnalyzer::from(Node::from(&self.scopes));
        let nested_scopes = block_analyzer.analyze(stmts)?;

        Ok(Some(nested_scopes))
    }

    fn analyze_declaration(&self, id: &str) -> Result<Option<Vec<Rc<Node>>>, ScopeAnalyzerErr> {
        self.scopes.declare(id);

        Ok(None)
    }

    fn analyze_function(
        &self,
        id: &str,
        stmt: &Box<Stmt>,
    ) -> Result<Option<Vec<Rc<Node>>>, ScopeAnalyzerErr> {
        let nested_scopes = self.analyze(stmt)?;
        self.scopes.declare(id);

        Ok(nested_scopes)
    }

    fn analyze_if(
        &self,
        cond: &Expr,
        tb: &Box<Stmt>,
        eb: &Option<Box<Stmt>>,
    ) -> Result<Option<Vec<Rc<Node>>>, ScopeAnalyzerErr> {
        self.analyze(cond)?;
        let mut scope = self.analyze(tb)?.map_or(Vec::new(), |s| s);
        // if elsebranch is defined merge with scope
        if let Some(branch) = eb {
            if let Some(res) = self.analyze(branch)? {
                scope.extend(res.into_iter());
            };
        }

        Ok(if scope.len() == 0 { None } else { Some(scope) })
    }
}
