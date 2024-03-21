use super::super::Token;
use crate::{constants::error_handler::PseudoError, lexer::lexer::Lexer};

pub trait Comparison {
    /// catch comparisons
    /// catch =, ==, <, >, <=, >=, !=
    /// catch &&, ||
    /// catch !
    fn encode_comparison(&mut self) -> Result<Token, PseudoError>;
}

impl <'a> Comparison for Lexer<'a> {
    fn encode_comparison(&mut self) -> Result<Token, PseudoError> {
        match self.current_char {
            Some('<') => {
                self.next_char();
                if let Some('=') = self.current_char {
                    self.next_char();
                    Ok(Token::LessThanEqual)
                } else {
                    Ok(Token::LessThan)
                }
            }
            Some('>') => {
                self.next_char();
                if let Some('=') = self.current_char {
                    self.next_char();
                    Ok(Token::GreaterThanEqual)
                } else {
                    Ok(Token::GreaterThan)
                }
            }
            Some('!') => {
                self.next_char();
                if let Some('=') = self.current_char {
                    self.next_char();
                    Ok(Token::NotEqual)
                } else {
                    Ok(Token::Not)
                }
            }
            Some('=') => {
                self.next_char();
                if let Some('=') = self.current_char {
                    self.next_char();
                    Ok(Token::Equal)
                } else {
                    Ok(Token::Assign)
                }
            }
            Some('&') => {
                self.next_char();
                if let Some('&') = self.current_char {
                    self.next_char();
                    Ok(Token::And)
                } else {
                    return Err(PseudoError::InvalidToken("Invalid character: &".to_string()));
                }
            }
            Some('|') => {
                self.next_char();
                if let Some('|') = self.current_char {
                    self.next_char();
                    Ok(Token::Or)
                } else {
                    return Err(PseudoError::InvalidToken("Invalid character: |".to_string()));
                }
            }
            _ => return Err(PseudoError::InvalidToken("Invalid character:".to_string())),
        }
    }
}