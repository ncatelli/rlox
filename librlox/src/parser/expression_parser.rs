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

    fn or<P>(self, parser: impl Fn() -> P + 'a) -> BoxedParser<'a, Output>
    where
        Self: Sized + 'a,
        Output: 'a,
        P: Parser<'a, Output> + 'a,
    {
        BoxedParser::new(either(self, parser))
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

fn either<'a, P1, P2, A>(parser1: P1, parser2: impl Fn() -> P2) -> impl Parser<'a, A>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, A>,
{
    move |input| match parser1.parse(input) {
        ok @ Ok(_) => ok,
        Err(_) => parser2().parse(input),
    }
}

pub fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
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

pub fn and_then<'a, P, F, A, B, NextP>(parser: P, f: F) -> impl Parser<'a, B>
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

pub fn token_type<'a>(expected: TokenType) -> impl Parser<'a, Token> {
    move |input: &'a [Token]| match input.get(0) {
        Some(next) if next.token_type == expected => Ok((&input[1..], next.clone())),
        _ => Err(input),
    }
}

pub fn expression<'a>() -> impl Parser<'a, Expr> {
    equality()
}

fn equality<'a>() -> impl Parser<'a, Expr> {
    join(
        unary(),
        join(
            token_type(TokenType::EqualEqual).or(|| token_type(TokenType::BangEqual)),
            comparison(),
        ),
    )
    .map(|(lhe, (token, rhe))| Expr::Binary(BinaryExpr::new(token, Box::new(lhe), Box::new(rhe))))
    .or(|| comparison())
}

fn comparison<'a>() -> impl Parser<'a, Expr> {
    join(
        unary(),
        join(
            token_type(TokenType::Greater)
                .or(|| token_type(TokenType::GreaterEqual))
                .or(|| token_type(TokenType::Less))
                .or(|| token_type(TokenType::LessEqual)),
            addition(),
        ),
    )
    .map(|(lhe, (token, rhe))| Expr::Binary(BinaryExpr::new(token, Box::new(lhe), Box::new(rhe))))
    .or(|| addition())
}

fn addition<'a>() -> impl Parser<'a, Expr> {
    join(
        unary(),
        join(
            token_type(TokenType::Plus).or(|| token_type(TokenType::Minus)),
            multiplication(),
        ),
    )
    .map(|(lhe, (token, rhe))| Expr::Binary(BinaryExpr::new(token, Box::new(lhe), Box::new(rhe))))
    .or(|| multiplication())
}

fn multiplication<'a>() -> impl Parser<'a, Expr> {
    join(
        unary(),
        join(
            token_type(TokenType::Star).or(|| token_type(TokenType::Slash)),
            unary(),
        ),
    )
    .map(|(lhe, (token, rhe))| Expr::Binary(BinaryExpr::new(token, Box::new(lhe), Box::new(rhe))))
    .or(|| unary())
}

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
        .map(|token| Expr::Literal(LiteralExpr::new(token)))
        .or(|| {
            right(
                token_type(TokenType::LeftParen),
                left(expression(), token_type(TokenType::RightParen)),
            )
            .map(|expr| Expr::Grouping(GroupingExpr::new(Box::new(expr))))
        })
}
