use crate::scanner::tokens::TokenType;

use super::helpers::compare_single_token_source_helper;

#[test]
fn scan_tokens_should_lex_digit() {
    compare_single_token_source_helper("123", TokenType::Number);
}

#[test]
fn scan_tokens_should_lex_floating_point() {
    compare_single_token_source_helper("123.45", TokenType::Number);
}
