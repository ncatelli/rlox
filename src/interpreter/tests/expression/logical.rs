use crate::ast::expression::{Expr, LogicalExpr};
use crate::interpreter::StatefulInterpreter;
use crate::pass::*;

macro_rules! logical_truth_table {
    ($l:literal, "or", $r: literal) => {
        Expr::Logical(LogicalExpr::Or(
            Box::new(Expr::Primary($crate::object::Object::Literal(
                $crate::object::Literal::Bool($l),
            ))),
            Box::new(Expr::Primary($crate::object::Object::Literal(
                $crate::object::Literal::Bool($r),
            ))),
        ))
    };
    ($l:literal, "and", $r: literal) => {
        Expr::Logical(LogicalExpr::And(
            Box::new(Expr::Primary($crate::object::Object::Literal(
                $crate::object::Literal::Bool($l),
            ))),
            Box::new(Expr::Primary($crate::object::Object::Literal(
                $crate::object::Literal::Bool($r),
            ))),
        ))
    };
}

macro_rules! expr_interpret {
    ($x:expr) => {
        StatefulInterpreter::new().tree_pass($x)
    };
}

#[test]
fn or_should_follow_defined_truth_table() {
    let true_true = logical_truth_table!(true, "or", true);
    let false_true = logical_truth_table!(false, "or", true);
    let true_false = logical_truth_table!(true, "or", false);
    let false_false = logical_truth_table!(false, "or", false);

    assert_eq!(Ok(obj_bool!(true)), expr_interpret!(true_true));
    assert_eq!(Ok(obj_bool!(true)), expr_interpret!(false_true));
    assert_eq!(Ok(obj_bool!(true)), expr_interpret!(true_false));
    assert_eq!(Ok(obj_bool!(false)), expr_interpret!(false_false));
}

#[test]
fn and_should_follow_defined_truth_table() {
    let true_true = logical_truth_table!(true, "and", true);
    let false_true = logical_truth_table!(false, "and", true);
    let true_false = logical_truth_table!(true, "and", false);
    let false_false = logical_truth_table!(false, "and", false);

    assert_eq!(Ok(obj_bool!(true)), expr_interpret!(true_true));
    assert_eq!(Ok(obj_bool!(false)), expr_interpret!(false_true));
    assert_eq!(Ok(obj_bool!(false)), expr_interpret!(true_false));
    assert_eq!(Ok(obj_bool!(false)), expr_interpret!(false_false));
}
