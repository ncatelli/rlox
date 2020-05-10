use crate::interpreter::{Interpreter, InterpreterErr};
use crate::parser::expression::{
    AdditionExpr, ComparisonExpr, Expr, MultiplicationExpr, PrimaryExpr, UnaryExpr,
};

#[cfg(test)]
mod tests;

macro_rules! bool_to_primary {
    ($x:expr) => {
        if $x {
            PrimaryExpr::True
        } else {
            PrimaryExpr::False
        }
    };
}

type InterpreterResult = Result<PrimaryExpr, InterpreterErr>;

pub struct ExpressionInterpreter {}

impl Interpreter<Expr, PrimaryExpr> for ExpressionInterpreter {
    fn interpret(&self, expr: Expr) -> InterpreterResult {
        match expr {
            Expr::Primary(expr) => self.interpret_primary(expr),
            Expr::Unary(expr) => self.interpret_unary(expr),
            Expr::Multiplication(expr) => self.interpret_multiplication(expr),
            Expr::Addition(expr) => self.interpret_addition(expr),
            Expr::Comparison(expr) => self.interpret_comparison(expr),
            _ => Ok(PrimaryExpr::Number(100.0)),
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
        ExpressionInterpreter {}
    }
    fn interpret_comparison(&self, expr: ComparisonExpr) -> Result<PrimaryExpr, InterpreterErr> {
        match expr {
            ComparisonExpr::Less(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(bool_to_primary!(left_val < right_val))
                    }
                    (Ok(l), Ok(r)) => Err(InterpreterErr::TypeErr(format!(
                        "Invalid operand for operator: {} < {}",
                        l, r
                    ))),
                    _ => Err(InterpreterErr::TypeErr(
                        "Invalid operand for operator".to_string(),
                    )),
                }
            }
            ComparisonExpr::LessEqual(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(bool_to_primary!(left_val <= right_val))
                    }
                    (Ok(l), Ok(r)) => Err(InterpreterErr::TypeErr(format!(
                        "Invalid operand for operator: {} <= {}",
                        l, r
                    ))),
                    _ => Err(InterpreterErr::TypeErr(
                        "Invalid operand for operator".to_string(),
                    )),
                }
            }
            ComparisonExpr::Greater(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(bool_to_primary!(left_val > right_val))
                    }
                    (Ok(l), Ok(r)) => Err(InterpreterErr::TypeErr(format!(
                        "Invalid operand for operator: {} > {}",
                        l, r
                    ))),
                    _ => Err(InterpreterErr::TypeErr(
                        "Invalid operand for operator".to_string(),
                    )),
                }
            }
            ComparisonExpr::GreaterEqual(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(bool_to_primary!(left_val >= right_val))
                    }
                    (Ok(l), Ok(r)) => Err(InterpreterErr::TypeErr(format!(
                        "Invalid operand for operator: {} >= {}",
                        l, r
                    ))),
                    _ => Err(InterpreterErr::TypeErr(
                        "Invalid operand for operator".to_string(),
                    )),
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
                (Ok(l), Ok(r)) => Err(InterpreterErr::TypeErr(format!(
                    "Invalid operand for operator: {} + {}",
                    l, r
                ))),
                _ => Err(InterpreterErr::TypeErr(
                    "Invalid operand for operator".to_string(),
                )),
            },
            AdditionExpr::Subtract(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(PrimaryExpr::Number(left_val - right_val))
                    }
                    (Ok(l), Ok(r)) => Err(InterpreterErr::TypeErr(format!(
                        "Invalid operand for operator: {} - {}",
                        l, r
                    ))),
                    _ => Err(InterpreterErr::TypeErr(
                        "Invalid operand for operator".to_string(),
                    )),
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
                    (Ok(l), Ok(r)) => Err(InterpreterErr::TypeErr(format!(
                        "Invalid operand for operator: {} * {}",
                        l, r
                    ))),
                    _ => Err(InterpreterErr::TypeErr(
                        "Invalid operand for operator".to_string(),
                    )),
                }
            }
            MultiplicationExpr::Divide(left, right) => {
                match (self.interpret(left), self.interpret(right)) {
                    (Ok(PrimaryExpr::Number(left_val)), Ok(PrimaryExpr::Number(right_val))) => {
                        Ok(PrimaryExpr::Number(left_val / right_val))
                    }
                    (Ok(l), Ok(r)) => Err(InterpreterErr::TypeErr(format!(
                        "Invalid operand for operator: {} / {}",
                        l, r
                    ))),
                    _ => Err(InterpreterErr::TypeErr(
                        "Invalid operand for operator".to_string(),
                    )),
                }
            }
        }
    }

    fn interpret_unary(&self, expr: UnaryExpr) -> InterpreterResult {
        match expr {
            UnaryExpr::Bang(ue) => match self.interpret(ue) {
                Ok(PrimaryExpr::False) => Ok(PrimaryExpr::True),
                Ok(PrimaryExpr::True) => Ok(PrimaryExpr::False),
                Err(e) => Err(InterpreterErr::TypeErr(e.to_string())),
                _ => Err(InterpreterErr::TypeErr(
                    "Invalid operand for operator".to_string(),
                )),
            },
            UnaryExpr::Minus(ue) => match self.interpret(ue) {
                Ok(PrimaryExpr::Number(v)) => Ok(PrimaryExpr::Number(v * -1.0)),
                Err(e) => Err(InterpreterErr::TypeErr(e.to_string())),
                _ => Err(InterpreterErr::TypeErr(
                    "Invalid operand for operator".to_string(),
                )),
            },
        }
    }

    fn interpret_primary(&self, expr: PrimaryExpr) -> InterpreterResult {
        Ok(expr)
    }
}
