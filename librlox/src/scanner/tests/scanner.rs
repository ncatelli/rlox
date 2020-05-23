use crate::ast::token::{Token, TokenType};
use crate::scanner::source_scanner::{LexResult, Scanner};

use std::option::Option::{None, Some};

#[test]
fn into_iter_should_return_characters_from_iterators() {
    let s = Scanner::new(";+-".to_string());
    let mut iter = s.into_iter();

    assert_eq!(
        iter.next(),
        Some(LexResult::Ok(Token {
            token_type: TokenType::Semicolon,
            value: None,
        }))
    );
    assert_eq!(
        iter.next(),
        Some(LexResult::Ok(Token {
            token_type: TokenType::Plus,
            value: None,
        }))
    );
    assert_eq!(
        iter.next(),
        Some(LexResult::Ok(Token {
            token_type: TokenType::Minus,
            value: None,
        }))
    );
    assert_eq!(
        iter.next(),
        Some(LexResult::Ok(Token {
            token_type: TokenType::EOF,
            value: None,
        }))
    );
    assert_eq!(iter.next(), None);
}
