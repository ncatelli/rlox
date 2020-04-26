use crate::parser::expression::{Expr, GroupingExpr, PrimaryExpr, UnaryExpr};
use crate::parser::expression::{MultiplicationExpr, MultiplicationExprOperator};
use crate::scanner::tokens::{Literal, Token, TokenType};
use std::option::Option;

#[test]
fn test_expression_formatter_should_pretty_print_an_ast() {
    let expr = Expr::Multiplication(MultiplicationExpr::new(
        MultiplicationExprOperator::Multiply,
        Box::new(Expr::Unary(UnaryExpr::new(
            Token::new(TokenType::Minus, Option::None),
            Box::new(Expr::Primary(PrimaryExpr::new(Token::new(
                TokenType::Number,
                Option::Some(Literal::Number(123.0)),
            )))),
        ))),
        Box::new(Expr::Grouping(GroupingExpr::new(Box::new(Expr::Primary(
            PrimaryExpr::new(Token::new(
                TokenType::Number,
                Option::Some(Literal::Number(45.7)),
            )),
        ))))),
    ));

    assert_eq!(
        "(* (- (123)) (Grouping (45.7)))".to_string(),
        format!("{}", expr)
    )
}
