use crate::scanner::tokens::{Token, TokenType};
use crate::scanner::*;

#[test]
fn lex_single_tokens_should_return_single_result() {
    let mut s = Scanner::new(";".to_string());
    let token_results = s.scan_tokens();

    assert_eq!(1, token_results.len());
    assert_eq!(
        token_results[0],
        LexResult::Ok(Token {
            token_type: TokenType::Semicolon,
            lexeme: ";".to_string(),
        })
    );
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