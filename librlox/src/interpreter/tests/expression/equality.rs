use crate::ast::expression::{EqualityExpr, Expr, MultiplicationExpr, PrimaryExpr};
use crate::interpreter::InterpreterMut;
use crate::interpreter::StatefulInterpreter;

macro_rules! primary_number {
    ($x:literal) => {
        Expr::Primary(PrimaryExpr::Number($x))
    };
}

macro_rules! primary_string {
    ($x:literal) => {
        Expr::Primary(PrimaryExpr::Str($x.to_string()))
    };
}

macro_rules! expr_interpret {
    ($x:expr) => {
        StatefulInterpreter::new().interpret($x)
    };
}

#[test]
fn equality_expr_should_evaluate_when_both_operands_are_numbers() {
    let less_expr = Expr::Equality(EqualityExpr::Equal(
        Box::new(primary_number!(5.0)),
        Box::new(primary_number!(5.0)),
    ));
    let less_equal_expr = Expr::Equality(EqualityExpr::Equal(
        Box::new(primary_number!(10.0)),
        Box::new(primary_number!(5.0)),
    ));
    let greater_expr = Expr::Equality(EqualityExpr::NotEqual(
        Box::new(primary_number!(10.0)),
        Box::new(primary_number!(5.0)),
    ));
    let greater_equal_expr = Expr::Equality(EqualityExpr::NotEqual(
        Box::new(primary_number!(5.0)),
        Box::new(primary_number!(5.0)),
    ));

    assert_eq!(Ok(PrimaryExpr::True), expr_interpret!(less_expr));
    assert_eq!(Ok(PrimaryExpr::False), expr_interpret!(less_equal_expr));
    assert_eq!(Ok(PrimaryExpr::True), expr_interpret!(greater_expr));
    assert_eq!(Ok(PrimaryExpr::False), expr_interpret!(greater_equal_expr));
}

#[test]
fn equality_expr_should_evaluate_when_both_operands_are_strings() {
    let less_expr = Expr::Equality(EqualityExpr::Equal(
        Box::new(primary_string!("hello")),
        Box::new(primary_string!("hello")),
    ));
    let less_equal_expr = Expr::Equality(EqualityExpr::Equal(
        Box::new(primary_string!("hello")),
        Box::new(primary_string!("world")),
    ));
    let greater_expr = Expr::Equality(EqualityExpr::NotEqual(
        Box::new(primary_string!("hello")),
        Box::new(primary_string!("world")),
    ));
    let greater_equal_expr = Expr::Equality(EqualityExpr::NotEqual(
        Box::new(primary_string!("hello")),
        Box::new(primary_string!("hello")),
    ));

    assert_eq!(Ok(PrimaryExpr::True), expr_interpret!(less_expr));
    assert_eq!(Ok(PrimaryExpr::False), expr_interpret!(less_equal_expr));
    assert_eq!(Ok(PrimaryExpr::True), expr_interpret!(greater_expr));
    assert_eq!(Ok(PrimaryExpr::False), expr_interpret!(greater_equal_expr));
}

#[test]
fn equality_expr_should_maintain_operator_precedence() {
    let expr = Expr::Equality(EqualityExpr::NotEqual(
        Box::new(Expr::Multiplication(MultiplicationExpr::Multiply(
            Box::new(primary_number!(1.0)),
            Box::new(primary_number!(-1.0)),
        ))),
        Box::new(primary_number!(1.0)),
    ));

    assert_eq!(Ok(PrimaryExpr::True), expr_interpret!(expr));
}
