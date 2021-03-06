extern crate parcel;
use crate::ast::expression::{
    AdditionExpr, ComparisonExpr, EqualityExpr, Expr, LogicalExpr, MultiplicationExpr, UnaryExpr,
};
use crate::ast::statement::Stmt;
use crate::ast::token::TokenType;
use crate::parser::expression_parser::expression;
use parcel::*;

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
fn should_parse_assignment_expression() {
    let input = vec![
        token_from_tt!(TokenType::Identifier, "test"),
        token_from_tt!(TokenType::Equal),
        token_from_tt!(TokenType::Number, "1.0", obj_number!(1.0)),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[3..],
            Expr::Assignment(
                identifier_name!("test"),
                Box::new(Expr::Primary(obj_number!(1.0)))
            )
        ))),
        expression().parse(&input)
    );
}

#[test]
fn should_parse_logical_or() {
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
fn should_parse_multiple_logical_or() {
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
                Box::new(Expr::Logical(LogicalExpr::Or(
                    Box::new(Expr::Primary(obj_bool!(false))),
                    Box::new(Expr::Primary(obj_bool!(true)))
                ))),
                Box::new(Expr::Primary(obj_bool!(true))),
            ))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn should_parse_logical_and() {
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
fn should_parse_multiple_logical_and() {
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
                Box::new(Expr::Logical(LogicalExpr::And(
                    Box::new(Expr::Primary(obj_bool!(false))),
                    Box::new(Expr::Primary(obj_bool!(true)))
                ))),
                Box::new(Expr::Primary(obj_bool!(true))),
            ))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn should_parse_equality_expression() {
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
fn should_parse_many_equality_expression() {
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
                Box::new(Expr::Equality(EqualityExpr::Equal(
                    Box::new(Expr::Primary(obj_number!(1.0))),
                    Box::new(Expr::Primary(obj_number!(1.0)))
                ))),
                Box::new(Expr::Primary(obj_number!(1.0))),
            ))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn should_parse_comparison_expression() {
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
fn should_parse_many_comparison_expression() {
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
                Box::new(Expr::Comparison(ComparisonExpr::GreaterEqual(
                    Box::new(Expr::Primary(obj_number!(1.0))),
                    Box::new(Expr::Primary(obj_number!(1.0)))
                ))),
                Box::new(Expr::Primary(obj_number!(1.0))),
            ))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn should_parse_addition_expression() {
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
fn should_parse_many_addition_expression() {
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
                Box::new(Expr::Addition(AdditionExpr::Add(
                    Box::new(Expr::Primary(obj_number!(1.0))),
                    Box::new(Expr::Primary(obj_number!(1.0)))
                ))),
                Box::new(Expr::Primary(obj_number!(1.0))),
            ))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn should_parse_multiplication_expression() {
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
fn should_parse_many_multiplication_expression() {
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
                Box::new(Expr::Multiplication(MultiplicationExpr::Multiply(
                    Box::new(Expr::Primary(obj_number!(1.0))),
                    Box::new(Expr::Primary(obj_number!(1.0)))
                ))),
                Box::new(Expr::Primary(obj_number!(1.0))),
            ))
        ))),
        expression().parse(&input)
    );
}

#[test]
fn should_parse_unary_expression() {
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
fn should_parse_call_expression_with_single_arg() {
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
                Box::new(Expr::Variable(identifier_name!("testfunc"),)),
                vec![Expr::Primary(obj_bool!(true))]
            )
        ))),
        expression().parse(&input)
    );
}

#[test]
fn should_parse_call_expression_with_multiple_arg() {
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
                Box::new(Expr::Variable(identifier_name!("testfunc"),)),
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
fn should_parse_get_expression() {
    let input = vec![
        token_from_tt!(TokenType::Identifier, "test_class"),
        token_from_tt!(TokenType::Dot),
        token_from_tt!(TokenType::Identifier, "test_param"),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[3..],
            Expr::Get(
                Box::new(Expr::Variable(identifier_name!("test_class"))),
                Box::new(Expr::Variable(identifier_name!("test_param"))),
            )
        ))),
        expression().parse(&input)
    );
}

#[test]
fn should_parse_nested_get_expression() {
    let input = vec![
        token_from_tt!(TokenType::Identifier, "test_class"),
        token_from_tt!(TokenType::Dot),
        token_from_tt!(TokenType::Identifier, "test_param"),
        token_from_tt!(TokenType::Dot),
        token_from_tt!(TokenType::Identifier, "test_nested_param"),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[5..],
            Expr::Get(
                Box::new(Expr::Variable(identifier_name!("test_class"),)),
                Box::new(Expr::Get(
                    Box::new(Expr::Variable(identifier_name!("test_param"))),
                    Box::new(Expr::Variable(identifier_name!("test_nested_param"))),
                ))
            )
        ))),
        expression().parse(&input)
    );
}

#[test]
fn should_parse_lambda_expression_with_no_params() {
    let input = vec![
        token_from_tt!(TokenType::Fun),
        token_from_tt!(TokenType::LeftParen),
        token_from_tt!(TokenType::RightParen),
        token_from_tt!(TokenType::LeftBrace),
        token_from_tt!(TokenType::Number, "5.0", obj_number!(5.0)),
        token_from_tt!(TokenType::Semicolon),
        token_from_tt!(TokenType::RightBrace),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[7..],
            Expr::Lambda(
                vec![],
                Box::new(Stmt::Block(vec![Stmt::Expression(Expr::Primary(
                    obj_number!(5.0)
                ))]))
            )
        ))),
        expression().parse(&input)
    );
}

#[test]
fn should_parse_lambda_expression_with_parameters() {
    let input = vec![
        token_from_tt!(TokenType::Fun),
        token_from_tt!(TokenType::LeftParen),
        token_from_tt!(TokenType::Identifier, "arg_one"),
        token_from_tt!(TokenType::RightParen),
        token_from_tt!(TokenType::LeftBrace),
        token_from_tt!(TokenType::Number, "5.0", obj_number!(5.0)),
        token_from_tt!(TokenType::Semicolon),
        token_from_tt!(TokenType::RightBrace),
    ];

    assert_eq!(
        Ok(MatchStatus::Match((
            &input[8..],
            Expr::Lambda(
                vec![identifier_name!("arg_one"),],
                Box::new(Stmt::Block(vec![Stmt::Expression(Expr::Primary(
                    obj_number!(5.0)
                ))]))
            )
        ))),
        expression().parse(&input)
    );
}

#[test]
fn should_parse_grouping_expression() {
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
fn should_throw_error_on_invalid_expression() {
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
