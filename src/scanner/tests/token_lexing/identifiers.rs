use crate::scanner::tokens::TokenType;

use super::helpers::compare_single_token_source_with_literal_helper;

#[test]
fn scan_tokens_should_lex_identifiers() {
    let identifier = "test_identifier_1_alpha";
    compare_single_token_source_with_literal_helper(
        identifier,
        identifier.to_string(),
        TokenType::Identifier,
    )
}

#[test]
fn scan_tokens_should_separate_identifier_on_non_alpha() {
    let identifier = "test_identifier_1_alpha\n";
    compare_single_token_source_with_literal_helper(
        identifier,
        identifier.trim().to_string(),
        TokenType::Identifier,
    )
}
