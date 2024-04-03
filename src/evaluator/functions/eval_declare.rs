use crate::constants::error_handler::PseudoError;

use super::super::{EvalResult, Evaluator};
pub trait EvalDeclare {
    fn eval_declare(&mut self, var: &String, datatype: &String) -> Result<(), PseudoError>;
}

impl EvalDeclare for Evaluator {
    fn eval_declare(&mut self, var: &String, datatype: &String)-> Result<(), PseudoError> {
        let datatype = match datatype.as_str() {
            "int" => EvalResult::Number(0),
            "str" => EvalResult::Str("".to_string()),
            "bool" => EvalResult::Boolean(false),
            "double" => EvalResult::Double(0.0),
            _ => return Err(PseudoError::TypeError("Invalid datatype".to_string())),
        };
        self.symbol_table.insert(var.clone(), datatype);
        Ok(())
    }
}
