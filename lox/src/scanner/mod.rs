use std::option::Option;
use std::option::Option::{None, Some};

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
            // Single character lexemes
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

            // Operators lexemes with optional additional characters
            '!' => match self.peek() {
                Some('=') => {
                    self.start = self.current - 1;
                    self.current += 1;
                    Ok(self.substring_into_token(TokenType::BangEqual))
                }
                _ => Ok(self.substring_into_token(TokenType::Bang)),
            },
            '=' => match self.peek() {
                Some('=') => {
                    self.start = self.current - 1;
                    self.current += 1;
                    Ok(self.substring_into_token(TokenType::EqualEqual))
                }
                _ => Ok(self.substring_into_token(TokenType::Equal)),
            },
            '<' => match self.peek() {
                Some('=') => {
                    self.start = self.current - 1;
                    self.current += 1;
                    Ok(self.substring_into_token(TokenType::LessEqual))
                }
                _ => Ok(self.substring_into_token(TokenType::Less)),
            },
            '>' => match self.peek() {
                Some('=') => {
                    self.start = self.current - 1;
                    self.current += 1;
                    Ok(self.substring_into_token(TokenType::GreaterEqual))
                }
                _ => Ok(self.substring_into_token(TokenType::Greater)),
            },

            // Slash
            '/' => match self.peek() {
                Some('/') => {
                    while let Some(next) = self.peek() {
                        match next {
                            '\n' => {
                                self.current += 1;
                                return Ok(self.substring_into_token(TokenType::Newline));
                            }
                            _ => self.current += 1,
                        }
                    }

                    Ok(Token::new(TokenType::EOF, "".to_string()))
                }
                _ => Ok(self.substring_into_token(TokenType::Slash)),
            },

            // Whitespace
            ' ' => Ok(self.substring_into_token(TokenType::Whitespace)),
            '\r' => Ok(self.substring_into_token(TokenType::Whitespace)),
            '\t' => Ok(self.substring_into_token(TokenType::Whitespace)),
            '\n' => {
                self.line += 1;
                Ok(self.substring_into_token(TokenType::Newline))
            }

            // Literals
            // Strings
            '"' => {
                self.start += 1;
                let start_line = self.line;
                while let Some(next) = self.peek() {
                    match next {
                        '"' => {
                            let token_str = self.substring_into_token(TokenType::Str);
                            self.current += 1;
                            return Ok(token_str);
                        }
                        _ => self.current += 1,
                    }
                }
                Err(format!(
                    "Unclosed at line: {}, position: {}.",
                    start_line, self.start
                ))
            }

            // Unknown lexemes
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

    fn substring(&self, start: usize, end: usize) -> &[char] {
        &self.source[start..end]
    }

    fn substring_into_token(&self, token_type: TokenType) -> Token {
        let token_range = &self.substring(self.start, self.current);
        let literal: String = token_range.iter().collect();

        Token::new(token_type, literal)
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn peek(&mut self) -> Option<char> {
        match self.is_at_end() {
            true => None,
            false => Some(self.source[self.current]),
        }
    }
}
