use crate::constants::error_handler::{KeywordError, PseudoError};

use super::super::{Expr,Statement, Token, ParseFactorTerm, Parser, Operator};
pub trait ParsePrintExpr {
    fn parse_print(&mut self) -> Result<Statement, PseudoError>;
    fn parse_expr(&mut self) -> Result<Expr, PseudoError>;
}

impl ParsePrintExpr for Parser {
    fn parse_print(&mut self) -> Result<Statement, PseudoError> {
        self.next_token();
        match &self.current_token {
            Some(Token::Ident(_)) => Ok(Statement::Print(self.parse_expr()?)),
            Some(Token::Array(_, _)) => Ok(Statement::Print(self.parse_expr()?)),
            Some(Token::Number(_) | Token::Str(_) | Token::Boolean(_) )=> Ok(Statement::Print(self.parse_expr()?)),
            _ => return Err(PseudoError::keyword(vec![Token::Ident("expression".to_string())], &self.current_token.as_ref().unwrap())),
        }
    }

     fn parse_expr(&mut self) -> Result<Expr, PseudoError> {
        let mut left = self.parse_term()?;
        while let Some(token) = &self.current_token {
            match token {
                Token::Plus | Token::Minus => {
                    let op = if matches!(token, Token::Plus) {
                        Operator::Add
                    } else {
                        Operator::Subtract
                    };
                    self.next_token();
                    let right = self.parse_term()?;
                    left = Expr::BinOp(Box::new(left), op, Box::new(right));
                }
                Token::Comma => {
                    self.next_token();
                    let right = self.parse_term()?;
                    left = Expr::Multi(vec![left, right]);
                }
                _ => break,
            };
        }
        Ok(left)
    }
}