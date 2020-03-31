use std::iter::Peekable;

pub mod tokens;

use tokens::{Token, TokenType};

pub struct Scanner {
    source: String,
    start: u64,
    end: usize,
    current: u64,
    line: u64, 
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let end = source.len();

        Scanner{
            source: source,
            start: 0,
            end: end,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut char_iter = self.source.chars().peekable();

        while let Some(_) = char_iter.peek() {
            let next_token = self.scan_token(&mut char_iter);
            tokens.push(next_token);
        }

        tokens.push(Token::new(TokenType::EOF, "".to_string(), self.line));
        tokens
    }

    fn scan_token(&self, ci: &mut Peekable<std::str::Chars>) -> Token {
        let c = ci.next().unwrap();
        let token_type = match c {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            _ => TokenType::String,
        };

        Token::new(token_type, c.to_string(), self.line)
    }
}