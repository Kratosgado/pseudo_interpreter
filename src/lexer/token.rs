#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Datatype tokens
    Number(i64),
    Str(String),
    Boolean(bool),

    // variable
    Variable,

    // arithmen tokens
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    LParen,
    RParen,

    // statement tokens
    Let,
    Ident(String), // identifier
    Assign,        // assignment
    EOL,           // end of line
    EOF,           // end of file

    // keywords
    Print,

    // comparisons
    Equals, // equality
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    NotEqual,

    // control tokens
    If,
    Else,
    While,
    For,
    Function,
    Return,
    Continue,
    Break,
}
