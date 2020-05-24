extern crate parcel;
use crate::ast::expression::{Expr, PrimaryExpr};
use crate::ast::statement::Stmt;
use crate::ast::token::{Token, TokenType};
use crate::object;
use crate::parser::statement_parser::statements;
use parcel::prelude::v1::*;
use parcel::MatchStatus;
use std::option::Option;

#[test]
fn test_parser_can_parse_declaration_stmt() {
    let identifier_token = Token::new(
        TokenType::Identifier,
        Option::Some(object::Object::Identifier("test".to_string())),
    );
    let literal_token = Token::new(
        TokenType::Literal,
        Option::Some(object::Object::Literal(object::Literal::Number(5.0))),
    );
    let input = vec![
        Token::new(TokenType::Var, Option::None),
        identifier_token.clone(),
        Token::new(TokenType::Equal, Option::None),
        literal_token.clone(),
        Token::new(TokenType::Semicolon, Option::None),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[5..],
            vec![Stmt::Declaration(
                "test".to_string(),
                Expr::Primary(PrimaryExpr::Number(5.0))
            )]
        ))),
        statements().parse(&input)
    );
}
