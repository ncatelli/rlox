use std::option::Option;
use std::option::Option::{None, Some};

use std::iter::Iterator;

pub mod tokens;
use tokens::{Token, TokenType};

#[cfg(test)]
mod tests;

pub type LexResult = Result<Token, String>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cursor {
    index: usize,
    col: usize,
    line: usize,
}

impl Cursor {
    pub fn new(index: usize, col: usize, line: usize) -> Cursor {
        Cursor {
            index: index,
            col: col,
            line: line,
        }
    }

    pub fn advance(cursor: Cursor) -> Cursor {
        Cursor {
            index: cursor.index + 1,
            col: cursor.col + 1,
            line: cursor.line,
        }
    }

    pub fn reverse(cursor: Cursor) -> Cursor {
        Cursor {
            index: cursor.index - 1,
            col: cursor.col - 1,
            line: cursor.line,
        }
    }

    pub fn newline(cursor: Cursor) -> Cursor {
        Cursor {
            index: cursor.index,
            col: 0,
            line: cursor.line + 1,
        }
    }
}

pub struct Scanner {
    source: Vec<char>,
    end: usize,
    had_errors: bool,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let chars: Vec<char> = source.chars().collect();
        let end = chars.len();
        Scanner {
            source: chars,
            end: end,
            had_errors: false,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<LexResult> {
        let mut tokens: Vec<LexResult> = Vec::new();
        let mut cursor = Cursor::new(0, 0, 1);

        while !self.is_at_end(cursor) {
            let (t, next_cursor) = match self.scan_token(cursor) {
                (Some(tok), nc) => (tok, nc),
                (None, nc) => {
                    cursor = Cursor::advance(nc);
                    continue;
                }
            };

            cursor = Cursor::advance(next_cursor);
            tokens.push(t);
        }

        tokens.push(Ok(Token::new(TokenType::EOF, "".to_string())));
        tokens
    }

    fn scan_token(&mut self, cursor: Cursor) -> (Option<LexResult>, Cursor) {
        let start = cursor;
        let current = self.char_at(start).unwrap();

        match current {
            // Single character lexemes
            '(' => (
                Some(Ok(self.substring_into_token(
                    start,
                    cursor,
                    TokenType::LeftParen,
                ))),
                cursor,
            ),
            ')' => (
                Some(Ok(self.substring_into_token(
                    start,
                    cursor,
                    TokenType::RightParen,
                ))),
                cursor,
            ),
            '{' => (
                Some(Ok(self.substring_into_token(
                    start,
                    cursor,
                    TokenType::LeftBrace,
                ))),
                cursor,
            ),
            '}' => (
                Some(Ok(self.substring_into_token(
                    start,
                    cursor,
                    TokenType::RightBrace,
                ))),
                cursor,
            ),
            ',' => (
                Some(Ok(self.substring_into_token(
                    start,
                    cursor,
                    TokenType::Comma,
                ))),
                cursor,
            ),
            '.' => (
                Some(Ok(self.substring_into_token(start, cursor, TokenType::Dot))),
                cursor,
            ),
            '-' => (
                Some(Ok(self.substring_into_token(
                    start,
                    cursor,
                    TokenType::Minus,
                ))),
                cursor,
            ),
            '+' => (
                Some(Ok(self.substring_into_token(
                    start,
                    cursor,
                    TokenType::Plus,
                ))),
                cursor,
            ),
            ';' => (
                Some(Ok(self.substring_into_token(
                    start,
                    cursor,
                    TokenType::Semicolon,
                ))),
                cursor,
            ),
            '*' => (
                Some(Ok(self.substring_into_token(
                    start,
                    cursor,
                    TokenType::Star,
                ))),
                cursor,
            ),

            // Operators lexemes with optional additional characters
            '!' => {
                let (lex_result, next_cursor) =
                    self.match_next_or(cursor, '=', TokenType::BangEqual, TokenType::Bang);
                (Some(lex_result), next_cursor)
            }
            '=' => {
                let (lex_result, next_cursor) =
                    self.match_next_or(cursor, '=', TokenType::EqualEqual, TokenType::Equal);
                (Some(lex_result), next_cursor)
            }
            '<' => {
                let (lex_result, next_cursor) =
                    self.match_next_or(cursor, '=', TokenType::LessEqual, TokenType::Less);
                (Some(lex_result), next_cursor)
            }
            '>' => {
                let (lex_result, next_cursor) =
                    self.match_next_or(cursor, '=', TokenType::GreaterEqual, TokenType::Greater);
                (Some(lex_result), next_cursor)
            }

            // Slash, potentially either comments or a plain slash
            '/' => {
                let peek = Cursor::advance(cursor);
                match self.char_at(peek) {
                    Some('/') => match self.match_simple_comment(peek) {
                        (Ok(_), next_cursor) => (None, next_cursor),
                        (Err(e), next_cursor) => (Some(Err(e)), next_cursor),
                    },
                    Some('*') => match self.match_c_comment(peek) {
                        (Ok(_), next_cursor) => (None, next_cursor),
                        (Err(e), next_cursor) => (Some(Err(e)), next_cursor),
                    },
                    _ => (
                        Some(Ok(self.substring_into_token(
                            start,
                            cursor,
                            TokenType::Slash,
                        ))),
                        cursor,
                    ),
                }
            }

            // Whitespace
            ' ' => (None, cursor),
            '\r' => (None, cursor),
            '\t' => (None, cursor),
            '\n' => (None, Cursor::newline(cursor)),

            // Literals
            // Strings
            '"' => {
                let (lex_result, next_cursor) = self.match_string(cursor);
                (Some(lex_result), next_cursor)
            }
            // Numbers
            '0'..='9' => {
                let (lex_result, next_cursor) = self.match_number(cursor);
                (Some(lex_result), next_cursor)
            }
            // Identifiers
            'a'..='z' | 'A'..='Z' => {
                let (lex_result, next_cursor) = self.match_identifier(cursor);
                (Some(lex_result), next_cursor)
            }
            // Unknown lexemes
            _ => {
                self.had_errors = true;
                (
                    Some(Err(format!(
                        "Lex error at line: {}, position: {}.",
                        cursor.line, cursor.col
                    ))),
                    cursor,
                )
            }
        }
    }

    fn match_next_or(
        &self,
        start: Cursor,
        _expected_next: char,
        if_matches: TokenType,
        if_no_match: TokenType,
    ) -> (LexResult, Cursor) {
        let current = Cursor::advance(start);

        match self.char_at(current) {
            Some(_expected_next) => (
                Ok(self.substring_into_token(start, current, if_matches)),
                current,
            ),
            _ => (
                Ok(self.substring_into_token(start, start, if_no_match)),
                start,
            ),
        }
    }

    fn match_simple_comment(&mut self, start: Cursor) -> (LexResult, Cursor) {
        let mut current = start;
        loop {
            current = Cursor::advance(current);
            match self.char_at(current) {
                Some('\n') => {
                    return (
                        Ok(Token::new(TokenType::Comment, "".to_string())),
                        Cursor::newline(current),
                    );
                }
                _ => (),
            }
        }
    }

    fn match_c_comment(&mut self, start: Cursor) -> (LexResult, Cursor) {
        let mut current = start;
        loop {
            current = Cursor::advance(current);
            match self.char_at(current) {
                Some('*') => {
                    let peek = Cursor::advance(current);
                    match self.char_at(peek) {
                        Some('/') => {
                            return (Ok(Token::new(TokenType::Comment, "".to_string())), peek);
                        }
                        _ => {
                            self.had_errors = true;
                            return (
                                Err(format!(
                                    "Invalid comment at line: {}, position: {}.",
                                    peek.line, peek.col
                                )),
                                peek,
                            );
                        }
                    }
                }
                _ => (),
            }
        }
    }

    fn match_string(&mut self, start: Cursor) -> (LexResult, Cursor) {
        let start = Cursor::advance(start);
        let mut current = start;

        loop {
            current = Cursor::advance(current);
            match self.char_at(current) {
                Some('"') => {
                    //reverse reader one step to negate quote
                    let token_str =
                        self.substring_into_token(start, Cursor::reverse(current), TokenType::Str);
                    return (Ok(token_str), Cursor::advance(current));
                }
                Some(_) => continue,
                None => {
                    return (
                        Err(format!(
                            "Unclosed string at line: {}, position: {}",
                            current.line, current.col
                        )),
                        current,
                    )
                }
            }
        }
    }

    fn match_number(&mut self, start: Cursor) -> (LexResult, Cursor) {
        let mut current = start;
        loop {
            current = Cursor::advance(current);
            match self.char_at(current) {
                Some('0'..='9') => (),
                Some('.') => {
                    let peek = Cursor::advance(current);
                    match self.char_at(peek) {
                        Some(next_after_dot) => match next_after_dot {
                            '0'..='9' => (),
                            _ => {
                                return (
                                    Err(format!(
                                        "Invalid number at line: {}, position: {}",
                                        current.line, current.col
                                    )),
                                    current,
                                )
                            }
                        },
                        None => {
                            return (
                                Err(format!(
                                    "Invalid number at line: {}, position: {}",
                                    current.line, current.col
                                )),
                                current,
                            )
                        }
                    }
                }
                _ => {
                    return (
                        Ok(self.substring_into_token(
                            start,
                            Cursor::reverse(current),
                            TokenType::Number,
                        )),
                        current,
                    );
                }
            }
        }
    }

    fn match_identifier(&mut self, start: Cursor) -> (LexResult, Cursor) {
        let mut current = start.clone();
        loop {
            current = Cursor::advance(current);
            match self.char_at(current) {
                Some('a'..='z') | Some('A'..='Z') | Some('0'..='9') | Some('_') => continue,
                _ => {
                    let t = self.substring_into_token(
                        start,
                        Cursor::reverse(current),
                        TokenType::Identifier,
                    );
                    let reserved_keyword = t.is_reserved_keyword();
                    if let Some(token_type) = reserved_keyword {
                        return (
                            Ok(self.substring_into_token(
                                start,
                                Cursor::reverse(current),
                                token_type,
                            )),
                            current,
                        );
                    }

                    return (Ok(t), current);
                }
            }
        }
    }

    fn is_at_end(&self, cursor: Cursor) -> bool {
        cursor.index >= self.end
    }

    fn substring(&self, start: Cursor, end: Cursor) -> &[char] {
        self.source.get(start.index..=end.index).unwrap()
    }

    fn substring_into_token(&self, start: Cursor, current: Cursor, token_type: TokenType) -> Token {
        let token_range = &self.substring(start, current);
        let literal: String = token_range.iter().collect();

        Token::new(token_type, literal)
    }

    fn char_at(&self, cursor: Cursor) -> Option<char> {
        let index = cursor.index;

        match self.source.get(index) {
            Some(c) => Some(*c),
            None => None,
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
