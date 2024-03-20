use crate::lexer::lexer::Lexer;
use super::super::Token;


pub trait Datatype{
    /// catch numbers
     fn number( &mut self) -> Token ;
    /// catch strings
  fn string( &mut self) -> Token ;
}

impl<'a> Datatype for Lexer<'a> {
    fn number(&mut self) -> Token {
        let mut number = String::new();
        while let Some(ch) = self.current_char {
            match ch {
                '0'..='9' | '.' => {
                    number.push(ch);
                    self.next_char();
                },
                _ => break,
            }
        }
        if number.contains('.') {
            Token::Float(number.parse().unwrap())
        } else {
            Token::Number(number.parse().unwrap())
        }
    }
    /// catch strings
  fn string(&mut self) -> Token {
        let mut string = String::new();
        self.next_char();
        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.next_char();
                break;
            }
            string.push(ch);
            self.next_char();
        }
        Token::Str(string)
    }
}