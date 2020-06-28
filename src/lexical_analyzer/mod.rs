use crate::ast::statement::Stmt;
use std::collections::HashMap;
use std::fmt;

#[cfg(test)]
mod tests;

/// LexicalAnalyzer defines a trait for lexical analysizing passes.
pub trait LexicalAnalyzer<A, B> {
    type Error;

    fn analyze(&self, input: A) -> Result<B, Self::Error>;
}

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

pub type Scope = HashMap<String, bool>;
pub type ScopeStack = Vec<Scope>;
pub type ScopeAnalyzerResult = Result<ScopeStack, ScopeAnalyzerErr>;

#[derive(Default)]
/// ScopeAnalyzers walks the tree, ensuring that variables and scopes resolve
/// to the expected values.
pub struct ScopeAnalyzer {}

impl ScopeAnalyzer {
    pub fn new() -> Self {
        ScopeAnalyzer {}
    }
}

impl LexicalAnalyzer<(ScopeStack, Vec<Stmt>), ScopeStack> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, input: (ScopeStack, Vec<Stmt>)) -> ScopeAnalyzerResult {
        let (mut scope, stmts) = input;
        for stmt in stmts {
            match self.analyze((scope, stmt)) {
                Ok(ret_scope) => scope = ret_scope, // assign scope to ret value
                Err(e) => return Err(e),
            };
        }
        Ok(scope)
    }
}

impl LexicalAnalyzer<(ScopeStack, Stmt), ScopeStack> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, input: (ScopeStack, Stmt)) -> ScopeAnalyzerResult {
        let (scope, stmt) = input;
        match stmt {
            Stmt::Block(stmts) => self.resolve_block_scope(scope, stmts),
            _ => Err(ScopeAnalyzerErr::Unspecified),
        }
    }
}

// Unpack boxed-Stmts
impl LexicalAnalyzer<(ScopeStack, Box<Stmt>), ScopeStack> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, input: (ScopeStack, Box<Stmt>)) -> ScopeAnalyzerResult {
        let (scope, boxed_stmt) = input;
        self.analyze((scope, *boxed_stmt))
    }
}

// Resolves Stmt-related types.
impl ScopeAnalyzer {
    fn resolve_block_scope(&self, scope: ScopeStack, stmts: Vec<Stmt>) -> ScopeAnalyzerResult {
        self.analyze((scope, stmts))
    }
}
