use crate::ast::expression::Expr;
use crate::environment::Environment;
use std::option::Option;

#[test]
fn environment_should_allow_setting_of_symbols() {
    let mut symtable = Environment::new();

    // unset var returns None
    assert_eq!(
        symtable.define("test".to_string(), Expr::Primary(obj_bool!(true))),
        Option::None
    );

    // Subsequent returns previous value
    assert_eq!(
        symtable.define("test".to_string(), Expr::Primary(obj_bool!(true))),
        Option::Some(Expr::Primary(obj_bool!(true)))
    );
}

#[test]
fn environment_should_allow_getting_of_symbols() {
    let mut symtable = Environment::new();
    let key = "test".to_string();

    assert_eq!(
        symtable.define(key.clone(), Expr::Primary(obj_bool!(true))),
        Option::None
    );

    assert_eq!(
        symtable.get(&key),
        Option::Some(&Expr::Primary(obj_bool!(true)))
    );
}
