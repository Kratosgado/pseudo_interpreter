
#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
}