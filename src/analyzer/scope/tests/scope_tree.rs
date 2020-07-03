use crate::analyzer::scope::{Node, Scope};

#[test]
fn scope_tree_new_returns_a_top_level_tree() {
    assert_eq!(
        Node {
            data: Scope::new(),
            children: None,
        },
        Node::new()
    )
}

#[test]
fn scope_tree_can_append_add_a_child_node() {
    assert_eq!(
        Node {
            data: Scope::new(),
            children: Some(vec![Node::new()].into_iter().collect()),
        },
        Node::new().add_child(Node::new())
    )
}
