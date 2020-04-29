use crate::parser::expression::*;
use crate::scanner::tokens::{Token, TokenType};
use std::option::Option::Some;

pub type ParseResult<'a, Output> = Result<(&'a [Token], Output), &'a [Token]>;

pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a [Token]) -> ParseResult<'a, Output>;

    fn map<F, NewOutput>(self, map_fn: F) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        F: Fn(Output) -> NewOutput + 'a,
    {
        BoxedParser::new(map(self, map_fn))
    }

    fn and_then<F, NextParser, NewOutput>(self, f: F) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        NextParser: Parser<'a, NewOutput> + 'a,
        F: Fn(Output) -> NextParser + 'a,
    {
        BoxedParser::new(and_then(self, f))
    }

    fn or<P>(self, thunk_to_parser: impl Fn() -> P + 'a) -> BoxedParser<'a, Output>
    where
        Self: Sized + 'a,
        Output: 'a,
        P: Parser<'a, Output> + 'a,
    {
        BoxedParser::new(either(self, thunk_to_parser))
    }
}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a [Token]) -> ParseResult<Output>,
{
    fn parse(&self, input: &'a [Token]) -> ParseResult<'a, Output> {
        self(input)
    }
}

pub struct BoxedParser<'a, Output> {
    parser: Box<dyn Parser<'a, Output> + 'a>,
}

impl<'a, Output> BoxedParser<'a, Output> {
    fn new<P>(parser: P) -> Self
    where
        P: Parser<'a, Output> + 'a,
    {
        BoxedParser {
            parser: Box::new(parser),
        }
    }
}

impl<'a, Output> Parser<'a, Output> for BoxedParser<'a, Output> {
    fn parse(&self, input: &'a [Token]) -> ParseResult<'a, Output> {
        self.parser.parse(input)
    }
}

fn either<'a, P1, P2, A>(parser1: P1, thunk_to_parser: impl Fn() -> P2) -> impl Parser<'a, A>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, A>,
{
    move |input| match parser1.parse(input) {
        ok @ Ok(_) => ok,
        Err(_) => thunk_to_parser().parse(input),
    }
}

fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B + 'a,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

fn and_then<'a, P, F, A, B, NextP>(parser: P, f: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    NextP: Parser<'a, B>,
    F: Fn(A) -> NextP,
{
    move |input| match parser.parse(input) {
        Ok((next_input, result)) => f(result).parse(next_input),
        Err(err) => Err(err),
    }
}

fn take_while<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result_acc: Vec<A> = Vec::new();
        while let Ok((next_input, result)) = parser.parse(input) {
            input = next_input;
            result_acc.push(result);
        }

        Ok((input, result_acc))
    }
}

fn join<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2
                .parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
    }
}

fn left<'a, P1: 'a, P2: 'a, R1: 'a, R2: 'a>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    join(parser1, parser2).map(|(left, _right)| left)
}

fn right<'a, P1: 'a, P2: 'a, R1: 'a, R2: 'a>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    join(parser1, parser2).map(|(_left, right)| right)
}

fn unzip<'a, A, B>(pair: Vec<(A, B)>) -> (Vec<A>, Vec<B>) {
    let mut left_vec: Vec<A> = vec![];
    let mut right_vec: Vec<B> = vec![];
    pair.into_iter().for_each(|(left, right)| {
        left_vec.push(left);
        right_vec.push(right);
    });
    (left_vec, right_vec)
}

fn token_type<'a>(expected: TokenType) -> impl Parser<'a, Token> {
    move |input: &'a [Token]| match input.get(0) {
        Some(next) if next.token_type == expected => Ok((&input[1..], next.clone())),
        _ => Err(input),
    }
}

/// Represents the entrypoint for expression parsing within the lox parser and
/// yields an Expr object after recursively descending through the expression
/// grammar
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::scanner::tokens::{Literal, TokenType, Token};
/// use librlox::parser::expression::*;
/// use librlox::parser::expression_parser::*;
/// use std::option::Option;
///
///
/// let literal_token = Token::new(TokenType::Number, Option::Some(Literal::Number(1.0)));
/// let seed_vec = vec![
///     literal_token.clone(),
/// ];
///
/// assert_eq!(
///     Ok((
///         &seed_vec[1..],
///         Expr::Primary(
///             PrimaryExpr::new(
///                 literal_token.clone()
///             )
///         )
///     )),
///     expression().parse(&seed_vec)
/// );
/// ```
pub fn expression<'a>() -> impl Parser<'a, Expr> {
    equality()
}

#[allow(clippy::redundant_closure)]
fn equality<'a>() -> impl Parser<'a, Expr> {
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
fn comparison<'a>() -> impl Parser<'a, Expr> {
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
fn addition<'a>() -> impl Parser<'a, Expr> {
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
fn multiplication<'a>() -> impl Parser<'a, Expr> {
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
fn unary<'a>() -> impl Parser<'a, Expr> {
    join(
        token_type(TokenType::Bang).or(|| token_type(TokenType::Minus)),
        primary(),
    )
    .map(|(token, lit)| Expr::Unary(UnaryExpr::new(token, Box::new(lit))))
    .or(|| primary())
}

fn primary<'a>() -> impl Parser<'a, Expr> {
    token_type(TokenType::True)
        .or(|| token_type(TokenType::False))
        .or(|| token_type(TokenType::Nil))
        .or(|| token_type(TokenType::Number))
        .or(|| token_type(TokenType::Str))
        .map(|token| Expr::Primary(PrimaryExpr::new(token)))
        .or(|| {
            right(
                token_type(TokenType::LeftParen),
                left(expression(), token_type(TokenType::RightParen)),
            )
            .map(|expr| Expr::Grouping(GroupingExpr::new(Box::new(expr))))
        })
}
