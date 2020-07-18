use crate::ast::expression::Expr;
use crate::ast::statement::Stmt;
use crate::interpreter::{ExprInterpreterErr, StatefulInterpreter, StmtInterpreterErr};
use crate::pass::*;

#[test]
fn should_return_a_value_when_specified() {
    let block = Stmt::Block(vec![Stmt::Return(Expr::Primary(obj_bool!(true)))]);
    let input = vec![Stmt::Return(Expr::Call(
        Box::new(Expr::Lambda(vec![], Box::new(block))),
        vec![],
    ))];

    assert_eq!(
        Ok(Some(obj_bool!(true))),
        StatefulInterpreter::new().tree_pass(input)
    );
}

#[test]
fn should_throw_error_on_arity_mismatch() {
    let block = Stmt::Block(vec![Stmt::Return(Expr::Primary(obj_bool!(true)))]);
    let input = vec![Stmt::Expression(Expr::Call(
        Box::new(Expr::Lambda(vec![], Box::new(block))),
        vec![Expr::Primary(obj_number!(5.0))],
    ))];

    assert_eq!(
        Err(StmtInterpreterErr::Expression(ExprInterpreterErr::CallErr(
            "Arity".to_string()
        ))),
        StatefulInterpreter::new().tree_pass(input)
    );
}
