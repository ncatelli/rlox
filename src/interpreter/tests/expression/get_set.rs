use crate::ast::expression::Expr;
use crate::class::Class;
use crate::instance::Instance;
use crate::interpreter::StatefulInterpreter;
use crate::pass::*;

#[test]
fn get_parameter_from_instance() {
    let ti = Instance::new(&Class::new(&identifier_name!("test")));
    ti.scope
        .clone()
        .define(&identifier_name!("test_param"), obj_bool!(true));

    assert_eq!(
        Ok(obj_bool!(true)),
        StatefulInterpreter::new().tree_pass(Expr::Get(
            Box::new(Expr::Primary(obj_instance!(ti))),
            Box::new(Expr::Variable(identifier_name!("test_param")))
        ))
    );
}
