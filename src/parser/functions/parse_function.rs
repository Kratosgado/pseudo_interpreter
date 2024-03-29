use crate::constants::error_handler::{KeywordError, PseudoError};

use super::{
    super::{
        Expr, ParsePrintExpr,
         Parser, Statement, Token,
    },
    parse_token::ParseToken,
};

pub trait ParseFunction {
    fn parse_function(&mut self) -> Result<Statement, PseudoError>;
}

impl ParseFunction for Parser {
    fn parse_function(&mut self) -> Result<Statement, PseudoError> {
        self.next_token();
        if let Some(Token::Ident(name)) = self.current_token.clone() {
            self.next_token();
            if let Some(Token::LParen) = self.current_token {
                self.next_token();
                let mut params: Vec<Expr> = Vec::new();
                while self.current_token != Some(Token::RParen) {
                    match &self.current_token {
                        Some(Token::Ident(param)) => {
                            params.push(Expr::Param(param.clone()));
                            self.next_token();
                        }
                        Some(Token::Comma) => self.next_token(),
                        _ => todo!("Invalid  function parameter"),
                    }
                }
                self.next_token();
                let mut ret_ment: Option<Expr> = None;
                let fstatements = self.parse_token(vec![Token::EndFunction, Token::Return])?;
                if self.current_token == Some(Token::Return) {
                    self.next_token();
                    ret_ment = Some(self.parse_expr()?);
                    let _ = self.parse_token(vec![Token::EndFunction])?;

                }
                Ok(Statement::Function(name.clone(), params, Box::new(fstatements), ret_ment))
            } else {
                return Err(PseudoError::keyword(vec![Token::LParen], &self.current_token.as_ref().unwrap()));
            }
        } else {
            return Err(PseudoError::keyword(vec![Token::Ident("function name".to_string())], &self.current_token.as_ref().unwrap()));
        }
    }
}
