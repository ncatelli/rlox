extern crate parcel;
use crate::scanner::tokens::{Literal, Token, TokenType};
use crate::scanner::*;
use parcel::Parser;

pub fn compare_single_token_source_helper(
    single_token_source: &str,
    expected_token_type: TokenType,
) {
    let source = single_token_source.to_string();
    let s = Scanner::new(source);
    let token_results = s.scan_tokens();

    assert_eq!(
        token_results[0],
        LexResult::Ok(Token {
            token_type: expected_token_type,
            literal: None,
        })
    );
}

pub fn compare_single_token_source_combinator_helper(
    single_token_source: &str,
    expected_token_type: TokenType,
) {
    let input: Vec<char> = single_token_source.chars().collect();
    let token_results = crate::scanner::source_scanner::scan_tokens_combinator().parse(&input);

    assert_eq!(
        token_results,
        Ok(parcel::MatchStatus::Match((
            &input[input.len()..],
            vec![Token {
                token_type: expected_token_type,
                literal: None,
            }]
        )))
    );
}

pub fn compare_single_token_source_with_literal_helper(
    single_token_source: &str,
    literal: Literal,
    expected_token_type: TokenType,
) {
    let source = single_token_source.to_string();
    let s = Scanner::new(source);
    let token_results = s.scan_tokens();

    assert_eq!(
        token_results[0],
        LexResult::Ok(Token {
            token_type: expected_token_type,
            literal: Some(literal),
        })
    );
}

pub fn compare_single_token_source_returns_none_helper(single_token_source: &str) {
    let source = single_token_source.to_string();
    let s = Scanner::new(source);
    let token_results = s.scan_tokens();

    assert_eq!(1, token_results.len());
    assert_eq!(
        token_results[0],
        LexResult::Ok(Token {
            token_type: TokenType::EOF,
            literal: None,
        })
    );
}
