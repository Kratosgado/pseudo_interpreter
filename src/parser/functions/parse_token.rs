use crate::parser::{ParseArray, ParseFunction, ParseInput};

use super::super::{
    ParseAssignment, ParseDeclare, ParseFor, ParseIf, ParsePrintExpr, ParseWhile, Parser,
    Statement, Token,
};
pub trait ParseToken {
    fn parse_token(&mut self, termintators: Vec<Token>) -> Vec<Statement>;
}

impl ParseToken for Parser {
    fn parse_token(&mut self, terminators: Vec<Token>) -> Vec<Statement> {
        let mut statements = Vec::new();
        while let Some(token) = &self.current_token {
            if terminators.contains(token) {
                if token != &Token::Else && token != &Token::Return{
                    self.next_token();
                }
                break;
            }
            match token {
                Token::Print => statements.push(self.parse_print()),
                Token::Input => statements.push(self.parse_input()),
                Token::Ident(_) => statements.push(self.parse_assignment()),
                Token::Array(_, _) => statements.push(self.parse_array()),
                Token::While => statements.push(self.parse_while()),
                Token::If => statements.push(self.parse_if()),
                Token::For => statements.push(self.parse_for()),
                Token::Function => statements.push(self.parse_function()),
                Token::EOL => self.next_token(),
                Token::Number(_) | Token::Str(_) | Token::Boolean(_) => {
                    statements.push(Statement::Expr(self.parse_expr()))
                }
                Token::Declare => statements.push(self.parse_declare()),
                Token::RParen => self.next_token(),
                _ => panic!("Unexpected token: {:?}, expected: {:?}", token, terminators),
            }
        }
        statements
    }
}
