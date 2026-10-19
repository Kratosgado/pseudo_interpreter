use super::EvalResult;

/// How a statement left off.
///
/// Statement evaluation hands one of these back instead of `()` so that an
/// early `Return` can travel out of nested blocks on the normal return path,
/// without a flag on the evaluator or a panic.
#[derive(Debug, Clone, PartialEq)]
pub enum Flow {
    /// Carry on with the next statement.
    Normal,
    /// Leave the current function call with this value.
    Return(EvalResult),
}
