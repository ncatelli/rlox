use crate::ast::statement;
use crate::ast::token;
use crate::environment::Environment;
use crate::object;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
    Func(Function),
}

impl Callable {
    pub fn new(fun: Function) -> Self {
        Callable::Func(fun)
    }

    pub fn call(&self, env: Rc<Environment>, args: Vec<object::Object>) -> object::Object {
        match self {
            Self::Func(f) => f.call(env, args),
        }
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

    pub fn call(&self, env: Rc<Environment>, args: Vec<object::Object>) -> object::Object {
        let local = Environment::from(&env);
        for (ident, arg) in self.params.iter().zip(args.into_iter()) {
            let lexeme = ident.lexeme.clone().unwrap();
            local.define(&lexeme, arg.clone());
        }

        obj_nil!()
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.params == other.params && self.body == other.body
    }
}
