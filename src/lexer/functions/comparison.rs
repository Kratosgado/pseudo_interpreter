use super::super::Token;
use crate::lexer::lexer::Lexer;

pub trait Comparison {
    /// catch comparisons
    /// catch =, ==, <, >, <=, >=, !=
    /// catch &&, ||
    /// catch !
    fn encode_comparison(&mut self) -> Token;
}

impl <'a> Comparison for Lexer<'a> {
    fn encode_comparison(&mut self) -> Token {
        match self.current_char {
            Some('<') => {
                self.next_char();
                if let Some('=') = self.current_char {
                    self.next_char();
                    Token::LessThanEqual
                } else {
                    Token::LessThan
                }
            }
            Some('>') => {
                self.next_char();
                if let Some('=') = self.current_char {
                    self.next_char();
                    Token::GreaterThanEqual
                } else {
                    Token::GreaterThan
                }
            }
            Some('!') => {
                self.next_char();
                if let Some('=') = self.current_char {
                    self.next_char();
                    Token::NotEqual
                } else {
                    Token::Not
                }
            }
            Some('=') => {
                self.next_char();
                if let Some('=') = self.current_char {
                    self.next_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            Some('&') => {
                self.next_char();
                if let Some('&') = self.current_char {
                    self.next_char();
                    Token::And
                } else {
                    panic!("Invalid character: &");
                }
            }
            Some('|') => {
                self.next_char();
                if let Some('|') = self.current_char {
                    self.next_char();
                    Token::Or
                } else {
                    panic!("Invalid character: |");
                }
            }
            _ => panic!("Invalid character"),
        }
    }
}