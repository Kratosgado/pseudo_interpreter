use crate::{
    constants::error_handler::PseudoError,
    evaluator::{evaluator::FuncArgs, Statement},
    Evaluator,
};

pub trait EvalFunction {
    fn eval_function(&mut self, statement: &Statement) -> Result<(), PseudoError>;
}

impl EvalFunction for Evaluator {
    fn eval_function(&mut self, statement: &Statement) -> Result<(), PseudoError> {
        if let Statement::Function(name, params, fstatements, ret_ment) = statement {
            let func_args = FuncArgs {
                params: params.clone(),
                statements: fstatements.clone(),
                ret_ment: ret_ment.clone(),
            };
            self.function_args.insert(name.clone(), func_args);
        }
        Ok(())
    }
}
