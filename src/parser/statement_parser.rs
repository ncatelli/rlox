extern crate parcel;
use super::combinators::token_type;
use crate::ast::expression::Expr;
use crate::ast::identifier::Identifier;
use crate::ast::statement::Stmt;
use crate::ast::token::{Token, TokenType};
use crate::parser::expression_parser::{expression, identifier};
use parcel::*;
use std::convert::TryFrom;

/// Represents the entrypoint for statement parsing within the lox parser and
/// yields a Vec<Stmt> representing the program statemnts.
pub fn statements<'a>() -> impl parcel::Parser<'a, &'a [Token], Vec<Stmt>> {
    parcel::one_or_more(statement())
}

#[allow(clippy::redundant_closure)]
fn statement<'a>() -> impl parcel::Parser<'a, &'a [Token], Stmt> {
    declaration_stmt()
        .or(|| class_declaration_stmt())
        .or(|| fun_declaration_stmt())
        .or(|| expression_stmt())
        .or(|| while_stmt())
        .or(|| for_stmt())
        .or(|| if_stmt())
        .or(|| print_stmt())
        .or(|| return_stmt())
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
fn fun_declaration_stmt<'a>() -> impl parcel::Parser<'a, &'a [Token], Stmt> {
    right(join(token_type(TokenType::Fun), function()))
}

#[allow(clippy::redundant_closure)]
fn class_declaration_stmt<'a>() -> impl parcel::Parser<'a, &'a [Token], Stmt> {
    right(join(
        token_type(TokenType::Class),
        join(
            identifier(),
            right(join(
                token_type(TokenType::LeftBrace),
                left(join(
                    zero_or_more(function()),
                    token_type(TokenType::RightBrace),
                )),
            )),
        ),
    ))
    .map(|(id, funcs)| Stmt::Class(id, funcs))
}

#[allow(clippy::redundant_closure)]
fn function<'a>() -> impl parcel::Parser<'a, &'a [Token], Stmt> {
    join(
        token_type(TokenType::Identifier),
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
    )
    .map(|(callee, (opt_args, body))| {
        let ident = Identifier::try_from(callee).unwrap();
        Stmt::Function(
            ident,
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
        let id = Identifier::try_from(id_tok).unwrap();

        Stmt::Declaration(id, expr)
    })
}

#[allow(clippy::redundant_closure)]
fn return_stmt<'a>() -> impl parcel::Parser<'a, &'a [Token], Stmt> {
    left(right(join(
        token_type(TokenType::Return),
        join(optional(expression()), token_type(TokenType::Semicolon)),
    )))
    .map(|optional_expr| {
        optional_expr.map_or(Stmt::Return(Expr::Primary(obj_nil!())), |expr| {
            Stmt::Return(expr)
        })
    })
}

#[allow(clippy::redundant_closure)]
pub fn block<'a>() -> impl parcel::Parser<'a, &'a [Token], Stmt> {
    right(join(
        token_type(TokenType::LeftBrace),
        left(join(statements(), token_type(TokenType::RightBrace))),
    ))
    .map(|stmts| Stmt::Block(stmts))
}

#[allow(clippy::redundant_closure)]
fn if_stmt<'a>() -> impl parcel::Parser<'a, &'a [Token], Stmt> {
    join(
        right(join(
            token_type(TokenType::If),
            left(join(
                right(join(token_type(TokenType::LeftParen), expression())),
                token_type(TokenType::RightParen),
            )),
        )),
        join(
            statement(),
            optional(right(join(token_type(TokenType::Else), statement()))),
        ),
    )
    .map(
        |(condition, (primary_branch, secondary_branch))| match secondary_branch {
            None => Stmt::If(condition, Box::new(primary_branch), None),
            Some(b) => Stmt::If(condition, Box::new(primary_branch), Some(Box::new(b))),
        },
    )
}

#[allow(clippy::redundant_closure)]
pub fn while_stmt<'a>() -> impl parcel::Parser<'a, &'a [Token], Stmt> {
    join(
        right(join(
            token_type(TokenType::While),
            left(join(
                right(join(token_type(TokenType::LeftParen), expression())),
                token_type(TokenType::RightParen),
            )),
        )),
        statement(),
    )
    .map(|(condition, stmt)| Stmt::While(condition, Box::new(stmt)))
}

#[allow(clippy::redundant_closure)]
pub fn for_stmt<'a>() -> impl parcel::Parser<'a, &'a [Token], Stmt> {
    join(
        right(join(
            token_type(TokenType::For),
            left(join(
                right(join(
                    token_type(TokenType::LeftParen),
                    join(
                        optional(
                            expression_stmt()
                                .or(|| declaration_stmt())
                                .or(|| nil_stmt()),
                        ),
                        join(
                            left(join(
                                optional(expression()),
                                token_type(TokenType::Semicolon),
                            )),
                            optional(expression()),
                        ),
                    ),
                )),
                token_type(TokenType::RightParen),
            )),
        )),
        statement(),
    )
    .map(|((initializer, (condition, incrementer)), stmt)| {
        let mut for_block: Vec<Stmt> = Vec::new();
        if let Some(init) = initializer {
            for_block.push(init)
        };

        let mut while_body: Vec<Stmt> = vec![stmt];
        if let Some(inc) = incrementer {
            while_body.push(Stmt::Expression(inc))
        };

        for_block.push(Stmt::While(
            condition.unwrap_or(Expr::Primary(obj_bool!(true))),
            Box::new(Stmt::Block(while_body)),
        ));

        Stmt::Block(for_block)
    })
}

fn nil_stmt<'a>() -> impl parcel::Parser<'a, &'a [Token], Stmt> {
    token_type(TokenType::Semicolon).map(|_| Stmt::Expression(Expr::Primary(obj_nil!())))
}
