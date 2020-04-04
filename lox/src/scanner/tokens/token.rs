use super::token_type::TokenType;
use std::fmt;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String) -> Token {
        Token {
            token_type: token_type,
            lexeme: lexeme.clone(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}
