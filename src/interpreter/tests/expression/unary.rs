use crate::ast::expression::{Expr, UnaryExpr};
use crate::interpreter::StatefulInterpreter;
use crate::pass::*;

#[test]
fn unary_expr_should_invert_bool_with_bang_operator() {
    let true_expr = Expr::Unary(UnaryExpr::Bang(Box::new(Expr::Primary(obj_bool!(true)))));
    let false_expr = Expr::Unary(UnaryExpr::Bang(Box::new(Expr::Primary(obj_bool!(false)))));

    assert_eq!(
        Ok(obj_bool!(false)),
        StatefulInterpreter::new().tree_pass(true_expr)
    );
    assert_eq!(
        Ok(obj_bool!(true)),
        StatefulInterpreter::new().tree_pass(false_expr)
    );
}

#[test]
fn unary_expr_should_negate_number_with_minus_operator() {
    let expr = Expr::Unary(UnaryExpr::Minus(Box::new(Expr::Primary(obj_number!(1.0)))));

    assert_eq!(
        Ok(obj_number!(-1.0)),
        StatefulInterpreter::new().tree_pass(expr)
    );
}
