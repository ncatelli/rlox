use crate::ast::statement;
use crate::ast::token;
use crate::environment::Environment;
use crate::object;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub enum CallError {
    Arity,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
    Func(Function),
    Static(StaticFunc),
}

impl Callable {
    pub fn new(fun: Function) -> Self {
        Callable::Func(fun)
    }

    pub fn call(&self, env: Rc<Environment>, args: Vec<object::Object>) -> object::Object {
        match self {
            Self::Func(f) => f.call(env, args),
            Self::Static(sf) => sf.call(env, args),
        }
    }
}

/// Function represents a lox runtime function.
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

/// StaticFuncCallback is a type that all static functions must implement. This
/// type takes an environment and a vector of objects, representing arguments for
/// use at call time.
type StaticFuncCallback = fn(Rc<Environment>, Vec<object::Object>) -> object::Object;

/// StaticFunc represents a static function to be called at a later date.
#[derive(Debug, Clone, PartialEq)]
pub struct StaticFunc {
    func: StaticFuncCallback,
}

impl StaticFunc {
    pub fn new(func: fn(Rc<Environment>, args: Vec<object::Object>) -> object::Object) -> Self {
        Self { func }
    }

    pub fn arity(&self) -> usize {
        0
    }

    pub fn call(&self, env: Rc<Environment>, args: Vec<object::Object>) -> object::Object {
        (self.func)(env, args)
    }
}
