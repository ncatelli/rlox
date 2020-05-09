use std::fmt;

/// TokenType represents any possible token that can be lexed by the lox
/// scanner.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Literal,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Comment,

    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenType::LeftParen => "(".to_string(),
                TokenType::RightParen => ")".to_string(),
                TokenType::LeftBrace => "{".to_string(),
                TokenType::RightBrace => "}".to_string(),
                TokenType::Comma => ",".to_string(),
                TokenType::Dot => ".".to_string(),
                TokenType::Minus => "-".to_string(),
                TokenType::Plus => "+".to_string(),
                TokenType::Semicolon => ";".to_string(),
                TokenType::Slash => "/".to_string(),
                TokenType::Star => "*".to_string(),
                TokenType::Bang => "!".to_string(),
                TokenType::BangEqual => "!=".to_string(),
                TokenType::Equal => "=".to_string(),
                TokenType::EqualEqual => "==".to_string(),
                TokenType::Greater => ">".to_string(),
                TokenType::GreaterEqual => ">=".to_string(),
                TokenType::Less => "<".to_string(),
                TokenType::LessEqual => "<=".to_string(),
                _ => format!("{:?}", self),
            }
        )
    }
}
