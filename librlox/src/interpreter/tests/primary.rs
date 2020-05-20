use crate::interpreter::expression::ExpressionInterpreter;
use crate::interpreter::Interpreter;
use crate::parser::expression::{Expr, PrimaryExpr};

#[test]
fn primary_expr_should_interpret_to_equivalent_primary() {
    let expr = Expr::Primary(PrimaryExpr::Number(5.0));

    assert_eq!(
        Ok(PrimaryExpr::Number(5.0)),
        ExpressionInterpreter::new().interpret(expr)
    );
}
