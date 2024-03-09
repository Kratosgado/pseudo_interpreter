#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i64),
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    EOF,
    Plus,
}
