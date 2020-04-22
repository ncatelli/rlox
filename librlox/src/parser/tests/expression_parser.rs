use crate::parser::expression::{
    AdditionExpr, AdditionExprOperator, ComparisonExpr, ComparisonExprOperator, EqualityExpr,
    EqualityExprOperator, Expr, GroupingExpr, MultiplicationExpr, MultiplicationExprOperator,
    PrimaryExpr, UnaryExpr,
};
use crate::parser::expression_parser::{expression, Parser};
use crate::scanner::tokens::{Literal, Token, TokenType};

fn match_literal_helper(token: Token) {
    let seed_vec = vec![token.clone()];

    assert_eq!(
        Ok((&seed_vec[1..], Expr::Primary(PrimaryExpr::new(token)))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_equality_expression() {
    let op_token = Token::new(TokenType::EqualEqual, Option::None);
    let literal_token = Token::new(TokenType::Number, Option::Some(Literal::Number(1.0)));
    let seed_vec = vec![
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
    ];

    assert_eq!(
        Ok((
            &seed_vec[3..],
            Expr::Equality(EqualityExpr::new(
                EqualityExprOperator::Equal,
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone()))),
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone())))
            ))
        )),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_comparison_expression() {
    let op_token = Token::new(TokenType::GreaterEqual, Option::None);
    let literal_token = Token::new(TokenType::Number, Option::Some(Literal::Number(1.0)));
    let seed_vec = vec![
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
    ];

    assert_eq!(
        Ok((
            &seed_vec[3..],
            Expr::Comparison(ComparisonExpr::new(
                ComparisonExprOperator::GreaterEqual,
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone()))),
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone())))
            ))
        )),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_addition_expression() {
    let op_token = Token::new(TokenType::Plus, Option::None);
    let literal_token = Token::new(TokenType::Number, Option::Some(Literal::Number(1.0)));
    let seed_vec = vec![
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
    ];

    assert_eq!(
        Ok((
            &seed_vec[3..],
            Expr::Addition(AdditionExpr::new(
                AdditionExprOperator::Addition,
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone()))),
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone())))
            ))
        )),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_multiplication_expression() {
    let op_token = Token::new(TokenType::Star, Option::None);
    let literal_token = Token::new(TokenType::Number, Option::Some(Literal::Number(1.0)));
    let seed_vec = vec![
        literal_token.clone(),
        op_token.clone(),
        literal_token.clone(),
    ];

    assert_eq!(
        Ok((
            &seed_vec[3..],
            Expr::Multiplication(MultiplicationExpr::new(
                MultiplicationExprOperator::Multiply,
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone()))),
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token.clone())))
            ))
        )),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_unary_expression() {
    let op_token = Token::new(TokenType::Bang, Option::None);
    let literal_token = Token::new(TokenType::Number, Option::Some(Literal::Number(1.0)));
    let seed_vec = vec![op_token.clone(), literal_token.clone()];

    assert_eq!(
        Ok((
            &seed_vec[2..],
            Expr::Unary(UnaryExpr::new(
                op_token,
                Box::new(Expr::Primary(PrimaryExpr::new(literal_token)))
            ))
        )),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_primary_expression() {
    match_literal_helper(Token::new(
        TokenType::Number,
        Option::Some(Literal::Number(1.0)),
    ))
}

#[test]
fn validate_parser_should_parse_grouping_expression() {
    let literal_token = Token::new(TokenType::Number, Option::Some(Literal::Number(1.0)));
    let seed_vec = vec![
        Token::new(TokenType::LeftParen, Option::None),
        literal_token.clone(),
        Token::new(TokenType::RightParen, Option::None),
    ];

    assert_eq!(
        Ok((
            &seed_vec[3..],
            Expr::Grouping(GroupingExpr::new(Box::new(Expr::Primary(
                PrimaryExpr::new(literal_token)
            ))))
        )),
        expression().parse(&seed_vec)
    );
}
