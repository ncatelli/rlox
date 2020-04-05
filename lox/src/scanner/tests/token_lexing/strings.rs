use crate::scanner::tokens::TokenType;

use super::helpers::compare_single_token_source_with_literal_helper;

#[test]
fn scan_tokens_should_lex_full_string() {
    compare_single_token_source_with_literal_helper("\"test\"", "test".to_string(), TokenType::Str)
}
