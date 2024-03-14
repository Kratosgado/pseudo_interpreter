use crate::evaluator::Expr;

use super::super::{EvalExpression, EvalResult, EvalStatement, Evaluator, Statement};
pub trait EvalFor {
    fn eval_for(&mut self, statement: &Statement);
}

impl EvalFor for Evaluator {
    fn eval_for(&mut self, statement: &Statement) {
        if let Statement::For(var, start, end, step, fstatement) = statement {
            let var = match var {
                Expr::Variable(val) => val.clone(),
                _ => panic!("Expected variable"),
            };
            let start = match start {
                Some(start) => self.evaluate_expr(start),
                None => self.symbol_table.get(&var).unwrap().clone(),
            };
            // let start = self.evaluate_expr(start);
            let end = self.evaluate_expr(end);
            let step = self.evaluate_expr(step);
            self.symbol_table.insert(var.clone(), start);
            while let Some(value) = self.symbol_table.get(&var) {
                if value <= &end {
                    for statement in fstatement.iter() {
                        self.evaluate_statement(statement);
                    }
                    let value = match (self.symbol_table.get(&var).unwrap(), &step) {
                        (EvalResult::Number(val), EvalResult::Number(step)) => {
                            EvalResult::Number(val + step)
                        }
                        _ => panic!("Expected number"),
                    };
                    self.symbol_table.insert(var.clone(), value);
                } else {
                    break;
                }
            }
        }
    }
}
