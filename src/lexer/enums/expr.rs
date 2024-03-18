use super::operator::Operator;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    BinOp(Box<Expr>, Operator, Box<Expr>),
    Str(String),
    Boolean(bool),
    Variable(String),
    ArrayVariable(String, Box<Expr>),

    // multi 
    Multi(Vec<Expr>),

    Param(String),
    FunctionCall(String, Vec<Expr>),

    // comparison
    Comparison(Box<Expr>, Operator, Box<Expr>),
}