use crate::parser::PrintExpr;

use super::super::{parser::Parser, Assignment, ParseIf, Statement, Token};

pub trait ParseWhile {
    fn parse_while(&mut self) -> Statement;
}

impl ParseWhile for Parser {
    fn parse_while(&mut self) -> Statement {
        self.next_token();
        let condition = self.parse_expr();
        let mut wstatement = Vec::new();
        
        if let Some(Token::Do) = &self.current_token {
            self.next_token();
            while let Some(token) = &self.current_token {
                match token {
                    Token::EndWhile => {
                        self.next_token();
                        break;
                    }
                    Token::Print => wstatement.push(self.parse_print()),
                    Token::Ident(_) => wstatement.push(self.parse_assignment()),
                    Token::If => {
                        let new_if = self.parse_if();
                        wstatement.push(new_if.clone());
                        self.if_stack.push(new_if);
                    }
                    Token::EOL => self.next_token(),
                    Token::EOF => break,
                    _ => {
                        let expr = self.parse_expr();
                        wstatement.push(Statement::Expr(expr));
                    }
                }
            }
        }
        Statement::While(condition, Box::new(wstatement))
    }
}
