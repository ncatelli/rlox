use crate::environment::Environment;
use crate::functions;
use crate::object;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn define_statics() -> Rc<Environment> {
    let glbls = Environment::new();
    glbls.define(
        &identifier_id!("clock"),
        obj_call!(Box::new(functions::Callable::Static(
            functions::StaticFunc::new(clock)
        ))),
    );
    glbls
}

fn clock(_env: Rc<Environment>, _args: Vec<object::Object>) -> object::Object {
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64;
    obj_number!(t)
}
