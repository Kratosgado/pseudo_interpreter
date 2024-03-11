use crate::{evaluator::Statement, Parser};
use super::super::Token;
use super::print_expr::PrintExpr;

pub trait Assignment {
    fn parse_assignment(&mut self) -> Statement;
}

impl Assignment for Parser {
    fn parse_assignment(&mut self) -> Statement {
        match &self.current_token {
            Some(Token::Ident(var)) => {
                let var = var.clone();
                self.next_token();
                if let Some(Token::Assign) = self.current_token {
                    self.next_token();
                    let expr = self.parse_expr();
                    Statement::Assignment(var, expr)
                } else {
                    panic!("Expected assignment operator");
                }
            }
            _ => todo!("Implement for other types of assignments"),
        }
    }
}