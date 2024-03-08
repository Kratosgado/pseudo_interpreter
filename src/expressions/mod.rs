use crate::operator::Operator;

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Operation(Box<Expr>, Operator, Box<Expr>),
}