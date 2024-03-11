pub mod lexer;
pub mod enums {
    pub mod expr;
    pub mod operator;
    pub mod statement;
    pub mod token;
}

pub mod functions {
    pub mod comparison;
    pub mod datatypes;
    pub mod identifier;
    pub mod operator;
}

use functions::{
    comparison::comparison,
    datatypes::Datatype,
    identifier::identifier,
    operator::operator,
};
use enums::token::Token;