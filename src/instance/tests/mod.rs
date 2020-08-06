use crate::ast::identifier::Identifier;
use crate::class::Class;
use crate::environment::Environment;
use crate::instance::Instance;
use crate::object::Object;
use std::option::Option;
use std::rc::Rc;

#[test]
fn instance_should_allow_getting_of_parameters() {
    let test_instance = Instance::new(&Class::new(&identifier_name!("test")));
    let symtable: Rc<Environment<Identifier, Object>> = test_instance.scope.clone();
    let key = identifier_name!("key");

    assert_eq!(
        symtable.define(&key, obj_bool!(true)),
        Option::Some(obj_bool!(true))
    );

    assert_eq!(test_instance.get(&key), Option::Some(obj_bool!(true)));
}
