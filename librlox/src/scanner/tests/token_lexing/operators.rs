use crate::scanner::tokens::TokenType;
use crate::scanner::*;

extern crate parcel;
use parcel::Parser;

use super::helpers::compare_single_token_source_returns_none_helper;

macro_rules! compare_literal_token {
    ($source:expr, $tt: expr) => {
        let input: Vec<char> = $source.chars().collect();
        let token_results = crate::scanner::source_scanner::scan_tokens_combinator().parse(&input);

        assert_eq!(
            token_results,
            Ok(parcel::MatchStatus::Match((
                &input[input.len()..],
                vec![Token {
                    token_type: $tt,
                    literal: None,
                }]
            )))
        );
    };
}

#[test]
fn scan_tokens_combinator_should_lex_single_character_lexemes() {
    compare_literal_token!("(", TokenType::LeftParen);
    compare_literal_token!(")", TokenType::RightParen);
    compare_literal_token!("{", TokenType::LeftBrace);
    compare_literal_token!("}", TokenType::RightBrace);
    compare_literal_token!(",", TokenType::Comma);
    compare_literal_token!(".", TokenType::Dot);
    compare_literal_token!("-", TokenType::Minus);
    compare_literal_token!("+", TokenType::Plus);
    compare_literal_token!(";", TokenType::Semicolon);
    compare_literal_token!("*", TokenType::Star);

    compare_literal_token!("!", TokenType::Bang);
    compare_literal_token!("=", TokenType::Equal);
    compare_literal_token!("<", TokenType::Less);
    compare_literal_token!(">", TokenType::Greater);
    compare_literal_token!("/", TokenType::Slash);
}

#[test]
fn scan_tokens_combinator_should_lex_multiple_character_operator_lexemes() {
    compare_literal_token!("!=", TokenType::BangEqual);
    compare_literal_token!("==", TokenType::EqualEqual);
    compare_literal_token!("<=", TokenType::LessEqual);
    compare_literal_token!(">=", TokenType::GreaterEqual);
}

#[test]
fn scan_tokens_should_lex_comments() {
    compare_single_token_source_returns_none_helper("// this is a test comment\n");
    compare_single_token_source_returns_none_helper("/* this is a test comment */");
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
