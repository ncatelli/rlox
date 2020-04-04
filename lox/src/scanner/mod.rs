use std::iter::Iterator;
use std::option::Option::{None, Some};

pub mod tokens;

use tokens::ordinal::Ordinal;
use tokens::{Token, TokenType};

#[derive(Clone, Copy)]
struct PreToken {
    literal: char,
    ordinal: Ordinal,
}

impl PreToken {
    pub fn new(literal: char, ordinal: Ordinal) -> PreToken {
        PreToken {
            literal: literal,
            ordinal: ordinal,
        }
    }
}

pub struct Scanner {
    source: Vec<PreToken>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: Scanner::preparse_tokens(source),
        }
    }

    fn preparse_tokens(source: String) -> Vec<PreToken> {
        let mut line = 0;
        let mut column = 0;

        source
            .chars()
            .map(|c| {
                let pt = PreToken {
                    literal: c,
                    ordinal: Ordinal(line, column),
                };

                match c {
                    '\n' => {
                        column = 0;
                        line += 1;
                    }
                    _ => column += 1,
                }

                pt
            })
            .collect()
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut iter = self.source.iter().peekable();
        let mut tokens: Vec<Token> = Vec::new();

        for pt in iter {
            let token = match pt.literal {
                '(' => Some(Token::new(
                    TokenType::LeftParen,
                    pt.literal.to_string(),
                    pt.ordinal,
                )),
                ')' => Some(Token::new(
                    TokenType::RightParen,
                    pt.literal.to_string(),
                    pt.ordinal,
                )),
                '{' => Some(Token::new(
                    TokenType::LeftBrace,
                    pt.literal.to_string(),
                    pt.ordinal,
                )),
                '}' => Some(Token::new(
                    TokenType::RightBrace,
                    pt.literal.to_string(),
                    pt.ordinal,
                )),
                ',' => Some(Token::new(
                    TokenType::Comma,
                    pt.literal.to_string(),
                    pt.ordinal,
                )),
                '.' => Some(Token::new(
                    TokenType::Dot,
                    pt.literal.to_string(),
                    pt.ordinal,
                )),
                '-' => Some(Token::new(
                    TokenType::Minus,
                    pt.literal.to_string(),
                    pt.ordinal,
                )),
                '+' => Some(Token::new(
                    TokenType::Plus,
                    pt.literal.to_string(),
                    pt.ordinal,
                )),
                ';' => Some(Token::new(
                    TokenType::Semicolon,
                    pt.literal.to_string(),
                    pt.ordinal,
                )),
                '*' => Some(Token::new(
                    TokenType::Star,
                    pt.literal.to_string(),
                    pt.ordinal,
                )),
                _ => None,
            };

            match token {
                Some(t) => tokens.push(t),
                None => tokens.push(Token::new(TokenType::EOF, "".to_string(), Ordinal(0,0))),
            }
        }

        tokens
    }
}
