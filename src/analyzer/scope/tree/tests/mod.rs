use crate::analyzer::scope::tree::Node;
use std::collections::HashMap;
use std::rc::Rc;

#[test]
fn node_should_allow_setting_of_symbols() {
    let symtable = Node::new();
    let key = "test";

    assert_eq!(symtable.declare(&key), 0);
    assert_eq!(symtable.declare(&key), 0);
}

#[test]
fn node_with_child_should_return_child_offset_when_set_on_child() {
    let parent = Node::new();
    let child = Node::from(&parent);
    let key = "test";

    assert_eq!(parent.declare(&key), 0);
    assert_eq!(child.declare(&key), 1);
}

#[test]
fn node_should_return_offset_when_resolving_variable() {
    let symtable = Node::new();
    let key = "test";
    symtable.declare(&key);

    let mut expected_locals: HashMap<String, usize> = HashMap::new();
    expected_locals.insert(key.to_string(), 0);

    assert_eq!(symtable.resolve_local(&key), Some(0));
    assert_eq!(
        expected_locals,
        Rc::try_unwrap(symtable).unwrap().locals.into_inner()
    )
}

#[test]
fn child_should_return_parent_offset_when_resolving_variable() {
    let parent = Node::new();
    let child = Node::from(&parent);
    let key = "test";
    parent.declare(&key);

    let mut expected_locals: HashMap<String, usize> = HashMap::new();
    expected_locals.insert(key.to_string(), 0);

    assert_eq!(child.resolve_local(&key), Some(0));
    assert_eq!(
        expected_locals,
        Rc::try_unwrap(child).unwrap().locals.into_inner()
    )
}

#[test]
fn new_node_should_have_no_parent() {
    let symtable = Node::new();

    match symtable.parent {
        Some(_) => assert!(false),
        None => assert!(true),
    }
}

#[test]
fn new_child_node_should_have_a_parent() {
    let parent = Node::new();
    let child = Node::from(&parent);

    match child.parent {
        Some(_) => assert!(true),
        None => assert!(false),
    }
}

#[test]
fn top_level_node_should_return_a_zero_offset() {
    let parent = Node::new();

    // assert parent has no ancestors
    assert_eq!(parent.offset(), 0)
}

#[test]
fn child_offset_should_reflect_count_of_ancestors() {
    let parent = Node::new();
    let child = Node::from(&parent);

    // assert parent has no ancestors
    assert_eq!(child.offset(), 1)
}
