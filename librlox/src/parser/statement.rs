use crate::parser::expression::Expr;
use std::fmt;

/// Represents, and encapsulates statement types possiblepossible in
/// lox currently. Further information can be found on each sub-type.
#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expression(e) => write!(f, "(Expression {})", &e),
            Self::Print(e) => write!(f, "(Print {})", &e),
        }
    }
}
