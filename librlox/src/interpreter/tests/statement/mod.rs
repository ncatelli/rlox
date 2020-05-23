use crate::ast::expression::{Expr, PrimaryExpr};
use crate::interpreter::InterpreterMut;
use crate::interpreter::StatefulInterpreter;
use crate::parser::statement::Stmt;

#[test]
fn expression_stmt_should_return_ok() {
    assert_eq!(
        Ok(()),
        StatefulInterpreter::new()
            .interpret(vec![Stmt::Expression(Expr::Primary(PrimaryExpr::True))])
    );
}

#[test]
fn print_stmt_should_return_ok() {
    assert_eq!(
        Ok(()),
        StatefulInterpreter::new().interpret(vec![Stmt::Print(Expr::Primary(PrimaryExpr::True))])
    );
}

#[test]
fn declaration_statement_should_set_persistent_global_symbol() {
    let stmt = Stmt::Declaration("test".to_string(), Expr::Primary(PrimaryExpr::True));
    let mut interpreter = StatefulInterpreter::new();
    interpreter.interpret(vec![stmt]).unwrap();
    assert_eq!(
        Some(&Expr::Primary(PrimaryExpr::True)),
        interpreter.globals.get(&"test".to_string())
    );
}
