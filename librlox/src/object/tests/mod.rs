use crate::object::{Literal, Object};

#[test]
fn bool_literal_object_converts_into_equivalent_primitive_bool() {
    assert_eq!(true, Object::Literal(Literal::Bool(true)).into());
    assert_eq!(false, Object::Literal(Literal::Bool(false)).into());
}

#[test]
fn number_literal_object_converts_into_equivalent_primitive_bool() {
    assert_eq!(true, Object::Literal(Literal::Number(5.0)).into());
    assert_eq!(false, Object::Literal(Literal::Number(0.0)).into());
}

#[test]
fn str_literal_object_converts_into_equivalent_primitive_bool() {
    assert_eq!(
        true,
        Object::Literal(Literal::Str("hello".to_string())).into()
    );
    assert_eq!(false, Object::Literal(Literal::Str("".to_string())).into());
}

#[test]
fn nil_literal_object_converts_into_equivalent_primitive_bool() {
    assert_eq!(false, Object::Literal(Literal::Nil).into());
}
