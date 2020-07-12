use crate::ast::identifier::Identifier;
use crate::environment::Environment;
use crate::functions;
use crate::object::Object;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn define_statics() -> Rc<Environment<Identifier, Object>> {
    let glbls = Environment::new();
    glbls.define(
        &identifier_id!("clock"),
        obj_call!(Box::new(functions::Callable::Static(
            functions::StaticFunc::new(clock)
        ))),
    );
    glbls
}

fn clock(_env: Rc<Environment<Identifier, Object>>, _args: Vec<Object>) -> Object {
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64;
    obj_number!(t)
}
