use crate::ast::expression::{AdditionExpr, Expr, MultiplicationExpr};
use crate::interpreter::InterpreterMut;
use crate::interpreter::StatefulInterpreter;

macro_rules! primary_number {
    ($x:literal) => {
        Expr::Primary($crate::object::Object::Literal(
            $crate::object::Literal::Number($x),
        ))
    };
}

macro_rules! multiplication_expr {
    ($x:expr, $y:expr) => {
        Expr::Multiplication(MultiplicationExpr::Multiply(Box::new($x), Box::new($y)))
    };
}

macro_rules! addition_expr {
    ($x:expr, $y:expr) => {
        Expr::Addition(AdditionExpr::Add(Box::new($x), Box::new($y)))
    };
}

#[test]
fn grouping_expr_should_interpret_to_equivalent_primary() {
    let expr = Expr::Grouping(Box::new(primary_number!(5.0)));

    assert_eq!(
        Ok(obj_number!(5.0)),
        StatefulInterpreter::new().interpret(expr)
    );
}

#[test]
fn grouping_expr_should_maintain_operator_precedence() {
    let expr = multiplication_expr!(
        primary_number!(5.0),
        Expr::Grouping(Box::new(addition_expr!(
            primary_number!(-1.0),
            primary_number!(2.0)
        )))
    );

    assert_eq!(
        Ok(obj_number!(5.0)),
        StatefulInterpreter::new().interpret(expr)
    );
}
