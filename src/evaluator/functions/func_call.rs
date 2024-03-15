use super::super::{EvalResult, Evaluator, EvalExpression, EvalStatement, Expr};

pub trait CallFunc {
    fn call_func(&mut self, name: &String, args: &Vec<Expr>) -> EvalResult;
}

impl CallFunc for Evaluator {
    fn call_func(&mut self, name: &String, args: &Vec<Expr>) -> EvalResult {
        if let Some(func) = self.function_args.get(name) {
            if func.params.len() != args.len() {
                panic!("function arguments not matching")
            }
            let mut p: Vec<EvalResult> = Vec::new();
            for param in args.iter() {
                p.push(self.evaluate_expr(param))
            }
            for statement in func.statements.iter() {
                self.evaluate_statement(statement);
            }
            func.ret_ment.clone();
            // func(self, args, &func.statements, &func.ret_ment);

            todo!("call function not completed")
        } else {
            panic!("function {} not defined", name)
        }
    }
}
