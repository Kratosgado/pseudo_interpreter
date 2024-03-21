use crate::parser::{ParseArray, ParseFunction, ParseInput};

use super::super::{
    ParseAssignment, ParseDeclare, ParseFor, ParseIf, ParsePrintExpr, ParseWhile, Parser,
    Statement, Token,
};
pub trait ParseToken {
    fn parse_token(&mut self) -> Statement;
}

impl ParseToken for Parser {
    fn parse_token(&mut self) -> Statement {
        match &self.current_token {
            Some(token) => match token {
                Token::Print => self.parse_print(),
                Token::Input => self.parse_input(),
                Token::Ident(_) => self.parse_assignment(),
                Token::Array(_, _) => self.parse_array(),
                Token::While => self.parse_while(),
                Token::If => self.parse_if(),
                Token::For => self.parse_for(),
                Token::Function => self.parse_function(),
                Token::EOL => {
                    self.next_token();
                    Statement::None
                }
                Token::Number(_) | Token::Str(_) | Token::Boolean(_) => {
                    Statement::Expr(self.parse_expr())
                }
                Token::Declare => self.parse_declare(),
                // Token::RParen => {self.next_token(); },
                _ => panic!("Unexpected token: {:?}", token),
            },
            None => panic!("No token to parse"),
        }
    }
}
