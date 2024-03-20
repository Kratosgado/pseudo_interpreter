use std::cmp::{PartialEq, PartialOrd};
use std::fmt::{self};

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
    fn add(&self, other: &Self) -> EvalResult;
    fn subtract(&self, other: &Self) -> EvalResult;
    fn multiply(&self, other: &Self) -> EvalResult;
    fn divide(&self, other: &Self) -> EvalResult;
    fn modulo(&self, other: &Self) -> EvalResult;
    fn greater_than(&self, other: &Self) -> EvalResult;
    fn less_than(&self, other: &Self) -> EvalResult;
    fn equal(&self, other: &Self) -> EvalResult;
    fn not_equal(&self, other: &Self) -> EvalResult;
    fn greater_or_equal(&self, other: &Self) -> EvalResult;
    fn less_or_equal(&self, other: &Self) -> EvalResult;
}

impl Operation for EvalResult {
    fn add(&self, other: &EvalResult) -> EvalResult {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => EvalResult::Number(n1 + n2),
            (EvalResult::Double(n1), EvalResult::Double(n2)) => EvalResult::Double(n1 + n2),
            (EvalResult::Str(s1), EvalResult::Str(s2)) => EvalResult::Str(format!("{}{}", s1, s2)),
            _ => panic!("Invalid operation"),
        }
    }

    fn subtract(&self, other: &EvalResult) -> EvalResult {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => EvalResult::Number(n1 - n2),
            (EvalResult::Double(n1), EvalResult::Double(n2)) => EvalResult::Double(n1 - n2),
            _ => panic!("Invalid operation"),
        }
    }

    fn multiply(&self, other: &EvalResult) -> EvalResult {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => EvalResult::Number(n1 * n2),
            (EvalResult::Double(n1), EvalResult::Double(n2)) => EvalResult::Double(n1 * n2),
            _ => panic!("Invalid operation"),
        }
    }

    fn divide(&self, other: &EvalResult) -> EvalResult {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => EvalResult::Number(n1 / n2),
            (EvalResult::Double(n1), EvalResult::Double(n2)) => EvalResult::Double(n1 / n2),
            _ => panic!("Invalid operation"),
        }
    }

    fn modulo(&self, other: &EvalResult) -> EvalResult {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => EvalResult::Number(n1 % n2),
            (EvalResult::Double(n1), EvalResult::Double(n2)) => EvalResult::Double(n1 % n2),
            _ => panic!("Invalid operation"),
        }
    }

    fn greater_than(&self, other: &EvalResult) -> EvalResult {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => EvalResult::Boolean(n1 > n2),
            (EvalResult::Double(n1), EvalResult::Double(n2)) => EvalResult::Boolean(n1 > n2),
            _ => panic!("Invalid operation"),
        }
    }

    fn less_than(&self, other: &EvalResult) -> EvalResult {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => EvalResult::Boolean(n1 < n2),
            (EvalResult::Double(n1), EvalResult::Double(n2)) => EvalResult::Boolean(n1 < n2),
            _ => panic!("Invalid operation"),
        }
    }

    fn greater_or_equal(&self, other: &EvalResult) -> EvalResult {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => EvalResult::Boolean(n1 >= n2),
            (EvalResult::Double(n1), EvalResult::Double(n2)) => EvalResult::Boolean(n1 >= n2),
            _ => panic!("Invalid operation"),
        }
    }

    fn equal(&self, other: &Self) -> EvalResult {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => EvalResult::Boolean(n1 == n2),
            (EvalResult::Double(n1), EvalResult::Double(n2)) => EvalResult::Boolean(n1 == n2),
            (EvalResult::Str(s1), EvalResult::Str(s2)) => EvalResult::Boolean(s1 == s2),
            (EvalResult::Boolean(b1), EvalResult::Boolean(b2)) => EvalResult::Boolean(b1 == b2),
            _ => panic!("Invalid operation"),
        }
    }
    fn not_equal(&self, other: &Self) -> EvalResult {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => EvalResult::Boolean(n1 != n2),
            (EvalResult::Double(n1), EvalResult::Double(n2)) => EvalResult::Boolean(n1 != n2),
            (EvalResult::Str(s1), EvalResult::Str(s2)) => EvalResult::Boolean(s1 != s2),
            (EvalResult::Boolean(b1), EvalResult::Boolean(b2)) => EvalResult::Boolean(b1 != b2),
            _ => panic!("Invalid operation"),
        }
    }
    fn less_or_equal(&self, other: &Self) -> EvalResult {
        match (self, other) {
            (EvalResult::Number(n1), EvalResult::Number(n2)) => EvalResult::Boolean(n1 <= n2),
            (EvalResult::Double(n1), EvalResult::Double(n2)) => EvalResult::Boolean(n1 <= n2),
            _ => panic!("Invalid operation"),
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
