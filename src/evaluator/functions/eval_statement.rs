use crate::{
    constants::error_handler::PseudoError,
    evaluator::{EvalDeclare, EvalFunction},
};

use super::super::{
    evaluator::Evaluator, EvalArray, EvalExpression, EvalFor, EvalIf, EvalResult, EvalWhile, Expr,
    Statement,
};
pub trait EvalStatement {
    fn evaluate_statement(&mut self, statement: &Statement, callnext: bool) -> Result<(), PseudoError>;
}

impl EvalStatement for Evaluator {
    fn evaluate_statement(&mut self, statement: &Statement, callnext: bool) -> Result<(), PseudoError> {
        match statement {
            Statement::Expr(expr) => {
                self.evaluate_expr(expr)?;
            }
            Statement::Print(expr) => {
                let exprs: Vec<Expr> = destruct_multi(expr)?;
                for expr in &exprs {
                    print!("{}", self.evaluate_expr(&expr)?);
                }
                if exprs.len() == 1 {
                    println!()
                }
            },
            Statement::Assignment(var, expr) => {
                let value = self.evaluate_expr(expr)?;
                if let Some(val) = self.symbol_table.get(var) {
                    if val.get_type() != value.get_type() {
                        return Err(PseudoError::TypeError(format!(
                            "expected {}, found {}",
                            val.get_type(),
                            value.get_type()
                        )));
                    }
                }
                self.symbol_table.insert(var.clone(), value);
            },
            Statement::If(_) => self.eval_if(statement, false)?,
            Statement::While(_, _) => self.eval_while(statement)?,
            Statement::For(_, _, _, _, _) => self.eval_for(statement)?,
            Statement::Input(var) => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let mut value: EvalResult = EvalResult::Str(input.trim().to_string());
                if let Some(val) = self.symbol_table.get(var) {
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
                    
                self.symbol_table.insert(var.clone(), value.clone());
            }
            Statement::AssignArray(_, _, _)
            | Statement::AssignIndex(_, _, _)
            | Statement::DeclareArray(_, _) => self.eval_array(statement)?,
            Statement::Function(_, _, _, _) => self.eval_function(statement)?,
            Statement::PrintMulti(exprs) => {
                for expr in exprs {
                    print!("{}", self.evaluate_expr(expr)?)
                }
            }
            Statement::Declare(var, datatype) => self.eval_declare(var, datatype)?,
            Statement::None => self.next_statement(),
            Statement::Break => return Ok(()),
        }
        if callnext {
            self.next_statement();
        }
        Ok(())
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
