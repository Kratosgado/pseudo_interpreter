use super::expr::Expr;

#[derive(Debug, Clone)]
pub enum Statement {
    Expr(Expr),
    Assignment(String, Expr),
    Print(Expr),
    // Add more variants as needed
    If( Expr, Box<Vec<Statement>>,Option<Box<Vec<Statement>>>),
    While(Expr, Box<Vec<Statement>>),
    For(Expr, Option<Expr>, Expr, Expr, Box<Vec<Statement>>),
}