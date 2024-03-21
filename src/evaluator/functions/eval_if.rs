use crate::constants::error_handler::PseudoError;

use super::super::{evaluator::Evaluator, EvalResult, EvalStatement, EvalExpression, Statement};

pub trait EvalIf {
    fn eval_if(&mut self, statement: &Statement) -> Result<(), PseudoError>;
}

impl EvalIf for Evaluator {
    fn eval_if(&mut self, statement: &Statement) -> Result<(), PseudoError> {
        Ok(if let Statement::If(condition, consequence, alternative) = statement {
            let condition = self.evaluate_expr(condition)?;
            if let EvalResult::Boolean(true) = condition {
                for statement in consequence.iter() {
                    self.evaluate_statement(statement)?;
                }
            } else {
                if let Some(alternative) = alternative {
                    for statement in alternative.iter() {
                        self.evaluate_statement(statement)?;
                    }
                }
            }
        })
    }
}
