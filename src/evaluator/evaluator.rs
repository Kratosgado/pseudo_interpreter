use super::{eval_result::EvalResult, EvalStatement, Expr, Statement};

use std::collections::HashMap;

pub type IFunc = dyn FnMut(&mut Evaluator, &Vec<Expr>, &Box<Vec<Statement>>,  &Option<Expr>) -> Option<Expr>;
pub struct FuncArgs {
    pub params: Vec<Expr>, 
    pub statements: Box<Vec<Statement>>,
    pub ret_ment:  Option<Expr>
}
pub struct Evaluator {
    statements: Vec<Statement>,
    pub current_statement: Option<Statement>,
    position: usize,
    pub symbol_table: HashMap<String, EvalResult>,
    pub array_table: HashMap<String, Vec<EvalResult>>,
    pub function_table: HashMap<String, Box<IFunc>>,
    pub function_args: HashMap<String, FuncArgs>
}

impl Evaluator {
    pub fn new(statements: Vec<Statement>) -> Self {
        let mut evaluator = Evaluator {
            statements,
            current_statement: None,
            position: 0,
            symbol_table: HashMap::new(),
            array_table: HashMap::new(),
            function_table: HashMap::new(),
            function_args: HashMap::new(),
        };
        evaluator.next_statement();
        evaluator
    }

    pub fn next_statement(&mut self) {
        if self.position < self.statements.len() {
            self.current_statement = Some(self.statements[self.position].clone());
            self.position += 1;
        } else {
            self.current_statement = None;
        }
    }

    pub fn evaluate(&mut self) {
        while let Some(statement) = self.current_statement.take() {
            self.evaluate_statement(&statement);
        }
    }
}
