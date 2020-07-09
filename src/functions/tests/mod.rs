use crate::ast::expression::Expr;
use crate::ast::statement::Stmt;
use crate::environment::Environment;

macro_rules! gen_func {
    () => {
        $crate::functions::Function::new(
            Environment::new(),
            vec![],
            Stmt::Block(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))]),
        )
    };
    ($params:expr) => {
        $crate::functions::Function::new(
            Environment::new(),
            $params,
            Stmt::Block(vec![Stmt::Expression(Expr::Primary(obj_bool!(true)))]),
        )
    };
}

macro_rules! gen_callable {
    ($f:expr) => {
        $crate::functions::Callable::Func($f)
    };
}

#[test]
fn arity_should_return_the_number_of_params_declared() {
    assert_eq!(0, gen_callable!(gen_func!()).arity());
    assert_eq!(
        1,
        gen_callable!(gen_func!(vec![identifier_id!("a"),])).arity()
    );
    assert_eq!(
        2,
        gen_callable!(gen_func!(vec![identifier_id!("a"), identifier_id!("b")])).arity()
    );
}
