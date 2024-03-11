
use crate::lexer::lexer::Lexer;
use super::super::Token;
/// catch comparisons
/// catch =, ==, <, >, <=, >=, !=
/// catch &&, ||
/// catch !
pub fn comparison(lexer: &mut Lexer) -> Token {
    match lexer.current_char {
        Some('<') => {
            lexer.next_char();
            if let Some('=') = lexer.current_char {
                lexer.next_char();
                Token::LessThanEqual
            } else {
                Token::LessThan
            }
        }
        Some('>') => {
            lexer.next_char();
            if let Some('=') = lexer.current_char {
                lexer.next_char();
                Token::GreaterThanEqual
            } else {
                Token::GreaterThan
            }
        }
        Some('!') => {
            lexer.next_char();
            if let Some('=') = lexer.current_char {
                lexer.next_char();
                Token::NotEqual
            } else {
                Token::Not
            }
        }
        Some('=') => {
            lexer.next_char();
            if let Some('=') = lexer.current_char {
                lexer.next_char();
                Token::Equal
            } else {
                Token::Assign
            }
        }
        Some('&') => {
            lexer.next_char();
            if let Some('&') = lexer.current_char {
                lexer.next_char();
                Token::And
            } else {
                panic!("Invalid character: &");
            }
        }
        Some('|') => {
            lexer.next_char();
            if let Some('|') = lexer.current_char {
                lexer.next_char();
                Token::Or
            } else {
                panic!("Invalid character: |");
            }
        }
        _ => panic!("Invalid character"),
    }
}