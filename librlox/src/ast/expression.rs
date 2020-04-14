use crate::scanner::tokens;

pub enum Expr {
    Binary(BinaryExpr),
    Unary(tokens::Token, Box<Expr>),
    Literal(tokens::Token),
    Grouping(Box<Expr>),
}

impl Expr {
    pub fn interpret(expr Box<Expr>) -> Box<Expr> {
        Box::new(expr)
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

pub struct LiteralExpr {
    expr: Box<Expr>,
}

impl LiteralExpr {
    pub fn new(expr: Box<Expr>) -> LiteralExpr {
        LiteralExpr {
            expr: expr,
        }
    }
}

pub struct GroupingExpr {
    expr: Box<Expr>,
}

impl GrouptingExpr {
    pub fn new(expr: Box<Expr>) -> BinaryExpr {
        GroupingExpr {
            expr: expr
        }
    }
}