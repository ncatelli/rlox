use crate::scanner::tokens::{Token, TokenType};
use crate::scanner::{LexResult, Scanner};

use std::option::Option::{None, Some};

#[test]
fn validate_advance_returns_the_next_unread_char() {
    let mut s = Scanner::new(";+-".to_string());

    assert_eq!(s.advance(), ';');
    assert_eq!(s.advance(), '+');
    assert_eq!(s.advance(), '-');
}

#[test]
fn assert_should_should_return_next_character_without_advancing_counter() {
    let mut s = Scanner::new(";+-".to_string());

    assert_eq!(s.peek(), Some(';'));
    assert_eq!(s.peek(), Some(';'));
    assert_eq!(s.peek(), Some(';'));
}

#[test]
fn into_iter_should_return_characters_from_iterators() {
    let s = Scanner::new(";+-".to_string());
    let mut iter = s.into_iter();

    assert_eq!(
        iter.next(),
        Some(LexResult::Ok(Token {
            token_type: TokenType::Semicolon,
            lexeme: ";".to_string(),
        }))
    );
    assert_eq!(
        iter.next(),
        Some(LexResult::Ok(Token {
            token_type: TokenType::Plus,
            lexeme: "+".to_string(),
        }))
    );
    assert_eq!(
        iter.next(),
        Some(LexResult::Ok(Token {
            token_type: TokenType::Minus,
            lexeme: "-".to_string(),
        }))
    );
    assert_eq!(iter.next(), None);
}
