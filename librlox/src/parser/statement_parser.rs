extern crate parcel;
use super::combinators::token_type;
use crate::parser::expression_parser::expression;
use crate::parser::statement::Stmt;
use crate::scanner::tokens::{Token, TokenType};
use parcel::*;

/// Represents the entrypoint for statement parsing within the lox parser and
/// yields an Stmt object after recursively descending through the statement
/// grammar
pub fn statement<'a>() -> impl parcel::Parser<'a, &'a [Token], Stmt> {
    expression_stmt()
}

#[allow(clippy::redundant_closure)]
fn expression_stmt<'a>() -> impl parcel::Parser<'a, &'a [Token], Stmt> {
    left(join(expression(), token_type(TokenType::Semicolon))).map(|expr| Stmt::Expression(expr))
}
