use super::expr::Expr;

#[derive(Debug, Clone)]
pub enum Statement {
    Expr(Expr),
    Assignment(String, Expr),
    Print(Expr),
    // Add more variants as needed
}