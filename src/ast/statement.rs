use crate::ast::expression::Expr;
use crate::ast::token;
use std::fmt;

/// Represents, and encapsulates statement types possiblepossible in
/// lox currently. Further information can be found on each sub-type.
#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Expression(Expr),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    Print(Expr),
    Function(String, Vec<token::Token>, Box<Stmt>),
    Declaration(String, Expr),
    Return(Expr),
    Block(Vec<Stmt>),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expression(e) => write!(f, "(Expression {})", &e),
            Self::If(e, tb, eb) => match eb {
                Some(eb) => write!(f, "(if ({}) ({}) ({}))", &e, &tb, &eb),
                None => write!(f, "(if ({}) ({}))", &e, &tb),
            },
            Self::While(e, stmt) => write!(f, "(While ({}) ({})", e, stmt),
            Self::Print(e) => write!(f, "(Print {})", &e),
            Self::Function(name, params, block) => {
                write!(f, "(Fun {}({:?}) {}", &name, &params, &block)
            }
            Self::Declaration(name, e) => write!(f, "(Declaration {} {}", &name, &e),
            Self::Return(e) => write!(f, "(Return {})", &e),
            Self::Block(stmts) => write!(f, "(Block {:?})", stmts),
        }
    }
}
