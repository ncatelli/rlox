use crate::scanner::tokens::TokenType;

use super::helpers::{
    compare_single_token_source_helper, compare_single_token_source_with_literal_helper,
};

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

#[test]
fn scan_tokens_should_lex_single_character_lexemes() {
    compare_single_token_source_helper("and", TokenType::And);
    compare_single_token_source_helper("or", TokenType::Or);
    compare_single_token_source_helper("print", TokenType::Print);
    compare_single_token_source_helper("return", TokenType::Return);
    compare_single_token_source_helper("super", TokenType::Super);
    compare_single_token_source_helper("class", TokenType::Class);
    compare_single_token_source_helper("this", TokenType::This);
    compare_single_token_source_helper("nil", TokenType::Nil);
    compare_single_token_source_helper("true", TokenType::True);
    compare_single_token_source_helper("false", TokenType::False);
    compare_single_token_source_helper("var", TokenType::Var);
    compare_single_token_source_helper("fun", TokenType::Fun);
    compare_single_token_source_helper("while", TokenType::While);
    compare_single_token_source_helper("for", TokenType::For);
    compare_single_token_source_helper("if", TokenType::If);
    compare_single_token_source_helper("else", TokenType::Else);
}
