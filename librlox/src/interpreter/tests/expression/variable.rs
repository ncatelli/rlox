use crate::ast::expression::{AdditionExpr, Expr, PrimaryExpr};
use crate::ast::statement::Stmt;
use crate::interpreter::InterpreterMut;
use crate::interpreter::StatefulInterpreter;

#[test]
fn declaration_statement_should_set_persistent_global_symbol() {
    let mut interpreter = StatefulInterpreter::new();
    interpreter
        .globals
        .define("a".to_string(), Expr::Primary(PrimaryExpr::Number(1.0)));
    interpreter
        .globals
        .define("b".to_string(), Expr::Primary(PrimaryExpr::Number(2.0)));

    assert_eq!(
        Ok(()),
        interpreter.interpret(vec![Stmt::Expression(Expr::Addition(AdditionExpr::Add(
            Box::new(Expr::Variable("a".to_string())),
            Box::new(Expr::Variable("b".to_string())),
        )))])
    );
}
