use crate::analyzer::scope::{Node, ScopeAnalyzer};
use crate::analyzer::SemanticAnalyzer;
use crate::ast::statement::Stmt;

#[test]
fn block_statement_with_return_should_return_value() {
    let stmts = vec![Stmt::Block(vec![])];

    assert_eq!(
        Ok(Node::new().add_child(Node::new())),
        ScopeAnalyzer::new().analyze(&stmts)
    );
}
