use crate::ast::expression::Expr;
use crate::ast::statement::Stmt;
use crate::functions;
use crate::interpreter::Interpreter;
use crate::interpreter::{ExprInterpreterErr, StatefulInterpreter, StmtInterpreterErr};

#[test]
fn should_return_ok_on_success() {
    let f = functions::Function::new(
        vec![],
        Stmt::Block(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))]),
    );

    let interpreter = StatefulInterpreter::new();
    interpreter
        .env
        .define(&"a", obj_call!(Box::new(functions::Callable::Func(f))));

    assert_eq!(
        Ok(obj_nil!()),
        interpreter.interpret(vec![Stmt::Expression(Expr::Call(
            Box::new(Expr::Variable(tok_identifier!("a"))),
            vec![]
        ))])
    );
}

#[test]
fn should_throw_error_on_arity_mismatch() {
    let f = functions::Function::new(
        vec![],
        Stmt::Block(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))]),
    );

    let interpreter = StatefulInterpreter::new();
    interpreter
        .env
        .define(&"a", obj_call!(Box::new(functions::Callable::Func(f))));

    assert_eq!(
        Err(StmtInterpreterErr::Expression(ExprInterpreterErr::CallErr(
            "Arity".to_string()
        ))),
        interpreter.interpret(vec![Stmt::Expression(Expr::Call(
            Box::new(Expr::Variable(tok_identifier!("a"))),
            vec![Expr::Primary(obj_number!(5.0))]
        ))])
    );
}
