use crate::constants::error_handler::{KeywordError, PseudoError};

use super::{super::{
    parser::Parser,
    ParsePrintExpr, Statement, Token,
}, parse_token::ParseToken};

pub trait ParseWhile {
    fn parse_while(&mut self) -> Result<Statement, PseudoError>;
}

impl ParseWhile for Parser {
    fn parse_while(&mut self) -> Result<Statement, PseudoError> {
        self.next_token();
        let condition = self.parse_expr();

        if let Some(Token::Do) = &self.current_token {
            self.next_token();
            let wstatement = self.parse_token(vec![Token::EndWhile])?;
            Ok(Statement::While(condition, Box::new(wstatement)))
        } else {
            return  Err(PseudoError::keyword(vec![Token::Do], &self.current_token.as_ref().unwrap()));
        }
    }
}
