use super::super::{EvalResult, Evaluator};
pub trait EvalDeclare {
    fn eval_declare(&mut self, var: &String, datatype: &String);
}

impl EvalDeclare for Evaluator {
    fn eval_declare(&mut self, var: &String, datatype: &String) {
        let datatype = match datatype.as_str() {
            "int" => EvalResult::Number(0),
            "str" => EvalResult::Str("".to_string()),
            "bool" => EvalResult::Boolean(false),
            "double" => EvalResult::Double(0.0),
            _ => panic!("Invalid datatype"),
        };
        self.symbol_table.insert(var.clone(), datatype);
        self.next_statement();
    }
}
