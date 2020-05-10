use crate::interpreter::Interpreter;
use crate::parser::expression::{Expr, PrimaryExpr, UnaryExpr};

#[test]
fn unary_expr_should_invert_bool_with_bang_operator() {
    let true_expr = UnaryExpr::Bang(Box::new(Expr::Primary(PrimaryExpr::True)));
    let false_expr = UnaryExpr::Bang(Box::new(Expr::Primary(PrimaryExpr::False)));

    assert_eq!(Ok(PrimaryExpr::False), true_expr.interpret());
    assert_eq!(Ok(PrimaryExpr::True), false_expr.interpret());
}

#[test]
fn unary_expr_should_negate_number_with_minus_operator() {
    let expr = UnaryExpr::Minus(Box::new(Expr::Primary(PrimaryExpr::Number(1.0))));

    assert_eq!(Ok(PrimaryExpr::Number(-1.0)), expr.interpret());
}
