use crate::{constants::error_handler::PseudoError, lexer::lexer::Lexer};
use super::super::Token;

pub trait Operator {
    fn encode_operator(&mut self)-> Result<Token, PseudoError>;
}

impl <'a> Operator for Lexer<'a> {
    fn encode_operator(&mut self)-> Result<Token, PseudoError> {
        match self.current_char {
            Some('+') => {
                self.next_char();
                Ok(Token::Plus)
            }
            Some('-') => {
                self.next_char();
                Ok(Token::Minus)
            }
            Some('*') => {
                self.next_char();
                Ok(Token::Multiply)
            }
            Some('/') => {
                self.next_char();
                if let Some('/') = self.current_char {
                    return Ok(Token::FloorDivide)
                } 
                return Ok(Token::Divide)
            }
            Some('%') => {
                self.next_char();
                Ok(Token::Modulo)
            }
            Some('(') => {
                self.next_char();
                Ok(Token::LParen)
            }
            Some(')') => {
                self.next_char();
                Ok(Token::RParen)
            }
            _ => return Err(PseudoError::InvalidToken("Invalid operator".to_string())),
        }
    }
}