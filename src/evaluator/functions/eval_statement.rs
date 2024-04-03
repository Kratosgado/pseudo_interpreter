use crate::{
    constants::error_handler::PseudoError,
    evaluator::{EvalDeclare, EvalFunction},
};

use super::super::{
    evaluator::Evaluator, EvalArray, EvalExpression, EvalFor, EvalIf, EvalResult, EvalWhile, Expr,
    Statement,
};
pub trait EvalStatement {
    fn eval_not_next_statement(&mut self, statement: &Statement, callnext: bool) -> Result<(), PseudoError>;
    fn evaluate_statement(&mut self, statement: &Statement) -> Result<(), PseudoError>;
}

impl EvalStatement for Evaluator {
    fn evaluate_statement(&mut self, statement: &Statement) -> Result<(), PseudoError> {
        match statement {
            Statement::Expr(expr) => {
                self.next_statement();
                self.evaluate_expr(expr)?;
                Ok(())
            }
            Statement::Print(expr) => Ok({
                let exprs: Vec<Expr> = destruct_multi(expr)?;
                for expr in &exprs {
                    print!("{}", self.evaluate_expr(&expr)?);
                }
                if exprs.len() == 1 {
                    println!()
                }
                self.next_statement();
            }),
            Statement::Assignment(var, expr) => Ok({
                self.next_statement();
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
            }),
            Statement::If(_) => self.eval_if(statement, true),
            Statement::While(_, _) => self.eval_while(statement),
            Statement::For(_, _, _, _, _) => self.eval_for(statement),
            Statement::Input(var) => {
                self.next_statement();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let mut value: EvalResult = EvalResult::Str(input.trim().to_string());
                if let Some(val) = self.symbol_table.get(var) {
                    if val.get_type() == "int" {
                        value = EvalResult::Number(
                            input
                                .trim()
                                .parse()
                                .expect("Invalid input: Expected integer"),
                        );
                    } else if val.get_type() == "double" {
                        value = EvalResult::Double(
                            input
                                .trim()
                                .parse()
                                .expect("Invalid input: Expected double"),
                        );
                    } else {
                        return Err(PseudoError::TypeError(format!(
                            "expected {}, found {}",
                            val.get_type(),
                            value.get_type()
                        )));
                    }
                }
                self.symbol_table.insert(var.clone(), value.clone());
                Ok(())
            }
            Statement::AssignArray(_, _, _)
            | Statement::AssignIndex(_, _, _)
            | Statement::DeclareArray(_, _) => self.eval_array(statement),
            Statement::Function(_, _, _, _) => self.eval_function(statement),
            Statement::PrintMulti(exprs) => {
                for expr in exprs {
                    print!("{}", self.evaluate_expr(expr)?)
                }
                self.next_statement();
                Ok(())
            }
            Statement::Declare(var, datatype) => self.eval_declare(var, datatype),
            Statement::None => Ok(self.next_statement()),
            Statement::Break => return Ok(()),
        }
    }

    fn eval_not_next_statement(&mut self, statement: &Statement, callnext: bool) -> Result<(), PseudoError> {
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
            }
            Statement::Assignment(var, expr) => {
                let value = self.evaluate_expr(expr)?;
                self.symbol_table.insert(var.clone(), value);
            }
            Statement::If(_) => self.eval_if(statement, callnext)?,
            Statement::While(_, _) => self.eval_while(statement)?,
            Statement::For(_, _, _, _, _) => self.eval_for(statement)?,
            Statement::Input(var) => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let value = EvalResult::Str(input.trim().to_string());
                self.symbol_table.insert(var.clone(), value);
            }
            Statement::AssignArray(_, _, _) => self.eval_array(statement)?,
            Statement::Function(_, _, _, _) => self.eval_function(statement)?,
            Statement::DeclareArray(_, _) => unimplemented!("Declare array not implemented"),
            Statement::AssignIndex(_, _, _) => unimplemented!("Assign index not implemented"),
            Statement::PrintMulti(exprs) => {
                for expr in exprs {
                    print!("{}", self.evaluate_expr(expr)?)
                }
            }
            Statement::Declare(_, _) => unimplemented!(),
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
