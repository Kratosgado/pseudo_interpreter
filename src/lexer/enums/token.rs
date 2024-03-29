use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Declare,
    As,
    Integer,
    Double,
    String,
    Bool,
    // Datatype tokens
    Number(i64),
    Float(f64),
    Str(String),
    Boolean(bool),
    Null,

    // variable
    Variable,
    Array(String, Box<Token>),

    // arithmen tokens
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    FloorDivide,

    // brackets and parenthesis
    LParen,
    RParen,
    LBracket,
    RBracket,
    LCurly,
    RCurly,

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
    ElseIf,
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

    Comma,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Declare => write!(f, "Declare"),
            Token::As => write!(f, "As"),
            Token::Integer => write!(f, "Integer"),
            Token::Double => write!(f, "Double"),
            Token::String => write!(f, "String"),
            Token::Bool => write!(f, "Bool"),
            Token::Number(n) => write!(f, "{}", n),
            Token::Float(n) => write!(f, "{}", n),
            Token::Str(s) => write!(f, "{}", s),
            Token::Boolean(b) => write!(f, "{}", b),
            // give all
            Token::Null => write!(f, "Null"),
            Token::Variable => write!(f, "Variable"),
            Token::Array(s, t) => write!(f, "{}[{}]", s, t),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Modulo => write!(f, "%"),
            Token::FloorDivide => write!(f, "//"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::LCurly => write!(f, "{{"),
            Token::RCurly => write!(f, "}}"),
            Token::Let => write!(f, "Let"),
            Token::Ident(s) => write!(f, "{}", s),
            Token::Assign => write!(f, "="),
            Token::EOL => write!(f, "EOL"),
            Token::EOF => write!(f, "EOF"),
            Token::Print => write!(f, "Print"),
            Token::Input => write!(f, "Input"),
            Token::Equal => write!(f, "=="),
            Token::LessThan => write!(f, "<"),
            Token::GreaterThan => write!(f, ">"),
            Token::LessThanEqual => write!(f, "<="),
            Token::GreaterThanEqual => write!(f, ">="),
            Token::NotEqual => write!(f, "!="),
            Token::Not => write!(f, "Not"),
            Token::And => write!(f, "And"),
            Token::Or => write!(f, "Or"),
            Token::If => write!(f, "If"),
            Token::Else => write!(f, "Else"),
            Token::EndIf => write!(f, "EndIf"),
            Token::Then => write!(f, "Then"),
            Token::Do => write!(f, "Do"),
            Token::While => write!(f, "While"),
            Token::EndWhile => write!(f, "EndWhile"),
            Token::For => write!(f, "For"),
            Token::To => write!(f, "To"),
            Token::Step => write!(f, "Step"),
            Token::EndFor => write!(f, "EndFor"),
            Token::Function => write!(f, "Function"),
            Token::EndFunction => write!(f, "EndFunction"),
            Token::Return => write!(f, "Return"),
            Token::Continue => write!(f, "Continue"),
            Token::Break => write!(f, "Break"),
            Token::Comma => write!(f, ","),
            Token::ElseIf => write!(f, "Elseif"),
            
        }
    }
}
