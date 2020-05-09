use crate::scanner::tokens;
use crate::utils::folder;
use std::fmt;

pub trait BinaryExpr {
    fn left(&self) -> &Expr;
    fn right(&self) -> &Expr;
}

/// Represents, and encapsulates one of the four types of expressions possible in
/// lox currently. Further information can be found on each sub-type.
///
#[derive(Debug, PartialEq)]
pub enum Expr {
    Equality(EqualityExpr),
    Comparison(ComparisonExpr),
    Addition(AdditionExpr),
    Multiplication(MultiplicationExpr),
    Unary(UnaryExpr),
    Primary(PrimaryExpr),
    Grouping(GroupingExpr),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Equality(e) => write!(f, "{}", &e),
            Expr::Comparison(e) => write!(f, "{}", &e),
            Expr::Addition(e) => write!(f, "{}", &e),
            Expr::Multiplication(e) => write!(f, "{}", &e),
            Expr::Unary(e) => write!(f, "{}", &e),
            Expr::Primary(e) => write!(f, "{}", &e),
            Expr::Grouping(e) => write!(f, "{}", &e),
        }
    }
}

// TODO finish implementing actual calculations on type
impl folder::Folder<Expr> for Expr {
    fn fold(&self) -> Expr {
        todo!()
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EqualityExprOperator {
    Equal,
    NotEqual,
}

impl EqualityExprOperator {
    pub fn from_token(token: tokens::Token) -> Result<EqualityExprOperator, String> {
        match token.token_type {
            tokens::TokenType::EqualEqual => Ok(EqualityExprOperator::Equal),
            tokens::TokenType::BangEqual => Ok(EqualityExprOperator::NotEqual),
            _ => Err(format!(
                "Unable to convert from {} to EqualityExprOperator",
                token.token_type
            )),
        }
    }
}

impl fmt::Display for EqualityExprOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EqualityExprOperator::Equal => write!(f, "=="),
            EqualityExprOperator::NotEqual => write!(f, "!="),
        }
    }
}

// Implement <EqualityExprOperator> == <tokens::Token>  comparisons
impl PartialEq<tokens::Token> for EqualityExprOperator {
    fn eq(&self, token: &tokens::Token) -> bool {
        match self {
            EqualityExprOperator::Equal => match token.token_type {
                tokens::TokenType::EqualEqual => true,
                _ => false,
            },
            EqualityExprOperator::NotEqual => match token.token_type {
                tokens::TokenType::BangEqual => true,
                _ => false,
            },
        }
    }
}

/// Represents Equality Lox expressions and stores an operation, along
/// with Boxed left and right hand expressions.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::scanner::tokens::{Literal, TokenType, Token};
/// use librlox::parser::expression::*;
/// use std::option::Option::Some;
///
/// let comparison = Expr::Equality(
///     EqualityExpr::new(
///         EqualityExprOperator::NotEqual,
///         Box::new(
///             Expr::Primary(
///                 PrimaryExpr::new(
///                     Token::new(TokenType::Number, Some(Literal::Number(10.0)))
///                 )
///             )
///         ),
///         Box::new(
///             Expr::Primary(
///                 PrimaryExpr::new(
///                     Token::new(TokenType::Number, Some(Literal::Number(5.0)))
///                 )
///             )
///         ),
///     )
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct EqualityExpr {
    operation: EqualityExprOperator,
    lhe: Box<Expr>,
    rhe: Box<Expr>,
}

impl EqualityExpr {
    pub fn new(op: EqualityExprOperator, lhe: Box<Expr>, rhe: Box<Expr>) -> EqualityExpr {
        EqualityExpr {
            operation: op,
            lhe,
            rhe,
        }
    }
}

impl fmt::Display for EqualityExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.operation, self.lhe, self.rhe)
    }
}

impl BinaryExpr for EqualityExpr {
    fn left(&self) -> &Expr {
        &self.lhe
    }

    fn right(&self) -> &Expr {
        &self.rhe
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ComparisonExprOperator {
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl ComparisonExprOperator {
    pub fn from_token(token: tokens::Token) -> Result<ComparisonExprOperator, String> {
        match token.token_type {
            tokens::TokenType::Greater => Ok(ComparisonExprOperator::Greater),
            tokens::TokenType::GreaterEqual => Ok(ComparisonExprOperator::GreaterEqual),
            tokens::TokenType::Less => Ok(ComparisonExprOperator::Less),
            tokens::TokenType::LessEqual => Ok(ComparisonExprOperator::LessEqual),
            _ => Err(format!(
                "Unable to convert from {} to ComparisonExprOperator",
                token.token_type
            )),
        }
    }
}

impl fmt::Display for ComparisonExprOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComparisonExprOperator::Greater => write!(f, ">"),
            ComparisonExprOperator::GreaterEqual => write!(f, ">="),
            ComparisonExprOperator::Less => write!(f, "<"),
            ComparisonExprOperator::LessEqual => write!(f, "<="),
        }
    }
}

// Implement <ComparisonExprOperator> == <tokens::Token>  comparisons
impl PartialEq<tokens::Token> for ComparisonExprOperator {
    fn eq(&self, token: &tokens::Token) -> bool {
        match self {
            ComparisonExprOperator::Greater => match token.token_type {
                tokens::TokenType::Greater => true,
                _ => false,
            },
            ComparisonExprOperator::GreaterEqual => match token.token_type {
                tokens::TokenType::GreaterEqual => true,
                _ => false,
            },
            ComparisonExprOperator::Less => match token.token_type {
                tokens::TokenType::Less => true,
                _ => false,
            },
            ComparisonExprOperator::LessEqual => match token.token_type {
                tokens::TokenType::LessEqual => true,
                _ => false,
            },
        }
    }
}

/// Represents Comparison Lox expressions and stores an operation, along
/// with Boxed left and right hand expressions.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::scanner::tokens::{Literal, TokenType, Token};
/// use librlox::parser::expression::*;
/// use std::option::Option::Some;
///
/// let comparison = Expr::Comparison(
///     ComparisonExpr::new(
///         ComparisonExprOperator::GreaterEqual,
///         Box::new(
///             Expr::Primary(
///                 PrimaryExpr::new(
///                     Token::new(TokenType::Number, Some(Literal::Number(10.0)))
///                 )
///             )
///         ),
///         Box::new(
///             Expr::Primary(
///                 PrimaryExpr::new(
///                     Token::new(TokenType::Number, Some(Literal::Number(5.0)))
///                 )
///             )
///         ),
///     )
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct ComparisonExpr {
    operation: ComparisonExprOperator,
    lhe: Box<Expr>,
    rhe: Box<Expr>,
}

impl ComparisonExpr {
    pub fn new(op: ComparisonExprOperator, lhe: Box<Expr>, rhe: Box<Expr>) -> ComparisonExpr {
        ComparisonExpr {
            operation: op,
            lhe,
            rhe,
        }
    }
}

impl fmt::Display for ComparisonExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.operation, self.lhe, self.rhe)
    }
}

impl BinaryExpr for ComparisonExpr {
    fn left(&self) -> &Expr {
        &self.lhe
    }

    fn right(&self) -> &Expr {
        &self.rhe
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AdditionExprOperator {
    Addition,
    Subraction,
}

impl AdditionExprOperator {
    pub fn from_token(token: tokens::Token) -> Result<AdditionExprOperator, String> {
        match token.token_type {
            tokens::TokenType::Plus => Ok(AdditionExprOperator::Addition),
            tokens::TokenType::Minus => Ok(AdditionExprOperator::Subraction),
            _ => Err(format!(
                "Unable to convert from {} to AdditionExprOperator",
                token.token_type
            )),
        }
    }
}

impl fmt::Display for AdditionExprOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdditionExprOperator::Addition => write!(f, "+"),
            AdditionExprOperator::Subraction => write!(f, "-"),
        }
    }
}

// Implement <AdditionExprOperator> == <tokens::Token>  comparisons
impl PartialEq<tokens::Token> for AdditionExprOperator {
    fn eq(&self, token: &tokens::Token) -> bool {
        match self {
            AdditionExprOperator::Addition => match token.token_type {
                tokens::TokenType::Plus => true,
                _ => false,
            },
            AdditionExprOperator::Subraction => match token.token_type {
                tokens::TokenType::Minus => true,
                _ => false,
            },
        }
    }
}

/// Represents Addition Lox expressions and stores an operation, along
/// with Boxed left and right hand expressions.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::scanner::tokens::{Literal, TokenType, Token};
/// use librlox::parser::expression::*;
/// use std::option::Option::Some;
///
/// let addition = Expr::Addition(
///     AdditionExpr::new(
///         AdditionExprOperator::Addition,
///         Box::new(
///             Expr::Primary(
///                 PrimaryExpr::new(
///                     Token::new(TokenType::Number, Some(Literal::Number(10.0)))
///                 )
///             )
///         ),
///         Box::new(
///             Expr::Primary(
///                 PrimaryExpr::new(
///                     Token::new(TokenType::Number, Some(Literal::Number(5.0)))
///                 )
///             )
///         ),
///     )
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct AdditionExpr {
    operation: AdditionExprOperator,
    lhe: Box<Expr>,
    rhe: Box<Expr>,
}

impl AdditionExpr {
    pub fn new(op: AdditionExprOperator, lhe: Box<Expr>, rhe: Box<Expr>) -> AdditionExpr {
        AdditionExpr {
            operation: op,
            lhe,
            rhe,
        }
    }
}

impl fmt::Display for AdditionExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.operation, self.lhe, self.rhe)
    }
}

impl BinaryExpr for AdditionExpr {
    fn left(&self) -> &Expr {
        &self.lhe
    }

    fn right(&self) -> &Expr {
        &self.rhe
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MultiplicationExprOperator {
    Multiply,
    Divide,
}

impl MultiplicationExprOperator {
    pub fn from_token(token: tokens::Token) -> Result<MultiplicationExprOperator, String> {
        match token.token_type {
            tokens::TokenType::Star => Ok(MultiplicationExprOperator::Multiply),
            tokens::TokenType::Slash => Ok(MultiplicationExprOperator::Divide),
            _ => Err(format!(
                "Unable to convert from {} to MultiplicationExprOperator",
                token.token_type
            )),
        }
    }
}

impl fmt::Display for MultiplicationExprOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MultiplicationExprOperator::Multiply => write!(f, "*"),
            MultiplicationExprOperator::Divide => write!(f, "/"),
        }
    }
}

// Implement <MultiplicationExprOperator> == <tokens::Token>  comparisons
impl PartialEq<tokens::Token> for MultiplicationExprOperator {
    fn eq(&self, token: &tokens::Token) -> bool {
        match self {
            MultiplicationExprOperator::Multiply => match token.token_type {
                tokens::TokenType::Star => true,
                _ => false,
            },
            MultiplicationExprOperator::Divide => match token.token_type {
                tokens::TokenType::Slash => true,
                _ => false,
            },
        }
    }
}

/// Represents Multiplication Lox expressions and stores an operation, along
/// with Boxed left and right hand expressions.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::scanner::tokens::{Literal, TokenType, Token};
/// use librlox::parser::expression::*;
/// use std::option::Option::Some;
///
/// let multiplication = Expr::Multiplication(
///     MultiplicationExpr::new(
///         MultiplicationExprOperator::Multiply,
///         Box::new(
///             Expr::Primary(
///                 PrimaryExpr::new(
///                     Token::new(TokenType::Number, Some(Literal::Number(10.0)))
///                 )
///             )
///         ),
///         Box::new(
///             Expr::Primary(
///                 PrimaryExpr::new(
///                     Token::new(TokenType::Number, Some(Literal::Number(5.0)))
///                 )
///             )
///         ),
///     )
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct MultiplicationExpr {
    operation: MultiplicationExprOperator,
    lhe: Box<Expr>,
    rhe: Box<Expr>,
}

impl MultiplicationExpr {
    pub fn new(
        op: MultiplicationExprOperator,
        lhe: Box<Expr>,
        rhe: Box<Expr>,
    ) -> MultiplicationExpr {
        MultiplicationExpr {
            operation: op,
            lhe,
            rhe,
        }
    }
}

impl fmt::Display for MultiplicationExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.operation, self.lhe, self.rhe)
    }
}

impl BinaryExpr for MultiplicationExpr {
    fn left(&self) -> &Expr {
        &self.lhe
    }

    fn right(&self) -> &Expr {
        &self.rhe
    }
}

/// Represents Unary Lox expressions and stores an operation token, along with
/// a single, right hand, expression.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::scanner::tokens::{Literal, TokenType, Token};
/// use librlox::parser::expression::*;
/// use std::option::Option::Some;
///
/// let unary = Expr::Unary(
///     UnaryExpr::new(
///         Token::new(TokenType::Minus, None),
///         Box::new(
///             Expr::Primary(
///                 PrimaryExpr::new(
///                     Token::new(TokenType::Number, Some(Literal::Number(5.0)))
///                 )
///             )
///         ),
///     )
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct UnaryExpr {
    operation: tokens::Token,
    rhe: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(operation: tokens::Token, rhe: Box<Expr>) -> UnaryExpr {
        UnaryExpr { operation, rhe }
    }
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.operation, self.rhe)
    }
}

/// Represents Literal Lox expressions and stores a single literal token value.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::scanner::tokens::{Literal, TokenType, Token};
/// use librlox::parser::expression::*;
/// use std::option::Option::Some;
///
/// let primary = Expr::Primary(
///     PrimaryExpr::new(
///         Token::new(TokenType::Number, Some(Literal::Number(5.0)))
///     )
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PrimaryExpr {
    literal: tokens::Token,
}

impl PrimaryExpr {
    pub fn new(literal: tokens::Token) -> PrimaryExpr {
        PrimaryExpr { literal }
    }
}

impl fmt::Display for PrimaryExpr {
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
/// use librlox::parser::expression::*;
/// use std::option::Option::Some;
///
/// let grouping = Expr::Grouping(
///     GroupingExpr::new(
///         Box::new(
///             Expr::Primary(
///                 PrimaryExpr::new(
///                     Token::new(TokenType::Number, Some(Literal::Number(5.0)))
///                 )
///             )
///         ),
///     )
/// );
/// ```
#[derive(Debug, PartialEq)]
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
