use crate::{
    constants::error_handler::PseudoError,
    evaluator::{eval_result::Operation, flow::Flow, Expr},
};

use super::super::{EvalExpression, EvalStatement, Evaluator, Statement};
pub trait EvalFor {
    fn eval_for(&mut self, statement: &Statement) -> Result<Flow, PseudoError>;
}

impl EvalFor for Evaluator {
    fn eval_for(&mut self, statement: &Statement) -> Result<Flow, PseudoError> {
        if let Statement::For(var, start, end, step, fstatement) = statement {
            let var = match var {
                Expr::Variable(val) => val.clone(),
                _ => {
                    return Err(PseudoError::VariableError(
                        "Expected a variable".to_string(),
                    ))
                }
            };
            let start = match start {
                Some(start) => self.evaluate_expr(start)?,
                None => self
                    .get_var(&var)
                    .ok_or_else(|| {
                        PseudoError::VariableError(format!("undefined variable: {}", var))
                    })?
                    .clone(),
            };
            let end = self.evaluate_expr(end)?;
            let step = self.evaluate_expr(step)?;
            self.set_var(var.clone(), start);
            while let Some(value) = self.get_var(&var).cloned() {
                if value != end {
                    if let Flow::Return(value) = self.evaluate_block(fstatement)? {
                        return Ok(Flow::Return(value));
                    }
                    let value = self
                        .get_var(&var)
                        .ok_or_else(|| {
                            PseudoError::VariableError(format!("undefined variable: {}", var))
                        })?
                        .clone();
                    let value = value.add(&step)?;

                    self.set_var(var.clone(), value);
                } else {
                    if let Flow::Return(value) = self.evaluate_block(fstatement)? {
                        return Ok(Flow::Return(value));
                    }
                    break;
                }
            }
        }
        Ok(Flow::Normal)
    }
}
