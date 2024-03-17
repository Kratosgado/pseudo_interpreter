use crate::evaluator::EvalFunction;

use super::super::{
    evaluator::Evaluator, EvalArray, EvalExpression, EvalFor, EvalIf, EvalResult, EvalWhile,
    Statement,
};
pub trait EvalStatement {
    fn eval_not_next_statement(&mut self, statement: &Statement);
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
            }
            Statement::AssignArray(_, _, _) => self.eval_array(statement),
            Statement::Function(_, _, _, _) => self.eval_function(statement),
            Statement::DeclareArray(_, _) => unimplemented!("Declare array not implemented"),
            Statement::AssignIndex(_, _, _) => unimplemented!("Assign index not implemented"),
        }
    }

    fn eval_not_next_statement(&mut self, statement: &Statement) {
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
            Statement::If(_, _, _) => self.eval_if(statement),
            Statement::While(_, _) => self.eval_while(statement),
            Statement::For(_, _, _, _, _) => self.eval_for(statement),
            Statement::Input(var) => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let value = EvalResult::Str(input.trim().to_string());
                self.symbol_table.insert(var.clone(), value);
            }
            Statement::AssignArray(_, _, _) => self.eval_array(statement),
            Statement::Function(_, _, _, _) => self.eval_function(statement),
            Statement::DeclareArray(_, _) => unimplemented!("Declare array not implemented"),
            Statement::AssignIndex(_, _, _) => unimplemented!("Assign index not implemented"),
        }
    }
}
