use crate::analyzer::SemanticAnalyzer;
use crate::ast::expression::Expr;
use crate::ast::identifier::Identifier;
use crate::environment::Environment;
use std::cell::Cell;
use std::fmt;
use std::rc::Rc;

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
    pub offset: Cell<usize>,
    pub env: Rc<Environment<Identifier, usize>>,
}

impl ScopeAnalyzer {
    pub fn new() -> ScopeAnalyzer {
        ScopeAnalyzer {
            offset: Cell::new(0),
            env: Environment::new(),
        }
    }

    pub fn from(&self, env: Rc<Environment<Identifier, usize>>) -> ScopeAnalyzer {
        ScopeAnalyzer {
            offset: Cell::new(self.offset.get()),
            env: env,
        }
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

pub type StmtSemanticAnalyzerResult = Result<Stmt, ScopeAnalyzerErr>;

impl SemanticAnalyzer<Vec<Stmt>, Vec<Stmt>> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, input: Vec<Stmt>) -> Result<Vec<Stmt>, ScopeAnalyzerErr> {
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
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, input: Stmt) -> StmtSemanticAnalyzerResult {
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
impl SemanticAnalyzer<Box<Stmt>, Stmt> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;
    fn analyze(&self, input: Box<Stmt>) -> StmtSemanticAnalyzerResult {
        self.analyze(*input)
    }
}

impl ScopeAnalyzer {
    fn declare_or_assign(&self, id: Identifier) -> Identifier {
        if self.env.has_key(&id) {
            let offset = self.env.get(&id).unwrap();
            Identifier::Id(offset)
        } else {
            let offset = self.offset.get();
            self.env.define(&id, self.offset.replace(offset + 1));
            Identifier::Id(offset)
        }
    }

    fn analyze_block(&self, stmts: Vec<Stmt>) -> StmtSemanticAnalyzerResult {
        let block_analyzer = self.from(Environment::from(&self.env));
        Ok(Stmt::Block(block_analyzer.analyze(stmts)?))
    }

    fn analyze_function(
        &self,
        fname: Identifier,
        params: Vec<Identifier>,
        body: Box<Stmt>,
    ) -> StmtSemanticAnalyzerResult {
        let fid = self.declare_or_assign(fname);

        let body_analyzer = self.from(Environment::from(&self.env));
        let param_ids: Vec<Identifier> = params
            .into_iter()
            .map(|param| body_analyzer.declare_or_assign(param))
            .collect();
        let analyzed_body = body_analyzer.analyze(body)?;

        Ok(Stmt::Function(fid, param_ids, Box::new(analyzed_body)))
    }

    fn analyze_if(
        &self,
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

    fn analyze_declaration(&self, id: Identifier, expr: Expr) -> StmtSemanticAnalyzerResult {
        match self.analyze(expr) {
            Ok(e) => Ok(Stmt::Declaration(self.declare_or_assign(id), e)),
            Err(e) => Err(e),
        }
    }
}
