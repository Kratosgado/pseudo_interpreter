use crate::lexer::expr::Expr;
use crate::lexer::operator::Operator;
use crate::lexer::statement::Statement;

pub struct Evaluator {
    statements: Vec<Statement>,
    current_statement: Option<Statement>,
    position: usize,
}

impl Evaluator {
    pub fn new(statements: Vec<Statement>) -> Self {
        let mut evaluator = Evaluator {
            statements,
            current_statement: None,
            position: 0,
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
                    let result = self.evaluate_expr(expr);
                    self.next_statement();
                }
                Statement::Print(expr) => {
                    let result = self.evaluate_expr(expr);
                    println!("{}", result);
                    self.next_statement();
                }
                Statement::Assignment(_, _) => todo!(),
            }
        }
    }

    fn evaluate_expr(&self, expr: &Expr) -> i64 {
        match expr {
            Expr::Number(num) => *num,
            Expr::BinOp(left, op, right) => {
                let left_val = self.evaluate_expr(left);
                let right_val = self.evaluate_expr(right);
                match op {
                    Operator::Add => left_val + right_val,
                    Operator::Subtract => left_val - right_val,
                    Operator::Multiply => left_val * right_val,
                    Operator::Divide => left_val / right_val,
                }
            }
        }
    }
}
