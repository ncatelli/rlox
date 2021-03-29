extern crate parcel;
use crate::ast::expression::*;
use crate::ast::identifier::Identifier;
use crate::ast::token::{Token, TokenType};
use crate::parser::combinators::{token_type, unzip};
use crate::parser::statement_parser::block;
use parcel::*;
use std::convert::TryFrom;

/// Represents the entrypoint for expression parsing within the lox parser and
/// yields an Expr object after recursively descending through the expression
/// grammar
///
/// # Examples
/// ```
/// extern crate parcel;
/// use rlox::ast::token::{TokenType, Token};
/// use rlox::ast::expression::*;
/// use rlox::parser::expression_parser::*;
/// use rlox::object;
/// use std::option::Option::Some;
/// use std::convert::TryFrom;
/// use parcel::*;
///
///
/// let literal_token = Token::new(TokenType::Number, 0, Some("1.0".to_string()), Some(object::Object::Literal(object::Literal::Number(1.0))));
/// let seed_vec = vec![
///     literal_token.clone(),
/// ];
///
/// assert_eq!(
///     Ok(MatchStatus::Match((
///         &seed_vec[1..],
///         Expr::Primary(object::Object::Literal(object::Literal::Number(1.0))
///         )
///     ))),
///     expression().parse(&seed_vec)
/// );
/// ```
pub fn expression<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    assignment()
}

#[allow(clippy::redundant_closure)]
fn assignment<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    join(
        token_type(TokenType::Identifier),
        right(join(token_type(TokenType::Equal), equality())),
    )
    .map(|(lhv, rhe)| Expr::Assignment(Identifier::try_from(lhv).unwrap(), Box::new(rhe)))
    .or(|| logical_or())
}

#[allow(clippy::redundant_closure)]
fn logical_or<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    join(
        logical_and(),
        one_or_more(right(join(token_type(TokenType::Or), logical_and()))),
    )
    .map(|(lhe, rhe)| {
        let mut postfix = rhe.into_iter().rev();
        let mut last: Expr = postfix.next().unwrap();

        for left in postfix {
            last = Expr::Logical(LogicalExpr::Or(Box::new(left), Box::new(last)))
        }
        Expr::Logical(LogicalExpr::Or(Box::new(lhe), Box::new(last)))
    })
    .or(|| logical_and())
}

#[allow(clippy::redundant_closure)]
fn logical_and<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    join(
        equality(),
        one_or_more(right(join(token_type(TokenType::And), equality()))),
    )
    .map(|(lhe, rhe)| {
        let mut postfix = rhe.into_iter().rev();
        let mut last: Expr = postfix.next().unwrap();

        for left in postfix {
            last = Expr::Logical(LogicalExpr::And(Box::new(left), Box::new(last)))
        }
        Expr::Logical(LogicalExpr::And(Box::new(lhe), Box::new(last)))
    })
    .or(|| equality())
}

#[allow(clippy::redundant_closure)]
fn equality<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    join(
        comparison(),
        parcel::zero_or_more(join(
            token_type(TokenType::EqualEqual).or(|| token_type(TokenType::BangEqual)),
            comparison(),
        ))
        .map(unzip),
    )
    .map(|(lhe, (operators, mut operands))| {
        operands.insert(0, lhe);
        (operands, operators)
    })
    .map(|(operands, operators)| {
        let mut operands_iter = operands.into_iter().rev();
        let operators_iter = operators.into_iter().rev();
        let mut last: Expr = operands_iter.next().unwrap();

        for op in operators_iter {
            // this is fairly safe due to the parser guaranteeing enough args.
            let left = operands_iter.next().unwrap();
            last = Expr::Equality(match op.token_type {
                TokenType::EqualEqual => EqualityExpr::Equal(Box::new(left), Box::new(last)),
                TokenType::BangEqual => EqualityExpr::NotEqual(Box::new(left), Box::new(last)),
                _ => panic!(format!("unexpected token: {}", op.token_type)),
            })
        }

        last
    })
    .or(|| comparison())
}

enum ComparisonOp {
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[allow(clippy::redundant_closure)]
fn comparison<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    join(
        addition(),
        parcel::zero_or_more(join(
            token_type(TokenType::Greater)
                .map(|_| ComparisonOp::Greater)
                .or(|| token_type(TokenType::GreaterEqual).map(|_| ComparisonOp::GreaterEqual))
                .or(|| token_type(TokenType::Less).map(|_| ComparisonOp::Less))
                .or(|| token_type(TokenType::LessEqual).map(|_| ComparisonOp::LessEqual)),
            addition(),
        ))
        .map(unzip),
    )
    .map(|(first_expr, (operators, operands))| {
        operators
            .into_iter()
            .zip(operands.into_iter())
            .fold(first_expr, |lhs, (operator, rhs)| match operator {
                ComparisonOp::Greater => {
                    Expr::Comparison(ComparisonExpr::Greater(Box::new(lhs), Box::new(rhs)))
                }
                ComparisonOp::GreaterEqual => {
                    Expr::Comparison(ComparisonExpr::GreaterEqual(Box::new(lhs), Box::new(rhs)))
                }
                ComparisonOp::Less => {
                    Expr::Comparison(ComparisonExpr::Less(Box::new(lhs), Box::new(rhs)))
                }
                ComparisonOp::LessEqual => {
                    Expr::Comparison(ComparisonExpr::LessEqual(Box::new(lhs), Box::new(rhs)))
                }
            })
    })
    .or(|| addition())
}

enum AdditionOp {
    Plus,
    Minus,
}

#[allow(clippy::redundant_closure)]
fn addition<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    join(
        multiplication(),
        parcel::zero_or_more(join(
            token_type(TokenType::Plus)
                .map(|_| AdditionOp::Plus)
                .or(|| token_type(TokenType::Minus).map(|_| AdditionOp::Minus)),
            multiplication(),
        ))
        .map(unzip),
    )
    .map(|(first_expr, (operators, operands))| {
        operators
            .into_iter()
            .zip(operands.into_iter())
            .fold(first_expr, |lhs, (operator, rhs)| match operator {
                AdditionOp::Plus => Expr::Addition(AdditionExpr::Add(Box::new(lhs), Box::new(rhs))),
                AdditionOp::Minus => {
                    Expr::Addition(AdditionExpr::Subtract(Box::new(lhs), Box::new(rhs)))
                }
            })
    })
    .or(|| multiplication())
}

enum MultipliactionOp {
    Star,
    Slash,
}

#[allow(clippy::redundant_closure)]
fn multiplication<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    join(
        unary(),
        parcel::zero_or_more(join(
            token_type(TokenType::Star)
                .map(|_| MultipliactionOp::Star)
                .or(|| token_type(TokenType::Slash).map(|_| MultipliactionOp::Slash)),
            unary(),
        ))
        .map(unzip),
    )
    .map(|(first_expr, (operators, operands))| {
        operators
            .into_iter()
            .zip(operands.into_iter())
            .fold(first_expr, |lhs, (operator, rhs)| match operator {
                MultipliactionOp::Star => {
                    Expr::Multiplication(MultiplicationExpr::Multiply(Box::new(lhs), Box::new(rhs)))
                }
                MultipliactionOp::Slash => {
                    Expr::Multiplication(MultiplicationExpr::Divide(Box::new(lhs), Box::new(rhs)))
                }
            })
    })
    .or(|| unary())
}

enum UnaryOp {
    Minus,
    Bang,
}

#[allow(clippy::redundant_closure)]
fn unary<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    join(
        token_type(TokenType::Bang)
            .map(|_| UnaryOp::Bang)
            .or(|| token_type(TokenType::Minus).map(|_| UnaryOp::Minus)),
        primary(),
    )
    .map(|(op, lit)| {
        Expr::Unary(match op {
            UnaryOp::Minus => UnaryExpr::Minus(Box::new(lit)),
            UnaryOp::Bang => UnaryExpr::Bang(Box::new(lit)),
        })
    })
    .or(|| call())
}

#[allow(clippy::redundant_closure)]
fn call<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    join(
        primary(),
        right(join(
            token_type(TokenType::LeftParen),
            left(join(
                optional(join(
                    expression(),
                    zero_or_more(right(join(token_type(TokenType::Comma), expression()))),
                )),
                token_type(TokenType::RightParen),
            )),
        )),
    )
    .map(|(callee, opt_args)| {
        Expr::Call(
            Box::new(callee),
            match opt_args {
                None => Vec::new(),
                Some(a) => {
                    let mut args = vec![a.0];
                    args.extend(a.1);
                    args
                }
            },
        )
    })
    .or(|| get())
}

#[allow(clippy::redundant_closure)]
fn get<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    join(
        primary(),
        right(join(token_type(TokenType::Dot), expression())),
    )
    .map(|(callee, param)| Expr::Get(Box::new(callee), Box::new(param)))
    .or(|| lambda())
}

#[allow(clippy::redundant_closure)]
fn lambda<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    right(join(
        token_type(TokenType::Fun),
        join(
            right(join(
                token_type(TokenType::LeftParen),
                left(join(
                    optional(join(
                        token_type(TokenType::Identifier),
                        zero_or_more(right(join(
                            token_type(TokenType::Comma),
                            token_type(TokenType::Identifier),
                        ))),
                    )),
                    token_type(TokenType::RightParen),
                )),
            )),
            block(),
        ),
    ))
    .map(|(opt_args, body)| {
        Expr::Lambda(
            opt_args.map_or(Vec::new(), |a| {
                let mut args = vec![Identifier::try_from(a.0).unwrap()];
                args.extend(
                    a.1.into_iter()
                        .map(|i| Identifier::try_from(i).unwrap())
                        .collect::<Vec<Identifier>>(),
                );
                args
            }),
            Box::new(body),
        )
    })
    .or(|| primary())
}

#[allow(clippy::redundant_closure)]
fn primary<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    parcel::one_of(vec![
        token_type(TokenType::True),
        token_type(TokenType::False),
        token_type(TokenType::Nil),
        token_type(TokenType::Number),
        token_type(TokenType::Str),
    ])
    .map(|token| Expr::Primary(token.object.unwrap()))
    .or(|| identifier().map(|id| Expr::Variable(id)))
    .or(|| {
        right(join(
            token_type(TokenType::LeftParen),
            left(join(expression(), token_type(TokenType::RightParen))),
        ))
        .map(|expr| Expr::Grouping(Box::new(expr)))
    })
}

#[allow(clippy::redundant_closure)]
pub fn identifier<'a>() -> impl parcel::Parser<'a, &'a [Token], Identifier> {
    token_type(TokenType::Identifier).map(|token| Identifier::try_from(token).unwrap())
}
