pub mod evaluator;
mod eval_result;

pub use super::lexer::enums::{
    statement::Statement,
    expr::Expr,
     operator::Operator
};

mod functions {
    pub mod arithmetics;
    pub mod comparison;
    pub mod expression;
    pub mod eval_if;
}
pub use eval_result::EvalResult;

pub use functions::{
    arithmetics::Arithmetics,
    comparison::Comparison,
    expression::Expression,
    eval_if::EvalIf,
};