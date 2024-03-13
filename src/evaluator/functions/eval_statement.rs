use super::{
    super::{evaluator::Evaluator, EvalExpression,EvalFor, EvalIf, EvalWhile, EvalResult, Statement},
    
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
            Statement::Input(var) => {
                self.next_statement();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let value = EvalResult::Str(input.trim().to_string());
                self.symbol_table.insert(var.clone(), value);
            },
        }
    }
}
