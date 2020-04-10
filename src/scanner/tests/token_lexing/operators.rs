use crate::scanner::tokens::TokenType;
use crate::scanner::*;

use super::helpers::{
    compare_single_token_source_helper, compare_single_token_source_returns_none_helper,
};

#[test]
fn scan_tokens_should_lex_single_character_lexemes() {
    compare_single_token_source_helper("(", TokenType::LeftParen);
    compare_single_token_source_helper(")", TokenType::RightParen);
    compare_single_token_source_helper("{", TokenType::LeftBrace);
    compare_single_token_source_helper("}", TokenType::RightBrace);
    compare_single_token_source_helper(",", TokenType::Comma);
    compare_single_token_source_helper(".", TokenType::Dot);
    compare_single_token_source_helper("-", TokenType::Minus);
    compare_single_token_source_helper("+", TokenType::Plus);
    compare_single_token_source_helper(";", TokenType::Semicolon);
    compare_single_token_source_helper("*", TokenType::Star);

    compare_single_token_source_helper("!", TokenType::Bang);
    compare_single_token_source_helper("=", TokenType::Equal);
    compare_single_token_source_helper("<", TokenType::Less);
    compare_single_token_source_helper(">", TokenType::Greater);
    compare_single_token_source_helper("/", TokenType::Slash);
}

#[test]
fn scan_tokens_should_lex_multiple_character_operator_lexemes() {
    compare_single_token_source_helper("!=", TokenType::BangEqual);
    compare_single_token_source_helper("==", TokenType::EqualEqual);
    compare_single_token_source_helper("<=", TokenType::LessEqual);
    compare_single_token_source_helper(">=", TokenType::GreaterEqual);
}

#[test]
fn scan_tokens_should_lex_comments() {
    compare_single_token_source_returns_none_helper("// this is a test comment\n");
    compare_single_token_source_returns_none_helper("/* this is a test comment */");
}

#[test]
fn scan_tokens_should_lex_whitespace() {
    compare_single_token_source_returns_none_helper(" ");
    compare_single_token_source_returns_none_helper("\r");
    compare_single_token_source_returns_none_helper("\t");
}

#[test]
fn scan_tokens_should_lex_newlines() {
    compare_single_token_source_returns_none_helper("\n");
}

#[test]
fn lex_unknown_token_returns_error_result() {
    let s = Scanner::new("%".to_string());
    let token_results = s.scan_tokens();

    assert_eq!(2, token_results.len());
    assert_eq!(
        token_results[0],
        LexResult::Err("Lex error at line: 1, position: 0.".to_string())
    );
}
