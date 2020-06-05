use crate::ast::expression::Expr;
use crate::ast::statement::Stmt;
use crate::interpreter::Interpreter;
use crate::interpreter::StatefulInterpreter;

#[test]
fn expression_stmt_should_return_ok() {
    assert_eq!(
        Ok(()),
        StatefulInterpreter::new()
            .interpret(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))])
    );
}

#[test]
fn print_stmt_should_return_ok() {
    assert_eq!(
        Ok(()),
        StatefulInterpreter::new().interpret(vec![Stmt::Print(Expr::Primary(obj_bool!(true)))])
    );
}

#[test]
fn declaration_statement_should_set_persistent_global_symbol() {
    let stmt = Stmt::Declaration("test".to_string(), Expr::Primary(obj_bool!(true)));
    let interpreter = StatefulInterpreter::new();
    interpreter.interpret(vec![stmt]).unwrap();
    assert_eq!(
        Some(obj_bool!(true)),
        interpreter.env.get(&"test".to_string())
    );
}

#[test]
fn block_statement_should_set_persistent_global_symbol() {
    let stmts = Stmt::Block(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))]);
    assert_eq!(Ok(()), StatefulInterpreter::new().interpret(stmts));
}

#[test]
fn if_statement_should_eval_to_primary_clause_if_condition_is_true() {
    let interpreter = StatefulInterpreter::new();
    let stmt = Stmt::If(
        Expr::Primary(obj_bool!(true)),
        Box::new(Stmt::Declaration(
            "test".to_string(),
            Expr::Primary(obj_bool!(true)),
        )),
        Option::Some(Box::new(Stmt::Declaration(
            "test".to_string(),
            Expr::Primary(obj_bool!(false)),
        ))),
    );

    interpreter.interpret(vec![stmt]).unwrap();
    assert_eq!(
        Some(obj_bool!(true)),
        interpreter.env.get(&"test".to_string())
    );
}

#[test]
fn if_statement_should_eval_to_else_clause_if_condition_is_false() {
    let interpreter = StatefulInterpreter::new();
    let stmt = Stmt::If(
        Expr::Primary(obj_bool!(false)),
        Box::new(Stmt::Declaration(
            "test".to_string(),
            Expr::Primary(obj_bool!(true)),
        )),
        Option::Some(Box::new(Stmt::Declaration(
            "test".to_string(),
            Expr::Primary(obj_bool!(false)),
        ))),
    );

    interpreter.interpret(vec![stmt]).unwrap();
    assert_eq!(
        Some(obj_bool!(false)),
        interpreter.env.get(&"test".to_string())
    );
}
