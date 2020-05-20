extern crate parcel;
use super::combinators::token_type;
use crate::parser::expression_parser::expression;
use crate::parser::statement::Stmt;
use crate::scanner::tokens::{Token, TokenType};
use parcel::*;

/// Represents the entrypoint for statement parsing within the lox parser and
/// yields a Vec<Stmt> representing the program statemnts.
pub fn statements<'a>() -> impl parcel::Parser<'a, &'a [Token], Vec<Stmt>> {
    parcel::one_or_more(statement())
}

#[allow(clippy::redundant_closure)]
fn statement<'a>() -> impl parcel::Parser<'a, &'a [Token], Stmt> {
    print_stmt().or(|| expression_stmt())
}

#[allow(clippy::redundant_closure)]
fn expression_stmt<'a>() -> impl parcel::Parser<'a, &'a [Token], Stmt> {
    left(join(expression(), token_type(TokenType::Semicolon))).map(|expr| Stmt::Expression(expr))
}

#[allow(clippy::redundant_closure)]
fn print_stmt<'a>() -> impl parcel::Parser<'a, &'a [Token], Stmt> {
    left(right(join(
        token_type(TokenType::Print),
        join(expression(), token_type(TokenType::Semicolon)),
    )))
    .map(|expr| Stmt::Print(expr))
}
