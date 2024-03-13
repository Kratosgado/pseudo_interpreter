use super::{eval_result::EvalResult, EvalStatement, Statement};

use std::collections::HashMap;

pub struct Evaluator {
    statements: Vec<Statement>,
    pub current_statement: Option<Statement>,
    position: usize,
    pub symbol_table: HashMap<String, EvalResult>,
}

impl Evaluator {
    pub fn new(statements: Vec<Statement>) -> Self {
        let mut evaluator = Evaluator {
            statements,
            current_statement: None,
            position: 0,
            symbol_table: HashMap::new(),
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
