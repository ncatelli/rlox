use crate::interpreter::expression::ExpressionInterpreter;
use crate::interpreter::Interpreter;
use crate::parser::expression::{AdditionExpr, Expr, MultiplicationExpr, PrimaryExpr};

macro_rules! primary_number {
    ($x:literal) => {
        Expr::Primary(PrimaryExpr::Number($x))
    };
}

macro_rules! multiplication_expr {
    ($x:item, $y:item) => {
        Expr::Multiplication(MultiplicationExpr::Multiply(Box::new($x), Box::new(y)))
    };
}

macro_rules! addition_expr {
    ($x:item, $y:item) => {
        Expr::Addition(AdditionExpr::Add(Box::new(x), Box::new(y)))
    };
}

#[test]
fn grouping_expr_should_interpret_to_equivalent_primary() {
    let expr = Expr::Grouping(Box::new(primary_number!(5.0)));

    assert_eq!(
        Ok(PrimaryExpr::Number(5.0)),
        ExpressionInterpreter::new().interpret(expr)
    );
}

/*
#[test]
fn grouping_expr_should_maintain_operator_precedence() {
    let expr = multiplication_expr!(
        primary_number! {5.0},
        Expr::Grouping(Box::new(addition_expr!(-1.0, 2.0))),
    );

    assert_eq!(
        Ok(PrimaryExpr::Number(5.0)),
        ExpressionInterpreter::new().interpret(expr)
    );
}
*/
