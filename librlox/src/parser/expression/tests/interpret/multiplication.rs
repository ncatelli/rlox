use crate::interpreter::Interpreter;
use crate::parser::expression::{Expr, MultiplicationExpr, PrimaryExpr};

#[test]
fn multiplication_expr_should_evaluate_when_both_operands_are_numbers() {
    let product_expr = MultiplicationExpr::Multiply(
        Box::new(Expr::Primary(PrimaryExpr::Number(5.0))),
        Box::new(Expr::Primary(PrimaryExpr::Number(2.0))),
    );
    let division_expr = MultiplicationExpr::Divide(
        Box::new(Expr::Primary(PrimaryExpr::Number(10.0))),
        Box::new(Expr::Primary(PrimaryExpr::Number(2.0))),
    );

    assert_eq!(Ok(PrimaryExpr::Number(10.0)), product_expr.interpret());
    assert_eq!(Ok(PrimaryExpr::Number(5.0)), division_expr.interpret());
}
