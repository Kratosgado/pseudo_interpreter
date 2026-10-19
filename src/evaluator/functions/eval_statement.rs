use crate::{
    constants::error_handler::PseudoError,
    evaluator::{flow::Flow, EvalDeclare, EvalFunction},
};

use super::super::{
    evaluator::Evaluator, EvalArray, EvalExpression, EvalFor, EvalIf, EvalResult, EvalWhile, Expr,
    Statement,
};
pub trait EvalStatement {
    /// Evaluate one statement. The returned [`Flow`] tells the caller whether
    /// to keep going or to leave the current function call.
    fn evaluate_statement(
        &mut self,
        statement: &Statement,
        callnext: bool,
    ) -> Result<Flow, PseudoError>;

    /// Evaluate a block, stopping at the first statement that returns.
    fn evaluate_block(&mut self, statements: &[Statement]) -> Result<Flow, PseudoError>;
}

impl EvalStatement for Evaluator {
    fn evaluate_statement(
        &mut self,
        statement: &Statement,
        callnext: bool,
    ) -> Result<Flow, PseudoError> {
        let mut flow = Flow::Normal;
        match statement {
            Statement::Expr(expr) => {
                self.evaluate_expr(expr)?;
            }
            Statement::Print(expr) => {
                let exprs: Vec<Expr> = destruct_multi(expr)?;
                for expr in &exprs {
                    let value = self.evaluate_expr(&expr)?;
                    write!(self.out(), "{}", value)?;
                }
                if exprs.len() == 1 {
                    writeln!(self.out())?;
                }
            }
            Statement::Assignment(var, expr) => {
                let value = self.evaluate_expr(expr)?;
                // Only compare against a binding in the scope we are writing
                // to; a local is free to shadow a global of another type.
                if let Some(val) = self.local_var(var) {
                    if val.get_type() != value.get_type() {
                        return Err(PseudoError::TypeError(format!(
                            "expected {}, found {}",
                            val.get_type(),
                            value.get_type()
                        )));
                    }
                }
                self.set_var(var.clone(), value);
            }
            Statement::If(_) => flow = self.eval_if(statement, false)?,
            Statement::While(_, _) => flow = self.eval_while(statement)?,
            Statement::For(_, _, _, _, _) => flow = self.eval_for(statement)?,
            Statement::Return(expr) => {
                if !self.in_function() {
                    return Err(PseudoError::StatementError(
                        "return outside of a function".to_string(),
                    ));
                }
                let value = match expr {
                    Some(expr) => self.evaluate_expr(expr)?,
                    None => EvalResult::Null,
                };
                // Hand the value up; every enclosing block passes it along
                // until `call_func` consumes it.
                return Ok(Flow::Return(value));
            }
            Statement::Input(var) => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let mut value: EvalResult = EvalResult::Str(input.trim().to_string());
                if let Some(val) = self.get_var(var) {
                    match val.get_type().as_str() {
                        "int" => {
                            value = EvalResult::Number(
                                input
                                    .trim()
                                    .parse()
                                    .map_err(|_| PseudoError::TypeError("expected integer".to_string()))?,
                            );
                        }
                        "double" => {
                            value = EvalResult::Double(
                                input
                                    .trim()
                                    .parse()
                                    .map_err(|_| PseudoError::TypeError("expected double".to_string()))?,
                            );
                        }
                        "bool" => {
                            value = EvalResult::Boolean(
                                input
                                    .trim()
                                    .parse()
                                    .map_err(|_| PseudoError::TypeError("expected boolean".to_string()))?,
                            );
                        }
                        "str" => {}
                        _ => return Err(PseudoError::TypeError(format!(
                            "expected {}, found {}",
                            val.get_type(),
                            value.get_type()
                        ))),
                    }
                    }

                self.set_var(var.clone(), value.clone());
            }
            Statement::AssignArray(_, _, _)
            | Statement::AssignIndex(_, _, _)
            | Statement::DeclareArray(_, _) => self.eval_array(statement)?,
            Statement::Function(_, _, _) => self.eval_function(statement)?,
            Statement::PrintMulti(exprs) => {
                for expr in exprs {
                    let value = self.evaluate_expr(expr)?;
                    write!(self.out(), "{}", value)?;
                }
            }
            Statement::Declare(var, datatype) => self.eval_declare(var, datatype)?,
            Statement::None => self.next_statement(),
            Statement::Break => return Ok(Flow::Normal),
        }
        if callnext {
            self.next_statement();
        }
        Ok(flow)
    }

    fn evaluate_block(&mut self, statements: &[Statement]) -> Result<Flow, PseudoError> {
        for statement in statements {
            if let Flow::Return(value) = self.evaluate_statement(statement, false)? {
                return Ok(Flow::Return(value));
            }
        }
        Ok(Flow::Normal)
    }
}

/// recursive function to evaluate multi expressions
pub fn destruct_multi(expr: &Expr) -> Result<Vec<Expr>, PseudoError> {
    match expr {
        Expr::Multi(exprs) => {
            let mut result = Vec::new();
            for expr in exprs {
                result.append(&mut destruct_multi(expr)?);
            }
            Ok(result)
        }
        _ => Ok(vec![expr.clone()]),
    }
}
