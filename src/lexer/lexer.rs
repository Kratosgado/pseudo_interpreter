use std::str::Chars;

use super::{Operator,Identifier, Comparison, Datatype, Token};


pub struct Lexer<'a> {
    chars: Chars<'a>,
    pub current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            chars: input.chars(),
            current_char: None,
        };
        lexer.next_char(); // Initialize the first character
        lexer
    }

    pub fn next_char(&mut self) {
        self.current_char = self.chars.next();
    }

    /// tokenize inputs
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.current_char {
            match ch {
                '0'..='9' => tokens.push(self.number()),
                'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.encode_identifier()),
                '+' | '-' | '*' | '/' | '%' | '(' | ')' => tokens.push(self.encode_operator()),
                '<' | '>' | '!' | '=' | '&' | '|' => tokens.push(self.encode_comparison()),
                ',' => {
                    self.next_char();
                    tokens.push(Token::Comma);
                },
                '[' => {
                    self.next_char();
                    tokens.push(Token::LBracket);
                },
                ']' => {
                    self.next_char();
                    tokens.push(Token::RBracket);
                },
                '\n' | ';' => {
                    tokens.push(Token::EOL);
                    self.next_char();
                }
                ' ' | '\t' | '\r' => self.next_char(),
                '"' => tokens.push(self.string()),
                _ => panic!("Invalid character: {}", ch),
            }
        }
        tokens
    }
}
