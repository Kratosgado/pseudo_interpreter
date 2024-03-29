use crate::constants::error_handler::PseudoError;

use super::super::{evaluator::Evaluator, EvalResult, EvalStatement, EvalExpression, Statement};

pub trait EvalIf {
    fn eval_if(&mut self, statement: &Statement) -> Result<(), PseudoError>;
}

impl EvalIf for Evaluator {
    fn eval_if(&mut self, statement: &Statement) -> Result<(), PseudoError> {
        Ok(if let Statement::If(ifstructure) = statement {
            let first_if = &ifstructure.ifcond;
            let cond = self.evaluate_expr(&first_if.cond)?;
            if let EvalResult::Boolean(true) = cond {
                for statement in first_if.consequence.iter() {
                    self.evaluate_statement(statement)?;
                }
            } else {
                if let Some(elseifs) = &ifstructure.elseifs {
                    for elseif in elseifs.iter() {
                        if EvalResult::Boolean(true) == self.evaluate_expr(&elseif.cond)? {
                            for statement  in elseif.consequence.iter() {
                                self.evaluate_statement(statement)?;
                            }
                            return Ok(());
                        }
                    }
                }
                if let Some(alternative) = &ifstructure.alternative{
                    for statement in alternative.iter() {
                        self.evaluate_statement(statement)?;
                    }
                }
            }
        })
    }
}
