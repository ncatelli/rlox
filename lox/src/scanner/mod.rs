pub mod tokens;

use tokens::{Token, TokenType};

struct PreToken {
    literal: char,
    column: u64,
    line: u64,
}

impl PreToken {
    pub fn new(literal: char, column: u64, line: u64) -> PreToken {
        PreToken {
            literal: literal,
            column: column,
            line: line,
        }
    }
}

pub struct Scanner {
    source: String,
    start: u64,
    end: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let end = source.len();

        Scanner {
            source: source,
            start: 0,
            end: end,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let pt = self.preparse_tokens();
        let mut pti = pt.into_iter(); 

        while let Some(c) = pti.next() {}

        tokens.push(Token::new(TokenType::EOF, "".to_string(), 0));
        tokens
    }

    fn scan_token(&self, c: PreToken) -> Token {
        let token_type = match c.literal {
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

        Token::new(token_type, c.literal.to_string(), 0)
    }

    fn preparse_tokens(&self) -> Vec<PreToken> {
        let mut line = 0;
        let mut column = 0;

        self.source.chars().map(|c| {
            let pt = PreToken {
                literal: c,
                line: line,
                column: column,
            };

            match c {
                '\n' => {
                    column = 0;
                    line += 1;
                },
                _ => column += 1,
            }

            pt
        }).collect()
    }
}
