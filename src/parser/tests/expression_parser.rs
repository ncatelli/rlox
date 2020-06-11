extern crate parcel;
use crate::ast::expression::{
    AdditionExpr, ComparisonExpr, EqualityExpr, Expr, LogicalExpr, MultiplicationExpr, UnaryExpr,
};
use crate::ast::token::{Token, TokenType};
use crate::parser::expression_parser::expression;
use parcel::*;

fn match_literal_helper(token: Token) {
    let seed_vec = vec![token.clone()];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[1..],
            Expr::Primary(token.object.unwrap())
        ))),
        expression().parse(&seed_vec)
    );
}

macro_rules! token_from_tt {
    ($tt:expr) => {
        $crate::ast::token::Token::new($tt, 1, Option::None, Option::None)
    };
    ($tt:expr, $lex:expr) => {
        $crate::ast::token::Token::new($tt, 1, Option::Some($lex.to_string()), Option::None)
    };
    ($tt:expr, $lex:expr, $val:expr) => {
        $crate::ast::token::Token::new($tt, 1, Option::Some($lex.to_string()), Option::Some($val))
    };
}

#[test]
fn parser_should_parse_assignment_expression() {
    let input = vec![
        token_from_tt!(TokenType::Identifier, "test"),
        token_from_tt!(TokenType::Equal),
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[3..],
            Expr::Assignment(
                Token::new(
                    TokenType::Identifier,
                    1,
                    Option::Some("test".to_string()),
                    Option::None
                ),
                Box::new(Expr::Primary(obj_number!(1.0)))
            )
        ))),
        expression().parse(&input)
    );
}

#[test]
fn parser_should_parse_logical_or() {
    let input = vec![
        token_from_tt!(TokenType::False, "false", obj_bool!(false)),
        token_from_tt!(TokenType::Or),
        token_from_tt!(TokenType::True, "true", obj_bool!(true)),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[3..],
            Expr::Logical(LogicalExpr::Or(
                Box::new(Expr::Primary(obj_bool!(false))),
                Box::new(Expr::Primary(obj_bool!(true)))
            ))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn parser_should_parse_multiple_logical_or() {
    let input = vec![
        token_from_tt!(TokenType::False, "false", obj_bool!(false)),
        token_from_tt!(TokenType::Or),
        token_from_tt!(TokenType::True, "true", obj_bool!(true)),
        token_from_tt!(TokenType::Or),
        token_from_tt!(TokenType::True, "true", obj_bool!(true)),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[5..],
            Expr::Logical(LogicalExpr::Or(
                Box::new(Expr::Primary(obj_bool!(false))),
                Box::new(Expr::Logical(LogicalExpr::Or(
                    Box::new(Expr::Primary(obj_bool!(true))),
                    Box::new(Expr::Primary(obj_bool!(true)))
                )))
            ))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn parser_should_parse_logical_and() {
    let input = vec![
        token_from_tt!(TokenType::False, "false", obj_bool!(false)),
        token_from_tt!(TokenType::And),
        token_from_tt!(TokenType::True, "true", obj_bool!(true)),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[3..],
            Expr::Logical(LogicalExpr::And(
                Box::new(Expr::Primary(obj_bool!(false))),
                Box::new(Expr::Primary(obj_bool!(true)))
            ))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn parser_should_parse_multiple_logical_and() {
    let input = vec![
        token_from_tt!(TokenType::False, "false", obj_bool!(false)),
        token_from_tt!(TokenType::And),
        token_from_tt!(TokenType::True, "true", obj_bool!(true)),
        token_from_tt!(TokenType::And),
        token_from_tt!(TokenType::True, "true", obj_bool!(true)),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[5..],
            Expr::Logical(LogicalExpr::And(
                Box::new(Expr::Primary(obj_bool!(false))),
                Box::new(Expr::Logical(LogicalExpr::And(
                    Box::new(Expr::Primary(obj_bool!(true))),
                    Box::new(Expr::Primary(obj_bool!(true)))
                )))
            ))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn parser_should_parse_equality_expression() {
    let input = vec![
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
        token_from_tt!(TokenType::EqualEqual),
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[3..],
            Expr::Equality(EqualityExpr::Equal(
                Box::new(Expr::Primary(obj_number!(1.0))),
                Box::new(Expr::Primary(obj_number!(1.0)))
            ))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn parser_should_parse_many_equality_expression() {
    let input = vec![
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
        token_from_tt!(TokenType::EqualEqual),
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
        token_from_tt!(TokenType::EqualEqual),
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[5..],
            Expr::Equality(EqualityExpr::Equal(
                Box::new(Expr::Primary(obj_number!(1.0))),
                Box::new(Expr::Equality(EqualityExpr::Equal(
                    Box::new(Expr::Primary(obj_number!(1.0))),
                    Box::new(Expr::Primary(obj_number!(1.0)))
                )))
            ))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn parser_should_parse_comparison_expression() {
    let input = vec![
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
        token_from_tt!(TokenType::GreaterEqual),
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[3..],
            Expr::Comparison(ComparisonExpr::GreaterEqual(
                Box::new(Expr::Primary(obj_number!(1.0))),
                Box::new(Expr::Primary(obj_number!(1.0)))
            ))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn parser_should_parse_many_comparison_expression() {
    let input = vec![
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
        token_from_tt!(TokenType::GreaterEqual),
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
        token_from_tt!(TokenType::GreaterEqual),
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[5..],
            Expr::Comparison(ComparisonExpr::GreaterEqual(
                Box::new(Expr::Primary(obj_number!(1.0))),
                Box::new(Expr::Comparison(ComparisonExpr::GreaterEqual(
                    Box::new(Expr::Primary(obj_number!(1.0))),
                    Box::new(Expr::Primary(obj_number!(1.0)))
                )))
            ))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn parser_should_parse_addition_expression() {
    let input = vec![
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
        token_from_tt!(TokenType::Plus),
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[3..],
            Expr::Addition(AdditionExpr::Add(
                Box::new(Expr::Primary(obj_number!(1.0))),
                Box::new(Expr::Primary(obj_number!(1.0)))
            ))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn parser_should_parse_many_addition_expression() {
    let input = vec![
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
        token_from_tt!(TokenType::Plus),
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
        token_from_tt!(TokenType::Plus),
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[5..],
            Expr::Addition(AdditionExpr::Add(
                Box::new(Expr::Primary(obj_number!(1.0))),
                Box::new(Expr::Addition(AdditionExpr::Add(
                    Box::new(Expr::Primary(obj_number!(1.0))),
                    Box::new(Expr::Primary(obj_number!(1.0)))
                )))
            ))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn parser_should_parse_multiplication_expression() {
    let input = vec![
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
        token_from_tt!(TokenType::Star),
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[3..],
            Expr::Multiplication(MultiplicationExpr::Multiply(
                Box::new(Expr::Primary(obj_number!(1.0))),
                Box::new(Expr::Primary(obj_number!(1.0)))
            ))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn parser_should_parse_many_multiplication_expression() {
    let input = vec![
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
        token_from_tt!(TokenType::Star),
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
        token_from_tt!(TokenType::Star),
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[5..],
            Expr::Multiplication(MultiplicationExpr::Multiply(
                Box::new(Expr::Primary(obj_number!(1.0))),
                Box::new(Expr::Multiplication(MultiplicationExpr::Multiply(
                    Box::new(Expr::Primary(obj_number!(1.0))),
                    Box::new(Expr::Primary(obj_number!(1.0)))
                )))
            ))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn parser_should_parse_unary_expression() {
    let input = vec![
        token_from_tt!(TokenType::Bang),
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[2..],
            Expr::Unary(UnaryExpr::Bang(Box::new(Expr::Primary(obj_number!(1.0)))))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn parser_should_parse_primary_expression() {
    match_literal_helper(Token::new(
        TokenType::Number,
        1,
        Option::Some("1.0".to_string()),
        Option::Some(obj_number!(1.0)),
    ))
}

#[test]
fn parser_should_parse_grouping_expression() {
    let input = vec![
        token_from_tt!(TokenType::LeftParen),
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
        token_from_tt!(TokenType::RightParen),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[3..],
            Expr::Grouping(Box::new(Expr::Primary(obj_number!(1.0))))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn parser_should_throw_error_on_invalid_expression() {
    let input = vec![
        token_from_tt!(TokenType::LeftParen),
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
        token_from_tt!(TokenType::Semicolon),
    ];

    assert_eq!(
        Ok(MatchStatus::NoMatch(&input[..])),
        expression().parse(&input)
    );
}
