use crate::ast::expression::Expr;
use crate::ast::identifier::Identifier;
use crate::ast::statement::Stmt;
use crate::class;
use crate::functions;
use crate::interpreter::StatefulInterpreter;
use crate::pass::*;

#[test]
fn expression_stmt_should_return_ok() {
    assert_eq!(
        Ok(None),
        StatefulInterpreter::new()
            .tree_pass(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))])
    );
}

#[test]
fn print_stmt_should_return_ok() {
    assert_eq!(
        Ok(None),
        StatefulInterpreter::new().tree_pass(vec![Stmt::Print(Expr::Primary(obj_bool!(true)))])
    );
}

#[test]
fn declaration_statement_should_set_persistent_global_symbol() {
    let stmt = Stmt::Declaration(
        Identifier::Name("test".to_string()),
        Expr::Primary(obj_bool!(true)),
    );
    let interpreter = StatefulInterpreter::new();
    interpreter.tree_pass(vec![stmt]).unwrap();
    assert_eq!(
        Some(obj_bool!(true)),
        interpreter.env.get(&Identifier::Name("test".to_string()))
    );
}

#[test]
fn return_statement_should_return_the_evaluated_expression_value() {
    let stmts = Stmt::Return(Expr::Primary(obj_bool!(true)));
    assert_eq!(
        Ok(Some(obj_bool!(true))),
        StatefulInterpreter::new().tree_pass(stmts)
    );
}

#[test]
fn block_statement_should_set_persistent_global_symbol() {
    let stmts = Stmt::Block(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))]);
    assert_eq!(Ok(None), StatefulInterpreter::new().tree_pass(stmts));
}

#[test]
fn block_statement_with_return_should_return_value() {
    let stmts = Stmt::Block(vec![
        Stmt::Expression(Expr::Primary(obj_bool!(true))),
        Stmt::Return(Expr::Primary(obj_number!(5.0))),
    ]);
    assert_eq!(
        Ok(Some(obj_number!(5.0))),
        StatefulInterpreter::new().tree_pass(stmts)
    );
}

#[test]
fn class_declaration_statement_should_set_persistent_symbol() {
    let input = Stmt::Class(identifier_name!("test"), vec![]);
    let interpreter = StatefulInterpreter::new();
    interpreter.tree_pass(input).unwrap();
    assert_eq!(
        Some(obj_class!(class::Class::new(&identifier_name!("test")))),
        interpreter.env.get(&Identifier::Name("test".to_string()))
    );
}

#[test]
fn function_declaration_statement_should_set_persistent_global_symbol() {
    let block = Stmt::Block(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))]);
    let stmt = Stmt::Function(
        Identifier::Name("test".to_string()),
        vec![],
        Box::new(block),
    );
    let interpreter = StatefulInterpreter::new();

    let f = functions::Function::new(
        interpreter.env.clone(),
        vec![],
        Stmt::Block(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))]),
    );
    let expected_call = obj_call!(Box::new(functions::Callable::Func(f)));

    interpreter.tree_pass(vec![stmt]).unwrap();
    assert_eq!(
        Some(expected_call),
        interpreter.env.get(&Identifier::Name("test".to_string()))
    );
}

#[test]
fn function_call_should_return_a_value_when_specified() {
    let block = Stmt::Block(vec![Stmt::Return(Expr::Primary(obj_bool!(true)))]);
    let input = vec![
        Stmt::Function(
            Identifier::Name("test".to_string()),
            vec![],
            Box::new(block),
        ),
        Stmt::Return(Expr::Call(
            Box::new(Expr::Variable(Identifier::Name("test".to_string()))),
            vec![],
        )),
    ];

    assert_eq!(
        Ok(Some(obj_bool!(true))),
        StatefulInterpreter::new().tree_pass(input)
    );
}

#[test]
fn if_statement_should_eval_to_primary_clause_if_condition_is_true() {
    let interpreter = StatefulInterpreter::new();
    let stmt = Stmt::If(
        Expr::Primary(obj_bool!(true)),
        Box::new(Stmt::Declaration(
            Identifier::Name("test".to_string()),
            Expr::Primary(obj_bool!(true)),
        )),
        Option::Some(Box::new(Stmt::Declaration(
            Identifier::Name("test".to_string()),
            Expr::Primary(obj_bool!(false)),
        ))),
    );

    interpreter.tree_pass(vec![stmt]).unwrap();
    assert_eq!(
        Some(obj_bool!(true)),
        interpreter.env.get(&Identifier::Name("test".to_string()))
    );
}

#[test]
fn if_statement_should_eval_to_else_clause_if_condition_is_false() {
    let interpreter = StatefulInterpreter::new();
    let stmt = Stmt::If(
        Expr::Primary(obj_bool!(false)),
        Box::new(Stmt::Declaration(
            Identifier::Name("test".to_string()),
            Expr::Primary(obj_bool!(true)),
        )),
        Option::Some(Box::new(Stmt::Declaration(
            Identifier::Name("test".to_string()),
            Expr::Primary(obj_bool!(false)),
        ))),
    );

    interpreter.tree_pass(vec![stmt]).unwrap();
    assert_eq!(
        Some(obj_bool!(false)),
        interpreter.env.get(&Identifier::Name("test".to_string()))
    );
}

#[test]
fn while_statement_should_eval_until_false() {
    let stmt = Stmt::While(
        Expr::Primary(obj_bool!(false)),
        Box::new(Stmt::Declaration(
            Identifier::Name("test".to_string()),
            Expr::Primary(obj_bool!(true)),
        )),
    );

    assert_eq!(Ok(None), StatefulInterpreter::new().tree_pass(vec![stmt]));
}
