use crate::interpreter::InterpreterMut;
use crate::interpreter::StatefulInterpreter;
use crate::parser::expression::{Expr, PrimaryExpr, UnaryExpr};

#[test]
fn unary_expr_should_invert_bool_with_bang_operator() {
    let true_expr = Expr::Unary(UnaryExpr::Bang(Box::new(Expr::Primary(PrimaryExpr::True))));
    let false_expr = Expr::Unary(UnaryExpr::Bang(Box::new(Expr::Primary(PrimaryExpr::False))));

    assert_eq!(
        Ok(PrimaryExpr::False),
        StatefulInterpreter::new().interpret(true_expr)
    );
    assert_eq!(
        Ok(PrimaryExpr::True),
        StatefulInterpreter::new().interpret(false_expr)
    );
}

#[test]
fn unary_expr_should_negate_number_with_minus_operator() {
    let expr = Expr::Unary(UnaryExpr::Minus(Box::new(Expr::Primary(
        PrimaryExpr::Number(1.0),
    ))));

    assert_eq!(
        Ok(PrimaryExpr::Number(-1.0)),
        StatefulInterpreter::new().interpret(expr)
    );
}
