use crate::analyzer::scope::ScopeAnalyzer;
use crate::analyzer::SemanticAnalyzer;
use crate::ast::expression::Expr;
use crate::ast::statement::Stmt;

#[test]
fn expression_stmt_should_return_ok() {
    let stmts = vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))];

    assert_eq!(Ok(stmts.clone()), ScopeAnalyzer::new().analyze(stmts));
}

#[test]
fn print_stmt_should_return_self() {
    let stmts = vec![Stmt::Print(Expr::Primary(obj_bool!(true)))];

    assert_eq!(Ok(stmts.clone()), ScopeAnalyzer::new().analyze(stmts));
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

    assert_eq!(Ok(output), ScopeAnalyzer::new().analyze(input));
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

    assert_eq!(Ok(output), ScopeAnalyzer::new().analyze(input));
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

    assert_eq!(Ok(output), ScopeAnalyzer::new().analyze(input));
}

#[test]
fn return_statement_should_return_self() {
    let stmts = vec![Stmt::Return(Expr::Primary(obj_bool!(true)))];

    assert_eq!(Ok(stmts.clone()), ScopeAnalyzer::new().analyze(stmts));
}

#[test]
fn block_statement_should_return_self() {
    let stmts = vec![Stmt::Block(vec![Stmt::Expression(Expr::Primary(
        obj_bool!(true),
    ))])];

    assert_eq!(Ok(stmts.clone()), ScopeAnalyzer::new().analyze(stmts));
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

    assert_eq!(Ok(output), ScopeAnalyzer::new().analyze(input));
}

#[test]
fn function_declaration_statement_should_return_self() {
    let block = Stmt::Block(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))]);
    let stmts = vec![Stmt::Function(
        identifier_name!("test"),
        vec![],
        Box::new(block),
    )];

    assert_eq!(Ok(stmts.clone()), ScopeAnalyzer::new().analyze(stmts));
}

#[test]
fn if_statement_should_return_self() {
    let stmts = vec![Stmt::If(
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

    assert_eq!(Ok(stmts.clone()), ScopeAnalyzer::new().analyze(stmts));
}

#[test]
fn while_statement_should_return_self() {
    let stmts = vec![Stmt::While(
        Expr::Primary(obj_bool!(false)),
        Box::new(Stmt::Declaration(
            identifier_name!("test"),
            Expr::Primary(obj_bool!(true)),
        )),
    )];

    assert_eq!(Ok(stmts.clone()), ScopeAnalyzer::new().analyze(stmts))
}
