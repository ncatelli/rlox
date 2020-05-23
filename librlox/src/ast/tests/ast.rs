use crate::ast::expression::{Expr, MultiplicationExpr, PrimaryExpr, UnaryExpr};
use crate::scanner::tokens::{Token, TokenType, Value};
use std::convert::TryFrom;
use std::option::Option;

#[test]
fn test_expression_formatter_should_pretty_print_an_ast() {
    let expr = Expr::Multiplication(MultiplicationExpr::Multiply(
        Box::new(Expr::Unary(UnaryExpr::Minus(Box::new(Expr::Primary(
            PrimaryExpr::try_from(Token::new(
                TokenType::Literal,
                Option::Some(Value::Number(123.0)),
            ))
            .unwrap(),
        ))))),
        Box::new(Expr::Grouping(Box::new(Expr::Primary(
            PrimaryExpr::try_from(Token::new(
                TokenType::Literal,
                Option::Some(Value::Number(45.7)),
            ))
            .unwrap(),
        )))),
    ));

    assert_eq!(
        "(* (- 123) (Grouping 45.7))".to_string(),
        format!("{}", expr)
    )
}