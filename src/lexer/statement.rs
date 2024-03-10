use super::token::Token;
use super::expr::Expr;

#[derive(Debug, Clone)]
pub enum Statement {
    Expr(Expr),
    Assignment(Token, Expr),
    Print(Expr),
    // Add more variants as needed
}