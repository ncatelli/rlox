use crate::ast::expression::{Expr, MultiplicationExpr, UnaryExpr};
use crate::ast::statement::Stmt;

#[test]
fn test_expression_formatter_should_pretty_print_an_ast() {
    let expr = Expr::Multiplication(MultiplicationExpr::Multiply(
        Box::new(Expr::Unary(UnaryExpr::Minus(Box::new(Expr::Primary(
            obj_number!(123.0),
        ))))),
        Box::new(Expr::Grouping(Box::new(Expr::Primary(obj_number!(45.7))))),
    ));

    assert_eq!(
        "(* (- 123) (Grouping 45.7))".to_string(),
        format!("{}", expr)
    )
}

#[test]
fn test_statement_formatter_should_pretty_print_an_ast() {
    let expr = Stmt::Expression(Expr::Unary(UnaryExpr::Minus(Box::new(Expr::Primary(
        obj_number!(123.0),
    )))));

    assert_eq!("(Expression (- 123))".to_string(), format!("{}", expr))
}
