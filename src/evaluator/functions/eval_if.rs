use super::super::{evaluator::Evaluator, EvalResult, Expression, Statement, EvalStatement};

pub trait EvalIf {
    fn eval_if(&mut self, statement: &Statement);
}

impl EvalIf for Evaluator {
    fn eval_if(&mut self, statement: &Statement) {
        if let Statement::If(condition, consequence, alternative) = statement {
            let condition = self.evaluate_expr(condition);
            if let EvalResult::Boolean(true) = condition {
                println!("consequences: {:?}", consequence);
                for statement in consequence.iter() {
                    self.evaluate_statement(statement);
                }
            } else {
                println!("alternative: {:?}", alternative);
                if let Some(alternative) = alternative {
                    for statement in alternative.iter() {
                        self.evaluate_statement(statement);
                    }
                }
            }
        }
    }

}
