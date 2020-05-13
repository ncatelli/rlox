extern crate parcel;
use crate::scanner::tokens::{Literal, TokenType};
use crate::scanner::*;
use parcel::Parser;

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
fn scan_tokens_combinator_should_lex_digit() {
    compare_literal!("123", Literal::Number(123.0), TokenType::Literal);
}

#[test]
fn scan_tokens_combinator_should_lex_floating_point() {
    compare_literal!("123.45", Literal::Number(123.45), TokenType::Literal);
}

// TODO: I can't think of a good way to express this with parcel currently.
/*
#[test]
fn scan_tokens_combinator_should_not_allow_trailing_decimal() {
    let input = "123.".to_string();
    let input_chars: Vec<char> = input.chars().collect();

    compare_literal!(
        input,
        //Ok(parcel::MatchStatus::Match((&input_chars[0..], vec![])))
        Err(_)
    );
}*/

#[test]
fn scan_tokens_combinator_should_allow_numbers_to_include_operators() {
    let input = "5+5".to_string();
    let input_chars: Vec<char> = input.chars().collect();

    compare_literal!(
        &input,
        Ok(parcel::MatchStatus::Match((
            &input_chars[input.len()..],
            vec![
                Token {
                    token_type: TokenType::Literal,
                    literal: Some(Literal::Number(5.0)),
                },
                Token {
                    token_type: TokenType::Plus,
                    literal: None,
                },
                Token {
                    token_type: TokenType::Literal,
                    literal: Some(Literal::Number(5.0)),
                },
            ]
        )))
    );
}
