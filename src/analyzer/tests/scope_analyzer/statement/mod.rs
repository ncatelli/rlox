use crate::analyzer::Analyzer;
use crate::analyzer::{Scope, ScopeAnalyzer};
use crate::ast::statement::Stmt;

#[test]
fn block_should_return_same_block_layer_after_run() {
    let stmts = Stmt::Block(vec![]);
    let scope = vec![Scope::new()];
    assert_eq!(
        Ok(vec![Scope::new()]),
        ScopeAnalyzer::new().analyze((scope, stmts))
    );
}
