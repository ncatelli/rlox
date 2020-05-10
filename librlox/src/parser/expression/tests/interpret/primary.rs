use crate::interpreter::Interpreter;
use crate::parser::expression::PrimaryExpr;

#[test]
fn primary_expr_should_interpret_to_equivalent_primary() {
    let expr = PrimaryExpr::Number(5.0);

    assert_eq!(Ok(PrimaryExpr::Number(5.0)), expr.interpret());
}
