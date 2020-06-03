extern crate parcel;
use super::combinators::token_type;
use crate::ast::statement::Stmt;
use crate::ast::token::{Token, TokenType};
use crate::parser::expression_parser::expression;
use parcel::*;

/// Represents the entrypoint for statement parsing within the lox parser and
/// yields a Vec<Stmt> representing the program statemnts.
pub fn statements<'a>() -> impl parcel::Parser<'a, &'a [Token], Vec<Stmt>> {
    parcel::one_or_more(statement())
}

#[allow(clippy::redundant_closure)]
fn statement<'a>() -> impl parcel::Parser<'a, &'a [Token], Stmt> {
    declaration_stmt()
        .or(|| print_stmt())
        .or(|| expression_stmt())
        .or(|| block())
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

#[allow(clippy::redundant_closure)]
fn declaration_stmt<'a>() -> impl parcel::Parser<'a, &'a [Token], Stmt> {
    right(join(
        token_type(TokenType::Var),
        join(
            token_type(TokenType::Identifier),
            right(join(
                token_type(TokenType::Equal),
                left(join(expression(), token_type(TokenType::Semicolon))),
            )),
        ),
    ))
    .map(|(id_tok, expr)| {
        let id = match id_tok.lexeme {
            Some(i) => i,
            _ => panic!("invalid Object specified in place of Identifier"),
        };

        Stmt::Declaration(id, expr)
    })
}

#[allow(clippy::redundant_closure)]
fn block<'a>() -> impl parcel::Parser<'a, &'a [Token], Stmt> {
    right(join(
        token_type(TokenType::LeftBrace),
        left(join(statements(), token_type(TokenType::RightBrace))),
    ))
    .map(|stmts| Stmt::Block(stmts))
}