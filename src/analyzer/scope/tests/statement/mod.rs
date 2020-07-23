use crate::analyzer::scope::ScopeAnalyzer;
use crate::ast::expression::Expr;
use crate::ast::statement::Stmt;
use crate::pass::*;

#[test]
fn expression_stmt_should_return_ok() {
    let stmts = vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))];

    assert_eq!(Ok(stmts.clone()), ScopeAnalyzer::new().tree_pass(stmts));
}

#[test]
fn print_stmt_should_return_self() {
    let stmts = vec![Stmt::Print(Expr::Primary(obj_bool!(true)))];

    assert_eq!(Ok(stmts.clone()), ScopeAnalyzer::new().tree_pass(stmts));
}

#[test]
fn declaration_statement_should_return_id_def() {
    let input = vec![Stmt::Declaration(
        identifier_name!("test"),
        Expr::Primary(obj_bool!(true)),
    )];

    let output = vec![Stmt::Declaration(
        identifier_id!(0),
        Expr::Primary(obj_bool!(true)),
    )];

    assert_eq!(Ok(output), ScopeAnalyzer::new().tree_pass(input));
}

#[test]
fn multiple_unique_declaration_statements_should_increment_id() {
    let input = vec![
        Stmt::Declaration(identifier_name!("test"), Expr::Primary(obj_bool!(true))),
        Stmt::Declaration(
            identifier_name!("test_again"),
            Expr::Primary(obj_bool!(false)),
        ),
    ];

    let output = vec![
        Stmt::Declaration(identifier_id!(0), Expr::Primary(obj_bool!(true))),
        Stmt::Declaration(identifier_id!(1), Expr::Primary(obj_bool!(false))),
    ];

    assert_eq!(Ok(output), ScopeAnalyzer::new().tree_pass(input));
}

#[test]
fn multiple_matching_declaration_statements_should_increment_id() {
    let input = vec![
        Stmt::Declaration(identifier_name!("test"), Expr::Primary(obj_bool!(true))),
        Stmt::Declaration(identifier_name!("test"), Expr::Primary(obj_bool!(false))),
    ];

    let output = vec![
        Stmt::Declaration(identifier_id!(0), Expr::Primary(obj_bool!(true))),
        Stmt::Declaration(identifier_id!(0), Expr::Primary(obj_bool!(false))),
    ];

    assert_eq!(Ok(output), ScopeAnalyzer::new().tree_pass(input));
}

#[test]
fn return_statement_should_return_self() {
    let stmts = vec![Stmt::Return(Expr::Primary(obj_bool!(true)))];

    assert_eq!(Ok(stmts.clone()), ScopeAnalyzer::new().tree_pass(stmts));
}

#[test]
fn block_statement_should_return_self() {
    let stmts = vec![Stmt::Block(vec![Stmt::Expression(Expr::Primary(
        obj_bool!(true),
    ))])];

    assert_eq!(Ok(stmts.clone()), ScopeAnalyzer::new().tree_pass(stmts));
}

#[test]
fn block_statement_should_analyze_child_stmts() {
    let input = vec![Stmt::Block(vec![Stmt::Declaration(
        identifier_name!("test"),
        Expr::Primary(obj_bool!(true)),
    )])];

    let output = vec![Stmt::Block(vec![Stmt::Declaration(
        identifier_id!(0),
        Expr::Primary(obj_bool!(true)),
    )])];

    assert_eq!(Ok(output), ScopeAnalyzer::new().tree_pass(input));
}

#[test]
fn function_declaration_statement_should_return_self() {
    let block = Stmt::Block(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))]);
    let input = vec![Stmt::Function(
        identifier_name!("test"),
        vec![],
        Box::new(block.clone()),
    )];
    let output = vec![Stmt::Function(
        identifier_id!(0),
        vec![],
        Box::new(block.clone()),
    )];

    assert_eq!(Ok(output), ScopeAnalyzer::new().tree_pass(input));
}

#[test]
fn if_statement_should_return_self() {
    let input = vec![Stmt::If(
        Expr::Primary(obj_bool!(true)),
        Box::new(Stmt::Declaration(
            identifier_name!("test"),
            Expr::Primary(obj_bool!(true)),
        )),
        Option::Some(Box::new(Stmt::Declaration(
            identifier_name!("test"),
            Expr::Primary(obj_bool!(false)),
        ))),
    )];

    let output = vec![Stmt::If(
        Expr::Primary(obj_bool!(true)),
        Box::new(Stmt::Declaration(
            identifier_id!(0),
            Expr::Primary(obj_bool!(true)),
        )),
        Option::Some(Box::new(Stmt::Declaration(
            identifier_id!(0),
            Expr::Primary(obj_bool!(false)),
        ))),
    )];

    assert_eq!(Ok(output), ScopeAnalyzer::new().tree_pass(input));
}

#[test]
fn while_statement_should_return_self() {
    let input = vec![Stmt::While(
        Expr::Primary(obj_bool!(false)),
        Box::new(Stmt::Declaration(
            identifier_name!("test"),
            Expr::Primary(obj_bool!(true)),
        )),
    )];

    let output = vec![Stmt::While(
        Expr::Primary(obj_bool!(false)),
        Box::new(Stmt::Declaration(
            identifier_id!(0),
            Expr::Primary(obj_bool!(true)),
        )),
    )];

    assert_eq!(Ok(output), ScopeAnalyzer::new().tree_pass(input))
}

#[test]
fn class_statement_should_return_self() {
    let input = vec![Stmt::Class(
        identifier_name!("test"),
        vec![Stmt::Function(
            identifier_name!("test_func"),
            vec![],
            Box::new(Stmt::Block(vec![Stmt::Print(Expr::Variable(
                identifier_name!("variable"),
            ))])),
        )],
    )];

    let output = vec![Stmt::Class(identifier_id!(0), vec![])];

    assert_eq!(Ok(output), ScopeAnalyzer::new().tree_pass(input))
}

#[test]
fn scope_should_shade_correctly() {
    let input = vec![
        Stmt::Declaration(
            identifier_name!("a"),
            Expr::Primary(obj_str!("global".to_string())),
        ),
        Stmt::Block(vec![
            Stmt::Function(
                identifier_name!("showA"),
                vec![],
                Box::new(Stmt::Block(vec![Stmt::Print(Expr::Variable(
                    identifier_name!("a"),
                ))])),
            ),
            Stmt::Expression(Expr::Call(
                Box::new(Expr::Variable(identifier_name!("showA"))),
                vec![],
            )),
            Stmt::Declaration(
                identifier_name!("a"),
                Expr::Primary(obj_str!("block".to_string())),
            ),
            Stmt::Expression(Expr::Call(
                Box::new(Expr::Variable(identifier_name!("showA"))),
                vec![],
            )),
        ]),
    ];

    let output = vec![
        Stmt::Declaration(
            identifier_id!(0),
            Expr::Primary(obj_str!("global".to_string())),
        ),
        Stmt::Block(vec![
            Stmt::Function(
                identifier_id!(1),
                vec![],
                Box::new(Stmt::Block(vec![Stmt::Print(Expr::Variable(
                    identifier_id!(0),
                ))])),
            ),
            Stmt::Expression(Expr::Call(
                Box::new(Expr::Variable(identifier_id!(1))),
                vec![],
            )),
            Stmt::Declaration(
                identifier_id!(2),
                Expr::Primary(obj_str!("block".to_string())),
            ),
            Stmt::Expression(Expr::Call(
                Box::new(Expr::Variable(identifier_id!(1))),
                vec![],
            )),
        ]),
    ];

    assert_eq!(Ok(output), ScopeAnalyzer::new().tree_pass(input))
}
