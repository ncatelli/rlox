use crate::interpreter::{Interpreter, InterpreterErr};
use crate::parser::expression::{Expr, MultiplicationExpr, PrimaryExpr, UnaryExpr};

macro_rules! primary_number {
    ($x:literal) => {
        Expr::Primary(PrimaryExpr::Number($x))
    };
}

macro_rules! primary_string {
    ($s:literal) => {
        Expr::Primary(PrimaryExpr::Str($s.to_string()))
    };
}

#[test]
fn multiplication_expr_should_evaluate_when_both_operands_are_numbers() {
    let product_expr = MultiplicationExpr::Multiply(
        Box::new(primary_number!(5.0)),
        Box::new(primary_number!(2.0)),
    );
    let division_expr = MultiplicationExpr::Divide(
        Box::new(primary_number!(10.0)),
        Box::new(primary_number!(2.0)),
    );

    assert_eq!(Ok(PrimaryExpr::Number(10.0)), product_expr.interpret());
    assert_eq!(Ok(PrimaryExpr::Number(5.0)), division_expr.interpret());
}

#[test]
fn multiplication_expr_should_err_when_operands_are_not_numbers() {
    let expr = MultiplicationExpr::Multiply(
        Box::new(primary_string!("hello")),
        Box::new(primary_string!("world")),
    );
    assert_eq!(
        Err(InterpreterErr::TypeErr(
            "Invalid operand for operator: hello * world".to_string()
        )),
        expr.interpret()
    );
}

#[test]
fn multiplication_expr_should_maintain_operator_precedence() {
    let expr = MultiplicationExpr::Multiply(
        Box::new(primary_number!(5.0)),
        Box::new(Expr::Unary(UnaryExpr::Minus(Box::new(primary_number!(
            1.0
        ))))),
    );

    assert_eq!(Ok(PrimaryExpr::Number(-5.0)), expr.interpret());
}
