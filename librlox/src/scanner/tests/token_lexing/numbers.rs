use crate::scanner::tokens::{Literal, TokenType, Value};
use crate::scanner::*;

use super::helpers::compare_single_token_source_with_literal_helper;

#[test]
fn scan_tokens_should_lex_digit() {
    compare_single_token_source_with_literal_helper(
        "123",
        Value::Literal(Literal::Number(123.0)),
        TokenType::Literal,
    );
}

#[test]
fn scan_tokens_should_lex_floating_point() {
    compare_single_token_source_with_literal_helper(
        "123.45",
        Value::Literal(Literal::Number(123.45)),
        TokenType::Literal,
    );
}

#[test]
fn scan_tokens_should_not_allow_trailing_decimal() {
    let source = "123.".to_string();
    let s = Scanner::new(source);
    let token_results = s.scan_tokens();

    assert_eq!(
        token_results[0],
        LexResult::Err("Invalid number at line: 1, position: 3".to_string())
    );
}

#[test]
fn scan_tokens_should_allow_numbers_to_include_operators() {
    let source = "5+5".to_string();
    let s = Scanner::new(source);
    let token_results = s.scan_tokens();

    assert_eq!(
        token_results,
        vec![
            LexResult::Ok(Token {
                token_type: TokenType::Literal,
                value: Some(Value::Literal(Literal::Number(5.0))),
            }),
            LexResult::Ok(Token {
                token_type: TokenType::Plus,
                value: None,
            }),
            LexResult::Ok(Token {
                token_type: TokenType::Literal,
                value: Some(Value::Literal(Literal::Number(5.0))),
            }),
            LexResult::Ok(Token {
                token_type: TokenType::EOF,
                value: None,
            })
        ]
    );
}
