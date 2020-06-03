extern crate parcel;
use crate::ast::expression::*;
use crate::ast::token::{Token, TokenType};
use crate::parser::combinators::{token_type, unzip};
use parcel::*;

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
    .map(|(lhv, rhe)| Expr::Assignment(lhv, Box::new(rhe)))
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

#[allow(clippy::redundant_closure)]
fn comparison<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    join(
        addition(),
        parcel::zero_or_more(join(
            token_type(TokenType::Greater)
                .or(|| token_type(TokenType::GreaterEqual))
                .or(|| token_type(TokenType::Less))
                .or(|| token_type(TokenType::LessEqual)),
            addition(),
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
            last = Expr::Comparison(match op.token_type {
                TokenType::Less => ComparisonExpr::Less(Box::new(left), Box::new(last)),
                TokenType::LessEqual => ComparisonExpr::LessEqual(Box::new(left), Box::new(last)),
                TokenType::Greater => ComparisonExpr::Greater(Box::new(left), Box::new(last)),
                TokenType::GreaterEqual => {
                    ComparisonExpr::GreaterEqual(Box::new(left), Box::new(last))
                }
                _ => panic!(format!("unexpected token: {}", op.token_type)),
            })
        }
        last
    })
    .or(|| addition())
}

#[allow(clippy::redundant_closure)]
fn addition<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    join(
        multiplication(),
        parcel::zero_or_more(join(
            token_type(TokenType::Plus).or(|| token_type(TokenType::Minus)),
            multiplication(),
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
            last = Expr::Addition(match op.token_type {
                TokenType::Plus => AdditionExpr::Add(Box::new(left), Box::new(last)),
                TokenType::Minus => AdditionExpr::Subtract(Box::new(left), Box::new(last)),
                _ => panic!(format!("unexpected token: {}", op.token_type)),
            })
        }
        last
    })
    .or(|| multiplication())
}

#[allow(clippy::redundant_closure)]
fn multiplication<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    join(
        unary(),
        parcel::zero_or_more(join(
            token_type(TokenType::Star).or(|| token_type(TokenType::Slash)),
            unary(),
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
            last = Expr::Multiplication(match op.token_type {
                TokenType::Star => MultiplicationExpr::Multiply(Box::new(left), Box::new(last)),
                TokenType::Slash => MultiplicationExpr::Divide(Box::new(left), Box::new(last)),
                _ => panic!(format!("unexpected token: {}", op.token_type)),
            })
        }
        last
    })
    .or(|| unary())
}

#[allow(clippy::redundant_closure)]
fn unary<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    join(
        token_type(TokenType::Bang).or(|| token_type(TokenType::Minus)),
        primary(),
    )
    .map(|(token, lit)| {
        Expr::Unary(match token.token_type {
            TokenType::Minus => UnaryExpr::Minus(Box::new(lit)),
            TokenType::Bang => UnaryExpr::Bang(Box::new(lit)),
            _ => panic!(format!("unexpected token: {}", token.token_type)),
        })
    })
    .or(|| primary())
}

#[allow(clippy::redundant_closure)]
fn primary<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    token_type(TokenType::True)
        .or(|| token_type(TokenType::False))
        .or(|| token_type(TokenType::Nil))
        .or(|| token_type(TokenType::Number))
        .or(|| token_type(TokenType::Str))
        .map(|token| Expr::Primary(token.object.unwrap()))
        .or(|| token_type(TokenType::Identifier).map(|token| Expr::Variable(token)))
        .or(|| {
            right(join(
                token_type(TokenType::LeftParen),
                left(join(expression(), token_type(TokenType::RightParen))),
            ))
            .map(|expr| Expr::Grouping(Box::new(expr)))
        })
}