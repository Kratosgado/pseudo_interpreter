use crate::{constants::error_handler::{KeywordError, PseudoError}, lexer::enums::token::Token};

use super::super::{Statement, Parser};


pub trait ParseDeclare {
    fn parse_declare(&mut self) -> Result<Statement, PseudoError>;   
}

impl ParseDeclare for Parser {
    fn parse_declare(&mut self) -> Result<Statement, PseudoError> {
        self.next_token();
        if let Some(Token::Ident(var)) = self.current_token.clone() {
            self.next_token();
            if let Some(Token::As) = self.current_token {
                self.next_token();
            }else {
                return Err(PseudoError::keyword(vec![Token::As], &self.current_token.as_ref().unwrap()));
            }
            let datatype = match self.current_token {
                Some(Token::Integer) => "int",
                Some(Token::String) => "str",
                Some(Token::Bool) => "bool",
                Some(Token::Double) =>"double",
                _ =>return Err(PseudoError::keyword(vec![Token::Ident("type: {integer, string, bool, double}".to_string())], &self.current_token.as_ref().unwrap())),
            };
            self.next_token();
            Ok(Statement::Declare(var, datatype.to_string()))
        } else {
            return Err(PseudoError::keyword(vec![Token::Ident("identifier".to_string())], &self.current_token.as_ref().unwrap()));
        }
    }
}