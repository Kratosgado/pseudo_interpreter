use crate::{constants::error_handler::PseudoError, evaluator::Expr};

use super::super::{EvalExpression, EvalResult, EvalStatement, Evaluator, Statement};
pub trait EvalFor {
    fn eval_for(&mut self, statement: &Statement ) -> Result<( ), PseudoError>;
}

impl EvalFor for Evaluator {
    fn eval_for(&mut self, statement: &Statement ) -> Result<( ), PseudoError> {
        if let Statement::For(var, start, end, step, fstatement) = statement {
            let var = match var {
                Expr::Variable(val) => val.clone(),
                _ => return Err(PseudoError::VariableError("Expected a variable".to_string())),
            };
            let start = match start {
                Some(start) => self.evaluate_expr(start)?,
                None => self
                    .symbol_table
                    .get(&var)
                    .expect("Variable not found")
                    .clone(),
            };
            let end = self.evaluate_expr(end)?;
            let step = self.evaluate_expr(step)?;
            self.symbol_table.insert(var.clone(), start);
            while let Some(value) = self.symbol_table.get(&var) {
                if value <= &end {
                    for statement in fstatement.iter() {
                        self.eval_not_next_statement(statement)?;
                    }
                    let value = match (self.symbol_table.get(&var).unwrap(), &step) {
                        (EvalResult::Number(val), EvalResult::Number(step)) => {
                            EvalResult::Number(val + step)
                        }
                        _ => return Err(PseudoError::TypeError("Expected a number".to_string())),
                    };
                    self.symbol_table.insert(var.clone(), value);
                } else {
                    break;
                }
            }
        }
        self.next_statement();
        Ok(())
    }
}
