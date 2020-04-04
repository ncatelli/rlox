use std::iter::Iterator;

pub mod tokens;

use tokens::{Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let chars: Vec<char> = source.chars().collect();
        Scanner {
            source: chars,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        tokens
    }
}
