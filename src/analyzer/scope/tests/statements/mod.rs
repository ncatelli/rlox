use crate::analyzer::scope::{Node, ScopeAnalyzer};
use crate::analyzer::SemanticAnalyzer;
use crate::ast::expression::Expr;
use crate::ast::statement::Stmt;

#[test]
fn block_statement_should_return_a_new_child() {
    let stmts = vec![Stmt::Block(vec![])];

    assert_eq!(
        Ok(Node::new().add_child(Node::new())),
        ScopeAnalyzer::new().analyze(&stmts)
    );
}

#[test]
fn declaration_statement_should_cause_an_assignment() {
    let stmts = vec![Stmt::Declaration(
        "test".to_string(),
        Expr::Primary(obj_nil!()),
    )];

    assert_eq!(
        Ok(Node::new().define("test")),
        ScopeAnalyzer::new().analyze(&stmts)
    );
}
