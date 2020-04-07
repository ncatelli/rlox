use crate::scanner::tokens::TokenType;
use crate::scanner::*;

use super::helpers::compare_single_token_source_helper;

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
    compare_single_token_source_helper("// this is a test comment\n", TokenType::Newline);
    compare_single_token_source_helper("/* this is a test comment */", TokenType::Newline);
}

#[test]
fn scan_tokens_should_lex_whitespace() {
    compare_single_token_source_helper(" ", TokenType::Whitespace);
    compare_single_token_source_helper("\r", TokenType::Whitespace);
    compare_single_token_source_helper("\t", TokenType::Whitespace);
}

#[test]
fn scan_tokens_should_lex_newlines() {
    compare_single_token_source_helper("\n", TokenType::Newline);
}

#[test]
fn scan_tokens_should_drop_everything_but_newline_when_encountering_comments() {
    compare_single_token_source_helper("// this is a comment\n", TokenType::Newline);
}

#[test]
fn lex_unknown_token_returns_error_result() {
    let mut s = Scanner::new("%".to_string());
    let token_results = s.scan_tokens();

    assert_eq!(1, token_results.len());
    assert_eq!(
        token_results[0],
        LexResult::Err("Lex error at line: 1, position: 1.".to_string())
    );
}
