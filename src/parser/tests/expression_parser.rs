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
fn validate_parser_should_parse_assignment_expression() {
    let op_token = Token::new(TokenType::Equal, 1, Option::None, Option::None);
    let id_token = Token::new(
        TokenType::Identifier,
        1,
        Option::Some("test".to_string()),
        Option::None,
    );
    let literal_token = Token::new(
        TokenType::Number,
        1,
        Option::Some("1.0".to_string()),
        Option::Some(obj_number!(1.0)),
    );
    let seed_vec = vec![id_token.clone(), op_token.clone(), literal_token.clone()];

    assert_eq!(
        Ok(MatchStatus::Match((
            &seed_vec[3..],
            Expr::Assignment(id_token.clone(), Box::new(Expr::Primary(obj_number!(1.0))))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn validate_parser_should_parse_logical_or() {
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
fn validate_parser_should_parse_multiple_logical_or() {
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
fn validate_parser_should_parse_logical_and() {
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
fn validate_parser_should_parse_multiple_logical_and() {
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
                Box::new(Expr::Primary(obj_number!(1.0))),
                Box::new(Expr::Primary(obj_number!(1.0)))
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
                Box::new(Expr::Primary(obj_number!(1.0))),
                Box::new(Expr::Equality(EqualityExpr::Equal(
                    Box::new(Expr::Primary(obj_number!(1.0))),
                    Box::new(Expr::Primary(obj_number!(1.0)))
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
                Box::new(Expr::Primary(obj_number!(1.0))),
                Box::new(Expr::Primary(obj_number!(1.0)))
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
                Box::new(Expr::Primary(obj_number!(1.0))),
                Box::new(Expr::Comparison(ComparisonExpr::GreaterEqual(
                    Box::new(Expr::Primary(obj_number!(1.0))),
                    Box::new(Expr::Primary(obj_number!(1.0)))
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
                Box::new(Expr::Primary(obj_number!(1.0))),
                Box::new(Expr::Primary(obj_number!(1.0)))
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
                Box::new(Expr::Primary(obj_number!(1.0))),
                Box::new(Expr::Addition(AdditionExpr::Add(
                    Box::new(Expr::Primary(obj_number!(1.0))),
                    Box::new(Expr::Primary(obj_number!(1.0)))
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
                Box::new(Expr::Primary(obj_number!(1.0))),
                Box::new(Expr::Primary(obj_number!(1.0)))
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
                Box::new(Expr::Primary(obj_number!(1.0))),
                Box::new(Expr::Multiplication(MultiplicationExpr::Multiply(
                    Box::new(Expr::Primary(obj_number!(1.0))),
                    Box::new(Expr::Primary(obj_number!(1.0)))
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
            Expr::Unary(UnaryExpr::Bang(Box::new(Expr::Primary(obj_number!(1.0)))))
        ))),
        expression().parse(&seed_vec)
    );
}

#[test]
fn parser_should_parse_call_expression_with_single_arg() {
    let input = vec![
        token_from_tt!(TokenType::Identifier, "testfunc"),
        token_from_tt!(TokenType::LeftParen),
        token_from_tt!(TokenType::True, "true", obj_bool!(true)),
        token_from_tt!(TokenType::RightParen),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[4..],
            Expr::Call(
                Box::new(Expr::Variable(token_from_tt!(
                    TokenType::Identifier,
                    "testfunc"
                ))),
                vec![Expr::Primary(obj_bool!(true))]
            )
        ))),
        expression().parse(&input)
    );
}

#[test]
fn parser_should_parse_call_expression_with_multiple_arg() {
    let input = vec![
        token_from_tt!(TokenType::Identifier, "testfunc"),
        token_from_tt!(TokenType::LeftParen),
        token_from_tt!(TokenType::True, "true", obj_bool!(true)),
        token_from_tt!(TokenType::Comma),
        token_from_tt!(TokenType::False, "False", obj_bool!(false)),
        token_from_tt!(TokenType::RightParen),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[6..],
            Expr::Call(
                Box::new(Expr::Variable(token_from_tt!(
                    TokenType::Identifier,
                    "testfunc"
                ))),
                vec![
                    Expr::Primary(obj_bool!(true)),
                    Expr::Primary(obj_bool!(false))
                ]
            )
        ))),
        expression().parse(&input)
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
            Expr::Grouping(Box::new(Expr::Primary(obj_number!(1.0))))
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
