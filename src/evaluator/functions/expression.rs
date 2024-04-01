use crate::{constants::error_handler::PseudoError, evaluator::MultiCondition};

use super::super::{Arithmetics, CallFunc, Comparison, EvalResult, Evaluator, Expr};

pub trait EvalExpression {
    fn evaluate_expr(&mut self, expr: &Expr) -> Result<EvalResult, PseudoError>;
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
    fn evaluate_expr(&mut self, expr: &Expr) -> Result<EvalResult, PseudoError> {
        match expr {
            Expr::Number(num) => Ok(EvalResult::Number(*num)),
            Expr::Float(num) => Ok(EvalResult::Double(*num)),
            Expr::Str(value) => Ok(EvalResult::Str(value.clone())),
            Expr::Variable(var) => {
                if let Some(value) = self.symbol_table.get(var) {
                    Ok(value.clone())
                } else {
                    return Err(PseudoError::VariableError(format!(
                        "undefined variable: {}",
                        var
                    )));
                }
            }
            Expr::Not(expr) => {
                // let res: EvalResult = self.evaluate_expr(expr)?;
                match self.evaluate_expr(expr)? {
                    EvalResult::Boolean(val) => Ok(EvalResult::Boolean(!val)),
                    _ => {
                        return Err(PseudoError::TypeError(
                            "Not must be followed by boolean expression".to_string(),
                        ))
                    }
                }
            }
            Expr::BinOp(left, op, right) => self.arithmetic_expr(left, op, right),
            Expr::Boolean(val) => Ok(EvalResult::Boolean(*val)),
            Expr::Comparison(left, op, right) => self.evaluate_comparison(left, op, right),
            Expr::MultiCondition(left, op, right ) => self.evaluate_multi_condition(left, op, right),
            Expr::ArrayVariable(var, index) => {
                if let Some(array) = self.array_table.get(var).cloned() {
                    let index = match self.evaluate_expr(&index)? {
                        EvalResult::Number(val) => val as usize,
                        _ => {
                            return Err(PseudoError::ValueError(
                                "Invalid indexing of array".to_string(),
                            ))
                        }
                    };
                    Ok(array.get(index).expect("Subscript out of range").clone())
                } else {
                    return Err(PseudoError::VariableError(format!(
                        "undefined variable: {}",
                        var
                    )));
                }
            }
            Expr::Param(_) => todo!("evaluator for parameters not implemented"),
            Expr::FunctionCall(name, args) => self.call_func(name, args),
            Expr::Multi(_) => unimplemented!("Multi expression not implemented"),
        }
    }
}
