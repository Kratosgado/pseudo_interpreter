pub mod parser;
pub use super::lexer::enums::{
     expr::Expr,
     operator::Operator,
     token::Token,
     statement::Statement,
};

mod functions {
     pub mod assignment;
     pub mod factor_term;
     pub mod comparison;
     pub mod print_expr;
     pub mod parse_if;
}

pub use functions::{
     assignment::Assignment,
     factor_term::FactorTerm,
     comparison::Comparison,
     print_expr::PrintExpr,
     parse_if::ParseIf,
};