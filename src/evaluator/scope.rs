use std::collections::HashMap;

use super::EvalResult;

/// The variables and arrays belonging to one level of execution: either the
/// top level program or a single function call.
#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub vars: HashMap<String, EvalResult>,
    pub arrays: HashMap<String, Vec<EvalResult>>,
}

impl Scope {
    pub fn new() -> Self {
        Scope::default()
    }
}
