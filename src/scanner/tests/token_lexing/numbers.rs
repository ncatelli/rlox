use crate::scanner::tokens::TokenType;
use crate::scanner::*;

use super::helpers::compare_single_token_source_helper;

#[test]
fn scan_tokens_should_lex_digit() {
    compare_single_token_source_helper("123", TokenType::Number);
}

#[test]
fn scan_tokens_should_lex_floating_point() {
    compare_single_token_source_helper("123.45", TokenType::Number);
}

#[test]
fn scan_tokens_should_not_allow_trailing_decimal() {
    let source = "123.".to_string();
    let mut s = Scanner::new(source);
    let token_results = s.scan_tokens();

    //assert_eq!(1, token_results.len());
    assert_eq!(
        token_results[0],
        LexResult::Err("Invalid number at line: 1, position: 3".to_string())
    );
}
