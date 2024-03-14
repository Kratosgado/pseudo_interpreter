use crate::{evaluator::Statement, Evaluator};


pub trait EvalFunction {
    fn eval_function(&self, statement: &Statement);
}

impl EvalFunction for Evaluator {
    fn eval_function(&self, statement: &Statement) {
        todo!("function evaluator not implemented")
    }
}