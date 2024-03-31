use super::expr::Expr;

#[derive(Debug, Clone)]
pub enum Statement {
    Declare(String, String),
    Expr(Expr),
    Assignment(String, Expr),
    Print(Expr),
    PrintMulti(Vec<Expr>),
    Input(String),
    None,

    // array
    AssignArray(String, Expr, Expr),
    DeclareArray(String, Expr),
    AssignIndex(String, Expr, Expr),

    If(IfStructure),

    While(Expr, Box<Vec<Statement>>),
    Break,
    For(Expr, Option<Expr>, Expr, Expr, Box<Vec<Statement>>),

    Function(String, Vec<Expr>, Box<Vec<Statement>>, Option<Expr>),
}

#[derive(Debug, Clone)]
pub struct IfStructure {
    pub ifcond: IfCond,
    pub elseifs: Option<Vec<IfCond>>,
    pub alternative: Option<Vec<Statement>>,
}

#[derive(Debug, Clone)]
pub struct IfCond {
    pub cond: Expr,
    pub consequence: Vec<Statement>,
}
