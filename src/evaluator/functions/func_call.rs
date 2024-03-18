use super::{super::{EvalExpression, EvalResult, EvalStatement, Evaluator, Expr}, eval_statement::destruct_multi};

pub trait CallFunc {
    fn call_func(&mut self, name: &String, args: &Expr) -> EvalResult;
}

impl CallFunc for Evaluator {
    fn call_func(&mut self, name: &String, args: &Expr) -> EvalResult {
        if let Some(func) = self.function_args.get(name).cloned() {
            let args = destruct_multi(args);
            if func.params.len() != args.len() {
                panic!("function arguments not matching")
            }
            let mut iter = 0;
            while iter < args.len() {
                let var = &func.params[iter];
                let val = self.evaluate_expr(&args[iter]);
                if let Expr::Param(var) = var {
                    self.symbol_table.insert(var.clone(), val);
                    iter += 1;
                }
            }
            for statement in func.statements.iter() {
                self.eval_not_next_statement(statement);
            }
            match func.ret_ment {
                Some(expr) => self.evaluate_expr(&expr),
                None => EvalResult::Null,
            }
        } else {
            panic!("function {} not defined", name)
        }
    }
}
