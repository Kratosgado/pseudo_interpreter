use std::str::Chars;

use super::token::Token;

pub struct Lexer<'T> {
    chars: Chars<'T>,
    current_char: Option<char>,
}

impl<'T> Lexer<'T> {
    pub fn new(input: &'T str) -> Self {
        let mut lexer = Lexer {
            chars: input.chars(),
            current_char: None,
        };
        lexer.next_char(); // Initialize the first character
        lexer
    }

    fn next_char(&mut self) {
        self.current_char = self.chars.next();
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.current_char {
            match ch {
                '0'..='9' => tokens.push(self.number()),
                
                '+' => {
                    tokens.push(Token::Plus);
                    self.next_char();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.next_char();
                }
                '*' => {
                    tokens.push(Token::Multiply);
                    self.next_char();
                }
                '/' => {
                    tokens.push(Token::Divide);
                    self.next_char();
                }
                '(' => {
                    tokens.push(Token::LParen);
                    self.next_char();
                }
                ')' => {
                    tokens.push(Token::RParen);
                    self.next_char();
                }
                ' ' | '\t' | '\n' | '\r' => {
                    self.next_char();
                }
                _ => panic!("Invalid character: {}", ch),
            }
        }
        tokens
    }

    fn number(&mut self) -> Token {
        let mut number = 0;
        while let Some('0'..='9') = self.current_char {
            number = number * 10 + self.current_char.unwrap().to_digit(10).unwrap() as i64;
            self.next_char();
        }
        Token::Number(number)
    }
}
