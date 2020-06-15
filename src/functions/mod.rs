use crate::ast::statement;
use crate::ast::token;
use crate::environment::Environment;
use crate::object;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Callable {
    fun: Function,
}

impl Callable {
    pub fn new(fun: Function) -> Self {
        Callable { fun }
    }

    pub fn call(&self, env: Rc<Environment>, args: Vec<object::Object>) -> object::Object {
        let local = Environment::from(&env);
        for (ident, arg) in self.fun.params.iter().zip(args.into_iter()) {
            let lexeme = ident.lexeme.clone().unwrap();
            local.define(&lexeme, arg.clone());
        }

        obj_nil!()
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    params: Vec<token::Token>,
    body: statement::Stmt,
}

impl Function {
    pub fn new(params: Vec<token::Token>, body: statement::Stmt) -> Self {
        Function { params, body }
    }

    pub fn arity(&self) -> usize {
        self.params.len()
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.params == other.params && self.body == other.body
    }
}
