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
        right(join(token_type(TokenType::Dot), identifier())),
    )
    .map(|(callee, param)| Expr::Get(Box::new(callee), param))
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
