use crate::interpreter;
use crate::scanner::tokens;
use std::fmt;

#[cfg(test)]
mod tests;

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
            Self::Equality(e) => write!(f, "{}", &e),
            Self::Comparison(e) => write!(f, "{}", &e),
            Self::Addition(e) => write!(f, "{}", &e),
            Self::Multiplication(e) => write!(f, "{}", &e),
            Self::Unary(e) => write!(f, "{}", &e),
            Self::Primary(e) => write!(f, "{}", &e),
            Self::Grouping(e) => write!(f, "{}", &e),
        }
    }
}

// TODO
impl interpreter::Interpreter<PrimaryExpr> for Expr {
    fn interpret(&self) -> Result<PrimaryExpr, interpreter::InterpreterErr> {
        match self {
            Self::Primary(expr) => expr.interpret(),
            Self::Unary(expr) => expr.interpret(),
            Self::Multiplication(expr) => expr.interpret(),
            _ => Ok(PrimaryExpr::Number(100.0)),
        }
    }
}

/// Represents Equality Lox expressions.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::scanner::tokens::{Literal, TokenType, Token};
/// use librlox::parser::expression::*;
/// use std::option::Option::Some;
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
#[derive(Debug, PartialEq)]
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
            Self::Multiply(left, right) => write!(f, "(* {} {})", left, right),
            Self::Divide(left, right) => write!(f, "(/ {} {})", left, right),
        }
    }
}

impl interpreter::Interpreter<PrimaryExpr> for MultiplicationExpr {
    fn interpret(&self) -> Result<PrimaryExpr, interpreter::InterpreterErr> {
        match self {
            Self::Multiply(left, right) => match (left.interpret(), right.interpret()) {
                (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                    Ok(PrimaryExpr::Number(left_val * right_val))
                }
                (Ok(l), Ok(r)) => Err(interpreter::InterpreterErr::TypeErr(format!(
                    "Invalid operand for operator: {} * {}",
                    l, r
                ))),
                _ => Err(interpreter::InterpreterErr::TypeErr(format!(
                    "Invalid operand for operator"
                ))),
            },
            Self::Divide(left, right) => match (left.interpret(), right.interpret()) {
                (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                    Ok(PrimaryExpr::Number(left_val / right_val))
                }
                (Ok(l), Ok(r)) => Err(interpreter::InterpreterErr::TypeErr(format!(
                    "Invalid operand for operator: {} / {}",
                    l, r
                ))),
                _ => Err(interpreter::InterpreterErr::TypeErr(format!(
                    "Invalid operand for operator"
                ))),
            },
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
            Self::Bang(expr) => write!(f, "(! {})", expr),
            Self::Minus(expr) => write!(f, "(- {})", expr),
        }
    }
}

impl interpreter::Interpreter<PrimaryExpr> for UnaryExpr {
    fn interpret(&self) -> Result<PrimaryExpr, interpreter::InterpreterErr> {
        match self {
            Self::Bang(expr) => match expr.interpret() {
                Ok(PrimaryExpr::False) => Ok(PrimaryExpr::True),
                Ok(PrimaryExpr::True) => Ok(PrimaryExpr::False),
                Err(e) => Err(interpreter::InterpreterErr::TypeErr(e.to_string())),
                _ => Err(interpreter::InterpreterErr::TypeErr(format!(
                    "Invalid operand for operator"
                ))),
            },
            Self::Minus(expr) => match expr.interpret() {
                Ok(PrimaryExpr::Number(v)) => Ok(PrimaryExpr::Number(v * -1.0)),
                Err(e) => Err(interpreter::InterpreterErr::TypeErr(e.to_string())),
                _ => Err(interpreter::InterpreterErr::TypeErr(format!(
                    "Invalid operand for operator"
                ))),
            },
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
                Self::False => "false".to_string(),
                Self::True => "true".to_string(),
                Self::Identifier(v) => v.clone(),
                Self::Str(v) => v.clone(),
                Self::Number(v) => format!("{}", v),
            }
        )
    }
}

impl interpreter::Interpreter<PrimaryExpr> for PrimaryExpr {
    fn interpret(&self) -> Result<PrimaryExpr, interpreter::InterpreterErr> {
        Ok(self.to_owned())
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
