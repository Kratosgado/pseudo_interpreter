use crate::{constants::error_handler::{KeywordError, PseudoError}, lexer::enums::token::Token};

use super::super::{Statement, Parser};

pub trait ParseInput {
    fn parse_input(&mut self) -> Result<Statement, PseudoError>;
}

impl ParseInput for Parser {
    fn parse_input(&mut self) -> Result<Statement, PseudoError> {
        self.next_token();
        match &self.current_token {
            Some(Token::Ident(var)) => {
                let var = var.clone();
                self.next_token();
                Ok(Statement::Input(var))
            }
            _ => return Err(PseudoError::keyword(vec![Token::Ident("identifier".to_string())], &self.current_token.as_ref().unwrap())),
        }
     
    }
}