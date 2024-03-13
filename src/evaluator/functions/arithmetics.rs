use crate::Evaluator;
use super::super::{Expr, EvalResult, Operator, EvalExpression};


pub trait Arithmetics {
    fn arithmetic_expr(&self, left: &Expr, op: &Operator, right: &Expr) -> EvalResult;
}

impl Arithmetics for Evaluator {
    fn arithmetic_expr(&self, left: &Expr, op: &Operator, right: &Expr) -> EvalResult {
        let left_val = match self.evaluate_expr(left) {
            EvalResult::Number(val) => val,
            _ => panic!("Expected number"),
        };
        let right_val = match self.evaluate_expr(right) {
            EvalResult::Number(val) => val,
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