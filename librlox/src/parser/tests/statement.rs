extern crate parcel;
use crate::parser::expression::{Expr, PrimaryExpr, UnaryExpr};
use crate::parser::statement::Stmt;
use crate::parser::statement_parser::statements;
use crate::scanner::tokens::{Literal, Token, TokenType, Value};
use parcel::prelude::v1::*;
use parcel::MatchStatus;
use std::convert::TryFrom;
use std::option::Option;

#[test]
fn test_expression_formatter_should_pretty_print_an_ast() {
    let expr = Stmt::Expression(Expr::Unary(UnaryExpr::Minus(Box::new(Expr::Primary(
        PrimaryExpr::try_from(Token::new(
            TokenType::Literal,
            Option::Some(Value::Literal(Literal::Number(123.0))),
        ))
        .unwrap(),
    )))));

    assert_eq!("(Expression (- 123))".to_string(), format!("{}", expr))
}

#[test]
fn test_parser_can_parse_declaration_stmt() {
    let identifier_token = Token::new(
        TokenType::Identifier,
        Option::Some(Value::Identifier("test".to_string())),
    );
    let literal_token = Token::new(
        TokenType::Literal,
        Option::Some(Value::Literal(Literal::Number(5.0))),
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
