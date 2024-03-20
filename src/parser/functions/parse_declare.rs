use crate::lexer::enums::token::Token;

use super::super::{Statement, Parser};


pub trait ParseDeclare {
    fn parse_declare(&mut self) -> Statement;   
}

impl ParseDeclare for Parser {
    fn parse_declare(&mut self) -> Statement {
        self.next_token();
        if let Some(Token::Ident(var)) = self.current_token.clone() {
            self.next_token();
            if let Some(Token::As) = self.current_token {
                self.next_token();
            }else {
                panic!("Expected 'AS' keyword after variable declaration")
            }
            let datatype = match self.current_token {
                Some(Token::Integer) => "int",
                Some(Token::String) => "str",
                Some(Token::Bool) => "bool",
                Some(Token::Double) =>"double",
                _ => panic!("Expected type"),
            };
            self.next_token();
            Statement::Declare(var, datatype.to_string())
        } else {
            panic!("Expected identifier")
        }
    }
}