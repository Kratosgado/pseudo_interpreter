use super::super::{EvalExpression, EvalResult, Expr, Operator};
use crate::Evaluator;

pub trait Arithmetics {
    fn arithmetic_expr(&mut self, left: &Expr, op: &Operator, right: &Expr) -> EvalResult;
}

impl Arithmetics for Evaluator {
    fn arithmetic_expr(&mut self, left: &Expr, op: &Operator, right: &Expr) -> EvalResult {
        let left_val = match self.evaluate_expr(left) {
            EvalResult::Number(val) => val,
            EvalResult::Str(val) => val.parse().expect("Could not parse string to integer"),
            _ => panic!("Expected number"),
        };
        let right_val = match self.evaluate_expr(right) {
            EvalResult::Number(val) => val,
            EvalResult::Str(val) => val.parse().expect("Could not parse string to integer"),
            _ => panic!("Expected a number"),
        };
        let result = match op {
            Operator::Add => left_val + right_val,
            Operator::Subtract => left_val - right_val,
            Operator::Multiply => left_val * right_val,
            Operator::Divide => left_val / right_val,
            Operator::Modulo => left_val % right_val,
            _ => panic!("Invalid arithmetic operator"),
        };
        EvalResult::Number(result)
    }
}
