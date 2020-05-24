use crate::ast::expression::{Expr, MultiplicationExpr, PrimaryExpr, UnaryExpr};
use crate::ast::statement::Stmt;
use crate::ast::token::{Token, TokenType};
use std::convert::TryFrom;
use std::option::Option;

#[test]
fn test_expression_formatter_should_pretty_print_an_ast() {
    let expr = Expr::Multiplication(MultiplicationExpr::Multiply(
        Box::new(Expr::Unary(UnaryExpr::Minus(Box::new(Expr::Primary(
            PrimaryExpr::try_from(Token::new(
                TokenType::Literal,
                Option::Some(obj_number!(123.0)),
            ))
            .unwrap(),
        ))))),
        Box::new(Expr::Grouping(Box::new(Expr::Primary(
            PrimaryExpr::try_from(Token::new(
                TokenType::Literal,
                Option::Some(obj_number!(45.7)),
            ))
            .unwrap(),
        )))),
    ));

    assert_eq!(
        "(* (- 123) (Grouping 45.7))".to_string(),
        format!("{}", expr)
    )
}

#[test]
fn test_statement_formatter_should_pretty_print_an_ast() {
    let expr = Stmt::Expression(Expr::Unary(UnaryExpr::Minus(Box::new(Expr::Primary(
        PrimaryExpr::try_from(Token::new(
            TokenType::Literal,
            Option::Some(obj_number!(123.0)),
        ))
        .unwrap(),
    )))));

    assert_eq!("(Expression (- 123))".to_string(), format!("{}", expr))
}
