use crate::ast::expression::{ComparisonExpr, Expr, MultiplicationExpr};
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
fn comparison_expr_should_evaluate_when_both_operands_are_numbers() {
    let less_expr = Expr::Comparison(ComparisonExpr::Less(
        Box::new(primary_number!(5.0)),
        Box::new(primary_number!(2.0)),
    ));
    let less_equal_expr = Expr::Comparison(ComparisonExpr::LessEqual(
        Box::new(primary_number!(5.0)),
        Box::new(primary_number!(5.0)),
    ));
    let greater_expr = Expr::Comparison(ComparisonExpr::Greater(
        Box::new(primary_number!(5.0)),
        Box::new(primary_number!(5.0)),
    ));
    let greater_equal_expr = Expr::Comparison(ComparisonExpr::GreaterEqual(
        Box::new(primary_number!(5.0)),
        Box::new(primary_number!(5.0)),
    ));

    assert_eq!(Ok(obj_bool!(false)), expr_interpret!(less_expr));
    assert_eq!(Ok(obj_bool!(true)), expr_interpret!(less_equal_expr));
    assert_eq!(Ok(obj_bool!(false)), expr_interpret!(greater_expr));
    assert_eq!(Ok(obj_bool!(true)), expr_interpret!(greater_equal_expr));
}

#[test]
fn comparison_expr_should_maintain_operator_precedence() {
    let expr = Expr::Comparison(ComparisonExpr::Less(
        Box::new(Expr::Multiplication(MultiplicationExpr::Multiply(
            Box::new(primary_number!(5.0)),
            Box::new(primary_number!(-1.0)),
        ))),
        Box::new(primary_number!(1.0)),
    ));

    assert_eq!(Ok(obj_bool!(true)), expr_interpret!(expr));
}
