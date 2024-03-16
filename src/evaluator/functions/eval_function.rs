use crate::{
    evaluator::{evaluator::FuncArgs, Statement},
    Evaluator,
};

pub trait EvalFunction {
    fn eval_function(&mut self, statement: &Statement);
}

impl EvalFunction for Evaluator {
    fn eval_function(&mut self, statement: &Statement) {
        if let Statement::Function(name, params, fstatements, ret_ment) = statement {
            let func_args = FuncArgs {
                params: params.clone(),
                statements: fstatements.clone(),
                ret_ment: ret_ment.clone(),
            };
            self.function_args.insert(name.clone(), func_args);
            self.next_statement();
            // todo!("function evaluator not finished")
        }
    }
}
