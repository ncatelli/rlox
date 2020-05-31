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
