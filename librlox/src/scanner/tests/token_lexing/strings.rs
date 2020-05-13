extern crate parcel;
use crate::scanner::tokens::{Literal, Token, TokenType};
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
}

#[test]
fn scan_tokens_combinator_should_lex_full_string() {
    compare_literal!(
        "\"test\"",
        Literal::Str("test".to_string()),
        TokenType::Literal
    );
}
