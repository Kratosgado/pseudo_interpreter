use crate::evaluator::eval_result::Operation;

use super::super::{evaluator::Evaluator, EvalExpression, EvalResult, Expr, Operator};

pub trait Comparison {
    fn evaluate_comparison(&mut self, left: &Expr, op: &Operator, right: &Expr) -> EvalResult;
}

impl Comparison for Evaluator {
    fn evaluate_comparison(&mut self, left: &Expr, op: &Operator, right: &Expr) -> EvalResult {
        let left_val = self.evaluate_expr(left);
        let right_val = self.evaluate_expr(right);

        match op {
            Operator::Equal => left_val.equal(&right_val),
            Operator::LessThan => left_val.less_than(&right_val),
            Operator::GreaterThan => left_val.greater_than(&right_val),
            Operator::LessThanEqual => left_val.less_or_equal(&right_val),
            Operator::GreaterThanEqual => left_val.greater_or_equal(&right_val),
            Operator::NotEqual => left_val.not_equal(&right_val),
            _ => panic!("Invalid comparison operator"),
        }
    }
}
