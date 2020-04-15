use crate::scanner::scanner::{LexResult, Scanner};
use crate::scanner::tokens::{Token, TokenType};

use std::option::Option::{None, Some};

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
