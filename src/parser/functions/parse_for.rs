use crate::constants::error_handler::{KeywordError, PseudoError};

use super::{
    super::{
        Expr, ParsePrintExpr,
        Parser, Statement, Token,
    },
    parse_token::ParseToken,
};

pub trait ParseFor {
    fn parse_for(&mut self) -> Result<Statement, PseudoError>;
}

impl ParseFor for Parser {
    fn parse_for(&mut self) -> Result<Statement, PseudoError> {
        self.next_token();
        let var = self.parse_expr()?;
        let mut start: Option<Expr> = None;
        let end: Expr;
        let mut step = Expr::Number(1);
        let mut fstatement = Vec::new();

        if let Some(Token::Assign) = &self.current_token {
            self.next_token();
            start = Some(self.parse_expr()?);
        }
        if let Some(Token::To) = &self.current_token {
            self.next_token();
            end = self.parse_expr()?;
            if let Some(Token::Step) = &self.current_token {
                self.next_token();
                step = self.parse_expr()?;
            }
            if let Some(Token::Do) = &self.current_token {
                self.next_token();
                 fstatement.extend(self.parse_token(vec![Token::EndFor])?);
            } else {
                return Err(PseudoError::keyword(vec![Token::Do], &self.current_token.as_ref().unwrap()));            };
            Ok(Statement::For(var, start, end, step, Box::new(fstatement)))
        } else {
            return Err(PseudoError::keyword(vec![Token::To], &self.current_token.as_ref().unwrap()));
        }
    }
}
