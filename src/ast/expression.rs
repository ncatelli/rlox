use crate::ast::statement;
use crate::ast::token;
use crate::object;
use std::convert;
use std::fmt;

/// Identifier functions as a replacement for variable names, offering a raw Id and a Hash.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Identifier {
    Hash(String), // todo change this to an actual hash
    Id(String),
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(ref s) => write!(f, "{}", s),
            Self::Hash(ref s) => write!(f, "{}", s),
        }
    }
}

impl convert::TryFrom<token::Token> for Identifier {
    type Error = &'static str;

    fn try_from(tok: token::Token) -> Result<Identifier, Self::Error> {
        match tok.lexeme {
            Some(lexeme) => Ok(Identifier::Id(lexeme)),
            None => Err("cannot convert token to identifier, lexeme not defined"),
        }
    }
}

impl From<&str> for Identifier {
    fn from(from: &str) -> Identifier {
        Identifier::Id(from.to_string())
    }
}

impl From<String> for Identifier {
    fn from(from: String) -> Identifier {
        Identifier::Id(from)
    }
}

/// Represents, and encapsulates one of the four types of expressions possible in
/// lox currently. Further information can be found on each sub-type.
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Assignment(Identifier, Box<Expr>),
    Logical(LogicalExpr),
    Equality(EqualityExpr),
    Comparison(ComparisonExpr),
    Addition(AdditionExpr),
    Multiplication(MultiplicationExpr),
    Unary(UnaryExpr),
    Call(Box<Expr>, Vec<Expr>),
    Primary(object::Object),
    Grouping(Box<Expr>),
    Lambda(Vec<Identifier>, Box<statement::Stmt>),
    Variable(Identifier),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assignment(i, e) => write!(f, "(= {:?} {})", &i, e),
            Self::Logical(e) => write!(f, "{}", &e),
            Self::Equality(e) => write!(f, "{}", &e),
            Self::Comparison(e) => write!(f, "{}", &e),
            Self::Addition(e) => write!(f, "{}", &e),
            Self::Multiplication(e) => write!(f, "{}", &e),
            Self::Unary(e) => write!(f, "{}", &e),
            Self::Primary(e) => write!(f, "{}", &e),
            Self::Grouping(e) => write!(f, "(Grouping {})", &e),
            Self::Variable(i) => write!(f, "(Var {:?})", &i),
            Self::Lambda(params, body) => write!(
                f,
                "(Lambda ({}) {})",
                params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
                body
            ),
            Self::Call(callee, args) => write!(
                f,
                "{}({})",
                callee,
                args.iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        }
    }
}

/// Represents Logical Lox expressions.
///
/// # Examples
/// ```
/// use rlox::ast::expression::*;
/// use rlox::object;
///
/// let Logical = Expr::Logical(
///     LogicalExpr::Or(
///         Box::new(
///             Expr::Primary(
///                 object::Object::Literal(
///                     object::Literal::Bool(false)
///                 )
///             )
///         ),
///         Box::new(
///             Expr::Primary(
///                 object::Object::Literal(
///                     object::Literal::Bool(true)
///                 )
///             )
///         ),
///     )
/// );
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum LogicalExpr {
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
}

impl fmt::Display for LogicalExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Or(left, right) => write!(f, "(or {} {})", left, right),
            Self::And(left, right) => write!(f, "(and {} {})", left, right),
        }
    }
}

/// Represents Equality Lox expressions.
///
/// # Examples
/// ```
/// use rlox::ast::expression::*;
/// use rlox::object;
///
/// let comparison = Expr::Equality(
///     EqualityExpr::NotEqual(
///         Box::new(
///             Expr::Primary(
///                 object::Object::Literal(
///                     object::Literal::Number(5.0)
///                 )
///             )
///         ),
///         Box::new(
///             Expr::Primary(
///                 object::Object::Literal(
///                     object::Literal::Number(5.0)
///                 )
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
/// use rlox::ast::expression::*;
/// use rlox::object;
///
/// let comparison = Expr::Comparison(
///     ComparisonExpr::GreaterEqual(
///         Box::new(
///             Expr::Primary(
///                 object::Object::Literal(
///                     object::Literal::Number(5.0)
///                 )
///             )
///         ),
///         Box::new(
///             Expr::Primary(
///                 object::Object::Literal(
///                     object::Literal::Number(5.0)
///                 )
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
/// use rlox::ast::expression::*;
/// use rlox::object;
///
/// let addition = Expr::Addition(
///     AdditionExpr::Add(
///         Box::new(
///             Expr::Primary(
///                 object::Object::Literal(
///                     object::Literal::Number(5.0)
///                 )
///             )
///         ),
///         Box::new(
///             Expr::Primary(
///                 object::Object::Literal(
///                     object::Literal::Number(5.0)
///                 )
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
/// use rlox::ast::expression::*;
/// use rlox::object;
///
/// let multiplication = Expr::Multiplication(
///     MultiplicationExpr::Multiply(
///         Box::new(
///             Expr::Primary(
///                 object::Object::Literal(object::Literal::Number(5.0))
///             )
///         ),
///         Box::new(
///             Expr::Primary(
///                 object::Object::Literal(object::Literal::Number(5.0))
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
/// use rlox::ast::expression::*;
/// use rlox::object;
///
/// let unary = Expr::Unary(
///     UnaryExpr::Minus(
///         Box::new(
///             Expr::Primary(
///                 object::Object::Literal(object::Literal::Number(5.0))
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

#[allow(unused_macros)]
macro_rules! identifier_id {
    ($id:expr) => {
        $crate::ast::expression::Identifier::Id($id.to_string())
    };
}
