use crate::Evaluator;
use super::super::{Expr, EvalResult, Arithmetics, Comparison};

pub trait EvalExpression {
    fn evaluate_expr(&self, expr: &Expr) -> EvalResult;
}

impl EvalExpression for Evaluator {
    
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
            Expr::ArrayVariable(var, index) => {
                if let Some(array) = self.array_table.get(var) {
                    let index = match self.evaluate_expr(&index) {
                        EvalResult::Number(val) => val as usize,
                        _ => panic!("invalid indexing of array"),
                    };
                    array.get(index).unwrap().clone()
                } else {
                    panic!("undefined array variable: {}", var)
                }
            },
        }
    }

}