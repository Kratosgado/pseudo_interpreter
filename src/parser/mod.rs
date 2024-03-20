pub mod parser;
mod functions {
    pub mod assignment;
    pub mod comparison;
    pub mod factor_term;
    pub mod parse_array;
    pub mod parse_for;
    pub mod parse_function;
    pub mod parse_if;
    pub mod parse_input;
    pub mod parse_while;
    pub mod print_expr;
    pub mod token;
    pub mod parse_declare;
}

pub use super::lexer::enums::{expr::Expr, operator::Operator, statement::Statement, token::Token};
pub use functions::{
    assignment::ParseAssignment, comparison::ParseComparison, factor_term::ParseFactorTerm,
    parse_array::ParseArray, parse_for::ParseFor, parse_function::ParseFunction, parse_if::ParseIf,
    parse_input::ParseInput, parse_while::ParseWhile, print_expr::ParsePrintExpr,
    token::ParseToken, parse_declare::ParseDeclare,
};
pub use parser::Parser;
