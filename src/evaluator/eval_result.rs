use std::cmp::{PartialEq, PartialOrd};
use std::fmt::{self};

use crate::constants::error_handler::PseudoError;

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum EvalResult {
    Number(i64),
    Double(f64),
    Str(String),
    Boolean(bool),
    Multi(Vec<EvalResult>),
    Null,
}

impl EvalResult {
    pub fn get_type(&self) -> String {
        match self {
            EvalResult::Number(_) => "int".to_string(),
            EvalResult::Double(_) => "double".to_string(),
            EvalResult::Str(_) => "str".to_string(),
            EvalResult::Boolean(_) => "bool".to_string(),
            EvalResult::Multi(_) => "multi".to_string(),
            EvalResult::Null => "null".to_string(),
        }
    }
}
pub trait Operation {
    fn add(&self, other: &Self) -> Result<EvalResult, PseudoError>;
    // fn subtract(&self, other: &Self) -> Result<EvalResult, PseudoError>;
    // fn multiply(&self, other: &Self) -> Result<EvalResult, PseudoError>;
    // fn divide(&self, other: &Self) -> Result<EvalResult, PseudoError>;
    // fn modulo(&self, other: &Self) -> Result<EvalResult, PseudoError>;
    fn greater_than(&self, other: &Self) -> Result<EvalResult, PseudoError>;
    fn less_than(&self, other: &Self) -> Result<EvalResult, PseudoError>;
    fn equal(&self, other: &Self) -> Result<EvalResult, PseudoError>;
    fn not_equal(&self, other: &Self) -> Result<EvalResult, PseudoError>;
    fn greater_or_equal(&self, other: &Self) -> Result<EvalResult, PseudoError>;
    fn less_or_equal(&self, other: &Self) -> Result<EvalResult, PseudoError>;
    fn and(&self, other: &Self) -> Result<EvalResult, PseudoError>;
    fn or(&self, other: &Self) -> Result<EvalResult, PseudoError>;
}

impl Operation for EvalResult {
    fn add(&self, other: &EvalResult) -> Result<EvalResult, PseudoError> {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => Ok(EvalResult::Number(n1 + n2)),
            (EvalResult::Double(n1), EvalResult::Double(n2)) => Ok(EvalResult::Double(n1 + n2)),
            (EvalResult::Str(s1), EvalResult::Str(s2)) => Ok(EvalResult::Str(format!("{}{}", s1, s2))),
            _ => return Err(PseudoError::InvalidOperation),
        }
    }

    // fn subtract(&self, other: &EvalResult) -> Result<EvalResult, PseudoError> {
    //     match (self, other) {
    //         (EvalResult::Number(n1), EvalResult::Number(n2)) => Ok(EvalResult::Number(n1 - n2)),
    //         (EvalResult::Double(n1), EvalResult::Double(n2)) => Ok(EvalResult::Double(n1 - n2)),
    //         _ => return Err(PseudoError::InvalidOperation),
    //     }
    // }

    // fn multiply(&self, other: &EvalResult) -> Result<EvalResult, PseudoError> {
    //     match (self, other) {
    //         (EvalResult::Number(n1), EvalResult::Number(n2)) => Ok(EvalResult::Number(n1 * n2)),
    //         (EvalResult::Double(n1), EvalResult::Double(n2)) => Ok(EvalResult::Double(n1 * n2)),
    //         _ => return Err(PseudoError::InvalidOperation),
    //     }
    // }

    // fn divide(&self, other: &EvalResult) -> Result<EvalResult, PseudoError> {
    //     match (self, other) {
    //         (EvalResult::Number(n1), EvalResult::Number(n2)) => Ok(EvalResult::Number(n1 / n2)),
    //         (EvalResult::Double(n1), EvalResult::Double(n2)) => Ok(EvalResult::Double(n1 / n2)),
    //         _ => return Err(PseudoError::InvalidOperation),
    //     }
    // }

    // fn modulo(&self, other: &EvalResult) -> Result<EvalResult, PseudoError> {
    //     match (self, other) {
    //         (EvalResult::Number(n1), EvalResult::Number(n2)) => Ok(EvalResult::Number(n1 % n2)),
    //         (EvalResult::Double(n1), EvalResult::Double(n2)) => Ok(EvalResult::Double(n1 % n2)),
    //         _ => return Err(PseudoError::InvalidOperation),
    //     }
    // }

    fn greater_than(&self, other: &EvalResult) -> Result<EvalResult, PseudoError> {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => Ok(EvalResult::Boolean(n1 > n2)),
            (EvalResult::Double(n1), EvalResult::Double(n2)) => Ok(EvalResult::Boolean(n1 > n2)),
            _ => return Err(PseudoError::InvalidOperation),
        }
    }

    fn less_than(&self, other: &EvalResult) -> Result<EvalResult, PseudoError> {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => Ok(EvalResult::Boolean(n1 < n2)),
            (EvalResult::Double(n1), EvalResult::Double(n2)) => Ok(EvalResult::Boolean(n1 < n2)),
            _ => return Err(PseudoError::InvalidOperation),
        }
    }

    fn greater_or_equal(&self, other: &EvalResult) -> Result<EvalResult, PseudoError> {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => Ok(EvalResult::Boolean(n1 >= n2)),
            (EvalResult::Double(n1), EvalResult::Double(n2)) => Ok(EvalResult::Boolean(n1 >= n2)),
            _ => return Err(PseudoError::InvalidOperation),
        }
    }

    fn equal(&self, other: &Self) -> Result<EvalResult, PseudoError> {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => Ok(EvalResult::Boolean(n1 == n2)),
            (EvalResult::Double(n1), EvalResult::Double(n2)) => Ok(EvalResult::Boolean(n1 == n2)),
            (EvalResult::Str(s1), EvalResult::Str(s2)) => Ok(EvalResult::Boolean(s1 == s2)),
            (EvalResult::Boolean(b1), EvalResult::Boolean(b2)) => Ok(EvalResult::Boolean(b1 == b2)),
            _ => return Err(PseudoError::InvalidOperation),
        }
    }
    fn not_equal(&self, other: &Self) -> Result<EvalResult, PseudoError> {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => Ok(EvalResult::Boolean(n1 != n2)),
            (EvalResult::Double(n1), EvalResult::Double(n2)) => Ok(EvalResult::Boolean(n1 != n2)),
            (EvalResult::Str(s1), EvalResult::Str(s2)) => Ok(EvalResult::Boolean(s1 != s2)),
            (EvalResult::Boolean(b1), EvalResult::Boolean(b2)) => Ok(EvalResult::Boolean(b1 != b2)),
            _ => return Err(PseudoError::InvalidOperation),
        }
    }
    fn less_or_equal(&self, other: &Self) -> Result<EvalResult, PseudoError> {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => Ok(EvalResult::Boolean(n1 <= n2)),
            (EvalResult::Double(n1), EvalResult::Double(n2)) => Ok(EvalResult::Boolean(n1 <= n2)),
            _ => return Err(PseudoError::InvalidOperation),
        }
    }
    fn and(&self, other: &Self) -> Result<EvalResult, PseudoError> {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => Ok(EvalResult::Number(n1 & n2)),
            (EvalResult::Boolean(b1), EvalResult::Boolean(b2)) => Ok(EvalResult::Boolean(b1 & b2)),
            _ => return Err(PseudoError::InvalidOperation),
        }
    }fn or(&self, other: &Self) -> Result<EvalResult, PseudoError> {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => Ok(EvalResult::Number(n1 | n2)),
            (EvalResult::Boolean(b1), EvalResult::Boolean(b2)) => Ok(EvalResult::Boolean(b1 | b2)),
            _ => return Err(PseudoError::InvalidOperation),
        }
    }
}

impl fmt::Display for EvalResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalResult::Number(n) => write!(f, "{}", n),
            EvalResult::Double(n) => write!(f, "{}", n),
            EvalResult::Str(s) => write!(f, "{}", s),
            EvalResult::Boolean(b) => write!(f, "{}", b),
            EvalResult::Null => write!(f, "null"),
            EvalResult::Multi(v) => write! (f, "{:?}", v ) // EvalResult::None => write!(f, "None"),
        }
    }
}
