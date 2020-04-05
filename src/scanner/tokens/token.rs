use super::token_type::TokenType;
use std::fmt;
use std::option::Option::{None, Some};

const RESERVED_KEYWORDS: &'static [(&'static str, TokenType)] = &[
    ("and", TokenType::And),
    ("or", TokenType::Or),
    ("print", TokenType::Print),
    ("return", TokenType::Return),
    ("super", TokenType::Super),
    ("class", TokenType::Class),
    ("this", TokenType::This),
    ("nil", TokenType::Nil),
    ("true", TokenType::True),
    ("false", TokenType::False),
    ("var", TokenType::Var),
    ("fun", TokenType::Fun),
    ("while", TokenType::While),
    ("for", TokenType::For),
    ("if", TokenType::If),
    ("else", TokenType::Else),
];

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String) -> Token {
        Token {
            token_type: token_type,
            lexeme: lexeme.clone(),
        }
    }

    pub fn is_reserved_keyword(&self) -> Option<TokenType> {
        match self.token_type {
            TokenType::Identifier => {
                for kw in RESERVED_KEYWORDS.iter() {
                    if *kw.0 == self.lexeme {
                        return Some(kw.1);
                    }
                }

                None
            }
            _ => None,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}
