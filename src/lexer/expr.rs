use super::operator::Operator;

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    BinOp(Box<Expr>, Operator, Box<Expr>),
}