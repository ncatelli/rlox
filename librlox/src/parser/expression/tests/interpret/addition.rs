use crate::interpreter::Interpreter;
use crate::parser::expression::{AdditionExpr, Expr, PrimaryExpr};

#[test]
fn addition_expr_should_evaluate_when_both_operands_are_numbers() {
    let addition_expr = AdditionExpr::Add(
        Box::new(Expr::Primary(PrimaryExpr::Number(5.0))),
        Box::new(Expr::Primary(PrimaryExpr::Number(2.0))),
    );
    let subtraction_expr = AdditionExpr::Subtract(
        Box::new(Expr::Primary(PrimaryExpr::Number(7.0))),
        Box::new(Expr::Primary(PrimaryExpr::Number(2.0))),
    );

    assert_eq!(Ok(PrimaryExpr::Number(7.0)), addition_expr.interpret());
    assert_eq!(Ok(PrimaryExpr::Number(5.0)), subtraction_expr.interpret());
}

#[test]
fn addition_expr_should_concatenate_strings() {
    let expr = AdditionExpr::Add(
        Box::new(Expr::Primary(PrimaryExpr::Str("hello".to_string()))),
        Box::new(Expr::Primary(PrimaryExpr::Str("world".to_string()))),
    );

    assert_eq!(
        Ok(PrimaryExpr::Str(format!("helloworld"))),
        expr.interpret()
    );
}
