use crate::ast::token::{Token, TokenType};
use crate::object;
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
            line: 1,
            lexeme: None,
            object: None,
        })
    );
}

pub fn compare_single_token_source_with_literal_helper(
    single_token_source: &str,
    lexeme: &str,
    obj: Option<object::Object>,
    expected_token_type: TokenType,
) {
    let source = single_token_source.to_string();
    let s = Scanner::new(source);
    let token_results = s.scan_tokens();

    assert_eq!(
        token_results[0],
        LexResult::Ok(Token {
            token_type: expected_token_type,
            line: 1,
            lexeme: if lexeme.len() > 0 {
                Some(lexeme.trim().to_string())
            } else {
                None
            },
            object: obj,
        })
    );
}

pub fn compare_single_token_source_returns_none_helper(single_token_source: &str, line: usize) {
    let source = single_token_source.to_string();
    let s = Scanner::new(source);
    let token_results = s.scan_tokens();

    assert_eq!(1, token_results.len());
    assert_eq!(
        token_results[0],
        LexResult::Ok(Token {
            token_type: TokenType::EOF,
            line: line,
            lexeme: None,
            object: None,
        })
    );
}
