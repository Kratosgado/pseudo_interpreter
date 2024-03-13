use crate::lexer::enums::token::Token;

use super::super::{Statement, Parser};

pub trait ParseInput {
    fn parse_input(&mut self) -> Statement;
}

impl ParseInput for Parser {
    fn parse_input(&mut self) -> Statement {
        self.next_token();
        match &self.current_token {
            Some(Token::Ident(var)) => {
                let var = var.clone();
                self.next_token();
                Statement::Input(var)
            }
            _ => panic!("Expected an identifier"),
        }
     
    }
}