use crate::lexer::lexer::Lexer;
use super::super::Token;

pub trait Operator {
    fn encode_operator(&mut self)-> Token;
}

impl <'a> Operator for Lexer<'a> {
    fn encode_operator(&mut self)-> Token {
        match self.current_char {
            Some('+') => {
                self.next_char();
                Token::Plus
            }
            Some('-') => {
                self.next_char();
                Token::Minus
            }
            Some('*') => {
                self.next_char();
                Token::Multiply
            }
            Some('/') => {
                self.next_char();
                if let Some('/') = self.current_char {
                    return Token::FloorDivide
                } 
                return Token::Divide
            }
            Some('%') => {
                self.next_char();
                Token::Modulo
            }
            Some('(') => {
                self.next_char();
                Token::LParen
            }
            Some(')') => {
                self.next_char();
                Token::RParen
            }
            _ => panic!("Invalid operator"),
        }
    }
}