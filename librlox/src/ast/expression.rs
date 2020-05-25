use crate::ast::token;
use crate::object;
use std::fmt;

/// Represents, and encapsulates one of the four types of expressions possible in
/// lox currently. Further information can be found on each sub-type.
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Equality(EqualityExpr),
    Comparison(ComparisonExpr),
    Addition(AdditionExpr),
    Multiplication(MultiplicationExpr),
    Unary(UnaryExpr),
    Primary(PrimaryExpr),
    Grouping(Box<Expr>),
    Variable(token::Token),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Equality(e) => write!(f, "{}", &e),
            Self::Comparison(e) => write!(f, "{}", &e),
            Self::Addition(e) => write!(f, "{}", &e),
            Self::Multiplication(e) => write!(f, "{}", &e),
            Self::Unary(e) => write!(f, "{}", &e),
            Self::Primary(e) => write!(f, "{}", &e),
            Self::Grouping(e) => write!(f, "(Grouping {})", &e),
            Self::Variable(i) => write!(f, "(Var {})", &i.lexeme.clone().unwrap()),
        }
    }
}

/// Represents Equality Lox expressions.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::ast::expression::*;
///
/// let comparison = Expr::Equality(
///     EqualityExpr::NotEqual(
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
#[derive(Debug, PartialEq, Clone)]
pub enum EqualityExpr {
    Equal(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),
}

impl fmt::Display for EqualityExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Equal(left, right) => write!(f, "(== {} {})", left, right),
            Self::NotEqual(left, right) => write!(f, "(!= {} {})", left, right),
        }
    }
}

/// Represents Comparison Lox expressions.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::ast::expression::*;
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
#[derive(Debug, PartialEq, Clone)]
pub enum ComparisonExpr {
    Less(Box<Expr>, Box<Expr>),
    LessEqual(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    GreaterEqual(Box<Expr>, Box<Expr>),
}

impl fmt::Display for ComparisonExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Less(left, right) => write!(f, "(< {} {})", left, right),
            Self::LessEqual(left, right) => write!(f, "(<= {} {})", left, right),
            Self::Greater(left, right) => write!(f, "(> {} {})", left, right),
            Self::GreaterEqual(left, right) => write!(f, "(>= {} {})", left, right),
        }
    }
}

/// Represents Addition Lox expressions.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::ast::expression::*;
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
#[derive(Debug, PartialEq, Clone)]
pub enum AdditionExpr {
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
}

impl fmt::Display for AdditionExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add(left, right) => write!(f, "(+ {} {})", left, right),
            Self::Subtract(left, right) => write!(f, "(- {} {})", left, right),
        }
    }
}

/// Represents Multiplication Lox expressions.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::ast::expression::*;
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
#[derive(Debug, PartialEq, Clone)]
pub enum MultiplicationExpr {
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
}

impl fmt::Display for MultiplicationExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Multiply(left, right) => write!(f, "(* {} {})", left, right),
            Self::Divide(left, right) => write!(f, "(/ {} {})", left, right),
        }
    }
}

/// Represents a unary Lox expressions.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::ast::expression::*;
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
#[derive(Debug, PartialEq, Clone)]
pub enum UnaryExpr {
    Bang(Box<Expr>),
    Minus(Box<Expr>),
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bang(expr) => write!(f, "(! {})", expr),
            Self::Minus(expr) => write!(f, "(- {})", expr),
        }
    }
}

/// Represents Literal Lox expressions and stores a single.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::ast::expression::*;
///
/// let primary = Expr::Primary(
///     PrimaryExpr::Number(5.0)
/// );
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum PrimaryExpr {
    Nil,
    True,
    False,
    Identifier(String),
    Str(String),
    Number(f64),
}

impl std::convert::From<bool> for PrimaryExpr {
    fn from(b: bool) -> Self {
        if b {
            PrimaryExpr::True
        } else {
            PrimaryExpr::False
        }
    }
}

impl std::convert::TryFrom<token::Token> for PrimaryExpr {
    type Error = String;

    fn try_from(t: token::Token) -> Result<Self, Self::Error> {
        match (t.token_type, t.object) {
            (token::TokenType::Nil, None) => Ok(PrimaryExpr::Nil),
            (token::TokenType::True, None) => Ok(PrimaryExpr::True),
            (token::TokenType::False, None) => Ok(PrimaryExpr::False),
            (token::TokenType::Identifier, Some(object::Object::Identifier(i))) => {
                Ok(PrimaryExpr::Identifier(i))
            }
            (token::TokenType::Str, Some(object::Object::Literal(object::Literal::Str(s)))) => {
                Ok(PrimaryExpr::Str(s))
            }
            (
                token::TokenType::Number,
                Some(object::Object::Literal(object::Literal::Number(n))),
            ) => Ok(PrimaryExpr::Number(n)),
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
                Self::Nil => "nil".to_string(),
                Self::False => "false".to_string(),
                Self::True => "true".to_string(),
                Self::Identifier(v) => v.clone(),
                Self::Str(v) => v.clone(),
                Self::Number(v) => format!("{}", v),
            }
        )
    }
}
