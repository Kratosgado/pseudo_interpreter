use super::expr::Expr;

#[derive(Debug, Clone)]
pub enum Statement {
    Expr(Expr),
    Assignment(String, Expr),
    Print(Expr),
    Input(String),

    // array
    AssignArray(String, Expr, Box<Vec<Expr>>),
    DeclareArray(String, Expr),
    AssignIndex(String, Expr, Expr),
        
    If( Expr, Box<Vec<Statement>>,Option<Box<Vec<Statement>>>),
    While(Expr, Box<Vec<Statement>>),
    For(Expr, Option<Expr>, Expr, Expr, Box<Vec<Statement>>),

    Function(String,Vec<Expr>, Box<Vec<Statement>>, Option<Expr>),
}