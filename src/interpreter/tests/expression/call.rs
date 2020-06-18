use crate::ast::expression::Expr;
use crate::ast::statement::Stmt;
use crate::functions;
use crate::interpreter::Interpreter;
use crate::interpreter::StatefulInterpreter;

#[test]
fn declaration_statement_should_set_persistent_global_symbol() {
    let f = functions::Function::new(
        vec![],
        Stmt::Block(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))]),
    );

    let interpreter = StatefulInterpreter::new();
    interpreter
        .env
        .define(&"a", obj_call!(Box::new(functions::Callable::Func(f))));

    assert_eq!(
        Ok(()),
        interpreter.interpret(vec![Stmt::Expression(Expr::Call(
            Box::new(Expr::Variable(tok_identifier!("a"))),
            vec![]
        ))])
    );
}
