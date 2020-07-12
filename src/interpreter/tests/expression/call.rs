use crate::ast::expression::Expr;
use crate::ast::statement::Stmt;
use crate::functions;
use crate::interpreter::Interpreter;
use crate::interpreter::{ExprInterpreterErr, StatefulInterpreter, StmtInterpreterErr};

#[test]
fn should_return_ok_on_success() {
    let interpreter = StatefulInterpreter::new();

    let f = functions::Function::new(
        interpreter.env.clone(),
        vec![],
        Stmt::Block(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))]),
    );

    interpreter.env.define(
        &identifier_name!("a"),
        obj_call!(Box::new(functions::Callable::Func(f))),
    );

    assert_eq!(
        Ok(None),
        interpreter.interpret(vec![Stmt::Expression(Expr::Call(
            Box::new(Expr::Variable(identifier_name!("a"))),
            vec![]
        ))])
    );
}

#[test]
fn should_throw_error_on_arity_mismatch() {
    let interpreter = StatefulInterpreter::new();

    let f = functions::Function::new(
        interpreter.env.clone(),
        vec![],
        Stmt::Block(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))]),
    );

    interpreter.env.define(
        &identifier_name!("a"),
        obj_call!(Box::new(functions::Callable::Func(f))),
    );

    assert_eq!(
        Err(StmtInterpreterErr::Expression(ExprInterpreterErr::CallErr(
            "Arity".to_string()
        ))),
        interpreter.interpret(vec![Stmt::Expression(Expr::Call(
            Box::new(Expr::Variable(identifier_name!("a"))),
            vec![Expr::Primary(obj_number!(5.0))]
        ))])
    );
}
