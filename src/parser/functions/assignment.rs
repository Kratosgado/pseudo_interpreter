use crate::constants::{error_handler::{KeywordError, PseudoError}, keywords::KEYWORDS};

use super::super::{ParsePrintExpr, Statement,Expr, Token, Parser};

pub trait ParseAssignment {
    fn parse_assignment(&mut self) -> Result<Statement, PseudoError>;
}

impl ParseAssignment for Parser {
    fn parse_assignment(&mut self) -> Result<Statement, PseudoError> {
        match &self.current_token {
            Some(Token::Ident(var)) => {
                let var = var.clone();
                self.next_token();
                if let Some(Token::Assign) = self.current_token {
                    self.next_token();
                    let expr = self.parse_expr()?;
                    Ok(Statement::Assignment(var, expr))
                }else if let Some(Token::LParen) = self.current_token {
                    self.next_token();
                    let mut args: Option<Expr> = None;
                    if self.current_token != Some(Token::RParen) {
                        args = Some(self.parse_expr()?);
                    }
                    Ok(Statement::Expr(Expr::FunctionCall(var, Box::new(args))))
                }
                 else {
                    return Err(PseudoError::keyword(vec![Token::Assign], &self.current_token.as_ref().unwrap()));
                }
            }
            _ => todo!("Implement for other types of assignments"),
        }
    }
}