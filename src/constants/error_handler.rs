use std::string::ParseError;

use crate::lexer::enums::token::Token;

#[derive(Debug)]
pub enum PseudoError {
    IoError(std::io::Error),
    ParseError(String),
    KeywordError(String),
    AssignmentError(String),
    StatementError(String),
    InvalidOperation,
    ValueError(String),
    InvalidToken(String),
    EvalError(String),
    VariableError(String),
    TypeError(String),
    RuntimeError(String),
    UnexpectedEOF,
}

impl From<std::io::Error> for PseudoError {
    fn from(err: std::io::Error) -> Self {
        PseudoError::IoError(err)
    }
}
impl From<ParseError> for PseudoError {
    fn from(err: ParseError) -> Self {
        PseudoError::ParseError(err.to_string())
    }
}

pub trait KeywordError {
    fn keyword(expected: Vec<Token>, found: &Token) -> Self;
}

// expected keyword error
impl KeywordError for PseudoError {
    fn keyword(expected: Vec<Token>, found: &Token) -> Self {
        PseudoError::KeywordError(format!("Expected: '{}', found: '{}'", expected[0], found))
    }
}
