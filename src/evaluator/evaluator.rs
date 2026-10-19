use super::{eval_result::EvalResult, flow::Flow, scope::Scope, EvalStatement, Expr, Statement};
use crate::constants::error_handler::PseudoError;
use std::collections::HashMap;
use std::io::{self, Write};

/// How much native stack recursive pseudo-calls may consume before the
/// evaluator reports an error.
///
/// A frame count is the wrong unit here: one pseudo-call costs wildly
/// different amounts of native stack depending on how deeply its body nests
/// expressions, so any fixed count is either unsafe on a small stack or
/// needlessly tight on a large one. Budgeting bytes tracks the real resource.
/// 1 MiB leaves room on the 2 MiB stack a Rust test thread gets by default.
pub const MAX_STACK_BYTES: usize = 1024 * 1024;

/// The address of a local, used as a stand-in for the current stack pointer.
#[inline(never)]
pub fn stack_probe() -> usize {
    let anchor = 0u8;
    // `black_box` keeps the compiler from optimising the local away.
    std::hint::black_box(&anchor) as *const u8 as usize
}

#[derive(Clone)]
pub struct FuncArgs {
    pub params: Vec<Expr>,
    pub statements: Box<Vec<Statement>>,
}
pub struct Evaluator {
    statements: Vec<Statement>,
    pub current_statement: Option<Statement>,
    position: usize,
    /// Top level variables and arrays.
    globals: Scope,
    /// One scope per in-progress function call; the last is the active one.
    frames: Vec<Scope>,
    pub function_args: HashMap<String, FuncArgs>,
    /// Where `print` writes. Defaults to stdout.
    out: Box<dyn Write>,
    /// Stack pointer when evaluation began, to measure recursion against.
    stack_base: usize,
}

impl Evaluator {
    pub fn new(statements: Vec<Statement>) -> Self {
        Evaluator::with_writer(statements, Box::new(io::stdout()))
    }

    /// Build an evaluator that sends `print` output to `out` instead of stdout.
    pub fn with_writer(statements: Vec<Statement>, out: Box<dyn Write>) -> Self {
        let mut evaluator = Evaluator {
            statements,
            current_statement: None,
            position: 0,
            globals: Scope::new(),
            frames: Vec::new(),
            function_args: HashMap::new(),
            out,
            stack_base: stack_probe(),
        };
        evaluator.next_statement();
        evaluator
    }

    /// How much native stack has been consumed since evaluation began. The
    /// stack grows downward on every platform this runs on, but compare both
    /// ways so the figure is never negative.
    pub fn stack_used(&self) -> usize {
        self.stack_base.abs_diff(stack_probe())
    }

    pub fn next_statement(&mut self) {
        if self.position < self.statements.len() {
            self.current_statement = Some(self.statements[self.position].clone());
            self.position += 1;
        } else {
            self.current_statement = None;
        }
    }

    pub fn evaluate(&mut self) -> Result<(), PseudoError> {
        while let Some(statement) = self.current_statement.take() {
            // A `Return` that reaches the top level has no call to leave.
            if let Flow::Return(_) = self.evaluate_statement(&statement, true)? {
                return Err(PseudoError::StatementError(
                    "return outside of a function".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn out(&mut self) -> &mut dyn Write {
        self.out.as_mut()
    }

    /// Are we inside a function call?
    pub fn in_function(&self) -> bool {
        !self.frames.is_empty()
    }

    pub fn call_depth(&self) -> usize {
        self.frames.len()
    }

    /// Start a new call scope. Pair every push with a [`pop_frame`](Self::pop_frame).
    pub fn push_frame(&mut self, scope: Scope) {
        self.frames.push(scope);
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop();
    }

    /// The scope writes land in: the active call's, else the globals.
    fn current_scope_mut(&mut self) -> &mut Scope {
        match self.frames.last_mut() {
            Some(frame) => frame,
            None => &mut self.globals,
        }
    }

    /// Look a variable up in the active call's scope, then the globals. Only
    /// those two are visible, so a callee never sees its caller's locals.
    pub fn get_var(&self, name: &str) -> Option<&EvalResult> {
        if let Some(frame) = self.frames.last() {
            if let Some(value) = frame.vars.get(name) {
                return Some(value);
            }
        }
        self.globals.vars.get(name)
    }

    /// The variable as seen in the scope being written to, ignoring any global
    /// it may shadow.
    pub fn local_var(&self, name: &str) -> Option<&EvalResult> {
        match self.frames.last() {
            Some(frame) => frame.vars.get(name),
            None => self.globals.vars.get(name),
        }
    }

    /// Bind a variable in the current scope. Inside a call this makes the name
    /// local, so it cannot overwrite the caller's variable of the same name.
    pub fn set_var(&mut self, name: String, value: EvalResult) {
        self.current_scope_mut().vars.insert(name, value);
    }

    pub fn get_array(&self, name: &str) -> Option<&Vec<EvalResult>> {
        if let Some(frame) = self.frames.last() {
            if let Some(array) = frame.arrays.get(name) {
                return Some(array);
            }
        }
        self.globals.arrays.get(name)
    }

    /// The array to mutate in place, from the scope writes go to.
    ///
    /// This deliberately does not fall back to the globals: writing an element
    /// is still a write, so it follows the same rule as [`set_var`](Self::set_var)
    /// and cannot reach out of the current call.
    pub fn get_array_mut(&mut self, name: &str) -> Option<&mut Vec<EvalResult>> {
        self.current_scope_mut().arrays.get_mut(name)
    }

    pub fn set_array(&mut self, name: String, values: Vec<EvalResult>) {
        self.current_scope_mut().arrays.insert(name, values);
    }
}
