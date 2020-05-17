use crate::environment::hashmap;
use crate::environment::Environment;
use crate::parser::expression::{Expr, PrimaryExpr};
use std::option::Option;

#[test]
fn environment_should_allow_setting_of_symbols() {
    let mut symtable = hashmap::Hashmap::new();

    // unset var returns None
    assert_eq!(
        symtable.define("test".to_string(), Expr::Primary(PrimaryExpr::True)),
        Option::None
    );

    // Subsequent returns previous value
    assert_eq!(
        symtable.define("test".to_string(), Expr::Primary(PrimaryExpr::True)),
        Option::Some(Expr::Primary(PrimaryExpr::True))
    );
}

#[test]
fn environment_should_allow_getting_of_symbols() {
    let mut symtable = hashmap::Hashmap::new();
    let key = "test".to_string();

    assert_eq!(
        symtable.define(key.clone(), Expr::Primary(PrimaryExpr::True)),
        Option::None
    );

    assert_eq!(
        symtable.get(&key),
        Option::Some(&Expr::Primary(PrimaryExpr::True))
    );
}
