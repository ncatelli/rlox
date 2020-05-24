use crate::ast::token::token_type::TokenType;
use crate::object;
use std::fmt;
use std::option::Option;
use std::option::Option::{None, Some};

const RESERVED_KEYWORDS: &[(&str, TokenType)] = &[
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
    pub line: usize,
    pub lexeme: Option<String>,
    pub object: Option<object::Object>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        line: usize,
        lexeme: Option<String>,
        object: Option<object::Object>,
    ) -> Token {
        Token {
            token_type: token_type,
            line: line,
            lexeme: lexeme,
            object: object,
        }
    }

    pub fn is_reserved_keyword(&self) -> Option<TokenType> {
        match self.token_type {
            TokenType::Identifier => match self.object {
                Some(object::Object::Identifier(ref id)) => {
                    for kw in RESERVED_KEYWORDS.iter() {
                        if kw.0 == id {
                            return Some(kw.1);
                        }
                    }
                    None
                }
                _ => None,
            },
            _ => None,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.object.as_ref() {
            Some(lit) => write!(f, "{}", lit),
            None => write!(f, "{}", self.token_type),
        }
    }
}
