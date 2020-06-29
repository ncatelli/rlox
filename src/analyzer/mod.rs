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

#[derive(PartialEq, Debug)]
/// ScopeStack wraps a Vec<Scope> and provides basic interations for interacting
/// with a stack of scopes, including pushing, popping, loading and storing
/// values.
pub struct ScopeStack {
    stack: Vec<Scope>,
}

impl ScopeStack {
    /// Instantiates a stack of scopes, creating the initial scope
    pub fn new() -> Self {
        ScopeStack {
            stack: vec![Scope::new()],
        }
    }

    /// Pushes a scope (s) onto the stack.
    pub fn push(self, s: Scope) -> Self {
        let mut vs: Vec<Scope> = Self::into(self);
        vs.push(s);
        vs.into()
    }

    /// pops a scope off the stack, returning the new stack and the scope.
    pub fn pop(self) -> (Self, Scope) {
        let mut vs: Vec<Scope> = Self::into(self);
        let scope = vs.pop().unwrap_or(Scope::new());
        (vs.into(), scope)
    }

    /// define writes a value into the current (top of stack) scope.
    pub fn define(self, v: &str) -> Self {
        let mut vs: Vec<Scope> = Self::into(self);
        let mut scope = vs.pop().unwrap_or(Scope::new());
        scope.insert(v.to_string());
        Self::from(vs).push(scope)
    }
}

impl From<Vec<Scope>> for ScopeStack {
    fn from(v: Vec<Scope>) -> Self {
        ScopeStack { stack: v }
    }
}

impl From<ScopeStack> for Vec<Scope> {
    fn from(ss: ScopeStack) -> Self {
        ss.stack
    }
}

pub type ScopeAnalyzerResult = Result<ScopeStack, ScopeAnalyzerErr>;

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
    fn resolve_block_stmt(&self, ss: ScopeStack, stmts: &Vec<Stmt>) -> ScopeAnalyzerResult {
        self.analyze((ss.push(Scope::new()), stmts))
            .map(|s| s.pop().0)
    }

    fn resolve_declaration_stmt(&self, scope: ScopeStack, name: &str) -> ScopeAnalyzerResult {
        Ok(scope.define(name))
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
