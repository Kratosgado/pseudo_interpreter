use super::super::{evaluator::Evaluator, EvalResult, Expression, Statement};

pub trait EvalIf {
    fn eval_if(&mut self, statement: &Statement);
    fn evaluate_statement(&mut self, statement: &Statement);
}

impl EvalIf for Evaluator {
    fn eval_if(&mut self, statement: &Statement) {
        if let Statement::IfStatement(condition, consequence, alternative) = statement {
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

    fn evaluate_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Expr(expr) => {
                self.evaluate_expr(expr);
            }
            Statement::Print(expr) => {
                let value = self.evaluate_expr(expr);
                println!("{}", value);
            }
            Statement::Assignment(var, expr) => {
                let value = self.evaluate_expr(expr);
                self.symbol_table.insert(var.clone(), value.clone());
            }
            Statement::IfStatement(_, _, _) => self.eval_if(statement),
        }
    }
}
