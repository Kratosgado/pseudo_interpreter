use crate::{constants::error_handler::PseudoError, evaluator::eval_result::Operation};

use super::super::{evaluator::Evaluator, EvalExpression, EvalResult, Expr, Operator};

pub trait MultiCondition {
    fn evaluate_multi_condition(&mut self, left: &Expr, op: &Operator, right: &Expr) -> Result<EvalResult, PseudoError>;
}

impl MultiCondition for Evaluator {
    fn evaluate_multi_condition(&mut self, left: &Expr, op: &Operator, right: &Expr) -> Result<EvalResult, PseudoError> {
        let left_val = self.evaluate_expr(left)?;
        let right_val = self.evaluate_expr(right)?;

        match op {
            Operator::And => Ok(left_val.and(&right_val)?),
            Operator::Or => Ok(left_val.or(&right_val)?),
            _ => return Err(PseudoError::TypeError("Invalid comparison operator".to_string())),
        }
    }
}
