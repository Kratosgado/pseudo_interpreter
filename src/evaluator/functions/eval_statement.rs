use super::{
    super::{evaluator::Evaluator, EvalIf, EvalWhile, EvalExpression, Statement},
    eval_for::EvalFor,
};
pub trait EvalStatement {
    fn evaluate_statement(&mut self, statement: &Statement);
}

impl EvalStatement for Evaluator {
    fn evaluate_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Expr(expr) => {
                self.next_statement();
                self.evaluate_expr(expr);
            }
            Statement::Print(expr) => {
                self.next_statement();
                let value = self.evaluate_expr(expr);
                println!("{}", value);
            }
            Statement::Assignment(var, expr) => {
                self.next_statement();
                let value = self.evaluate_expr(expr);
                self.symbol_table.insert(var.clone(), value);
            }
            Statement::If(_, _, _) => self.eval_if(statement),
            Statement::While(_, _) => self.eval_while(statement),
            Statement::For(_, _, _, _, _) => self.eval_for(statement),
        }
    }
}
