use super::token_type::TokenType;
use std::fmt;
use std::option::Option;
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
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Identifier(s) => write!(f, "{}", &s),
            Literal::Str(s) => write!(f, "{}", &s),
            Literal::Number(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Option<Literal>,
}

impl Token {
    pub fn new(token_type: TokenType, literal: Option<Literal>) -> Token {
        Token {
            token_type: token_type,
            literal: literal,
        }
    }

    pub fn is_reserved_keyword(&self) -> Option<TokenType> {
        match self.token_type {
            TokenType::Identifier => match self.literal {
                Some(Literal::Identifier(ref id)) => {
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
        match self.literal.as_ref() {
            Some(lit) => write!(f, "{}", lit),
            None => write!(f, "{}", self.token_type),
        }
    }
}
