use crate::scanner::tokens;
use std::fmt;

pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(LiteralExpr),
    Grouping(GroupingExpr),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary(e) => write!(f, "{}", &e),
            Expr::Unary(e) => write!(f, "{}", &e),
            Expr::Literal(e) => write!(f, "{}", &e),
            Expr::Grouping(e) => write!(f, "{}", &e),
        }
    }
}

pub struct BinaryExpr {
    operation: tokens::Token,
    lhe: Box<Expr>,
    rhe: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(op: tokens::Token, lhe: Box<Expr>, rhe: Box<Expr>) -> BinaryExpr {
        BinaryExpr {
            operation: op,
            lhe: lhe,
            rhe: rhe,
        }
    }
}

impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.operation, self.lhe, self.rhe)
    }
}

pub struct UnaryExpr {
    operation: tokens::Token,
    expr: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(op: tokens::Token, expr: Box<Expr>) -> UnaryExpr {
        UnaryExpr {
            operation: op,
            expr: expr,
        }
    }
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.operation, self.expr)
    }
}

pub struct LiteralExpr {
    literal: Box<tokens::Token>,
}

impl LiteralExpr {
    pub fn new(literal: tokens::Token) -> LiteralExpr {
        LiteralExpr {
            literal: Box::new(literal),
        }
    }
}

impl fmt::Display for LiteralExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.literal)
    }
}

pub struct GroupingExpr {
    expr: Box<Expr>,
}

impl GroupingExpr {
    pub fn new(expr: Box<Expr>) -> GroupingExpr {
        GroupingExpr { expr: expr }
    }
}

impl fmt::Display for GroupingExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Grouping {})", self.expr)
    }
}
