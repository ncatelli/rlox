use crate::scanner::tokens;
use std::fmt;

/// Represents, and encapsulates one of the four types of expressions possible in
/// lox currently. Further information can be found on each sub-type.
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
///                 PrimaryExpr::Number(5.0)
///             )
///         ),
///         Box::new(
///             Expr::Primary(
///                 PrimaryExpr::Number(5.0)
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

/// Represents Comparison Lox expressions.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::scanner::tokens::{Literal, TokenType, Token};
/// use librlox::parser::expression::*;
/// use std::option::Option::Some;
///
/// let comparison = Expr::Comparison(
///     ComparisonExpr::GreaterEqual(
///         Box::new(
///             Expr::Primary(
///                 PrimaryExpr::Number(5.0)
///             )
///         ),
///         Box::new(
///             Expr::Primary(
///                 PrimaryExpr::Number(5.0)
///             )
///         ),
///     )
/// );
/// ```
#[derive(Debug, PartialEq)]
pub enum ComparisonExpr {
    Less(Box<Expr>, Box<Expr>),
    LessEqual(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    GreaterEqual(Box<Expr>, Box<Expr>),
}

impl fmt::Display for ComparisonExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComparisonExpr::Less(left, right) => write!(f, "(< {} {})", left, right),
            ComparisonExpr::LessEqual(left, right) => write!(f, "(<= {} {})", left, right),
            ComparisonExpr::Greater(left, right) => write!(f, "(> {} {})", left, right),
            ComparisonExpr::GreaterEqual(left, right) => write!(f, "(>= {} {})", left, right),
        }
    }
}

/// Represents Addition Lox expressions.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::scanner::tokens::{Literal, TokenType, Token};
/// use librlox::parser::expression::*;
/// use std::option::Option::Some;
///
/// let addition = Expr::Addition(
///     AdditionExpr::Add(
///         Box::new(
///             Expr::Primary(
///                 PrimaryExpr::Number(5.0)
///             )
///         ),
///         Box::new(
///             Expr::Primary(
///                 PrimaryExpr::Number(5.0)
///             )
///         ),
///     )
/// );
/// ```
#[derive(Debug, PartialEq)]
pub enum AdditionExpr {
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
}

impl fmt::Display for AdditionExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdditionExpr::Add(left, right) => write!(f, "(+ {} {})", left, right),
            AdditionExpr::Subtract(left, right) => write!(f, "(- {} {})", left, right),
        }
    }
}

/// Represents Multiplication Lox expressions.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::scanner::tokens::{Literal, TokenType, Token};
/// use librlox::parser::expression::*;
/// use std::option::Option::Some;
///
/// let multiplication = Expr::Multiplication(
///     MultiplicationExpr::Multiply(
///         Box::new(
///             Expr::Primary(
///                 PrimaryExpr::Number(5.0)
///             )
///         ),
///         Box::new(
///             Expr::Primary(
///                 PrimaryExpr::Number(5.0)
///             )
///         ),
///     )
/// );
/// ```
#[derive(Debug, PartialEq)]
pub enum MultiplicationExpr {
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
}

impl fmt::Display for MultiplicationExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MultiplicationExpr::Multiply(left, right) => write!(f, "(* {} {})", left, right),
            MultiplicationExpr::Divide(left, right) => write!(f, "(/ {} {})", left, right),
        }
    }
}

/// Represents a unary Lox expressions.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::scanner::tokens::{Literal, TokenType, Token};
/// use librlox::parser::expression::*;
/// use std::option::Option::Some;
///
/// let unary = Expr::Unary(
///     UnaryExpr::Minus(
///         Box::new(
///             Expr::Primary(
///                 PrimaryExpr::Number(5.0)
///             )
///         )
///     )
/// );
/// ```
#[derive(Debug, PartialEq)]
pub enum UnaryExpr {
    Bang(Box<Expr>),
    Minus(Box<Expr>),
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryExpr::Bang(expr) => write!(f, "(! {})", expr),
            UnaryExpr::Minus(expr) => write!(f, "(- {})", expr),
        }
    }
}

/// Represents Literal Lox expressions and stores a single.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::scanner::tokens::{Literal, TokenType, Token};
/// use librlox::parser::expression::*;
/// use std::option::Option::Some;
///
/// let primary = Expr::Primary(
///     PrimaryExpr::Number(5.0)
/// );
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum PrimaryExpr {
    True,
    False,
    Identifier(String),
    Str(String),
    Number(f64),
}

impl std::convert::TryFrom<tokens::Token> for PrimaryExpr {
    type Error = String;

    fn try_from(t: tokens::Token) -> Result<Self, Self::Error> {
        match (t.token_type, t.literal) {
            (tokens::TokenType::True, None) => Ok(PrimaryExpr::True),
            (tokens::TokenType::False, None) => Ok(PrimaryExpr::False),
            (tokens::TokenType::Literal, Some(tokens::Literal::Identifier(v))) => {
                Ok(PrimaryExpr::Identifier(v))
            }
            (tokens::TokenType::Literal, Some(tokens::Literal::Str(v))) => Ok(PrimaryExpr::Str(v)),
            (tokens::TokenType::Literal, Some(tokens::Literal::Number(v))) => {
                Ok(PrimaryExpr::Number(v))
            }
            // Placeholder
            _ => Err(format!("invalid token: {}", t.token_type)),
        }
    }
}

impl fmt::Display for PrimaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PrimaryExpr::False => "false".to_string(),
                PrimaryExpr::True => "true".to_string(),
                PrimaryExpr::Identifier(v) => v.clone(),
                PrimaryExpr::Str(v) => v.clone(),
                PrimaryExpr::Number(v) => format!("{}", v),
            }
        )
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
///                 PrimaryExpr::Number(5.0)
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
