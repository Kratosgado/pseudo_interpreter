use super::{expr::Expr, token::Token};

#[derive(Debug, Clone)]
pub enum Statement {
    Declare(String, String),
    Expr(Expr),
    Assignment(String, Expr),
    Print(Expr),
    PrintMulti(Vec<Expr>),
    Input(String),

    // array
    AssignArray(String, Expr, Expr),
    DeclareArray(String, Expr),
    AssignIndex(String, Expr, Expr),
        
    If( Expr, Box<Vec<Statement>>,Option<Box<Vec<Statement>>>),
    While(Expr, Box<Vec<Statement>>),
    For(Expr, Option<Expr>, Expr, Expr, Box<Vec<Statement>>),

    Function(String,Vec<Expr>, Box<Vec<Statement>>, Option<Expr>),
}