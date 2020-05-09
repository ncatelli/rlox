extern crate parcel;
use crate::parser::expression::*;
use crate::scanner::tokens::{Token, TokenType};
use parcel::*;
use std::convert::TryFrom;
use std::option::Option::Some;

fn take_while<'a, P, A: 'a, B>(parser: P) -> impl Parser<'a, A, Vec<B>>
where
    A: Copy + 'a,
    P: Parser<'a, A, B>,
{
    move |mut input| {
        let mut result_acc: Vec<B> = Vec::new();
        while let Ok(MatchStatus::Match((next_input, result))) = parser.parse(input) {
            input = next_input;
            result_acc.push(result);
        }

        Ok(MatchStatus::Match((input, result_acc)))
    }
}

fn unzip<A, B>(pair: Vec<(A, B)>) -> (Vec<A>, Vec<B>) {
    let mut left_vec: Vec<A> = vec![];
    let mut right_vec: Vec<B> = vec![];
    pair.into_iter().for_each(|(left, right)| {
        left_vec.push(left);
        right_vec.push(right);
    });
    (left_vec, right_vec)
}

fn token_type<'a>(expected: TokenType) -> impl parcel::Parser<'a, &'a [Token], Token> {
    move |input: &'a [Token]| match input.get(0) {
        Some(next) if next.token_type == expected => {
            Ok(parcel::MatchStatus::Match((&input[1..], next.clone())))
        }
        _ => Ok(parcel::MatchStatus::NoMatch(input)),
    }
}

/// Represents the entrypoint for expression parsing within the lox parser and
/// yields an Expr object after recursively descending through the expression
/// grammar
///
/// # Examples
/// ```
/// extern crate librlox;
/// extern crate parcel;
/// use librlox::scanner::tokens::{Literal, TokenType, Token};
/// use librlox::parser::expression::*;
/// use librlox::parser::expression_parser::*;
/// use std::option::Option;
/// use std::convert::TryFrom;
/// use parcel::*;
///
///
/// let literal_token = Token::new(TokenType::Literal, Option::Some(Literal::Number(1.0)));
/// let seed_vec = vec![
///     literal_token.clone(),
/// ];
///
/// assert_eq!(
///     Ok(MatchStatus::Match((
///         &seed_vec[1..],
///         Expr::Primary(
///             PrimaryExpr::try_from(
///                 literal_token.clone()
///             ).unwrap()
///         )
///     ))),
///     expression().parse(&seed_vec)
/// );
/// ```
pub fn expression<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    equality()
}

#[allow(clippy::redundant_closure)]
fn equality<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    join(
        comparison(),
        take_while(join(
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
            last = Expr::Equality(EqualityExpr::new(
                EqualityExprOperator::from_token(op).unwrap(),
                Box::new(left),
                Box::new(last),
            ))
        }
        last
    })
    .or(|| comparison())
}

#[allow(clippy::redundant_closure)]
fn comparison<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    join(
        addition(),
        take_while(join(
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
            last = Expr::Comparison(ComparisonExpr::new(
                ComparisonExprOperator::from_token(op).unwrap(),
                Box::new(left),
                Box::new(last),
            ))
        }
        last
    })
    .or(|| addition())
}

#[allow(clippy::redundant_closure)]
fn addition<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    join(
        multiplication(),
        take_while(join(
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
            last = Expr::Addition(AdditionExpr::new(
                AdditionExprOperator::from_token(op).unwrap(),
                Box::new(left),
                Box::new(last),
            ))
        }
        last
    })
    .or(|| multiplication())
}

#[allow(clippy::redundant_closure)]
fn multiplication<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    join(
        unary(),
        take_while(join(
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
            last = Expr::Multiplication(MultiplicationExpr::new(
                MultiplicationExprOperator::from_token(op).unwrap(),
                Box::new(left),
                Box::new(last),
            ))
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
        Expr::Unary(UnaryExpr::new(
            UnaryExprOperator::from_token(token).unwrap(),
            Box::new(lit),
        ))
    })
    .or(|| primary())
}

fn primary<'a>() -> impl parcel::Parser<'a, &'a [Token], Expr> {
    token_type(TokenType::True)
        .or(|| token_type(TokenType::False))
        .or(|| token_type(TokenType::Nil))
        .or(|| token_type(TokenType::Literal))
        .map(|token| Expr::Primary(PrimaryExpr::try_from(token).unwrap()))
        .or(|| {
            right(join(
                token_type(TokenType::LeftParen),
                left(join(expression(), token_type(TokenType::RightParen))),
            ))
            .map(|expr| Expr::Grouping(GroupingExpr::new(Box::new(expr))))
        })
}
