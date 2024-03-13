use super::expr::Expr;

#[derive(Debug, Clone)]
pub enum Statement {
    Expr(Expr),
    Assignment(String, Expr),
    Print(Expr),
    // Add more variants as needed
    If( Expr, Box<Vec<Statement>>,Option<Box<Vec<Statement>>>),
    While(Expr, Box<Vec<Statement>>)
}

pub struct IFElseBlock {
    pub if_block: Vec<Statement>,
    pub else_block: Option<Vec<Statement>>,
}