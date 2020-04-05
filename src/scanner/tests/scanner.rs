use crate::scanner::*;

use std::option::Option::Some;

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
