extern crate parcel;
use crate::scanner::tokens::{Token, TokenType};
use crate::scanner::*;

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
