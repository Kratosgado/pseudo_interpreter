use crate::{constants::error_handler::PseudoError, evaluator::eval_result::Operation};

use super::{
    super::{EvalExpression, EvalResult, EvalStatement, Evaluator, Expr},
    eval_statement::destruct_multi,
};

pub trait CallFunc {
    fn call_func(&mut self, name: &String, args: &Option<Expr>) -> Result<EvalResult, PseudoError>;
}

impl CallFunc for Evaluator {
    fn call_func(&mut self, name: &String, args: &Option<Expr>) -> Result<EvalResult, PseudoError> {
        if vec!["pow", "power", "exp"].contains(&name.to_lowercase().as_str()) {
            let mut arg = vec![];
            match args {
                Some(p) => {
                    arg = destruct_multi(p)?;
                }
                None => {}
            };
            if arg.len() != 2 {
                return Err(PseudoError::VariableError(format!(
                    "expected {} arguments, found {}",
                    2,
                    arg.len()
                )));
            }
            let power = self.evaluate_expr(&arg.pop().unwrap())?;
            let base = self.evaluate_expr(&arg.pop().unwrap())?;
            Ok(base.power(&power)?)
        }else if let Some(func) = self.function_args.get(name).cloned() {
            let mut arg = vec![];
            match args {
                Some(p) => {
                    arg = destruct_multi(p)?;
                }
                None => {}
            };
            if func.params.len() != arg.len() {
                return Err(PseudoError::VariableError(format!(
                    "expected {} arguments, found {}",
                    func.params.len(),
                    arg.len()
                )));
            }

            let mut iter = 0;
            while iter < arg.len() {
                let var = &func.params[iter];
                let val = self.evaluate_expr(&arg[iter])?;
                if let Expr::Param(var) = var {
                    self.symbol_table.insert(var.clone(), val);
                    iter += 1;
                }
            }
            for statement in func.statements.iter() {
                self.evaluate_statement(statement, false)?;
            }
            match func.ret_ment {
                Some(expr) => self.evaluate_expr(&expr),
                None => Ok(EvalResult::Null),
            }
        } else {
            return Err(PseudoError::VariableError(format!(
                "function {} not defined",
                name
            )));
        }
    }
}
