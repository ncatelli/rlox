use crate::parser::expression::{BinaryExpr, Expr, LiteralExpr, UnaryExpr};
use crate::parser::expression_parser::*;
use crate::scanner::tokens::{Literal, Token, TokenType};

fn match_literal_helper(token: Token) {
    let seed_vec = vec![token.clone()];

    assert_eq!(
        Ok((&seed_vec[1..], Expr::Literal(LiteralExpr::new(token)))),
        expression().parse(&seed_vec)
    );
}

fn match_binary_helper(op: Token, literal: Token) {
    let literal_token = literal.clone();
    let seed_vec = vec![literal_token.clone(), op.clone(), literal_token.clone()];

    assert_eq!(
        Ok((
            &seed_vec[3..],
            Expr::Binary(BinaryExpr::new(
                op.clone(),
                Box::new(Expr::Literal(LiteralExpr::new(literal_token.clone()))),
                Box::new(Expr::Literal(LiteralExpr::new(literal_token.clone())))
            ))
        )),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_equality_expression() {
    let op_token = Token::new(TokenType::EqualEqual, Option::None);
    let literal_token = Token::new(TokenType::Number, Option::Some(Literal::Number(1.0)));
    match_binary_helper(op_token, literal_token);
}

#[test]
fn validate_parser_should_parse_comparison_expression() {
    let op_token = Token::new(TokenType::GreaterEqual, Option::None);
    let literal_token = Token::new(TokenType::Number, Option::Some(Literal::Number(1.0)));
    match_binary_helper(op_token, literal_token);
}

#[test]
fn validate_parser_should_parse_addition_expression() {
    let op_token = Token::new(TokenType::Plus, Option::None);
    let literal_token = Token::new(TokenType::Number, Option::Some(Literal::Number(1.0)));
    match_binary_helper(op_token, literal_token);
}

#[test]
fn validate_parser_should_parse_multiplication_expression() {
    let op_token = Token::new(TokenType::Star, Option::None);
    let literal_token = Token::new(TokenType::Number, Option::Some(Literal::Number(1.0)));
    match_binary_helper(op_token, literal_token);
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
                Box::new(Expr::Literal(LiteralExpr::new(literal_token)))
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
