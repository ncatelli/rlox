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
fn declaration_statement_should_return_self() {
    let stmts = vec![Stmt::Declaration(
        identifier_id!("test"),
        Expr::Primary(obj_bool!(true)),
    )];

    assert_eq!(Ok(stmts.clone()), ScopeAnalyzer::new().analyze(stmts));
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
fn function_declaration_statement_should_return_self() {
    let block = Stmt::Block(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))]);
    let stmts = vec![Stmt::Function(
        identifier_id!("test"),
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
            identifier_id!("test"),
            Expr::Primary(obj_bool!(true)),
        )),
        Option::Some(Box::new(Stmt::Declaration(
            identifier_id!("test"),
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
            identifier_id!("test"),
            Expr::Primary(obj_bool!(true)),
        )),
    )];

    assert_eq!(Ok(stmts.clone()), ScopeAnalyzer::new().analyze(stmts))
}
