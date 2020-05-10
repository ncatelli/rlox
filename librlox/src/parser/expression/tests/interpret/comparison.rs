use crate::interpreter::Interpreter;
use crate::parser::expression::{ComparisonExpr, Expr, MultiplicationExpr, PrimaryExpr};

macro_rules! primary_number {
    ($x:literal) => {
        Expr::Primary(PrimaryExpr::Number($x))
    };
}

#[test]
fn comparison_expr_should_evaluate_when_both_operands_are_numbers() {
    let less_expr = ComparisonExpr::Less(
        Box::new(primary_number!(5.0)),
        Box::new(primary_number!(2.0)),
    );
    let less_equal_expr = ComparisonExpr::LessEqual(
        Box::new(primary_number!(5.0)),
        Box::new(primary_number!(5.0)),
    );
    let greater_expr = ComparisonExpr::Greater(
        Box::new(primary_number!(5.0)),
        Box::new(primary_number!(5.0)),
    );
    let greater_equal_expr = ComparisonExpr::GreaterEqual(
        Box::new(primary_number!(5.0)),
        Box::new(primary_number!(5.0)),
    );

    assert_eq!(Ok(PrimaryExpr::False), less_expr.interpret());
    assert_eq!(Ok(PrimaryExpr::True), less_equal_expr.interpret());
    assert_eq!(Ok(PrimaryExpr::False), greater_expr.interpret());
    assert_eq!(Ok(PrimaryExpr::True), greater_equal_expr.interpret());
}

#[test]
fn comparison_expr_should_maintain_operator_precedence() {
    let expr = ComparisonExpr::Less(
        Box::new(Expr::Multiplication(MultiplicationExpr::Multiply(
            Box::new(primary_number!(5.0)),
            Box::new(primary_number!(-1.0)),
        ))),
        Box::new(primary_number!(1.0)),
    );

    assert_eq!(Ok(PrimaryExpr::True), expr.interpret());
}
