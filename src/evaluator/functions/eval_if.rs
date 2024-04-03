use crate::constants::error_handler::PseudoError;

use super::super::{evaluator::Evaluator, EvalExpression, EvalResult, EvalStatement, Statement};

pub trait EvalIf {
    fn eval_if(&mut self, statement: &Statement, callnext: bool) -> Result<(), PseudoError>;
}

impl EvalIf for Evaluator {
    fn eval_if(&mut self, statement: &Statement, callnext: bool) -> Result<(), PseudoError> {
        if let Statement::If(ifstructure) = statement {
            let first_if = &ifstructure.ifcond;
            let cond = self.evaluate_expr(&first_if.cond)?;
            if EvalResult::Boolean(true) == cond || EvalResult::Number(1) == cond {
                for statement in first_if.consequence.iter() {
                    self.evaluate_statement(statement, callnext)?;
                }
            } else {
                if let Some(elseifs) = &ifstructure.elseifs {
                    for elseif in elseifs.iter() {
                        let cond = self.evaluate_expr(&elseif.cond)?;
                        if EvalResult::Boolean(true) == cond || EvalResult::Number(1) == cond {
                            for statement in elseif.consequence.iter() {
                                self.evaluate_statement(statement, callnext)?;
                            }
                            return Ok(());
                        }
                    }
                }
                if let Some(alternative) = &ifstructure.alternative {
                    for statement in alternative.iter() {
                        self.evaluate_statement(statement, callnext)?;
                    }
                }
            }
        }
        Ok(())
    }
}
