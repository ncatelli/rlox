use crate::analyzer::scope::{Node, Scope};

macro_rules! node {
    () => {
        $crate::analyzer::scope::Node {
            data: $crate::analyzer::scope::Scope::new(),
            children: None,
        }
    };
    ($children:expr) => {
        $crate::analyzer::scope::Node {
            data: $crate::analyzer::scope::Scope::new(),
            children: Some($children.into_iter().collect()),
        }
    };
}

#[test]
fn node_new_returns_a_top_level_tree() {
    assert_eq!(node!(), Node::new())
}

#[test]
fn node_can_append_add_a_child_node() {
    assert_eq!(node!(vec![Node::new()]), Node::new().add_child(Node::new()))
}

#[test]
fn node_can_flatten_to_a_vec() {
    let flattened: Vec<Scope> = Node::new()
        .add_child(Node::new())
        .add_child(Node::new().add_child(Node::new()))
        .into();

    assert_eq!(
        vec![Scope::new(), Scope::new(), Scope::new(), Scope::new()],
        flattened
    )
}
