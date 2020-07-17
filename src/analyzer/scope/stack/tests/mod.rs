use crate::analyzer::scope::stack::{Scope, ScopeStack};

#[test]
fn push_should_append_to_stack() {
    let mut ss = ScopeStack::new();

    ss.push(vec![identifier_name!("a")]);
    let ssv: Vec<Scope> = ss.into();

    assert_eq!(ssv, vec![vec![], vec![identifier_name!("a")]])
}

#[test]
fn get_offset_should_return_correct_variable_offset() {
    let mut ss = ScopeStack::new();
    ss.push(vec![
        identifier_name!("a"),
        identifier_name!("b"),
        identifier_name!("c"),
    ]);
    ss.push(vec![identifier_name!("d")]);
    ss.push(vec![identifier_name!("e"), identifier_name!("f")]);
    ss.push(vec![identifier_name!("g")]);

    assert_eq!(Some(2), ss.get_offset(&identifier_name!("c")))
}
