use std::str::Chars;

use super::token::Token;

pub struct Lexer<'a> {
    chars: Chars<'a>,
    current_char: Option<char>,
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

    fn next_char(&mut self) {
        self.current_char = self.chars.next();
    }

    /// tokenize inputs
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.current_char {
            match ch {
                '0'..='9' => tokens.push(self.number()),
                'a'..='z' | 'A'..='Z' => tokens.push(self.identifier()),
                '+' | '-' | '*' | '/' | '%' | '(' | ')' => tokens.push(self.operator()),
                '=' => tokens.push(self.equals()),
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

    /// catch numbers
    fn number(&mut self) -> Token {
        let mut number = 0;
        while let Some('0'..='9') = self.current_char {
            number = number * 10 + self.current_char.unwrap().to_digit(10).unwrap() as i64;
            self.next_char();
        }
        Token::Number(number)
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

    /// catch = and ==
    fn equals(&mut self) -> Token {
        self.next_char();
        if let Some('=') = self.current_char {
            self.next_char();
            Token::Equals
        } else {
            Token::Assign
        }
    }

    /// catch operators
    fn operator(&mut self) -> Token {
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
                Token::Divide
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

    // catch variables
    fn identifier(&mut self) -> Token {
        let mut id = String::new();

        while let Some(ch) = self.current_char {
            match ch {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    id.push(ch);
                    self.next_char();
                }
                _ => break,
            }
        }
        match id.to_lowercase().as_str() {
            "print" | "output" | "display" => Token::Print,
            _ => Token::Ident(id),
        }
    }
}
