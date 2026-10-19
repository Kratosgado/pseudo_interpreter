use crate::{
    constants::error_handler::PseudoError,
    evaluator::{flow::Flow, EvalStatement, Statement},
    Evaluator,
};

use super::super::{EvalExpression, EvalResult};

pub trait EvalWhile {
    fn eval_while(&mut self, statement: &Statement) -> Result<Flow, PseudoError>;
}

impl EvalWhile for Evaluator {
    fn eval_while(&mut self, statement: &Statement) -> Result<Flow, PseudoError> {
        if let Statement::While(condition, wstatements) = statement {
            while let Ok(EvalResult::Boolean(true)) = self.evaluate_expr(condition) {
                // A `return` in the body leaves the loop and the call at once.
                if let Flow::Return(value) = self.evaluate_block(wstatements)? {
                    return Ok(Flow::Return(value));
                }
            }
        }
        Ok(Flow::Normal)
    }
}
