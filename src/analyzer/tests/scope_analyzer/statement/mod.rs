use crate::analyzer::Analyzer;
use crate::analyzer::{ScopeAnalyzer, ScopeStack};
use crate::ast::statement::Stmt;

#[test]
fn block_should_return_same_block_layer_after_run() {
    let stmts = Stmt::Block(vec![]);
    assert_eq!(
        Ok(ScopeStack::new()),
        ScopeAnalyzer::new().analyze((ScopeStack::new(), &stmts))
    );
}
