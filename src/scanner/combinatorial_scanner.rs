use parcel::{parsers::character::expect_character, prelude::v1::*};
use std::iter::Iterator;
use std::option::Option::{None, Some};

use crate::ast::token::{Token, TokenType};

type LexError = String;

/// LexResult is an alias that represents the result of an attempt to lex a
/// single character token. Returning either the Token or a string containing
/// positional data for the error.
pub type LexResult = Result<Token, LexError>;

/// Scanner takes a string representing lox source and attempts to convert the
/// source into a vector of either Tokens or lexical errors.
///
/// # Examples
/// ```
/// use rlox::scanner;
/// let source = "* ; - \"hello world\" 1234.5".to_string();
/// let s = scanner::Scanner::new(source);
///
/// let _tokens = s.scan_tokens().into_iter();
/// ```
pub struct Scanner {
    source: Vec<char>,
    end: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let chars: Vec<char> = source.chars().collect();
        let end = chars.len();
        Scanner { source: chars, end }
    }

    pub fn scan_tokens(&self) -> Vec<LexResult> {
        todo!()
    }
}

pub fn token<'a>() -> impl Parser<'a, &'a [char], Token> {
    parcel::one_of(vec![
        expect_character('(').map(|_| Token::new(TokenType::LeftParen, 0, None, None)),
        expect_character(')').map(|_| Token::new(TokenType::RightParen, 0, None, None)),
        expect_character('[').map(|_| Token::new(TokenType::LeftBrace, 0, None, None)),
        expect_character('}').map(|_| Token::new(TokenType::RightBrace, 0, None, None)),
    ])
}
