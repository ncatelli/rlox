use crate::scanner::tokens;
use std::fmt;

/// Represents, and encapsulates one of the four types of expressions possible in
/// lox currently. Further information can be found on each sub-type.
///
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

/// Represents Binary Lox expressions and stores an operation token, along with
/// Boxed left and right hand expressions.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::scanner::tokens::{Literal, TokenType, Token};
/// use librlox::ast::expression::*;
/// use std::option::Option::Some;
///
/// let unary = Expr::Binary(
///     BinaryExpr::new(
///         Token::new(TokenType::Minus, None),
///         Box::new(
///             Expr::Literal(
///                 LiteralExpr::new(
///                     Token::new(TokenType::Number, Some(Literal::Number(10.0)))
///                 )
///             )
///         ),
///         Box::new(
///             Expr::Literal(
///                 LiteralExpr::new(
///                     Token::new(TokenType::Number, Some(Literal::Number(5.0)))
///                 )
///             )
///         ),
///     )
/// );
/// ```
pub struct BinaryExpr {
    operation: tokens::Token,
    lhe: Box<Expr>,
    rhe: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(op: tokens::Token, lhe: Box<Expr>, rhe: Box<Expr>) -> BinaryExpr {
        BinaryExpr {
            operation: op,
            lhe,
            rhe,
        }
    }
}

impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.operation, self.lhe, self.rhe)
    }
}

/// Represents Unary Lox expressions and stores an operation token, along with
/// a single, right hand, expression.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::scanner::tokens::{Literal, TokenType, Token};
/// use librlox::ast::expression::*;
/// use std::option::Option::Some;
///
/// let unary = Expr::Unary(
///     UnaryExpr::new(
///         Token::new(TokenType::Minus, None),
///         Box::new(
///             Expr::Literal(
///                 LiteralExpr::new(
///                     Token::new(TokenType::Number, Some(Literal::Number(5.0)))
///                 )
///             )
///         ),
///     )
/// );
/// ```
pub struct UnaryExpr {
    operation: tokens::Token,
    expr: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(op: tokens::Token, expr: Box<Expr>) -> UnaryExpr {
        UnaryExpr {
            operation: op,
            expr,
        }
    }
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.operation, self.expr)
    }
}

/// Represents Literal Lox expressions and stores a single literal token value.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::scanner::tokens::{Literal, TokenType, Token};
/// use librlox::ast::expression::*;
/// use std::option::Option::Some;
///
/// let literal = Expr::Literal(
///     LiteralExpr::new(
///         Token::new(TokenType::Number, Some(Literal::Number(5.0)))
///     )
/// );
/// ```
pub struct LiteralExpr {
    literal: tokens::Token,
}

impl LiteralExpr {
    pub fn new(literal: tokens::Token) -> LiteralExpr {
        LiteralExpr { literal }
    }
}

impl fmt::Display for LiteralExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.literal)
    }
}

/// Acts as a logical grouping for sub-expressions taking a single boxed
/// expression.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::scanner::tokens::{Literal, TokenType, Token};
/// use librlox::ast::expression::*;
/// use std::option::Option::Some;
///
/// let grouping = Expr::Grouping(
///     GroupingExpr::new(
///         Box::new(
///             Expr::Literal(
///                 LiteralExpr::new(
///                     Token::new(TokenType::Number, Some(Literal::Number(5.0)))
///                 )
///             )
///         ),
///     )
/// );
/// ```
pub struct GroupingExpr {
    expr: Box<Expr>,
}

impl GroupingExpr {
    pub fn new(expr: Box<Expr>) -> GroupingExpr {
        GroupingExpr { expr }
    }
}

impl fmt::Display for GroupingExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Grouping {})", self.expr)
    }
}
