use crate::lexer::lexer::Lexer;
use super::super::Token;


/// catch operators
pub fn operator(lexer: &mut Lexer) -> Token {
    match lexer.current_char {
        Some('+') => {
            lexer.next_char();
            Token::Plus
        }
        Some('-') => {
            lexer.next_char();
            Token::Minus
        }
        Some('*') => {
            lexer.next_char();
            Token::Multiply
        }
        Some('/') => {
            lexer.next_char();
            Token::Divide
        }
        Some('%') => {
            lexer.next_char();
            Token::Modulo
        }
        Some('(') => {
            lexer.next_char();
            Token::LParen
        }
        Some(')') => {
            lexer.next_char();
            Token::RParen
        }
        _ => panic!("Invalid operator"),
    }
}
