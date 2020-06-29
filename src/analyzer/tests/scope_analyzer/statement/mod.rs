use crate::analyzer::Analyzer;
use crate::analyzer::{ScopeAnalyzer, ScopeStack};
use crate::ast::expression::Expr;
use crate::ast::statement::Stmt;

#[test]
fn block_should_return_same_block_layer_after_run() {
    let stmts = Stmt::Block(vec![]);
    assert_eq!(
        Ok(ScopeStack::new()),
        ScopeAnalyzer::new().analyze((ScopeStack::new(), &stmts))
    );
}

#[test]
fn declaration_stmts_should_assign_a_key_to_current_scope() {
    let stmt = Stmt::Declaration("test".to_string(), Expr::Primary(obj_bool!(true)));
    let stmts = vec![stmt];

    assert_eq!(
        Ok(ScopeStack::new().define("test")),
        ScopeAnalyzer::new().analyze((ScopeStack::new(), &stmts))
    );
}
