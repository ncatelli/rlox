extern crate parcel;
use parcel::Parser;
use std::iter::Iterator;
use std::option::Option;
use std::option::Option::{None, Some};

use super::tokens::{Literal, Token, TokenType};

/// LexResult is an alias that represents the result of an attempt to lex a
/// single character token. Returning either the Token or a string containing
/// positional data for the error.
pub type LexResult = Result<Token, String>;

/// Cursor stores positional data for the scanner. Actively tracking index into
/// the source Vector, the current column and line of the token being parsed.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Cursor {
    index: usize,
    col: usize,
    line: usize,
}

impl Cursor {
    fn new(index: usize, col: usize, line: usize) -> Cursor {
        Cursor { index, col, line }
    }

    fn advance(cursor: Cursor) -> Cursor {
        Cursor {
            index: cursor.index + 1,
            col: cursor.col + 1,
            line: cursor.line,
        }
    }

    fn reverse(cursor: Cursor) -> Cursor {
        Cursor {
            index: cursor.index - 1,
            col: cursor.col - 1,
            line: cursor.line,
        }
    }

    fn newline(cursor: Cursor) -> Cursor {
        Cursor {
            index: cursor.index,
            col: 0,
            line: cursor.line + 1,
        }
    }
}

/// Scanner takes a string representing lox source and attempts to convert the
/// source into a vector of either Tokens or lexical errors.
///
/// # Examples
/// ```
/// extern crate librlox;
/// use librlox::scanner;
/// let source = "* ; - \"hello world\" 1234.5".to_string();
/// let s = scanner::Scanner::new(source);
///
/// let _tokens = s.scan_tokens().into_iter();
/// ```
pub struct Scanner {
    source: Vec<char>,
    end: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let chars: Vec<char> = source.chars().collect();
        let end = chars.len();
        Scanner { source: chars, end }
    }

    pub fn scan_tokens(&self) -> Vec<LexResult> {
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

        tokens.push(Ok(Token::new(TokenType::EOF, None)));
        tokens
    }

    fn scan_token(&self, cursor: Cursor) -> (Option<LexResult>, Cursor) {
        let start = cursor;
        let current = self.char_at(start).unwrap();

        match current {
            // Single character lexemes
            '(' => (Some(Ok(Token::new(TokenType::LeftParen, None))), cursor),
            ')' => (Some(Ok(Token::new(TokenType::RightParen, None))), cursor),
            '{' => (Some(Ok(Token::new(TokenType::LeftBrace, None))), cursor),
            '}' => (Some(Ok(Token::new(TokenType::RightBrace, None))), cursor),
            ',' => (Some(Ok(Token::new(TokenType::Comma, None))), cursor),
            '.' => (Some(Ok(Token::new(TokenType::Dot, None))), cursor),
            '-' => (Some(Ok(Token::new(TokenType::Minus, None))), cursor),
            '+' => (Some(Ok(Token::new(TokenType::Plus, None))), cursor),
            ';' => (Some(Ok(Token::new(TokenType::Semicolon, None))), cursor),
            '*' => (Some(Ok(Token::new(TokenType::Star, None))), cursor),

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
                    _ => (Some(Ok(Token::new(TokenType::Slash, None))), cursor),
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
            _ => (
                Some(Err(format!(
                    "Lex error at line: {}, position: {}.",
                    cursor.line, cursor.col
                ))),
                cursor,
            ),
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
            Some(_expected_next) => (Ok(Token::new(if_matches, None)), current),
            _ => (Ok(Token::new(if_no_match, None)), start),
        }
    }

    fn match_simple_comment(&self, start: Cursor) -> (LexResult, Cursor) {
        let mut current = start;
        loop {
            current = Cursor::advance(current);
            if let Some('\n') = self.char_at(current) {
                return (
                    Ok(Token::new(TokenType::Comment, None)),
                    Cursor::newline(current),
                );
            }
        }
    }

    fn match_c_comment(&self, start: Cursor) -> (LexResult, Cursor) {
        let mut current = start;
        loop {
            current = Cursor::advance(current);
            if let Some('*') = self.char_at(current) {
                let peek = Cursor::advance(current);
                match self.char_at(peek) {
                    Some('/') => {
                        return (Ok(Token::new(TokenType::Comment, None)), peek);
                    }
                    _ => {
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
        }
    }

    fn match_string(&self, start: Cursor) -> (LexResult, Cursor) {
        let start = Cursor::advance(start);
        let mut current = start;

        loop {
            current = Cursor::advance(current);
            match self.char_at(current) {
                Some('"') => {
                    //reverse reader one step to negate quote
                    let literal_str = self
                        .substring(start, Cursor::reverse(current))
                        .iter()
                        .collect();
                    return (
                        Ok(Token::new(TokenType::Str, Some(Literal::Str(literal_str)))),
                        current,
                    );
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

    fn match_number(&self, start: Cursor) -> (LexResult, Cursor) {
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
                    //reverse reader one step to negate quote
                    let literal_str: String = self
                        .substring(start, Cursor::reverse(current))
                        .iter()
                        .collect();

                    return match literal_str.parse() {
                        Ok(n) => (
                            Ok(Token::new(TokenType::Number, Some(Literal::Number(n)))),
                            current,
                        ),
                        Err(_) => (
                            Err(format!(
                                "Invalid number at line: {}, position: {}",
                                current.line, current.col,
                            )),
                            current,
                        ),
                    };
                }
            }
        }
    }

    fn match_identifier(&self, start: Cursor) -> (LexResult, Cursor) {
        let mut current = start;
        loop {
            current = Cursor::advance(current);
            match self.char_at(current) {
                Some('a'..='z') | Some('A'..='Z') | Some('0'..='9') | Some('_') => continue,
                _ => {
                    let literal_str: String = self
                        .substring(start, Cursor::reverse(current))
                        .iter()
                        .collect();
                    let t = Token::new(
                        TokenType::Identifier,
                        Some(Literal::Identifier(literal_str)),
                    );

                    return match t.is_reserved_keyword() {
                        Some(token_type) => (Ok(Token::new(token_type, None)), current),
                        None => (Ok(t), current),
                    };
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

    fn char_at(&self, cursor: Cursor) -> Option<char> {
        let index = cursor.index;

        match self.source.get(index) {
            Some(c) => Some(*c),
            None => None,
        }
    }
}

// parsers
fn zero_or_more<'a, P, A: 'a, B>(parser: P) -> impl parcel::Parser<'a, A, Vec<B>>
where
    A: Copy + 'a,
    P: parcel::Parser<'a, A, B>,
{
    move |mut input| {
        let mut result_acc: Vec<B> = Vec::new();
        while let Ok(parcel::MatchStatus::Match((next_input, result))) = parser.parse(input) {
            input = next_input;
            result_acc.push(result);
        }

        Ok(parcel::MatchStatus::Match((input, result_acc)))
    }
}

fn whitespace<'a>() -> impl parcel::Parser<'a, &'a [char], char> {
    match_char(' ')
        .or(|| match_char('\t'))
        .or(|| match_char('\r'))
        .or(|| match_char('\n'))
}

fn numeric<'a>() -> impl parcel::Parser<'a, &'a [char], char> {
    move |input: &'a [char]| match input.get(0) {
        Some(next) if next.is_digit(10) => Ok(parcel::MatchStatus::Match((&input[1..], *next))),
        _ => Ok(parcel::MatchStatus::NoMatch(input)),
    }
}

fn alpha<'a>() -> impl parcel::Parser<'a, &'a [char], char> {
    move |input: &'a [char]| match input.get(0) {
        Some(next) if next.is_alphabetic() => Ok(parcel::MatchStatus::Match((&input[1..], *next))),
        _ => Ok(parcel::MatchStatus::NoMatch(input)),
    }
}

fn not_char<'a>(expected: char) -> impl parcel::Parser<'a, &'a [char], char> {
    move |input: &'a [char]| match input.get(0) {
        Some(next) if *next != expected => Ok(parcel::MatchStatus::Match((&input[1..], *next))),
        _ => Ok(parcel::MatchStatus::NoMatch(input)),
    }
}

fn match_char<'a>(expected: char) -> impl parcel::Parser<'a, &'a [char], char> {
    move |input: &'a [char]| match input.get(0) {
        Some(next) if *next == expected => Ok(parcel::MatchStatus::Match((&input[1..], *next))),
        _ => Ok(parcel::MatchStatus::NoMatch(input)),
    }
}

fn two_char_token<'a>() -> impl parcel::Parser<'a, &'a [char], Token> {
    parcel::join(match_char('!'), match_char('='))
        .or(|| parcel::join(match_char('='), match_char('=')))
        .or(|| parcel::join(match_char('<'), match_char('=')))
        .or(|| parcel::join(match_char('>'), match_char('=')))
        .map(|chars| match chars {
            ('!', '=') => Token::new(TokenType::BangEqual, None),
            ('=', '=') => Token::new(TokenType::EqualEqual, None),
            ('<', '=') => Token::new(TokenType::LessEqual, None),
            ('>', '=') => Token::new(TokenType::GreaterEqual, None),
            // unreachable case
            _ => Token::new(TokenType::EOF, None),
        })
}

fn single_char_token<'a>() -> impl parcel::Parser<'a, &'a [char], Token> {
    match_char('(')
        .or(|| match_char(')'))
        .or(|| match_char('{'))
        .or(|| match_char('}'))
        .or(|| match_char(','))
        .or(|| match_char('.'))
        .or(|| match_char('-'))
        .or(|| match_char('+'))
        .or(|| match_char(';'))
        .or(|| match_char('*'))
        .or(|| match_char('!'))
        .or(|| match_char('='))
        .or(|| match_char('<'))
        .or(|| match_char('>'))
        .map(|c| Token::from(c))
}

/// TODO: can simplify this a bit more. I don't think there is a need for the
/// parent or combinator
fn match_number<'a>() -> impl parcel::Parser<'a, &'a [char], Token> {
    parcel::or(
        parcel::take_while(numeric()).map(|n| {
            Token::new(
                TokenType::Number,
                Some(Literal::Number(
                    n.into_iter().collect::<String>().parse().unwrap(),
                )),
            )
        }),
        || {
            parcel::join(
                parcel::take_while(numeric()),
                parcel::right(parcel::join(match_char('.'), zero_or_more(numeric()))),
            )
            .map(|(mut whole, mut decimal)| {
                whole.push('.');
                whole.append(&mut decimal);
                Token::new(
                    TokenType::Number,
                    Some(Literal::Number(
                        whole.into_iter().collect::<String>().parse().unwrap(),
                    )),
                )
            })
        },
    )
}

fn match_string<'a>() -> impl parcel::Parser<'a, &'a [char], Token> {
    parcel::right(parcel::join(
        match_char('"'),
        parcel::left(parcel::join(
            parcel::take_while(not_char('"')),
            match_char('"'),
        )),
    ))
    .map(|literal| {
        Token::new(
            TokenType::Str,
            Some(Literal::Str(literal.into_iter().collect())),
        )
    })
}

impl IntoIterator for Scanner {
    type Item = LexResult;
    type IntoIter = ScannerIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        let tokens = self.scan_tokens();
        let token_length = tokens.len();

        ScannerIntoIterator {
            tokens,
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
        if self.index < self.end {
            self.index += 1;
            Some(self.tokens[self.index - 1].clone())
        } else {
            None
        }
    }
}
