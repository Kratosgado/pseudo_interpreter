use crate::{
    constants::error_handler::PseudoError,
    evaluator::{
        eval_result::Operation,
        evaluator::MAX_STACK_BYTES,
        flow::Flow,
        scope::Scope,
    },
};

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
        } else if let Some(func) = self.function_args.get(name).cloned() {
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
            // Stop before the native stack runs out, so a missing base case is
            // an error the program can report rather than a hard crash.
            if self.stack_used() >= MAX_STACK_BYTES {
                return Err(PseudoError::RuntimeError(format!(
                    "recursion too deep calling '{}': exceeded {} KiB of stack after {} nested calls",
                    name,
                    MAX_STACK_BYTES / 1024,
                    self.call_depth()
                )));
            }

            // Evaluate the arguments in the caller's scope, before the callee's
            // frame exists, so they see the caller's variables.
            let mut frame = Scope::new();
            for (param, arg) in func.params.iter().zip(arg.iter()) {
                let value = self.evaluate_expr(arg)?;
                match param {
                    Expr::Param(param) => {
                        frame.vars.insert(param.clone(), value);
                    }
                    _ => {
                        return Err(PseudoError::VariableError(
                            "Expected a parameter name".to_string(),
                        ))
                    }
                }
            }

            self.push_frame(frame);
            let flow = self.evaluate_block(&func.statements);
            // Pop whether the body returned, fell through, or errored, so a
            // failed call never leaves its locals behind.
            self.pop_frame();

            match flow? {
                Flow::Return(value) => Ok(value),
                // Falling off the end of a function yields Null.
                Flow::Normal => Ok(EvalResult::Null),
            }
        } else {
            return Err(PseudoError::VariableError(format!(
                "function {} not defined",
                name
            )));
        }
    }
}
