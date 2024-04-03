use crate::{
    constants::error_handler::PseudoError,
    evaluator::{eval_result::Operation, Expr},
};

use super::super::{EvalExpression, EvalStatement, Evaluator, Statement};
pub trait EvalFor {
    fn eval_for(&mut self, statement: &Statement) -> Result<(), PseudoError>;
}

impl EvalFor for Evaluator {
    fn eval_for(&mut self, statement: &Statement) -> Result<(), PseudoError> {
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
                    .symbol_table
                    .get(&var)
                    .expect("Variable not found")
                    .clone(),
            };
            let end = self.evaluate_expr(end)?;
            let step = self.evaluate_expr(step)?;
            self.symbol_table.insert(var.clone(), start);
            while let Some(value) = self.symbol_table.get(&var) {
                if value != &end {
                    for statement in fstatement.iter() {
                        self.evaluate_statement(statement, false)?;
                    }
                    let value = self.symbol_table.get(&var).unwrap();
                    let value = value.add(&step)?;

                    self.symbol_table.insert(var.clone(), value);
                } else {
                    for statement in fstatement.iter() {
                        self.evaluate_statement(statement, false)?;
                    }
                    break;
                }
            }
        }
        Ok(())
    }
}
