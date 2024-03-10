#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Datatype tokens
    Number(i64),
    Str(String),
    Bool(bool),
    
    // arithmen tokens
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    Plus,

    // statement tokens
    Let,
    Ident(String), // identifier
    Assign,        // assignment
    Equals,        // equality
    EOL,           // end of line
    EOF,           // end of file

    // keywords
    Print,

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
