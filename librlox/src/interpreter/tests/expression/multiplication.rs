use crate::ast::expression::{Expr, MultiplicationExpr, PrimaryExpr, UnaryExpr};
use crate::interpreter::ExprInterpreterErr;
use crate::interpreter::InterpreterMut;
use crate::interpreter::StatefulInterpreter;

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

macro_rules! expr_interpret {
    ($x:expr) => {
        StatefulInterpreter::new().interpret($x)
    };
}

#[test]
fn multiplication_expr_should_evaluate_when_both_operands_are_numbers() {
    let product_expr = Expr::Multiplication(MultiplicationExpr::Multiply(
        Box::new(primary_number!(5.0)),
        Box::new(primary_number!(2.0)),
    ));
    let division_expr = Expr::Multiplication(MultiplicationExpr::Divide(
        Box::new(primary_number!(10.0)),
        Box::new(primary_number!(2.0)),
    ));

    assert_eq!(Ok(PrimaryExpr::Number(10.0)), expr_interpret!(product_expr));
    assert_eq!(Ok(PrimaryExpr::Number(5.0)), expr_interpret!(division_expr));
}

#[test]
fn multiplication_expr_should_err_when_operands_are_not_numbers() {
    let expr = Expr::Multiplication(MultiplicationExpr::Multiply(
        Box::new(primary_string!("hello")),
        Box::new(primary_string!("world")),
    ));
    assert_eq!(
        Err(ExprInterpreterErr::BinaryExpr(
            "*",
            PrimaryExpr::Str("hello".to_string()),
            PrimaryExpr::Str("world".to_string()),
        )),
        expr_interpret!(expr)
    );
}

#[test]
fn multiplication_expr_should_maintain_operator_precedence() {
    let expr = Expr::Multiplication(MultiplicationExpr::Multiply(
        Box::new(primary_number!(5.0)),
        Box::new(Expr::Unary(UnaryExpr::Minus(Box::new(primary_number!(
            1.0
        ))))),
    ));

    assert_eq!(Ok(PrimaryExpr::Number(-5.0)), expr_interpret!(expr));
}
