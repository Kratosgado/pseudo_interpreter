use crate::lexer::enums::token::Token;

use super::super::{parser::Parser, PrintExpr, Statement, Assignment};

pub trait ParseIf {
    fn parse_if(&mut self) -> Statement;
}

impl ParseIf for Parser {
    fn parse_if(&mut self) -> Statement {
        self.next_token();
        let condition = self.parse_expr();
        let mut consequence: Vec<Statement> = Vec::new();

        if let Some(Token::Then) = self.current_token {
            self.next_token();
            while let Some(token) = &self.current_token {
                match token {
                    Token::EndIf => {
                        self.next_token();
                        break;
                    }
                    Token::Print => consequence.push(self.parse_print()),
                    Token::Ident(_) => consequence.push(self.parse_assignment()),
                    Token::EOL => self.next_token(),
                    Token::EOF => break,
                    _ => {
                        let expr = self.parse_expr();
                        consequence.push(Statement::Expr(expr));
                    }
                    // _ => {
                    //     consequence = self.parse();
                    //     panic!("Expected 'EndIf' keyword");
                    // }
                }
            }
        } else {
            panic!("Expected 'then' after if condition");
        }

        Statement::If(condition, Box::new(consequence), None)
    }
}
