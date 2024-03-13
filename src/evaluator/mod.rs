mod eval_result;
pub mod evaluator;
mod functions {
    pub mod arithmetics;
    pub mod comparison;
    pub mod eval_for;
    pub mod eval_if;
    pub mod eval_statement;
    pub mod eval_while;
    pub mod expression;
}

pub use eval_result::EvalResult;
pub use evaluator::Evaluator;

pub use super::lexer::enums::{expr::Expr, operator::Operator, statement::Statement};
pub use functions::{
    arithmetics::Arithmetics, comparison::Comparison, eval_for::EvalFor, eval_if::EvalIf,
    eval_statement::EvalStatement, eval_while::EvalWhile, expression::EvalExpression,
};
