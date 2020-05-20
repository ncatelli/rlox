extern crate parcel;
use crate::parser::expression::{Expr, PrimaryExpr, UnaryExpr};
use crate::parser::statement::Stmt;
use crate::scanner::tokens::{Token, TokenType, Value};
use std::convert::TryFrom;
use std::option::Option;

#[test]
fn test_expression_formatter_should_pretty_print_an_ast() {
    let expr = Stmt::Expression(Expr::Unary(UnaryExpr::Minus(Box::new(Expr::Primary(
        PrimaryExpr::try_from(Token::new(
            TokenType::Literal,
            Option::Some(Value::Number(123.0)),
        ))
        .unwrap(),
    )))));

    assert_eq!("(Expression (- 123))".to_string(), format!("{}", expr))
}
