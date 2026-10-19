use crate::{constants::error_handler::PseudoError, evaluator::flow::Flow};

use super::super::{evaluator::Evaluator, EvalExpression, EvalResult, EvalStatement, Statement};

pub trait EvalIf {
    fn eval_if(&mut self, statement: &Statement, callnext: bool) -> Result<Flow, PseudoError>;
}

impl EvalIf for Evaluator {
    fn eval_if(&mut self, statement: &Statement, callnext: bool) -> Result<Flow, PseudoError> {
        if let Statement::If(ifstructure) = statement {
            let first_if = &ifstructure.ifcond;
            let cond = self.evaluate_expr(&first_if.cond)?;
            if EvalResult::Boolean(true) == cond || EvalResult::Number(1) == cond {
                for statement in first_if.consequence.iter() {
                    if let Flow::Return(value) = self.evaluate_statement(statement, callnext)? {
                        return Ok(Flow::Return(value));
                    }
                }
            } else {
                if let Some(elseifs) = &ifstructure.elseifs {
                    for elseif in elseifs.iter() {
                        let cond = self.evaluate_expr(&elseif.cond)?;
                        if EvalResult::Boolean(true) == cond || EvalResult::Number(1) == cond {
                            for statement in elseif.consequence.iter() {
                                if let Flow::Return(value) =
                                    self.evaluate_statement(statement, callnext)?
                                {
                                    return Ok(Flow::Return(value));
                                }
                            }
                            return Ok(Flow::Normal);
                        }
                    }
                }
                if let Some(alternative) = &ifstructure.alternative {
                    for statement in alternative.iter() {
                        if let Flow::Return(value) = self.evaluate_statement(statement, callnext)? {
                            return Ok(Flow::Return(value));
                        }
                    }
                }
            }
        }
        Ok(Flow::Normal)
    }
}
