use crate::parser::ParseInput;

use super::super::{
    ParseAssignment, ParseIf, ParsePrintExpr, ParseWhile, Parser, Statement, Token,ParseFor
};
pub trait ParseToken {
    fn parse_token(&mut self) -> Vec<Statement>;
}

impl ParseToken for Parser {
    fn parse_token(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while let Some(token) = &self.current_token {
            match token {
                Token::Print => statements.push(self.parse_print()),
                Token::Input => statements.push(self.parse_input()),
                Token::Ident(_) => statements.push(self.parse_assignment()),
                Token::While => statements.push(self.parse_while()),
                Token::If => statements.push(self.parse_if()),
                Token::For => statements.push(self.parse_for()),
                Token::EOL => self.next_token(),
                Token::EOF => break,
                Token::Number(_) | Token::Str(_) | Token::Boolean(_) => {
                    statements.push(Statement::Expr(self.parse_expr()))
                }
                _ => panic!("Unexpected token: {:?}", token),
            }
        }
        statements
    }
}