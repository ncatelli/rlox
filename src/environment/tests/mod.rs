use crate::environment::Environment;
use std::option::Option;

#[test]
fn environment_should_allow_setting_of_symbols() {
    let symtable = Environment::new();
    let key = identifier_id!("test");

    // unset var returns None
    assert_eq!(
        symtable.define(&key, obj_bool!(true)),
        Option::Some(obj_bool!(true))
    );

    assert_eq!(
        symtable.define(&key, obj_bool!(false)),
        Option::Some(obj_bool!(false))
    );
}

#[test]
fn environment_should_allow_getting_of_symbols() {
    let symtable = Environment::new();
    let key = identifier_id!("key");

    assert_eq!(
        symtable.define(&key, obj_bool!(true)),
        Option::Some(obj_bool!(true))
    );

    assert_eq!(symtable.get(&key), Option::Some(obj_bool!(true)));
}

#[test]
fn environment_should_return_none_if_assign_of_undefined_symbol() {
    let symtable = Environment::new();
    let key = identifier_id!("test");

    // unset var returns None
    assert_eq!(symtable.assign(&key, obj_bool!(true)), Option::None);
}

#[test]
fn environment_should_return_some_if_assign_of_undefined_symbol() {
    let symtable = Environment::new();
    let key = identifier_id!("test");

    symtable.define(&key, obj_bool!(true));

    // Subsequent returns previous value
    assert_eq!(
        symtable.assign(&key, obj_bool!(false)),
        Option::Some(obj_bool!(false))
    );
}

#[test]
fn new_environment_should_have_no_parent() {
    let symtable = Environment::new();

    match symtable.parent {
        Some(_) => assert!(false),
        None => assert!(true),
    }
}

#[test]
fn new_child_environment_should_have_a_parent() {
    let parent = Environment::new();
    let child = Environment::from(&parent);

    match child.parent {
        Some(_) => assert!(true),
        None => assert!(false),
    }
}

#[test]
fn child_environment_should_be_able_to_reference_parent_symbols() {
    let parent = Environment::new();
    let child = Environment::from(&parent);
    let key = identifier_id!("test");

    parent.define(&key, obj_bool!(true));

    assert_eq!(child.get(&key), Option::Some(obj_bool!(true)))
}

#[test]
fn child_environment_should_be_able_to_assign_symbols_to_parents() {
    let parent = Environment::new();
    let child = Environment::from(&parent);
    let key = identifier_id!("test");

    parent.define(&key, obj_bool!(true));
    child.assign(&key, obj_bool!(false));

    assert_eq!(child.get(&key), Option::Some(obj_bool!(false)))
}
