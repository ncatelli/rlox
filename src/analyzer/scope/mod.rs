use crate::analyzer::SemanticAnalyzer;
use crate::ast::expression::{
    AdditionExpr, ComparisonExpr, EqualityExpr, Expr, LogicalExpr, MultiplicationExpr, UnaryExpr,
};
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
            Expr::Addition(expr) => self.analyze_addition(expr),
            Expr::Comparison(expr) => self.analyze_comparison(expr),
            Expr::Equality(expr) => self.analyze_equality(expr),
            Expr::Multiplication(expr) => self.analyze_multiplication(expr),
            Expr::Grouping(expr) => self.analyze(expr),
            Expr::Lambda(args, body) => self.analyze_lambda(args, body),
            Expr::Logical(expr) => self.analyze_logical(expr),
            Expr::Unary(expr) => self.analyze_unary(expr),
            Expr::Call(_id, _args) => todo!(),
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

    fn analyze_addition(&self, ae: &AdditionExpr) -> Result<usize, ScopeAnalyzerErr> {
        let (le, re) = match ae {
            AdditionExpr::Add(le, re) => (le, re),
            AdditionExpr::Subtract(le, re) => (le, re),
        };

        self.analyze(re)?;
        Ok(self.analyze(le)?)
    }

    fn analyze_multiplication(&self, me: &MultiplicationExpr) -> Result<usize, ScopeAnalyzerErr> {
        let (le, re) = match me {
            MultiplicationExpr::Divide(le, re) => (le, re),
            MultiplicationExpr::Multiply(le, re) => (le, re),
        };

        self.analyze(re)?;
        Ok(self.analyze(le)?)
    }

    fn analyze_comparison(&self, ce: &ComparisonExpr) -> Result<usize, ScopeAnalyzerErr> {
        let (le, re) = match ce {
            ComparisonExpr::Greater(le, re) => (le, re),
            ComparisonExpr::GreaterEqual(le, re) => (le, re),
            ComparisonExpr::Less(le, re) => (le, re),
            ComparisonExpr::LessEqual(le, re) => (le, re),
        };

        self.analyze(re)?;
        Ok(self.analyze(le)?)
    }

    fn analyze_equality(&self, ee: &EqualityExpr) -> Result<usize, ScopeAnalyzerErr> {
        let (le, re) = match ee {
            EqualityExpr::Equal(le, re) => (le, re),
            EqualityExpr::NotEqual(le, re) => (le, re),
        };

        self.analyze(re)?;
        Ok(self.analyze(le)?)
    }

    fn analyze_unary(&self, ue: &UnaryExpr) -> Result<usize, ScopeAnalyzerErr> {
        self.analyze(match ue {
            UnaryExpr::Bang(expr) => expr,
            UnaryExpr::Minus(expr) => expr,
        })
    }

    fn analyze_logical(&self, le: &LogicalExpr) -> Result<usize, ScopeAnalyzerErr> {
        let (le, re) = match le {
            LogicalExpr::And(le, re) => (le, re),
            LogicalExpr::Or(le, re) => (le, re),
        };

        self.analyze(re)?;
        Ok(self.analyze(le)?)
    }

    fn analyze_lambda(
        &self,
        args: &Vec<token::Token>,
        body: &Box<Stmt>,
    ) -> Result<usize, ScopeAnalyzerErr> {
        todo!()
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
            Stmt::While(expr, body) => self.analyze_while(expr, body),
            Stmt::Function(id, args, stmt) => self.analyze_function(id, args, stmt),
            Stmt::If(expr, tb, eb) => self.analyze_if(expr, tb, eb),
            Stmt::Print(expr) => self.analyze(expr).map(|_| None),
            Stmt::Return(expr) => self.analyze(expr).map(|_| None),
            Stmt::Expression(expr) => self.analyze(expr).map(|_| None),
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
        _args: &Vec<token::Token>,
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

    fn analyze_while(
        &self,
        cond: &Expr,
        body: &Box<Stmt>,
    ) -> Result<Option<Vec<Rc<Node>>>, ScopeAnalyzerErr> {
        self.analyze(cond)?;
        self.analyze(body)
    }
}
