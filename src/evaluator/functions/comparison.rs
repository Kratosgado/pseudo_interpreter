use crate::{constants::error_handler::PseudoError, evaluator::eval_result::Operation};

use super::super::{evaluator::Evaluator, EvalExpression, EvalResult, Expr, Operator};

pub trait Comparison {
    fn evaluate_comparison(&mut self, left: &Expr, op: &Operator, right: &Expr) -> Result<EvalResult, PseudoError>;
}

impl Comparison for Evaluator {
    fn evaluate_comparison(&mut self, left: &Expr, op: &Operator, right: &Expr) -> Result<EvalResult, PseudoError> {
        let left_val = self.evaluate_expr(left)?;
        let right_val = self.evaluate_expr(right)?;

        match op {
            Operator::Equal => Ok(left_val.equal(&right_val)),
            Operator::LessThan => Ok(left_val.less_than(&right_val)),
            Operator::GreaterThan => Ok(left_val.greater_than(&right_val)),
            Operator::LessThanEqual => Ok(left_val.less_or_equal(&right_val)),
            Operator::GreaterThanEqual => Ok(left_val.greater_or_equal(&right_val)),
            Operator::NotEqual => Ok(left_val.not_equal(&right_val)),
            _ => return Err(PseudoError::TypeError("Invalid comparison operator".to_string())),
        }
    }
}
