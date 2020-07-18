use crate::ast::expression::{AdditionExpr, Expr, MultiplicationExpr};
use crate::interpreter::StatefulInterpreter;
use crate::pass::*;

macro_rules! primary_number {
    ($x:literal) => {
        Expr::Primary($crate::object::Object::Literal(
            $crate::object::Literal::Number($x),
        ))
    };
}

macro_rules! expr_interpret {
    ($x:expr) => {
        StatefulInterpreter::new().tree_pass($x)
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

    assert_eq!(Ok(obj_number!(7.0)), expr_interpret!(addition_expr));
    assert_eq!(Ok(obj_number!(5.0)), expr_interpret!(subtraction_expr));
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

    assert_eq!(Ok(obj_number!(-4.0)), expr_interpret!(expr));
}

#[test]
fn addition_expr_should_concatenate_strings() {
    let expr = Expr::Addition(AdditionExpr::Add(
        Box::new(Expr::Primary(obj_str!("hello".to_string()))),
        Box::new(Expr::Primary(obj_str!("world".to_string()))),
    ));

    assert_eq!(Ok(obj_str!(format!("helloworld"))), expr_interpret!(expr));
}
