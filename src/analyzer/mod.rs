use crate::ast::expression::Expr;
use crate::ast::statement::Stmt;
use std::collections::HashSet;
use std::fmt;

#[cfg(test)]
mod tests;

/// Analyzer defines a trait for analysizing passes against the AST.
pub trait Analyzer<A, B> {
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

pub type Scope = HashSet<String>;

pub type ScopeStack = Vec<Scope>;
pub type ScopeAnalyzerResult = Result<ScopeStack, ScopeAnalyzerErr>;

/// begin_scope creates a new scope, pushing it to the top of the stack.
fn begin_scope(mut s: ScopeStack) -> ScopeStack {
    s.push(Scope::new());
    s
}

fn end_scope(mut s: ScopeStack) -> ScopeStack {
    s.pop();
    s
}

fn define_on_current_scope(mut s: ScopeStack, v: &str) -> ScopeStack {
    let mut current = s.pop().unwrap_or(Scope::new());
    current.insert(v.to_string());
    s.push(current);
    s
}

#[derive(Default, Debug, PartialEq)]
/// ScopeAnalyzers walks the tree, ensuring that variables and scopes resolve
/// to the expected values.
pub struct ScopeAnalyzer {}

impl ScopeAnalyzer {
    // Instantiates a new ScopeAnalyzer
    pub fn new() -> Self {
        ScopeAnalyzer::default()
    }
}

impl Analyzer<(ScopeStack, &Vec<Stmt>), ScopeStack> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, input: (ScopeStack, &Vec<Stmt>)) -> ScopeAnalyzerResult {
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

impl Analyzer<(ScopeStack, &Stmt), ScopeStack> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, input: (ScopeStack, &Stmt)) -> ScopeAnalyzerResult {
        let (scope, stmt) = input;
        match stmt {
            &Stmt::Block(ref stmts) => self.resolve_block_stmt(scope, stmts),
            &Stmt::Declaration(ref name, _) => self.resolve_declaration_stmt(scope, name),
            _ => Err(ScopeAnalyzerErr::Unspecified),
        }
    }
}

// Resolves Stmt-related types.
impl ScopeAnalyzer {
    fn resolve_block_stmt(&self, scope: ScopeStack, stmts: &Vec<Stmt>) -> ScopeAnalyzerResult {
        self.analyze((begin_scope(scope), stmts))
            .map(|s| end_scope(s))
    }

    fn resolve_declaration_stmt(&self, scope: ScopeStack, name: &str) -> ScopeAnalyzerResult {
        Ok(define_on_current_scope(scope, name))
    }
}

// Expr Analyzer

impl Analyzer<(ScopeStack, Expr), ScopeStack> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, input: (ScopeStack, Expr)) -> ScopeAnalyzerResult {
        let (_scope, stmt) = input;
        match stmt {
            _ => Err(ScopeAnalyzerErr::Unspecified),
        }
    }
}

/// Unpack boxed-Expr
impl Analyzer<(ScopeStack, Box<Expr>), ScopeStack> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, input: (ScopeStack, Box<Expr>)) -> ScopeAnalyzerResult {
        let (scope, boxed_expr) = input;
        self.analyze((scope, *boxed_expr))
    }
}
