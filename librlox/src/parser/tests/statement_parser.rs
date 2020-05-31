extern crate parcel;
use crate::ast::expression::Expr;
use crate::ast::statement::Stmt;
use crate::ast::token::{Token, TokenType};
use crate::parser::statement_parser::statements;
use parcel::prelude::v1::*;
use parcel::MatchStatus;
use std::option::Option;

#[test]
fn parser_can_parse_declaration_stmt() {
    let identifier_token = Token::new(
        TokenType::Identifier,
        1,
        Option::Some("test".to_string()),
        None,
    );
    let literal_token = Token::new(
        TokenType::Number,
        1,
        Option::Some("5.0".to_string()),
        Option::Some(obj_number!(5.0)),
    );
    let input = vec![
        Token::new(TokenType::Var, 1, Option::None, Option::None),
        identifier_token.clone(),
        Token::new(TokenType::Equal, 1, Option::None, Option::None),
        literal_token.clone(),
        Token::new(TokenType::Semicolon, 1, Option::None, Option::None),
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
fn parser_can_parse_print_stmt() {
    let literal_token = Token::new(
        TokenType::Number,
        1,
        Option::Some("5.0".to_string()),
        Option::Some(obj_number!(5.0)),
    );
    let input = vec![
        Token::new(TokenType::Print, 1, Option::None, Option::None),
        literal_token.clone(),
        Token::new(TokenType::Semicolon, 1, Option::None, Option::None),
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
fn parser_can_parse_block_stmt() {
    let literal_token = Token::new(
        TokenType::Number,
        1,
        Option::Some("5.0".to_string()),
        Option::Some(obj_number!(5.0)),
    );
    let input = vec![
        Token::new(TokenType::LeftParen, 1, Option::None, Option::None),
        Token::new(TokenType::Print, 1, Option::None, Option::None),
        literal_token.clone(),
        Token::new(TokenType::Semicolon, 1, Option::None, Option::None),
        Token::new(TokenType::RightParen, 1, Option::None, Option::None),
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
