use crate::parser::expression::MultiplicationExpr;
use crate::parser::expression::{Expr, GroupingExpr, PrimaryExpr, UnaryExpr};
use crate::scanner::tokens::{Literal, Token, TokenType};
use std::convert::TryFrom;
use std::option::Option;

#[test]
fn test_expression_formatter_should_pretty_print_an_ast() {
    let expr = Expr::Multiplication(MultiplicationExpr::Multiply(
        Box::new(Expr::Unary(UnaryExpr::Minus(Box::new(Expr::Primary(
            PrimaryExpr::try_from(Token::new(
                TokenType::Literal,
                Option::Some(Literal::Number(123.0)),
            ))
            .unwrap(),
        ))))),
        Box::new(Expr::Grouping(GroupingExpr::new(Box::new(Expr::Primary(
            PrimaryExpr::try_from(Token::new(
                TokenType::Literal,
                Option::Some(Literal::Number(45.7)),
            ))
            .unwrap(),
        ))))),
    ));

    assert_eq!(
        "(* (- 123) (Grouping 45.7))".to_string(),
        format!("{}", expr)
    )
}
