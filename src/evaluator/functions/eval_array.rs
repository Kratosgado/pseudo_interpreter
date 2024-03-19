use crate::evaluator::{EvalExpression, EvalResult};

use super::{super::{Evaluator, Statement}, eval_statement::destruct_multi};
pub trait EvalArray {
    fn eval_array(&mut self, statement: &Statement);
}

impl EvalArray for Evaluator {
    fn eval_array(&mut self, statement: &Statement) {
        match statement {
            Statement::AssignArray(var, _, values) => {
                self.next_statement();

                // let size = self.evaluate_expr(size);

                let values = destruct_multi(values);
                let mut arr_values = Vec::new();
                for value in values.iter() {
                    arr_values.push(self.evaluate_expr(value))
                }
                self.array_table.insert(var.clone(), arr_values);
            }
            Statement::DeclareArray(var, size) => {
                self.next_statement();

                let size = self.evaluate_expr(size);
                if let EvalResult::Number(size) = size {
                    self.array_table
                        .insert(var.clone(), vec![EvalResult::Null; size as usize]);
                } else {
                    panic!("Invalid size")
                }
            }
            Statement::AssignIndex(var, index, value) => {
                let index = self.evaluate_expr(index);
                let value = self.evaluate_expr(value);
                if let Some(arr) = self.array_table.get_mut(var) {
                    if let EvalResult::Number(index) = index {
                        arr[index as usize] = value;
                    } else {
                        panic!("Invalid index")
                    }
                } else {
                    panic!("Array not found")
                }
            }
            _ => (),
        }
    }
}
