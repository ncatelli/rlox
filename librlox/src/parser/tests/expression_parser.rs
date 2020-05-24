extern crate parcel;
use crate::ast::expression::{
    AdditionExpr, ComparisonExpr, EqualityExpr, Expr, MultiplicationExpr, PrimaryExpr, UnaryExpr,
};
use crate::ast::token::{Token, TokenType};
use crate::parser::expression_parser::expression;
use parcel::*;
use std::convert::TryFrom;

fn match_literal_helper(token: Token) {
    let seed_vec = vec![token.clone()];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[1..],
            Expr::Primary(PrimaryExpr::try_from(token).unwrap())
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_equality_expression() {
    let op_token = Token::new(TokenType::EqualEqual, 1, Option::None, Option::None);
    let literal_token = Token::new(
        TokenType::Number,
        1,
        Option::Some("1.0".to_string()),
        Option::Some(obj_number!(1.0)),
    );
    let seed_vec = vec![
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[3..],
            Expr::Equality(EqualityExpr::Equal(
                Box::new(Expr::Primary(
                    PrimaryExpr::try_from(literal_token.clone()).unwrap()
                )),
                Box::new(Expr::Primary(
                    PrimaryExpr::try_from(literal_token.clone()).unwrap()
                ))
            ))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_many_equality_expression() {
    let op_token = Token::new(TokenType::EqualEqual, 1, Option::None, Option::None);
    let literal_token = Token::new(
        TokenType::Number,
        1,
        Option::Some("1.0".to_string()),
        Option::Some(obj_number!(1.0)),
    );
    let seed_vec = vec![
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[5..],
            Expr::Equality(EqualityExpr::Equal(
                Box::new(Expr::Primary(
                    PrimaryExpr::try_from(literal_token.clone()).unwrap()
                )),
                Box::new(Expr::Equality(EqualityExpr::Equal(
                    Box::new(Expr::Primary(
                        PrimaryExpr::try_from(literal_token.clone()).unwrap()
                    )),
                    Box::new(Expr::Primary(
                        PrimaryExpr::try_from(literal_token.clone()).unwrap()
                    ))
                )))
            ))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_comparison_expression() {
    let op_token = Token::new(TokenType::GreaterEqual, 1, Option::None, Option::None);
    let literal_token = Token::new(
        TokenType::Number,
        1,
        Option::Some("1.0".to_string()),
        Option::Some(obj_number!(1.0)),
    );
    let seed_vec = vec![
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[3..],
            Expr::Comparison(ComparisonExpr::GreaterEqual(
                Box::new(Expr::Primary(
                    PrimaryExpr::try_from(literal_token.clone()).unwrap()
                )),
                Box::new(Expr::Primary(
                    PrimaryExpr::try_from(literal_token.clone()).unwrap()
                ))
            ))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_many_comparison_expression() {
    let op_token = Token::new(TokenType::GreaterEqual, 1, Option::None, Option::None);
    let literal_token = Token::new(
        TokenType::Number,
        1,
        Option::Some("1.0".to_string()),
        Option::Some(obj_number!(1.0)),
    );
    let seed_vec = vec![
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[5..],
            Expr::Comparison(ComparisonExpr::GreaterEqual(
                Box::new(Expr::Primary(
                    PrimaryExpr::try_from(literal_token.clone()).unwrap()
                )),
                Box::new(Expr::Comparison(ComparisonExpr::GreaterEqual(
                    Box::new(Expr::Primary(
                        PrimaryExpr::try_from(literal_token.clone()).unwrap()
                    )),
                    Box::new(Expr::Primary(
                        PrimaryExpr::try_from(literal_token.clone()).unwrap()
                    ))
                )))
            ))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_addition_expression() {
    let op_token = Token::new(TokenType::Plus, 1, Option::None, Option::None);
    let literal_token = Token::new(
        TokenType::Number,
        1,
        Option::Some("1.0".to_string()),
        Option::Some(obj_number!(1.0)),
    );
    let seed_vec = vec![
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[3..],
            Expr::Addition(AdditionExpr::Add(
                Box::new(Expr::Primary(
                    PrimaryExpr::try_from(literal_token.clone()).unwrap()
                )),
                Box::new(Expr::Primary(
                    PrimaryExpr::try_from(literal_token.clone()).unwrap()
                ))
            ))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_many_addition_expression() {
    let op_token = Token::new(TokenType::Plus, 1, Option::None, Option::None);
    let literal_token = Token::new(
        TokenType::Number,
        1,
        Option::Some("1.0".to_string()),
        Option::Some(obj_number!(1.0)),
    );
    let seed_vec = vec![
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[5..],
            Expr::Addition(AdditionExpr::Add(
                Box::new(Expr::Primary(
                    PrimaryExpr::try_from(literal_token.clone()).unwrap()
                )),
                Box::new(Expr::Addition(AdditionExpr::Add(
                    Box::new(Expr::Primary(
                        PrimaryExpr::try_from(literal_token.clone()).unwrap()
                    )),
                    Box::new(Expr::Primary(
                        PrimaryExpr::try_from(literal_token.clone()).unwrap()
                    ))
                )))
            ))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_multiplication_expression() {
    let op_token = Token::new(TokenType::Star, 1, Option::None, Option::None);
    let literal_token = Token::new(
        TokenType::Number,
        1,
        Option::Some("1.0".to_string()),
        Option::Some(obj_number!(1.0)),
    );
    let seed_vec = vec![
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[3..],
            Expr::Multiplication(MultiplicationExpr::Multiply(
                Box::new(Expr::Primary(
                    PrimaryExpr::try_from(literal_token.clone()).unwrap()
                )),
                Box::new(Expr::Primary(
                    PrimaryExpr::try_from(literal_token.clone()).unwrap()
                ))
            ))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_many_multiplication_expression() {
    let op_token = Token::new(TokenType::Star, 1, Option::None, Option::None);
    let literal_token = Token::new(
        TokenType::Number,
        1,
        Option::Some("1.0".to_string()),
        Option::Some(obj_number!(1.0)),
    );
    let seed_vec = vec![
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[5..],
            Expr::Multiplication(MultiplicationExpr::Multiply(
                Box::new(Expr::Primary(
                    PrimaryExpr::try_from(literal_token.clone()).unwrap()
                )),
                Box::new(Expr::Multiplication(MultiplicationExpr::Multiply(
                    Box::new(Expr::Primary(
                        PrimaryExpr::try_from(literal_token.clone()).unwrap()
                    )),
                    Box::new(Expr::Primary(
                        PrimaryExpr::try_from(literal_token.clone()).unwrap()
                    ))
                )))
            ))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_unary_expression() {
    let op_token = Token::new(TokenType::Bang, 1, Option::None, Option::None);
    let literal_token = Token::new(
        TokenType::Number,
        1,
        Option::Some("1.0".to_string()),
        Option::Some(obj_number!(1.0)),
    );
    let seed_vec = vec![op_token.clone(), literal_token.clone()];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[2..],
            Expr::Unary(UnaryExpr::Bang(Box::new(Expr::Primary(
                PrimaryExpr::try_from(literal_token).unwrap()
            ))))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_primary_expression() {
    match_literal_helper(Token::new(
        TokenType::Number,
        1,
        Option::Some("1.0".to_string()),
        Option::Some(obj_number!(1.0)),
    ))
}

#[test]
fn validate_parser_should_parse_grouping_expression() {
    let literal_token = Token::new(
        TokenType::Number,
        1,
        Option::Some("1.0".to_string()),
        Option::Some(obj_number!(1.0)),
    );
    let seed_vec = vec![
        Token::new(TokenType::LeftParen, 1, Option::None, Option::None),
        literal_token.clone(),
        Token::new(TokenType::RightParen, 1, Option::None, Option::None),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[3..],
            Expr::Grouping(Box::new(Expr::Primary(
                PrimaryExpr::try_from(literal_token).unwrap()
            )))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_throw_error_on_invalid_expression() {
    let literal_token = Token::new(
        TokenType::Number,
        1,
        Option::Some("1.0".to_string()),
        Option::Some(obj_number!(1.0)),
    );
    let seed_vec = vec![
        Token::new(TokenType::LeftParen, 1, Option::None, Option::None),
        literal_token.clone(),
        Token::new(TokenType::Semicolon, 1, Option::None, Option::None),
    ];

    assert_eq!(
        Ok(MatchStatus::NoMatch(&seed_vec[..])),
        expression().parse(&seed_vec)
    );
}
