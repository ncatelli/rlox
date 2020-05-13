extern crate parcel;
use crate::scanner::tokens::{Literal, Token, TokenType};
use parcel::prelude::v1::*;

macro_rules! compare_single_token {
    ($source:expr, $tt: expr) => {
        let input: Vec<char> = $source.chars().collect();
        let token_results = crate::scanner::source_scanner::scan_tokens_combinator().parse(&input);

        assert_eq!(
            token_results,
            Ok(parcel::MatchStatus::Match((
                &input[input.len()..],
                vec![Token {
                    token_type: $tt,
                    literal: None,
                }]
            )))
        );
    };
}

macro_rules! compare_literal {
    ($source:expr, $lit:expr, $tt: expr) => {
        let input: Vec<char> = $source.chars().collect();
        let token_results = crate::scanner::source_scanner::scan_tokens_combinator().parse(&input);

        assert_eq!(
            token_results,
            Ok(parcel::MatchStatus::Match((
                &input[input.len()..],
                vec![Token {
                    token_type: $tt,
                    literal: Some($lit),
                }]
            )))
        );
    };
    ($source:expr, $assertion:expr) => {
        let input: Vec<char> = $source.chars().collect();
        let token_results = crate::scanner::source_scanner::scan_tokens_combinator().parse(&input);

        assert_eq!(token_results, $assertion);
    };
}

#[test]
fn scan_tokens_combinator_should_lex_identifiers() {
    compare_literal!(
        "test_identifier_1_alpha",
        Literal::Identifier("test_identifier_1_alpha".to_string()),
        TokenType::Literal
    );
}

#[test]
fn scan_tokens_combinator_should_separate_identifier_on_non_alpha() {
    let input = "test_identifier_1_alpha\n";
    let input_chars: Vec<char> = input.chars().collect();

    compare_literal!(
        &input,
        Ok(parcel::MatchStatus::Match((
            &input_chars[input.len() - 1..],
            vec![Token {
                token_type: TokenType::Literal,
                literal: Some(Literal::Identifier(input.trim().to_string())),
            },]
        )))
    );
}

#[test]
fn scan_tokens_combinator_should_lex_reserved_keywords() {
    compare_single_token!("and", TokenType::And);
    compare_single_token!("or", TokenType::Or);
    compare_single_token!("print", TokenType::Print);
    compare_single_token!("return", TokenType::Return);
    compare_single_token!("super", TokenType::Super);
    compare_single_token!("class", TokenType::Class);
    compare_single_token!("this", TokenType::This);
    compare_single_token!("nil", TokenType::Nil);
    compare_single_token!("true", TokenType::True);
    compare_single_token!("false", TokenType::False);
    compare_single_token!("var", TokenType::Var);
    compare_single_token!("fun", TokenType::Fun);
    compare_single_token!("while", TokenType::While);
    compare_single_token!("for", TokenType::For);
    compare_single_token!("if", TokenType::If);
    compare_single_token!("else", TokenType::Else);
}
