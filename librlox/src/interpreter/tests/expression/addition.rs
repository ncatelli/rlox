use crate::ast::expression::{AdditionExpr, Expr, MultiplicationExpr, PrimaryExpr};
use crate::interpreter::InterpreterMut;
use crate::interpreter::StatefulInterpreter;

macro_rules! primary_number {
    ($x:literal) => {
        Expr::Primary(PrimaryExpr::Number($x))
    };
}

macro_rules! expr_interpret {
    ($x:expr) => {
        StatefulInterpreter::new().interpret($x)
    };
}

#[test]
fn addition_expr_should_evaluate_when_both_operands_are_numbers() {
    let addition_expr = Expr::Addition(AdditionExpr::Add(
        Box::new(primary_number!(5.0)),
        Box::new(primary_number!(2.0)),
    ));
    let subtraction_expr = Expr::Addition(AdditionExpr::Subtract(
        Box::new(primary_number!(7.0)),
        Box::new(primary_number!(2.0)),
    ));

    assert_eq!(Ok(PrimaryExpr::Number(7.0)), expr_interpret!(addition_expr));
    assert_eq!(
        Ok(PrimaryExpr::Number(5.0)),
        expr_interpret!(subtraction_expr)
    );
}

#[test]
fn addition_expr_should_maintain_operator_precedence() {
    let expr = Expr::Addition(AdditionExpr::Add(
        Box::new(Expr::Multiplication(MultiplicationExpr::Multiply(
            Box::new(primary_number!(5.0)),
            Box::new(primary_number!(-1.0)),
        ))),
        Box::new(primary_number!(1.0)),
    ));

    assert_eq!(Ok(PrimaryExpr::Number(-4.0)), expr_interpret!(expr));
}

#[test]
fn addition_expr_should_concatenate_strings() {
    let expr = Expr::Addition(AdditionExpr::Add(
        Box::new(Expr::Primary(PrimaryExpr::Str("hello".to_string()))),
        Box::new(Expr::Primary(PrimaryExpr::Str("world".to_string()))),
    ));

    assert_eq!(
        Ok(PrimaryExpr::Str(format!("helloworld"))),
        expr_interpret!(expr)
    );
}
