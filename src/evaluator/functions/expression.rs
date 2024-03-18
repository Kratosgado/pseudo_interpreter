use super::super::{Arithmetics, CallFunc, Comparison, EvalResult, Evaluator, Expr};

pub trait EvalExpression {
    fn evaluate_expr(&mut self, expr: &Expr) -> EvalResult;
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
    fn evaluate_expr(&mut self, expr: &Expr) -> EvalResult {
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
            Expr::Comparison(left, op, right) => self.evaluate_comparison(left, op, right),
            Expr::ArrayVariable(var, index) => {
                if let Some(array) = self.array_table.get(var).cloned() {
                    let index = match self.evaluate_expr(&index) {
                        EvalResult::Number(val) => val as usize,
                        _ => panic!("invalid indexing of array"),
                    };
                    array.get(index).expect("Subscript out of range").clone()
                } else {
                    panic!("undefined array variable: {}", var)
                }
            }
            Expr::Param(_) => todo!("evaluator for parameters not implemented"),
            Expr::FunctionCall(name, args) => self.call_func(name, args),
            Expr::Multi(_) => unimplemented!("Multi expression not implemented"),
        }
    }
}
