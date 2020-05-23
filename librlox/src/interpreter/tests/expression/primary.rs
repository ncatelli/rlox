use crate::ast::expression::{Expr, PrimaryExpr};
use crate::interpreter::InterpreterMut;
use crate::interpreter::StatefulInterpreter;

#[test]
fn primary_expr_should_interpret_to_equivalent_primary() {
    let expr = Expr::Primary(PrimaryExpr::Number(5.0));

    assert_eq!(
        Ok(PrimaryExpr::Number(5.0)),
        StatefulInterpreter::new().interpret(expr)
    );
}
