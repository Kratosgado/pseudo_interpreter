
use crate::{constants::error_handler::PseudoError, evaluator::Expr, lexer::enums::token::Token, parser::ParsePrintExpr};

use super::super::{Parser, Statement};

pub trait ParseArray {
    fn parse_array(&mut self) -> Result<Statement, PseudoError>;
}

impl ParseArray for Parser {
    fn parse_array(&mut self) -> Result<Statement, PseudoError> {
        if let Some(Token::Array(var, size)) = self.current_token.clone() {
            let size = match size.as_ref() {
                Token::Number(val) => Expr::Number(*val),
                Token::Ident(var) => Expr::Variable(var.clone()),
                _ => panic!("Invalid array size"),
            };
            self.next_token();
            if let Some(Token::Assign) = &self.current_token {
                self.next_token();
                if let Some(Token::LBracket) = &self.current_token {
                    self.next_token();
                    let indices = self.parse_expr()?;
                    self.next_token();
                    Ok(Statement::AssignArray(var.clone(), size, indices))
                } else {
                    Ok(Statement::AssignIndex(var.clone(), size, self.parse_expr()?))
                }
            } else {
                Ok(Statement::DeclareArray(var.clone(), size))
            }
        } else {
            return  Err(PseudoError::AssignmentError("Invalid array assignment".to_string()));
        }
    }
}
