use crate::ast::expression::Expr;
use crate::ast::identifier::Identifier;
use crate::ast::statement::Stmt;
use crate::environment::Environment;
use crate::functions;
use crate::object::Object;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn define_statics_ast() -> Vec<Stmt> {
    vec![Stmt::Declaration(
        identifier_name!("clock"),
        Expr::Primary(obj_call!(Box::new(functions::Callable::Static(
            functions::StaticFunc::new(clock)
        )))),
    )]
}

fn clock(_env: Rc<Environment<Identifier, Object>>, _args: Vec<Object>) -> Object {
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64;
    obj_number!(t)
}
