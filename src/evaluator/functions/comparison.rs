use super::super::{EvalResult, Expr, Operator, evaluator::Evaluator, Expression};


pub trait Comparison {
    fn evaluate_comparison(&self, left: &Expr, op: &Operator, right: &Expr) -> EvalResult;
}

impl Comparison for Evaluator {
    
    fn evaluate_comparison(&self, left: &Expr, op: &Operator, right: &Expr) -> EvalResult {
        let left_val = match self.evaluate_expr(left) {
            EvalResult::Number(val) => val,
            _ => panic!("Expected number"),
        };
        let right_val = match self.evaluate_expr(right) {
            EvalResult::Number(val) => val,
            _ => panic!("Expected a number"),
        };
        let result = match op {
            Operator::Equal => left_val == right_val,
            Operator::LessThan => left_val < right_val,
            Operator::GreaterThan => left_val > right_val,
            Operator::LessThanEqual => left_val <= right_val,
            Operator::GreaterThanEqual => left_val >= right_val,
            Operator::NotEqual => left_val != right_val,
            _ => panic!("Invalid comparison operator"),
        };
        EvalResult::Boolean(result)
    }
}