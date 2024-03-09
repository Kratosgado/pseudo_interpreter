use crate::lexer::expr::Expr;
use crate::lexer::operator::Operator;

pub struct Evaluator;

impl Evaluator {
    pub fn evaluate(expr: &Expr) -> i64 {
        match expr {
            Expr::Number(num) => *num,
            Expr::BinOp(left , op , right) => {
                let left_val = Evaluator::evaluate(left);
                let right_val = Evaluator::evaluate(right);
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