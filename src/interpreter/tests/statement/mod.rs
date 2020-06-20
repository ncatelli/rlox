use crate::ast::expression::Expr;
use crate::ast::statement::Stmt;
use crate::ast::token::{Token, TokenType};
use crate::functions;
use crate::interpreter::Interpreter;
use crate::interpreter::StatefulInterpreter;

#[test]
fn expression_stmt_should_return_ok() {
    assert_eq!(
        Ok(obj_nil!()),
        StatefulInterpreter::new()
            .interpret(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))])
    );
}

#[test]
fn print_stmt_should_return_ok() {
    assert_eq!(
        Ok(obj_nil!()),
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
fn return_statement_should_return_the_evaluated_expression_value() {
    let stmts = Stmt::Return(Expr::Primary(obj_bool!(true)));
    assert_eq!(
        Ok(obj_bool!(true)),
        StatefulInterpreter::new().interpret(stmts)
    );
}

#[test]
fn block_statement_should_set_persistent_global_symbol() {
    let stmts = Stmt::Block(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))]);
    assert_eq!(Ok(obj_nil!()), StatefulInterpreter::new().interpret(stmts));
}

#[test]
fn block_statement_with_return_should_return_value() {
    let stmts = Stmt::Block(vec![
        Stmt::Expression(Expr::Primary(obj_bool!(true))),
        Stmt::Return(Expr::Primary(obj_number!(5.0))),
    ]);
    assert_eq!(
        Ok(obj_number!(5.0)),
        StatefulInterpreter::new().interpret(stmts)
    );
}

#[test]
fn function_declaration_statement_should_set_persistent_global_symbol() {
    let block = Stmt::Block(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))]);
    let stmt = Stmt::Function("test".to_string(), vec![], Box::new(block));
    let interpreter = StatefulInterpreter::new();

    let f = functions::Function::new(
        vec![],
        Stmt::Block(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))]),
    );
    let expected_call = obj_call!(Box::new(functions::Callable::Func(f)));

    interpreter.interpret(vec![stmt]).unwrap();
    assert_eq!(
        Some(expected_call),
        interpreter.env.get(&"test".to_string())
    );
}

#[test]
fn function_call_should_return_a_value_when_specified() {
    let block = Stmt::Block(vec![Stmt::Return(Expr::Primary(obj_bool!(true)))]);
    let input = vec![
        Stmt::Function("test".to_string(), vec![], Box::new(block)),
        Stmt::Expression(Expr::Call(
            Box::new(Expr::Variable(Token::new(
                TokenType::Identifier,
                1,
                Some("test".to_string()),
                None,
            ))),
            vec![],
        )),
    ];

    assert_eq!(
        Ok(obj_bool!(true)),
        StatefulInterpreter::new().interpret(input)
    );
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

#[test]
fn while_statement_should_eval_until_false() {
    let stmt = Stmt::While(
        Expr::Primary(obj_bool!(false)),
        Box::new(Stmt::Declaration(
            "test".to_string(),
            Expr::Primary(obj_bool!(true)),
        )),
    );

    assert_eq!(
        Ok(obj_nil!()),
        StatefulInterpreter::new().interpret(vec![stmt])
    );
}
