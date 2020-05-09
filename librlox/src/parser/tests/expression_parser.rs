extern crate parcel;
use crate::parser::expression::{
    AdditionExpr, AdditionExprOperator, ComparisonExpr, ComparisonExprOperator, EqualityExpr,
    EqualityExprOperator, Expr, GroupingExpr, MultiplicationExpr, MultiplicationExprOperator,
    PrimaryExpr, UnaryExpr, UnaryExprOperator,
};
use crate::parser::expression_parser::expression;
use crate::scanner::tokens::{Literal, Token, TokenType};
use parcel::*;

fn match_literal_helper(token: Token) {
    let seed_vec = vec![token.clone()];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[1..],
            Expr::Primary(PrimaryExpr::new(token))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_equality_expression() {
    let op_token = Token::new(TokenType::EqualEqual, Option::None);
    let literal_token = Token::new(TokenType::Literal, Option::Some(Literal::Number(1.0)));
    let seed_vec = vec![
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[3..],
            Expr::Equality(EqualityExpr::new(
                EqualityExprOperator::Equal,
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone()))),
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone())))
            ))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_many_equality_expression() {
    let op_token = Token::new(TokenType::EqualEqual, Option::None);
    let literal_token = Token::new(TokenType::Literal, Option::Some(Literal::Number(1.0)));
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
            Expr::Equality(EqualityExpr::new(
                EqualityExprOperator::Equal,
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone()))),
                Box::new(Expr::Equality(EqualityExpr::new(
                    EqualityExprOperator::Equal,
                    Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone()))),
                    Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone())))
                )))
            ))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_comparison_expression() {
    let op_token = Token::new(TokenType::GreaterEqual, Option::None);
    let literal_token = Token::new(TokenType::Literal, Option::Some(Literal::Number(1.0)));
    let seed_vec = vec![
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[3..],
            Expr::Comparison(ComparisonExpr::new(
                ComparisonExprOperator::GreaterEqual,
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone()))),
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone())))
            ))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_many_comparison_expression() {
    let op_token = Token::new(TokenType::GreaterEqual, Option::None);
    let literal_token = Token::new(TokenType::Literal, Option::Some(Literal::Number(1.0)));
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
            Expr::Comparison(ComparisonExpr::new(
                ComparisonExprOperator::GreaterEqual,
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone()))),
                Box::new(Expr::Comparison(ComparisonExpr::new(
                    ComparisonExprOperator::GreaterEqual,
                    Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone()))),
                    Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone())))
                )))
            ))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_addition_expression() {
    let op_token = Token::new(TokenType::Plus, Option::None);
    let literal_token = Token::new(TokenType::Literal, Option::Some(Literal::Number(1.0)));
    let seed_vec = vec![
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[3..],
            Expr::Addition(AdditionExpr::new(
                AdditionExprOperator::Addition,
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone()))),
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone())))
            ))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_many_addition_expression() {
    let op_token = Token::new(TokenType::Plus, Option::None);
    let literal_token = Token::new(TokenType::Literal, Option::Some(Literal::Number(1.0)));
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
            Expr::Addition(AdditionExpr::new(
                AdditionExprOperator::Addition,
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone()))),
                Box::new(Expr::Addition(AdditionExpr::new(
                    AdditionExprOperator::Addition,
                    Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone()))),
                    Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone())))
                )))
            ))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_multiplication_expression() {
    let op_token = Token::new(TokenType::Star, Option::None);
    let literal_token = Token::new(TokenType::Literal, Option::Some(Literal::Number(1.0)));
    let seed_vec = vec![
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[3..],
            Expr::Multiplication(MultiplicationExpr::new(
                MultiplicationExprOperator::Multiply,
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone()))),
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone())))
            ))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_many_multiplication_expression() {
    let op_token = Token::new(TokenType::Star, Option::None);
    let literal_token = Token::new(TokenType::Literal, Option::Some(Literal::Number(1.0)));
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
            Expr::Multiplication(MultiplicationExpr::new(
                MultiplicationExprOperator::Multiply,
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone()))),
                Box::new(Expr::Multiplication(MultiplicationExpr::new(
                    MultiplicationExprOperator::Multiply,
                    Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone()))),
                    Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone())))
                )))
            ))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_unary_expression() {
    let op_token = Token::new(TokenType::Bang, Option::None);
    let literal_token = Token::new(TokenType::Literal, Option::Some(Literal::Number(1.0)));
    let seed_vec = vec![op_token.clone(), literal_token.clone()];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[2..],
            Expr::Unary(UnaryExpr::new(
                UnaryExprOperator::Bang,
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token)))
            ))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_primary_expression() {
    match_literal_helper(Token::new(
        TokenType::Literal,
        Option::Some(Literal::Number(1.0)),
    ))
}

#[test]
fn validate_parser_should_parse_grouping_expression() {
    let literal_token = Token::new(TokenType::Literal, Option::Some(Literal::Number(1.0)));
    let seed_vec = vec![
        Token::new(TokenType::LeftParen, Option::None),
        literal_token.clone(),
        Token::new(TokenType::RightParen, Option::None),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[3..],
            Expr::Grouping(GroupingExpr::new(Box::new(Expr::Primary(
                PrimaryExpr::new(literal_token)
            ))))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_throw_error_on_invalid_expression() {
    let literal_token = Token::new(TokenType::Literal, Option::Some(Literal::Number(1.0)));
    let seed_vec = vec![
        Token::new(TokenType::LeftParen, Option::None),
        literal_token.clone(),
        Token::new(TokenType::Semicolon, Option::None),
    ];

    assert_eq!(
        Ok(MatchStatus::NoMatch(&seed_vec[..])),
        expression().parse(&seed_vec)
    );
}
