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
fn declaration_should_add_var_to_symbol_table() {
    let stmt = vec![Stmt::Declaration(
        "test".to_string(),
        Expr::Primary(obj_bool!(true)),
    )];

    // setup expected values
    let root = Node::new();
    root.declare("test");

    let scopes: Vec<Rc<Node>> = ScopeAnalyzer::new()
        .analyze(&stmt)
        .unwrap()
        .into_iter()
        .map(|node| node)
        .collect();

    assert_eq!(vec![root], scopes);
}

#[test]
fn declaration_should_assign_definition_to_correct_node_when_nesting() {
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

#[test]
/// This is a shitty test that I should fix.
fn variable_lookups_should_walk_to_owning_symbol_table() {
    let stmt = vec![
        Stmt::Declaration("test".to_string(), Expr::Primary(obj_bool!(true))),
        Stmt::Block(vec![Stmt::Expression(Expr::Variable(tok_identifier!(
            "test"
        )))]),
    ];

    // setup expected values
    // this should set a variable (test) on parent.
    let parent = Node::new();
    let child = Node::from(&parent);
    parent.declare("test");
    child.resolve_local(&"test");

    let scopes: Vec<Rc<Node>> = ScopeAnalyzer::new()
        .analyze(&stmt)
        .unwrap()
        .into_iter()
        .map(|node| node)
        .collect();

    assert_eq!(vec![parent, child], scopes);
}
