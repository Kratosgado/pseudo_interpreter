
use crate::evaluator::EvalExpression;

use super::super::{Evaluator, Statement};
pub trait EvalArray {
    fn eval_array(&mut self, statement: &Statement);
}

impl EvalArray for Evaluator {
    fn eval_array(&mut self, statement: &Statement) {
        self.next_statement();
        if let Statement::AssignArray(var , _ , values) = statement  {
            // let size = self.evaluate_expr(size);
            
            let mut arr_values = Vec::new();
            for value in values.iter() {
                arr_values.push(self.evaluate_expr(value))
            }
            self.array_table.insert(var.clone(), arr_values);
        }
    }
}