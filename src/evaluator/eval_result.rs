use std::fmt;
use std::cmp::{PartialEq, PartialOrd};


#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum EvalResult {
    Number(i64),
    Str(String),
    Boolean(bool),
    // None,
}

impl fmt::Display for EvalResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalResult::Number(n) => write!(f, "{}", n),
            EvalResult::Str(s) => write!(f, "{}", s),
            EvalResult::Boolean(b) => write!(f, "{}", b),
            // EvalResult::None => write!(f, "None"),
        }
    }
}
