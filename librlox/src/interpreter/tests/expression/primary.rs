use crate::ast::expression::Expr;
use crate::interpreter::InterpreterMut;
use crate::interpreter::StatefulInterpreter;

#[test]
fn primary_expr_should_interpret_to_equivalent_primary() {
    let expr = Expr::Primary(obj_number!(5.0));

    assert_eq!(
        Ok(obj_number!(5.0)),
        StatefulInterpreter::new().interpret(expr)
    );
}
