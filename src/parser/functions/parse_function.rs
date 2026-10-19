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
    fn parse_return(&mut self) -> Result<Statement, PseudoError>;
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
                // `Return` is an ordinary statement now, so the body runs
                // straight through to `EndFunction`.
                let fstatements = self.parse_token(vec![Token::EndFunction])?;
                Ok(Statement::Function(name.clone(), params, Box::new(fstatements)))
            } else {
                return Err(PseudoError::keyword(vec![Token::LParen], &self.current_token.as_ref().unwrap()));
            }
        } else {
            return Err(PseudoError::keyword(vec![Token::Ident("function name".to_string())], &self.current_token.as_ref().unwrap()));
        }
    }

    fn parse_return(&mut self) -> Result<Statement, PseudoError> {
        self.next_token();
        // A bare `return` is one with nothing left on its line. Anything else
        // is the start of the returned expression.
        match &self.current_token {
            Some(Token::EOL) | Some(Token::EOF) | None => Ok(Statement::Return(None)),
            // A block may also end right after the return, with no newline.
            Some(Token::EndFunction) | Some(Token::EndIf) | Some(Token::Else)
            | Some(Token::ElseIf) | Some(Token::EndWhile) | Some(Token::EndFor) => {
                Ok(Statement::Return(None))
            }
            _ => Ok(Statement::Return(Some(self.parse_expr()?))),
        }
    }
}
