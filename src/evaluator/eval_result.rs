use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum EvalResult {
    Number(i64),
    Str(String),
    Bool(bool),
    None,
}

impl fmt::Display for EvalResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalResult::Number(n) => write!(f, "{}", n),
            EvalResult::Str(s) => write!(f, "{}", s),
            EvalResult::Bool(b) => write!(f, "{}", b),
            EvalResult::None => write!(f, "None"),
        }
    }
}