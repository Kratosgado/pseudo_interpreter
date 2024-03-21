use crate::{
    evaluator::{EvalStatement, Statement},
    Evaluator,
};

use super::super::{EvalExpression, EvalResult};

pub trait EvalWhile {
    fn eval_while(&mut self, statement: &Statement);
}

impl EvalWhile for Evaluator {
    fn eval_while(&mut self, statement: &Statement) {
        if let Statement::While(condition, wstatements) = statement {
            // let mut condition = self.evaluate_expr(condition);
            while let EvalResult::Boolean(true) = self.evaluate_expr(condition) {
                for statement in wstatements.iter() {
                    self.eval_not_next_statement(statement);
                }
            }
            self.next_statement();
        }
    }

   
}
