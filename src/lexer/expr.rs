use super::operator::Operator;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    BinOp(Box<Expr>, Operator, Box<Expr>),
    Str(String),
    Boolean(bool),
    Variable(String),

    // comparison
    Equals(Box<Expr>, Box<Expr>),
    LessThan(Box<Expr>, Box<Expr>),
    GreaterThan(Box<Expr>, Box<Expr>),
    LessThanEqual(Box<Expr>, Box<Expr>),
    GreaterThanEqual(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),
    
}