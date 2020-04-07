use std::option::Option;
use std::option::Option::{None, Some};

use std::iter::Iterator;

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
            self.start = self.current;
            let t = match self.scan_token() {
                Some(tok) => tok,
                None => continue,
            };

            tokens.push(t);
        }

        tokens
    }

    fn scan_token(&mut self) -> Option<LexResult> {
        let c = self.advance();

        match c {
            // Single character lexemes
            '(' => Some(Ok(self.substring_into_token(TokenType::LeftParen))),
            ')' => Some(Ok(self.substring_into_token(TokenType::RightParen))),
            '{' => Some(Ok(self.substring_into_token(TokenType::LeftBrace))),
            '}' => Some(Ok(self.substring_into_token(TokenType::RightBrace))),
            ',' => Some(Ok(self.substring_into_token(TokenType::Comma))),
            '.' => Some(Ok(self.substring_into_token(TokenType::Dot))),
            '-' => Some(Ok(self.substring_into_token(TokenType::Minus))),
            '+' => Some(Ok(self.substring_into_token(TokenType::Plus))),
            ';' => Some(Ok(self.substring_into_token(TokenType::Semicolon))),
            '*' => Some(Ok(self.substring_into_token(TokenType::Star))),

            // Operators lexemes with optional additional characters
            '!' => Some(self.match_next_or('=', TokenType::BangEqual, TokenType::Bang)),
            '=' => Some(self.match_next_or('=', TokenType::EqualEqual, TokenType::Equal)),
            '<' => Some(self.match_next_or('=', TokenType::LessEqual, TokenType::Less)),
            '>' => Some(self.match_next_or('=', TokenType::GreaterEqual, TokenType::Greater)),

            // Slash, potentially either comments or a plain slash
            '/' => match self.peek() {
                Some('/') => match self.match_simple_comment() {
                    Ok(_) => None,
                    Err(e) => Some(Err(e)),
                },
                Some('*') => match self.match_c_comment() {
                    Ok(_) => None,
                    Err(e) => Some(Err(e)),
                },
                _ => Some(Ok(self.substring_into_token(TokenType::Slash))),
            },

            // Whitespace
            ' ' => None,
            '\r' => None,
            '\t' => None,
            '\n' => {
                self.line += 1;
                None
            }

            // Literals
            // Strings
            '"' => Some(self.match_string()),
            // Numbers
            '0'..='9' => Some(self.match_number()),
            // Identifiers
            'a'..='z' | 'A'..='Z' => Some(self.is_identifier()),

            // Unknown lexemes
            _ => {
                self.had_errors = true;
                Some(Err(format!(
                    "Lex error at line: {}, position: {}.",
                    self.line, self.current
                )))
            }
        }
    }

    fn match_next_or(
        &mut self,
        _expected_next: char,
        if_matches: TokenType,
        if_no_match: TokenType,
    ) -> LexResult {
        match self.peek() {
            Some(_expected_next) => {
                self.start = self.current - 1;
                self.current += 1;
                Ok(self.substring_into_token(if_matches))
            }
            _ => Ok(self.substring_into_token(if_no_match)),
        }
    }

    fn match_simple_comment(&mut self) -> LexResult {
        while let Some(next) = self.peek() {
            match next {
                '\n' => {
                    self.current += 1;
                    return Ok(Token::new(TokenType::Comment, "".to_string()));
                }
                _ => self.current += 1,
            }
        }

        Ok(Token::new(TokenType::EOF, "".to_string()))
    }

    fn match_c_comment(&mut self) -> LexResult {
        self.current += 1;

        while let Some(next) = self.peek() {
            match next {
                '*' => {
                    self.current += 1;
                    match self.peek() {
                        Some('/') => {
                            self.current += 1;
                            return Ok(Token::new(TokenType::Comment, "".to_string()));
                        }
                        _ => {
                            self.had_errors = true;
                            return Err(format!(
                                "Invalid comment at line: {}, position: {}.",
                                self.line, self.current
                            ));
                        }
                    }
                }
                _ => self.current += 1,
            }
        }

        Ok(Token::new(TokenType::EOF, "".to_string()))
    }

    fn match_string(&mut self) -> LexResult {
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
            "Unclosed string at line: {}, position: {}",
            start_line, self.start
        ))
    }

    fn match_number(&mut self) -> LexResult {
        while let Some(next) = self.peek() {
            match next {
                '0'..='9' => self.current += 1,
                '.' => {
                    self.current += 1;
                    match self.peek() {
                        Some(next_after_dot) => match next_after_dot {
                            '0'..='9' => self.current += 1,
                            _ => {
                                return Err(format!(
                                    "Invalid number at line: {}, position: {}",
                                    self.line, self.current
                                ))
                            }
                        },
                        None => {
                            return Err(format!(
                                "Invalid number at line: {}, position: {}",
                                self.line, self.current
                            ))
                        }
                    }
                }
                _ => return Ok(self.substring_into_token(TokenType::Number)),
            }
        }

        Ok(self.substring_into_token(TokenType::Number))
    }

    fn is_identifier(&mut self) -> LexResult {
        while let Some(next) = self.peek() {
            match next {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => self.current += 1,
                _ => {
                    let t = self.substring_into_token(TokenType::Identifier);
                    let reserved_keyword = t.is_reserved_keyword();
                    if let Some(token_type) = reserved_keyword {
                        return Ok(self.substring_into_token(token_type));
                    }

                    return Ok(t);
                }
            }
        }

        let t = self.substring_into_token(TokenType::Identifier);
        let reserved_keyword = t.is_reserved_keyword();
        if let Some(token_type) = reserved_keyword {
            return Ok(self.substring_into_token(token_type));
        }

        return Ok(t);
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

impl IntoIterator for Scanner {
    type Item = LexResult;
    type IntoIter = ScannerIntoIterator;

    fn into_iter(mut self) -> Self::IntoIter {
        let tokens = self.scan_tokens();
        let token_length = tokens.len();

        ScannerIntoIterator {
            tokens: tokens,
            index: 0,
            end: token_length,
        }
    }
}

pub struct ScannerIntoIterator {
    tokens: Vec<LexResult>,
    index: usize,
    end: usize,
}

impl Iterator for ScannerIntoIterator {
    type Item = LexResult;

    fn next(&mut self) -> Option<LexResult> {
        let result = match self.index < self.end {
            true => Some(self.tokens[self.index].clone()),
            false => None,
        };

        self.index += 1;
        result
    }
}
