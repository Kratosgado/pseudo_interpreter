pub mod parser;
mod functions {
     pub mod token;
     pub mod assignment;
     pub mod factor_term;
     pub mod comparison;
     pub mod print_expr;
     pub mod parse_if;
     pub mod parse_while;
     pub mod parse_for;
     pub mod parse_input;
     pub mod  parse_array;
}

pub use parser::Parser;
pub use super::lexer::enums::{
     expr::Expr,
     operator::Operator,
     token::Token,
     statement::Statement,
};
pub use functions::{
     token::ParseToken,
     assignment::ParseAssignment,
     factor_term::ParseFactorTerm,
     comparison::ParseComparison,
     print_expr::ParsePrintExpr,
     parse_if::ParseIf,
     parse_while::ParseWhile,
     parse_for::ParseFor,
     parse_input::ParseInput,
     parse_array::ParseArray,
};