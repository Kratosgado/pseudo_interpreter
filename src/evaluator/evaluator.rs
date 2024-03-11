use super::eval_result::EvalResult;
use crate::lexer::expr::Expr;
use crate::lexer::operator::Operator;
use crate::lexer::statement::Statement;

use std::collections::HashMap;

pub struct Evaluator {
    statements: Vec<Statement>,
    current_statement: Option<Statement>,
    position: usize,
    symbol_table: HashMap<String, EvalResult>,
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

    fn next_statement(&mut self) {
        if self.position < self.statements.len() {
            self.current_statement = Some(self.statements[self.position].clone());
            self.position += 1;
        } else {
            self.current_statement = None;
        }
    }

    pub fn evaluate(&mut self) {
        while let Some(statement) = &self.current_statement {
            match statement {
                Statement::Expr(expr) => {
                    self.evaluate_expr(expr);
                    self.next_statement();
                }
                Statement::Print(expr) => {
                    let value = self.evaluate_expr(expr);
                    println!("{}", value);
                    self.next_statement();
                }
                Statement::Assignment(var, expr) => {
                    let value = self.evaluate_expr(expr);
                    self.symbol_table.insert(var.clone(), value.clone());
                    self.next_statement();
                }
            }
        }
    }

    /// Evaluates an expression and returns the result.
    ///
    /// # Panics
    ///
    /// Panics if performing an operation on a non-number.
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    fn evaluate_expr(&self, expr: &Expr) -> EvalResult {
        match expr {
            Expr::Number(num) => EvalResult::Number(*num),
            Expr::Str(value) => EvalResult::Str(value.clone()),
            Expr::Variable(var) => {
                if let Some(value) = self.symbol_table.get(var) {
                    value.clone()
                } else {
                    panic!("undefined variable: {}", var)
                }
            }
            Expr::BinOp(left, op, right) => self.arithmetic_expr(left, op, right),
            Expr::Boolean(val) => EvalResult::Boolean(*val),
            Expr::Comparison(left, op, right  ) => self.evaluate_comparison(left, op, right),
        }
    }

    fn arithmetic_expr(&self, left: &Expr, op: &Operator, right: &Expr) -> EvalResult {
        let left_val = match self.evaluate_expr(left) {
            EvalResult::Number(val) => val,
            _ => panic!("Expected number"),
        };
        let right_val = match self.evaluate_expr(right) {
            EvalResult::Number(val) => val,
            _ => panic!("Expected a number"),
        };
        let result = match op {
            Operator::Add => left_val + right_val,
            Operator::Subtract => left_val - right_val,
            Operator::Multiply => left_val * right_val,
            Operator::Divide => left_val / right_val,
            Operator::Modulo => left_val % right_val,
            _ => panic!("Invalid arithmetic operator"),

        };
        EvalResult::Number(result)
    }
    fn evaluate_comparison(&self, left: &Expr, op: &Operator, right: &Expr) -> EvalResult {
        let left_val = match self.evaluate_expr(left) {
            EvalResult::Number(val) => val,
            _ => panic!("Expected number"),
        };
        let right_val = match self.evaluate_expr(right) {
            EvalResult::Number(val) => val,
            _ => panic!("Expected a number"),
        };
        let result = match op {
            Operator::Equal => left_val == right_val,
            Operator::LessThan => left_val < right_val,
            Operator::GreaterThan => left_val > right_val,
            Operator::LessThanEqual => left_val <= right_val,
            Operator::GreaterThanEqual => left_val >= right_val,
            Operator::NotEqual => left_val != right_val,
            _ => panic!("Invalid comparison operator"),
        };
        EvalResult::Boolean(result)
    }
}
