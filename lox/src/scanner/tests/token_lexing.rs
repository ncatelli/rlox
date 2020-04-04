use crate::scanner::tokens::{Token, TokenType};
use crate::scanner::*;

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

}

#[test]
fn scan_tokens_should_lex_multiple_character_operator_lexemes() {
    compare_single_token_source_helper("!=", TokenType::BangEqual);
    compare_single_token_source_helper("==", TokenType::EqualEqual);
    compare_single_token_source_helper("<=", TokenType::LessEqual);
    compare_single_token_source_helper(">=", TokenType::GreaterEqual);
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

fn compare_single_token_source_helper(single_token_source: &str, expected_token_type: TokenType) {
    let source = single_token_source.to_string();
    let mut s = Scanner::new(source);
    let token_results = s.scan_tokens();

    //assert_eq!(1, token_results.len());
    assert_eq!(
        token_results[0],
        LexResult::Ok(Token {
            token_type: expected_token_type,
            lexeme: single_token_source.to_string(),
        })
    );
}
