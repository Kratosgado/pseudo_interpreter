
use super::super::{evaluator::Evaluator, EvalResult, Expression, Statement, EvalIf, EvalWhile};
pub trait EvalStatement {
    fn evaluate_statement(&mut self, statement: &Statement);
}

impl EvalStatement for Evaluator {
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
                self.symbol_table.insert(var.clone(), value);
            }
            Statement::IfStatement(_, _, _) => self.eval_if(statement),
            Statement::While(_, _) => self.eval_while(statement),
        }
    }
    
}