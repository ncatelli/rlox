use super::token_type::TokenType;
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

/// Literal functions to encapsulate literal values to be embedded in their
/// corresponding Token type.
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
            token_type,
            literal,
        }
    }

    pub fn is_reserved_keyword(&self) -> Option<TokenType> {
        match self.token_type {
            TokenType::Literal => match self.literal {
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

// TODO
impl From<char> for Token {
    fn from(item: char) -> Self {
        match item {
            '(' => Token::new(TokenType::LeftParen, None),
            ')' => Token::new(TokenType::RightParen, None),
            '{' => Token::new(TokenType::LeftBrace, None),
            '}' => Token::new(TokenType::RightBrace, None),
            ',' => Token::new(TokenType::Comma, None),
            '.' => Token::new(TokenType::Dot, None),
            '-' => Token::new(TokenType::Minus, None),
            '+' => Token::new(TokenType::Plus, None),
            ';' => Token::new(TokenType::Semicolon, None),
            '*' => Token::new(TokenType::Star, None),
            _ => Token::new(TokenType::EOF, None),
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
