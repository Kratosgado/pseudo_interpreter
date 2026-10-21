use std::string::ParseError;

use crate::lexer::enums::token::Token;

#[derive(Debug)]
pub enum PseudoError {
    IoError(std::io::Error),
    ParseError { msg: String, line: usize },
    KeywordError { msg: String, line: usize },
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

impl PseudoError {
    pub fn parse(msg: impl Into<String>, line: usize) -> Self {
        PseudoError::ParseError { msg: msg.into(), line }
    }

    pub fn line_of(&self) -> Option<usize> {
        match self {
            PseudoError::ParseError { line, .. } => Some(*line),
            PseudoError::KeywordError { line, .. } => Some(*line),
            _ => None,
        }
    }

    pub fn message(&self) -> String {
        match self {
            PseudoError::ParseError { msg, .. } => msg.clone(),
            PseudoError::KeywordError { msg, .. } => msg.clone(),
            PseudoError::AssignmentError(m) => m.clone(),
            PseudoError::StatementError(m) => m.clone(),
            PseudoError::ValueError(m) => m.clone(),
            PseudoError::InvalidToken(m) => m.clone(),
            PseudoError::EvalError(m) => m.clone(),
            PseudoError::VariableError(m) => m.clone(),
            PseudoError::TypeError(m) => m.clone(),
            PseudoError::RuntimeError(m) => m.clone(),
            PseudoError::IoError(e) => e.to_string(),
            PseudoError::InvalidOperation => "Invalid operation".to_string(),
            PseudoError::UnexpectedEOF => "Unexpected end of file".to_string(),
        }
    }
}

impl From<std::io::Error> for PseudoError {
    fn from(err: std::io::Error) -> Self {
        PseudoError::IoError(err)
    }
}

impl From<ParseError> for PseudoError {
    fn from(err: ParseError) -> Self {
        PseudoError::ParseError { msg: err.to_string(), line: 0 }
    }
}

pub trait KeywordError {
    fn keyword(expected: Vec<Token>, found: &Token, line: usize) -> Self;
}

impl KeywordError for PseudoError {
    fn keyword(expected: Vec<Token>, found: &Token, line: usize) -> Self {
        PseudoError::KeywordError {
            msg: format!("Expected: '{}', found: '{}'", expected[0], found),
            line,
        }
    }
}
