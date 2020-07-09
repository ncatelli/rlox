use crate::ast::expression::Identifier;
use crate::ast::statement;
use crate::environment::Environment;
use crate::interpreter;
use crate::interpreter::Interpreter;
use crate::object;
use std::fmt;
use std::rc::Rc;

#[cfg(test)]
mod tests;

/// CallError represents an error while attempting to make a function call be
/// it a runtime error or an arity error.
#[derive(Debug, Clone, Copy)]
pub enum CallError {
    Arity,
    Unknown,
}

impl fmt::Display for CallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown call error"),
            Self::Arity => write!(f, "argument count doesn't match function arity"),
        }
    }
}

/// CallResult wraps an object or error return value on a call.
type CallResult = Result<object::Object, CallError>;

/// Callable represents a callable function, whether static or runtime,
/// providing methods for invoking and checking the arity of the method.
#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
    Func(Function),
    Static(StaticFunc),
}

impl Callable {
    pub fn new(fun: Function) -> Self {
        Callable::Func(fun)
    }

    /// arity dispatches to each corresponding functions arity method,
    /// Returning the usize of the function signature arity.
    pub fn arity(&self) -> usize {
        match self {
            Self::Func(f) => f.arity(),
            Self::Static(sf) => sf.arity(),
        }
    }

    /// Call attempts to invoke each correspondings call method.
    pub fn call(&self, env: Rc<Environment>, args: Vec<object::Object>) -> CallResult {
        let arity_match = self.arity() == args.len();

        match (arity_match, self) {
            (true, Self::Func(f)) => f.call(env, args),
            (true, Self::Static(sf)) => sf.call(env, args),
            (false, _) => Err(CallError::Arity),
        }
    }
}

/// Function represents a lox runtime function.
#[derive(Debug, Clone)]
pub struct Function {
    closure: Rc<Environment>,
    params: Vec<Identifier>,
    body: statement::Stmt,
}

impl Function {
    pub fn new(closure: Rc<Environment>, params: Vec<Identifier>, body: statement::Stmt) -> Self {
        Function {
            closure,
            params,
            body,
        }
    }

    pub fn arity(&self) -> usize {
        self.params.len()
    }

    pub fn call(&self, _env: Rc<Environment>, args: Vec<object::Object>) -> CallResult {
        let local = Environment::from(&self.closure);
        for (ident, arg) in self.params.iter().zip(args.into_iter()) {
            local.define(&ident, arg.clone());
        }

        let intptr = interpreter::StatefulInterpreter::from(local);
        match intptr.interpret(self.body.clone()) {
            Ok(rv) => Ok(rv.unwrap_or(obj_nil!())),
            Err(_) => Err(CallError::Unknown),
        }
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

    pub fn call(&self, env: Rc<Environment>, args: Vec<object::Object>) -> CallResult {
        Ok((self.func)(env, args))
    }
}
