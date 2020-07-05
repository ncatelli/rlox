use crate::analyzer::scope::tree::Node;
use crate::analyzer::scope::ScopeAnalyzer;
use crate::analyzer::SemanticAnalyzer;
use crate::ast::expression::Expr;
use crate::ast::statement::Stmt;
use std::rc::Rc;

#[test]
fn single_stmt_scope_generate_and_ascending_queue_of_offset_nodes() {
    let stmts = Stmt::Block(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))]);

    let scope_offsets: Vec<usize> = ScopeAnalyzer::new()
        .analyze(&stmts)
        .unwrap()
        .unwrap()
        .into_iter()
        .map(|node| node.offset())
        .collect();

    assert_eq!(vec![1], scope_offsets);
}

#[test]
fn single_statement_analyze_should_capture_nested_scopes() {
    let stmts = Stmt::Block(vec![Stmt::Block(vec![Stmt::Expression(Expr::Primary(
        obj_bool!(true),
    ))])]);

    let sa = ScopeAnalyzer::new();

    let scope_offsets: Vec<usize> = sa
        .analyze(&stmts)
        .unwrap()
        .unwrap()
        .into_iter()
        .map(|node| node.offset())
        .collect();

    assert_eq!(vec![1, 2], scope_offsets);
}

#[test]
fn declaration_should_add_var_to_locals() {
    let stmt = vec![Stmt::Block(vec![Stmt::Declaration(
        "test".to_string(),
        Expr::Primary(obj_bool!(true)),
    )])];

    // setup expected values
    let parent = Node::new();
    let child = Node::from(&parent);
    child.declare("test");

    let scopes: Vec<Rc<Node>> = ScopeAnalyzer::new()
        .analyze(&stmt)
        .unwrap()
        .into_iter()
        .map(|node| node)
        .collect();

    assert_eq!(vec![parent, child], scopes);
}
