use crate::scanner::tokens::{Token, TokenType};
use crate::scanner::{Cursor, LexResult, Scanner};

use std::option::Option::{None, Some};

#[test]
fn assert_char_at_returns_the_next_unread_char() {
    let s = Scanner::new(";+-".to_string());
    let mut cursor = Cursor::new(0, 0, 1);

    assert_eq!(s.char_at(cursor), Some(';'));

    cursor = Cursor::advance(cursor);
    assert_eq!(s.char_at(cursor), Some('+'));

    cursor = Cursor::advance(cursor);
    assert_eq!(s.char_at(cursor), Some('-'));
}

#[test]
fn assert_should_should_return_next_character_without_advancing_counter() {
    let s = Scanner::new(";+-".to_string());
    let cursor = Cursor::new(0, 0, 1);

    assert_eq!(s.char_at(cursor), Some(';'));
    assert_eq!(s.char_at(Cursor::advance(cursor)), Some('+'));
    assert_eq!(s.char_at(cursor), Some(';'));
}

#[test]
fn assert_match_next_or_handles_lookahead() {
    let s = Scanner::new("!=".to_string());
    let cursor = Cursor::new(0, 0, 1);

    let (tok, next_cursor) = s.match_next_or(cursor, '=', TokenType::BangEqual, TokenType::Bang);

    assert_eq!(Ok(Token::new(TokenType::BangEqual, None)), tok);
    assert_eq!(Cursor::new(1, 1, 1), next_cursor);
}

#[test]
fn assert_match_next_or_handles_missing_lookahead() {
    let s = Scanner::new("!".to_string());
    let cursor = Cursor::new(0, 0, 1);

    let (tok, next_cursor) = s.match_next_or(cursor, '=', TokenType::BangEqual, TokenType::Bang);

    assert_eq!(Ok(Token::new(TokenType::Bang, None)), tok);
    assert_eq!(Cursor::new(0, 0, 1), next_cursor);
}

#[test]
fn into_iter_should_return_characters_from_iterators() {
    let s = Scanner::new(";+-".to_string());
    let mut iter = s.into_iter();

    assert_eq!(
        iter.next(),
        Some(LexResult::Ok(Token {
            token_type: TokenType::Semicolon,
            literal: None,
        }))
    );
    assert_eq!(
        iter.next(),
        Some(LexResult::Ok(Token {
            token_type: TokenType::Plus,
            literal: None,
        }))
    );
    assert_eq!(
        iter.next(),
        Some(LexResult::Ok(Token {
            token_type: TokenType::Minus,
            literal: None,
        }))
    );
    assert_eq!(
        iter.next(),
        Some(LexResult::Ok(Token {
            token_type: TokenType::EOF,
            literal: None,
        }))
    );
    assert_eq!(iter.next(), None);
}
