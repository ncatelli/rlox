extern crate parcel;
use crate::ast::expression::{AdditionExpr, ComparisonExpr, Expr};
use crate::ast::statement::Stmt;
use crate::ast::token::TokenType;
use crate::parser::statement_parser::statements;
use parcel::prelude::v1::*;
use parcel::MatchStatus;
use std::option::Option;

macro_rules! token_from_tt {
    ($tt:expr) => {
        $crate::ast::token::Token::new($tt, 1, Option::None, Option::None)
    };
    ($tt:expr, $lex:expr) => {
        $crate::ast::token::Token::new($tt, 1, Option::Some($lex.to_string()), Option::None)
    };
    ($tt:expr, $lex:expr, $val:expr) => {
        $crate::ast::token::Token::new($tt, 1, Option::Some($lex.to_string()), Option::Some($val))
    };
}

#[test]
fn can_parse_declaration_stmt() {
    let input = vec![
        token_from_tt!(TokenType::Var),
        token_from_tt!(TokenType::Identifier, "test"),
        token_from_tt!(TokenType::Equal),
        token_from_tt!(TokenType::Number, "5.0", obj_number!(5.0)),
        token_from_tt!(TokenType::Semicolon),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[5..],
            vec![Stmt::Declaration(
                "test".to_string(),
                Expr::Primary(obj_number!(5.0))
            )]
        ))),
        statements().parse(&input)
    );
}

#[test]
fn can_parse_function_declaration_stmt() {
    let input = vec![
        token_from_tt!(TokenType::Fun),
        token_from_tt!(TokenType::Identifier, "test"),
        token_from_tt!(TokenType::LeftParen),
        token_from_tt!(TokenType::Identifier, "arg_one"),
        token_from_tt!(TokenType::RightParen),
        token_from_tt!(TokenType::LeftBrace),
        token_from_tt!(TokenType::Number, "5.0", obj_number!(5.0)),
        token_from_tt!(TokenType::Semicolon),
        token_from_tt!(TokenType::RightBrace),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[9..],
            vec![Stmt::Function(
                "test".to_string(),
                vec![token_from_tt!(TokenType::Identifier, "arg_one")],
                Box::new(Stmt::Block(vec![Stmt::Expression(Expr::Primary(
                    obj_number!(5.0)
                ))]))
            )]
        ))),
        statements().parse(&input)
    );
}

#[test]
fn can_parse_print_stmt() {
    let input = vec![
        token_from_tt!(TokenType::Print),
        token_from_tt!(TokenType::Number, "5.0", obj_number!(5.0)),
        token_from_tt!(TokenType::Semicolon),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[3..],
            vec![Stmt::Print(Expr::Primary(obj_number!(5.0)))]
        ))),
        statements().parse(&input)
    );
}

#[test]
fn can_parse_return_stmt() {
    let input = vec![
        token_from_tt!(TokenType::Return),
        token_from_tt!(TokenType::Number, "5.0", obj_number!(5.0)),
        token_from_tt!(TokenType::Semicolon),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[3..],
            vec![Stmt::Return(Expr::Primary(obj_number!(5.0)))]
        ))),
        statements().parse(&input)
    );
}

#[test]
fn can_parse_block_stmt() {
    let input = vec![
        token_from_tt!(TokenType::LeftBrace),
        token_from_tt!(TokenType::Print),
        token_from_tt!(TokenType::Number, "5.0", obj_number!(5.0)),
        token_from_tt!(TokenType::Semicolon),
        token_from_tt!(TokenType::RightBrace),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[5..],
            vec![Stmt::Block(vec![Stmt::Print(Expr::Primary(obj_number!(
                5.0
            )))])]
        ))),
        statements().parse(&input)
    );
}

#[test]
fn can_parse_if_stmt_with_no_else_clause() {
    let input = vec![
        token_from_tt!(TokenType::If),
        token_from_tt!(TokenType::LeftParen),
        token_from_tt!(TokenType::True, "true", obj_bool!(true)),
        token_from_tt!(TokenType::RightParen),
        token_from_tt!(TokenType::Number, "5.0", obj_number!(5.0)),
        token_from_tt!(TokenType::Semicolon),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[6..],
            vec![Stmt::If(
                Expr::Primary(obj_bool!(true)),
                Box::new(Stmt::Expression(Expr::Primary(obj_number!(5.0)))),
                Option::None
            )]
        ))),
        statements().parse(&input)
    );
}

#[test]
fn can_parse_if_stmt_with_else_clause() {
    let input = vec![
        token_from_tt!(TokenType::If),
        token_from_tt!(TokenType::LeftParen),
        token_from_tt!(TokenType::True, "true", obj_bool!(true)),
        token_from_tt!(TokenType::RightParen),
        token_from_tt!(TokenType::Number, "5.0", obj_number!(5.0)),
        token_from_tt!(TokenType::Semicolon),
        token_from_tt!(TokenType::Else),
        token_from_tt!(TokenType::Number, "5.0", obj_number!(5.0)),
        token_from_tt!(TokenType::Semicolon),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[9..],
            vec![Stmt::If(
                Expr::Primary(obj_bool!(true)),
                Box::new(Stmt::Expression(Expr::Primary(obj_number!(5.0)))),
                Option::Some(Box::new(Stmt::Expression(Expr::Primary(obj_number!(5.0)))))
            )]
        ))),
        statements().parse(&input)
    );
}

#[test]
fn can_parse_while_stmt() {
    let input = vec![
        token_from_tt!(TokenType::While),
        token_from_tt!(TokenType::LeftParen),
        token_from_tt!(TokenType::True, "true", obj_bool!(true)),
        token_from_tt!(TokenType::RightParen),
        token_from_tt!(TokenType::True, "true", obj_bool!(true)),
        token_from_tt!(TokenType::Semicolon),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[6..],
            vec![Stmt::While(
                Expr::Primary(obj_bool!(true)),
                Box::new(Stmt::Expression(Expr::Primary(obj_bool!(true)))),
            )]
        ))),
        statements().parse(&input)
    );
}

#[test]
fn can_parse_for_stmt() {
    let input = vec![
        token_from_tt!(TokenType::For),
        token_from_tt!(TokenType::LeftParen),
        token_from_tt!(TokenType::Var),
        token_from_tt!(TokenType::Identifier, "test"),
        token_from_tt!(TokenType::Equal),
        token_from_tt!(TokenType::Number, "1", obj_number!(1.0)),
        token_from_tt!(TokenType::Semicolon),
        token_from_tt!(TokenType::Identifier, "test"),
        token_from_tt!(TokenType::Less),
        token_from_tt!(TokenType::Number, "5", obj_number!(5.0)),
        token_from_tt!(TokenType::Semicolon),
        token_from_tt!(TokenType::Identifier, "test"),
        token_from_tt!(TokenType::Equal),
        token_from_tt!(TokenType::Identifier, "test"),
        token_from_tt!(TokenType::Plus),
        token_from_tt!(TokenType::Number, "1", obj_number!(1.0)),
        token_from_tt!(TokenType::RightParen),
        token_from_tt!(TokenType::Print),
        token_from_tt!(TokenType::Identifier, "test"),
        token_from_tt!(TokenType::Semicolon),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[20..],
            vec![Stmt::Block(vec![
                Stmt::Declaration("test".to_string(), Expr::Primary(obj_number!(1.0))),
                Stmt::While(
                    Expr::Comparison(ComparisonExpr::Less(
                        Box::new(Expr::Variable(token_from_tt!(
                            TokenType::Identifier,
                            "test"
                        ))),
                        Box::new(Expr::Primary(obj_number!(5.0)))
                    )),
                    Box::new(Stmt::Block(vec![
                        Stmt::Print(Expr::Variable(token_from_tt!(
                            TokenType::Identifier,
                            "test"
                        ))),
                        Stmt::Expression(Expr::Assignment(
                            token_from_tt!(TokenType::Identifier, "test"),
                            Box::new(Expr::Addition(AdditionExpr::Add(
                                Box::new(Expr::Variable(token_from_tt!(
                                    TokenType::Identifier,
                                    "test"
                                ))),
                                Box::new(Expr::Primary(obj_number!(1.0)))
                            )))
                        ))
                    ]))
                )
            ])]
        ))),
        statements().parse(&input)
    );
}
