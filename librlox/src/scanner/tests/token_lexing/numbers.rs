use crate::scanner::tokens::{Literal, TokenType};
use crate::scanner::*;

use super::helpers::compare_single_token_source_with_literal_helper;

#[test]
fn scan_tokens_should_lex_digit() {
    compare_single_token_source_with_literal_helper(
        "123",
        Literal::Number(123.0),
        TokenType::Literal,
    );
}

#[test]
fn scan_tokens_should_lex_floating_point() {
    compare_single_token_source_with_literal_helper(
        "123.45",
        Literal::Number(123.45),
        TokenType::Literal,
    );
}

#[test]
fn scan_tokens_should_not_allow_trailing_decimal() {
    let source = "123.".to_string();
    let s = Scanner::new(source);
    let token_results = s.scan_tokens();

    //assert_eq!(1, token_results.len());
    assert_eq!(
        token_results[0],
        LexResult::Err("Invalid number at line: 1, position: 3".to_string())
    );
}
