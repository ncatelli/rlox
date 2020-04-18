use crate::parser::expression::*;
use crate::scanner::tokens::{Token, TokenType};
use std::option::Option::Some;

pub type ParseResult<'a, Output> = Result<(&'a [Token], Output), &'a [Token]>;

pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a [Token]) -> ParseResult<'a, Output>;
}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a [Token]) -> ParseResult<Output>,
{
    fn parse(&self, input: &'a [Token]) -> ParseResult<'a, Output> {
        self(input)
    }
}

/// Parser wraps the functionality of converting the tokens from the scanner
/// into a corresponding AST.
pub struct TokenParser {
    tokens: Vec<Token>,
}

impl TokenParser {
    pub fn new(tokens: Vec<Token>) -> TokenParser {
        TokenParser { tokens }
    }

    pub fn parse(&self) -> Result<Expr, String> {
        Err("Placeholder".to_string())
    }
}

pub fn first<'a, P, A>(parsers: &'a [P]) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
{
    move |input| {
        for parser in parsers {
            match parser.parse(input) {
                ok @ Ok(_) => return ok,
                Err(_) => continue,
            };
        }

        Err(input)
    }
}

pub fn either<'a, P1, P2, A>(parser1: P1, parser2: P2) -> impl Parser<'a, A>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, A>,
{
    move |input| match parser1.parse(input) {
        ok @ Ok(_) => ok,
        Err(_) => parser2.parse(input),
    }
}

pub fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

pub fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
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

pub fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(left, _right)| left)
}

pub fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_left, right)| right)
}

pub fn token_type<'a>(expected: TokenType) -> impl Parser<'a, Token> {
    move |input: &'a [Token]| match input.get(0) {
        Some(next) if next.token_type == expected => Ok((&input[1..], next.clone())),
        _ => Err(input),
    }
}

pub fn unary<'a>() -> impl Parser<'a, Expr> {
    map(
        pair(
            either(token_type(TokenType::Bang), token_type(TokenType::Minus)),
            primary(),
        ),
        |(token, lit)| Expr::Unary(UnaryExpr::new(token, Box::new(lit))),
    )
}

pub fn primary<'a>() -> impl Parser<'a, Expr> {
    map(
        either(
            token_type(TokenType::True),
            either(
                token_type(TokenType::False),
                either(
                    token_type(TokenType::Nil),
                    either(token_type(TokenType::Number), token_type(TokenType::Str)),
                ),
            ),
        ),
        |token| Expr::Literal(LiteralExpr::new(token)),
    )
}
