use crate::analyzer::Analyzer;
use crate::analyzer::{ScopeAnalyzer, ScopeStack};
use crate::ast::expression::Expr;

#[ignore]
#[test]
fn block_should_return_same_block_layer_after_run() {
    let input = Expr::Primary(obj_bool!(true));

    assert_eq!(
        Ok(ScopeStack::new()),
        ScopeAnalyzer::new().analyze((ScopeStack::new(), &input))
    );
}
