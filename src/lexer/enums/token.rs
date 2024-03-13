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
    Input,

    // comparisons
    Equal, // equality
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    NotEqual,
    Not,
    And,
    Or,

    // control tokens
    If,
    Else,
    EndIf,
    Then, 
    
    // loops
    Do,
    While,
    EndWhile,
    For,
    To,
    Step,
    EndFor,

    // function tokens
    Function,
    EndFunction,
    Return,
    Continue,
    Break,
}