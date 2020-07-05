use crate::environment::Environment;
use std::option::Option;

#[test]
fn environment_should_allow_setting_of_symbols() {
    let symtable = Environment::new();
    let key = "test";

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
    let key = "test";

    assert_eq!(
        symtable.define(&key, obj_bool!(true)),
        Option::Some(obj_bool!(true))
    );

    assert_eq!(symtable.get(&key), Option::Some(obj_bool!(true)));
}

#[test]
fn environment_should_return_none_if_assign_of_undefined_symbol() {
    let symtable = Environment::new();

    // unset var returns None
    assert_eq!(symtable.assign(&"test", obj_bool!(true)), Option::None);
}

#[test]
fn environment_should_return_some_if_assign_of_undefined_symbol() {
    let symtable = Environment::new();

    symtable.define(&"test", obj_bool!(true));

    // Subsequent returns previous value
    assert_eq!(
        symtable.assign(&"test", obj_bool!(false)),
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
    let key = "test";

    parent.define(&key, obj_bool!(true));

    assert_eq!(child.get(&key), Option::Some(obj_bool!(true)))
}

#[test]
fn child_environment_should_be_able_to_assign_symbols_to_parents() {
    let parent = Environment::new();
    let child = Environment::from(&parent);
    let key = "test";

    parent.define(&key, obj_bool!(true));
    child.assign(&key, obj_bool!(false));

    assert_eq!(child.get(&key), Option::Some(obj_bool!(false)))
}

#[test]
fn child_environment_should_be_able_to_a_specific_parent_symbols_via_assign_at() {
    let root = Environment::new(); // offset 0
    let first = Environment::from(&root); // offset 1
    let second = Environment::from(&first); // offset 2
    let third = Environment::from(&second); // offset 3
    let key = "test";
    first.define(&key, obj_bool!(true));
    second.define(&key, obj_bool!(true));

    // on upward walk it shoudl find the first key, corresponding to second node.
    assert_eq!(
        third.assign_at(1, &key, obj_nil!()),
        Option::Some(obj_nil!())
    );

    // on get_at upward walk it should pull the key referenced by 1st offst.
    assert_eq!(first.get(&key), Option::Some(obj_nil!()));
    assert_eq!(second.get(&key), Option::Some(obj_bool!(true)));
}

#[test]
fn child_environment_should_be_able_to_reference_specific_parent_symbols_via_get_at() {
    let root = Environment::new(); // offset 0
    let first = Environment::from(&root); // offset 1
    let second = Environment::from(&first); // offset 2
    let third = Environment::from(&second); // offset 3
    let key = "test";
    first.define(&key, obj_bool!(true));
    second.define(&key, obj_bool!(false));

    // on upward walk it shoudl find the first key, corresponding to second node.
    assert_eq!(third.get(&key), Option::Some(obj_bool!(false)));

    // on get_at upward walk it should pull the key referenced by 1st offst.
    assert_eq!(third.get_at(&key, 1), Option::Some(obj_bool!(true)));
}

#[test]
fn child_environment_should_ignore_horizontal_nodes_() {
    let root = Environment::new(); // offset 0
    let lfirst = Environment::from(&root); // offset 1
    let rfirst = Environment::from(&root); // offset 1
    let lsecond = Environment::from(&lfirst); // offset 2
    let lthird = Environment::from(&lsecond); // offset 3
    let key = "test";
    lfirst.define(&key, obj_bool!(true));
    rfirst.define(&key, obj_nil!());
    lsecond.define(&key, obj_bool!(false));

    // on upward walk it shoudl find the first key, corresponding to second node.
    assert_eq!(lthird.get(&key), Option::Some(obj_bool!(false)));
    assert_eq!(rfirst.get(&key), Option::Some(obj_nil!()));
    // on get_at upward walk it should pull the key referenced by 1st offst.
    assert_eq!(lthird.get_at(&key, 1), Option::Some(obj_bool!(true)));
}

#[test]
fn top_level_environment_should_return_a_zero_offset() {
    let parent = Environment::new();

    // assert parent has no ancestors
    assert_eq!(parent.offset(), 0)
}

#[test]
fn child_offset_should_reflect_count_of_ancestors() {
    let parent = Environment::new();
    let child = Environment::from(&parent);

    // assert parent has no ancestors
    assert_eq!(child.offset(), 1)
}
