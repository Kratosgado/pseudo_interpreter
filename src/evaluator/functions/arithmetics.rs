use super::super::{EvalExpression, EvalResult, Expr, Operator};
use crate::{constants::error_handler::PseudoError, Evaluator};

pub trait Arithmetics {
    fn arithmetic_expr(&mut self, left: &Expr, op: &Operator, right: &Expr) -> Result<EvalResult, PseudoError>;
}

impl Arithmetics for Evaluator {
    fn arithmetic_expr(&mut self, left: &Expr, op: &Operator, right: &Expr) -> Result<EvalResult, PseudoError> {
        let left_val: i64 = match self.evaluate_expr(left)? {
            EvalResult::Number(val) => val,
            EvalResult::Str(val) => val.parse().expect("Could not parse string to integer"),
            _ => return Err(PseudoError::TypeError("Expected a number".to_string())),
        };
        let right_val = match self.evaluate_expr(right)? {
            EvalResult::Number(val) => val,
            EvalResult::Str(val) => val.parse().expect("Could not parse string to integer"),
            _ => panic!("Expected a number"),
        };
        // println!("left_val: {:?}, right_val: {:?}", left_val, right_val); // Debug
        let result = match op {
            Operator::Add => left_val + right_val,
            Operator::Subtract => left_val - right_val,
            Operator::Multiply => left_val * right_val,
            Operator::Divide => left_val / right_val,
            Operator::Modulo => left_val % right_val,
            _ => panic!("Invalid arithmetic operator"),
        };
        Ok(EvalResult::Number(result))
    }
}
