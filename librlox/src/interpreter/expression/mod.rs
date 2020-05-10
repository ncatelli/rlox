use crate::interpreter::{Interpreter, InterpreterErr};
use crate::parser::expression::{
    AdditionExpr, ComparisonExpr, EqualityExpr, Expr, MultiplicationExpr, PrimaryExpr, UnaryExpr,
};

#[cfg(test)]
mod tests;

macro_rules! type_error {
    () => {
        Err(InterpreterErr::TypeErr(
            "Invalid operand for operator".to_string(),
        ))
    };
    ($error:expr) => {
        Err(InterpreterErr::TypeErr($error.to_string()))
    };
    ($left:expr, $op:literal, $right:expr) => {
        Err(InterpreterErr::TypeErr(format!(
            "Invalid operand for operator: {} {} {}",
            $left, $op, $right
        )))
    };
}

type InterpreterResult = Result<PrimaryExpr, InterpreterErr>;

#[derive(Default)]
pub struct ExpressionInterpreter {}

impl Interpreter<Expr, PrimaryExpr> for ExpressionInterpreter {
    fn interpret(&self, expr: Expr) -> InterpreterResult {
        match expr {
            Expr::Grouping(expr) => self.interpret(expr),
            Expr::Primary(expr) => self.interpret_primary(expr),
            Expr::Unary(expr) => self.interpret_unary(expr),
            Expr::Multiplication(expr) => self.interpret_multiplication(expr),
            Expr::Addition(expr) => self.interpret_addition(expr),
            Expr::Comparison(expr) => self.interpret_comparison(expr),
            Expr::Equality(expr) => self.interpret_equality(expr),
        }
    }
}

/// This functions only to unpack an Expr and dispatch to the upstream Interpreter<Expr, PrimaryExpr> implementation
impl Interpreter<Box<Expr>, PrimaryExpr> for ExpressionInterpreter {
    fn interpret(&self, expr: Box<Expr>) -> InterpreterResult {
        self.interpret(*expr)
    }
}

impl ExpressionInterpreter {
    pub fn new() -> ExpressionInterpreter {
        ExpressionInterpreter::default()
    }

    fn interpret_equality(&self, expr: EqualityExpr) -> Result<PrimaryExpr, InterpreterErr> {
        match expr {
            EqualityExpr::Equal(left, right) => match (self.interpret(left), self.interpret(right))
            {
                (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => Ok(
                    PrimaryExpr::from((left_val - right_val).abs() < std::f64::EPSILON),
                ),
                (Ok(PrimaryExpr::Str(left_val)), Ok(PrimaryExpr::Str(right_val))) => {
                    Ok(PrimaryExpr::from(left_val == right_val))
                }
                (Ok(l), Ok(r)) => type_error!(l, "==", r),
                _ => type_error!(),
            },
            EqualityExpr::NotEqual(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => Ok(
                        PrimaryExpr::from((left_val - right_val).abs() > std::f64::EPSILON),
                    ),
                    (Ok(PrimaryExpr::Str(left_val)), Ok(PrimaryExpr::Str(right_val))) => {
                        Ok(PrimaryExpr::from(left_val != right_val))
                    }
                    (Ok(l), Ok(r)) => type_error!(l, "!=", r),
                    _ => type_error!(),
                }
            }
        }
    }

    fn interpret_comparison(&self, expr: ComparisonExpr) -> Result<PrimaryExpr, InterpreterErr> {
        match expr {
            ComparisonExpr::Less(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(PrimaryExpr::from(left_val < right_val))
                    }
                    (Ok(l), Ok(r)) => type_error!(l, "<", r),
                    _ => type_error!(),
                }
            }
            ComparisonExpr::LessEqual(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(PrimaryExpr::from(left_val <= right_val))
                    }
                    (Ok(l), Ok(r)) => type_error!(l, "<=", r),
                    _ => type_error!(),
                }
            }
            ComparisonExpr::Greater(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(PrimaryExpr::from(left_val > right_val))
                    }
                    (Ok(l), Ok(r)) => type_error!(l, ">", r),
                    _ => type_error!(),
                }
            }
            ComparisonExpr::GreaterEqual(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(PrimaryExpr::from(left_val >= right_val))
                    }
                    (Ok(l), Ok(r)) => type_error!(l, ">=", r),
                    _ => type_error!(),
                }
            }
        }
    }

    fn interpret_addition(&self, expr: AdditionExpr) -> Result<PrimaryExpr, InterpreterErr> {
        match expr {
            AdditionExpr::Add(left, right) => match (self.interpret(left), self.interpret(right)) {
                (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                    Ok(PrimaryExpr::Number(left_val + right_val))
                }
                (Ok(PrimaryExpr::Str(left_val)), Ok(PrimaryExpr::Str(right_val))) => {
                    Ok(PrimaryExpr::Str(format!("{}{}", left_val, right_val)))
                }
                (Ok(l), Ok(r)) => type_error!(l, "+", r),
                _ => type_error!(),
            },
            AdditionExpr::Subtract(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(PrimaryExpr::Number(left_val - right_val))
                    }
                    (Ok(l), Ok(r)) => type_error!(l, "-", r),
                    _ => type_error!(),
                }
            }
        }
    }

    fn interpret_multiplication(
        &self,
        expr: MultiplicationExpr,
    ) -> Result<PrimaryExpr, InterpreterErr> {
        match expr {
            MultiplicationExpr::Multiply(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(PrimaryExpr::Number(left_val * right_val))
                    }
                    (Ok(l), Ok(r)) => type_error!(l, "*", r),
                    _ => type_error!(),
                }
            }
            MultiplicationExpr::Divide(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(PrimaryExpr::Number(left_val / right_val))
                    }
                    (Ok(l), Ok(r)) => type_error!(l, "/", r),
                    _ => type_error!(),
                }
            }
        }
    }

    fn interpret_unary(&self, expr: UnaryExpr) -> InterpreterResult {
        match expr {
            UnaryExpr::Bang(ue) => match self.interpret(ue) {
                Ok(expr) => Ok(PrimaryExpr::from(!is_true(expr))),
                Err(e) => type_error!(e),
            },
            UnaryExpr::Minus(ue) => match self.interpret(ue) {
                Ok(PrimaryExpr::Number(v)) => Ok(PrimaryExpr::Number(v * -1.0)),
                Err(e) => type_error!(e),
                _ => type_error!(),
            },
        }
    }

    fn interpret_primary(&self, expr: PrimaryExpr) -> InterpreterResult {
        Ok(expr)
    }
}

fn is_true(expr: PrimaryExpr) -> bool {
    match expr {
        PrimaryExpr::Nil => false,
        PrimaryExpr::False => false,
        _ => true,
    }
}
