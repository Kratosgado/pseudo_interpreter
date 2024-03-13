use crate::Evaluator;
use super::super::{Expr, EvalResult, Arithmetics, Comparison};

pub trait Expression {
    fn evaluate_expr(&self, expr: &Expr) -> EvalResult;
}

impl Expression for Evaluator {
    
    /// Evaluates an expression and returns the result.
    ///
    /// # Panics
    ///
    /// Panics if performing an operation on a non-number.
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    fn evaluate_expr(&self, expr: &Expr) -> EvalResult {
        match expr {
            Expr::Number(num) => EvalResult::Number(*num),
            Expr::Str(value) => EvalResult::Str(value.clone()),
            Expr::Variable(var) => {
                if let Some(value) = self.symbol_table.get(var) {
                    value.clone()
                } else {
                    panic!("undefined variable: {}", var)
                }
            }
            Expr::BinOp(left, op, right) => self.arithmetic_expr(left, op, right),
            Expr::Boolean(val) => EvalResult::Boolean(*val),
            Expr::Comparison(left, op, right  ) => self.evaluate_comparison(left, op, right),
        }
    }

}