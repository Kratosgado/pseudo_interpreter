use crate::{
    constants::error_handler::PseudoError, evaluator::{EvalStatement, Statement}, Evaluator
};

use super::super::{EvalExpression, EvalResult};

pub trait EvalWhile {
    fn eval_while(&mut self, statement: &Statement) -> Result<(), PseudoError>;
}

impl EvalWhile for Evaluator {
    fn eval_while(&mut self, statement: &Statement) -> Result<(), PseudoError> {
        Ok(if let Statement::While(condition, wstatements) = statement {
            // let mut condition = self.evaluate_expr(condition);
            while let Ok(EvalResult::Boolean(true)) = self.evaluate_expr(condition) {
                for statement in wstatements.iter() {
                    self.eval_not_next_statement(statement)?;
                }
            }
            self.next_statement();
        })
    }

   
}
