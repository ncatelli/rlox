pub mod tokens;
use tokens::{Token, TokenType};

#[cfg(test)]
mod tests;

pub type LexResult = Result<Token, String>;

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    end: usize,
    had_errors: bool,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let chars: Vec<char> = source.chars().collect();
        let end = chars.len();
        Scanner {
            source: chars,
            start: 0,
            current: 0,
            line: 1,
            end: end,
            had_errors: false,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<LexResult> {
        let mut tokens: Vec<LexResult> = Vec::new();

        while !self.is_at_end() {
            let t = self.scan_token();
            tokens.push(t);
        }

        tokens
    }

    fn scan_token(&mut self) -> Result<Token, String> {
        let c = self.advance();

        match c {
            '(' => Ok(self.substring_into_token(TokenType::LeftParen)),
            ')' => Ok(self.substring_into_token(TokenType::RightParen)),
            '{' => Ok(self.substring_into_token(TokenType::LeftBrace)),
            '}' => Ok(self.substring_into_token(TokenType::RightBrace)),
            ',' => Ok(self.substring_into_token(TokenType::Comma)),
            '.' => Ok(self.substring_into_token(TokenType::Dot)),
            '-' => Ok(self.substring_into_token(TokenType::Minus)),
            '+' => Ok(self.substring_into_token(TokenType::Plus)),
            ';' => Ok(self.substring_into_token(TokenType::Semicolon)),
            '*' => Ok(self.substring_into_token(TokenType::Star)),
            _ => {
                self.had_errors = true;
                Err(format!(
                    "Lex error at line: {}, position: {}.",
                    self.line, self.current
                ))
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.end
    }

    fn substring_into_token(&self, token_type: TokenType) -> Token {
        let token_range = &self.source[self.start..self.current];
        let literal: String = token_range.iter().collect();

        Token::new(token_type, literal)
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }
}
