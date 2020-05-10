use crate::interpreter::{Interpreter, InterpreterErr};
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

#[test]
fn multiplication_expr_should_err_when_operands_are_not_numbers() {
    let expr = MultiplicationExpr::Multiply(
        Box::new(Expr::Primary(PrimaryExpr::Str("hello".to_string()))),
        Box::new(Expr::Primary(PrimaryExpr::Str("world".to_string()))),
    );
    assert_eq!(
        Err(InterpreterErr::TypeErr(
            "Invalid operand for operator: hello * world".to_string()
        )),
        expr.interpret()
    );
}
