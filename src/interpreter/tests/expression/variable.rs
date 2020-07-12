use crate::ast::expression::{AdditionExpr, Expr};
use crate::ast::statement::Stmt;
use crate::interpreter::Interpreter;
use crate::interpreter::StatefulInterpreter;

#[test]
fn declaration_statement_should_set_persistent_global_symbol() {
    let interpreter = StatefulInterpreter::new();
    interpreter
        .env
        .define(&identifier_name!("a"), obj_number!(1.0));
    interpreter
        .env
        .define(&identifier_name!("b"), obj_number!(2.0));

    assert_eq!(
        Ok(None),
        interpreter.interpret(vec![Stmt::Expression(Expr::Addition(AdditionExpr::Add(
            Box::new(Expr::Variable(identifier_name!("a"))),
            Box::new(Expr::Variable(identifier_name!("b"))),
        )))])
    );
}
